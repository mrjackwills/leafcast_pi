use crate::{
    C,
    app_env::{AppEnv, EnvTimeZone},
};
use image::imageops::FilterType;
use std::{
    io::Cursor,
    time::{Instant, SystemTime},
};
use time::OffsetDateTime;
use tokio::{fs, process::Command};
use tracing::{debug, error, info};

#[derive(Clone, Copy, Debug)]
struct FileSize {
    original: usize,
    converted: usize,
}

impl FileSize {
    const fn default() -> Self {
        Self {
            original: 0,
            converted: 0,
        }
    }
}

struct WH {
    width: u32,
    height: u32,
}

impl WH {
    const fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }
}

enum Dimension {
    Big,
    Small,
}

impl Dimension {
    const fn get_dimensions(&self) -> WH {
        match self {
            Self::Big => WH::new(3280, 2464),
            Self::Small => WH::new(600, 450),
        }
    }
}

#[derive(Debug)]
pub struct Camera {
    image_webp: Vec<u8>,
    image_timestamp: SystemTime,
    file_size: FileSize,
    rotation: String,
    timezone: EnvTimeZone,
    location_images: String,
}

impl Camera {
    /// Setup camera, take a photo immediately in order to fill values instead of using Option<T>
    pub async fn init(app_envs: &AppEnv) -> Self {
        let mut camera = Self {
            image_webp: vec![],
            image_timestamp: SystemTime::now(),
            file_size: FileSize::default(),
            rotation: app_envs.rotation.to_string(),
            timezone: C!(app_envs.timezone),
            location_images: C!(app_envs.location_images),
        };
        let photo_buffer = camera.photograph().await;
        camera.convert_to_webp(&photo_buffer).await;
        camera
    }

    // Need to properly handle errors here!
    // return result?
    async fn photograph(&mut self) -> Vec<u8> {
        info!("Taking photo");
        let dimensions = Dimension::Big.get_dimensions();
        self.image_timestamp = SystemTime::now();
        let buffer = Command::new("libcamera-still")
            .args([
                "-q",
                "95",
                "-n",
                "--rotation",
                &self.rotation.to_string(),
                "--width",
                &dimensions.width.to_string(),
                "--height",
                &dimensions.height.to_string(),
                "--immediate",
                "-o",
                "-",
            ])
            .output()
            .await
            .map_or_else(
                |e| {
                    error!("{:?}", e);
                    vec![]
                },
                |o| o.stdout,
            );
        info!("Photo taken");
        self.file_size.original = buffer.len();
        buffer
    }

    // /// Take a small
    // pub async fn force_update(&mut self) {
    //     let buffer = Self::take_photo(self).await;
    //     Self::convert_to_webp(self, &buffer).await;
    // }

    /// Convert a given u8 slice to a webp, update self info
    /// Will execute the conversion on a spawn blocking tokio thread
    async fn convert_to_webp(&mut self, buffer: &[u8]) {
        let now = Instant::now();
        if let Ok(mut image) = image::load_from_memory_with_format(buffer, image::ImageFormat::Jpeg)
        {
            if let Ok(webp) = tokio::task::spawn_blocking(move || {
                let dimensions = Dimension::Small.get_dimensions();
                image = image.resize(dimensions.width, dimensions.height, FilterType::Nearest);
                debug!("resize took: {}ms", now.elapsed().as_millis());
                let mut buf_writer = Cursor::new(Vec::new());
                let now = Instant::now();
                if let Err(e) = image::write_buffer_with_format(
                    &mut buf_writer,
                    image.as_bytes(),
                    image.width(),
                    image.height(),
                    image.color(),
                    image::ImageFormat::WebP,
                ) {
                    error!("{:?}", e);
                    error!("image::write_buffer error");
                }
                debug!("conversion took: {}ms", now.elapsed().as_millis());
                buf_writer.into_inner()
            })
            .await
            {
                self.file_size.converted = webp.len();
                self.image_webp = webp;
            }
        }
    }

    /// Take a photo, and update self.web with that new photo
    pub async fn take_photo(&mut self) -> Vec<u8> {
        let photo_buffer = Self::photograph(self).await;
        Self::convert_to_webp(self, &photo_buffer).await;
        photo_buffer
    }

    // store the image as web64 instead of &[u8]
    pub fn get_webp(&self) -> &[u8] {
        &self.image_webp
    }

    /// Return the timestamp of the latest image
    pub const fn get_timestamp(&self) -> SystemTime {
        self.image_timestamp
    }

    /// Return converted filesize in bytes
    pub const fn get_size_converted(&self) -> usize {
        self.file_size.converted
    }

    /// Return original filesize in bytes
    pub const fn get_size_original(&self) -> usize {
        self.file_size.original
    }

    /// get the filesize of the original and onverted image
    pub const fn get_sizes(&self) -> (usize, usize) {
        (self.get_size_converted(), self.get_size_original())
    }

    /// Save the photo to disk
    pub async fn save_to_disk(&self, photo: Vec<u8>) {
        let date_time =
            OffsetDateTime::from(self.get_timestamp()).to_offset(self.timezone.get_offset());
        let file_name = format!(
            "{}_{:0>2}-{:0>2}-{:0>2}",
            date_time.date(),
            date_time.hour(),
            date_time.minute(),
            date_time.second()
        );
        if self.get_size_converted() > 10000 {
            let file_name = format!("{}/{file_name}.jpg", self.location_images);
            if let Err(e) = fs::write(file_name, photo).await {
                error!(%e);
                error!("Error saving to disk");
            }
        }
    }
}

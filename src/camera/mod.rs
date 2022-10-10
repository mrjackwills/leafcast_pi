use futures_util::Future;
use image::imageops::FilterType;
use std::{
    pin::Pin,
    sync::atomic::{AtomicBool, Ordering},
    time::{Instant, SystemTime},
};
use time::{OffsetDateTime, UtcOffset};
use tokio::{fs, process::Command};
use tracing::{debug, error, trace};
use webp::Encoder;

use crate::env::AppEnv;

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
    in_use: AtomicBool,
    image_webp: Vec<u8>,
    image_timestamp: SystemTime,
    file_size: FileSize,
    rotation: String,
    retry_count: u8,
    utc_offset: UtcOffset,
    location_images: String,
}

impl Camera {
    /// Setup camera, take a photo immediately in order to fill values instead of using Option<T>
    pub async fn init(app_envs: &AppEnv) -> Self {
        let mut camera = Self {
            in_use: AtomicBool::new(false),
            image_webp: vec![],
            image_timestamp: SystemTime::now(),
            file_size: FileSize::default(),
            rotation: app_envs.rotation.to_string(),
            retry_count: 0,
            utc_offset: app_envs.utc_offset,
            location_images: app_envs.location_images.clone(),
        };
        let photo_buffer = camera.photograph().await;
        camera.convert_to_webp(&photo_buffer);
        camera
    }

    // Need to properly handle errors here!
    // return result?
    fn photograph<'a>(&'a mut self) -> Pin<Box<dyn Future<Output = Vec<u8>> + 'a + Send>> {
        Box::pin(async move {
            // issue with this?
            // maybe include a counter so that it will only try 10 times?
            if self.in_use.load(Ordering::SeqCst) {
                tokio::time::sleep(std::time::Duration::from_millis(100)).await;
                return Self::photograph(self).await;
            }
            debug!("Taking photo");
            self.in_use.store(true, Ordering::SeqCst);
            let dimensions = Dimension::Big.get_dimensions();
            self.image_timestamp = SystemTime::now();
            let buffer = Command::new("libcamera-still")
                .args([
                    "-q",
                    "95",
                    "-n",
                    "--rotation",
                    &self.rotation.clone(),
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
            debug!("Photo taken");
            self.file_size.original = buffer.len();
            self.in_use.store(false, Ordering::SeqCst);
            self.retry_count = 0;
            buffer
        })
    }

    // /// Take a small
    // pub async fn force_update(&mut self) {
    //     let buffer = Self::take_photo(self).await;
    //     Self::convert_to_webp(self, &buffer).await;
    // }

    /// Convert a given u8 slice to a webp, update self info
    fn convert_to_webp(&mut self, buffer: &[u8]) {
        let now = Instant::now();

        match image::load_from_memory_with_format(buffer, image::ImageFormat::Jpeg) {
            Ok(mut image) => {
                let dimensions = Dimension::Small.get_dimensions();
                image = image.resize(dimensions.width, dimensions.height, FilterType::Nearest);
                trace!("resize took: {}ms", now.elapsed().as_millis());
                match Encoder::from_image(&image) {
                    Ok(encoder) => {
                        let photo_webp = encoder.encode(85f32).to_vec();
                        let encode_time = format!("webp encode: {}ms", now.elapsed().as_millis());
                        trace!(%encode_time);
                        self.file_size.converted = photo_webp.len();
                        self.image_webp = photo_webp;
                    }
                    Err(e) => {
                        error!(%e);
                        error!("webp encoder error");
                    }
                };
            }
            Err(e) => {
                error!("{:?}", e);
            }
        }
    }

    /// Take a photo, and update self.web with that new photo
    pub async fn take_photo(&mut self) -> Vec<u8> {
        let photo_buffer = Self::photograph(self).await;
        Self::convert_to_webp(self, &photo_buffer);
        photo_buffer
    }

    pub fn get_webp(&self) -> &[u8] {
        &self.image_webp
    }

    pub const fn get_timestamp(&self) -> SystemTime {
        self.image_timestamp
    }

    pub const fn get_size_converted(&self) -> usize {
        self.file_size.converted
    }

    pub const fn get_size_original(&self) -> usize {
        self.file_size.original
    }

    /// get the filesize of the original and onverted image
    pub const fn get_sizes(&self) -> (usize, usize) {
        (self.get_size_converted(), self.get_size_original())
    }

    /// Save the photo to disk
    pub async fn save_to_disk(&mut self, photo: Vec<u8>) {
        let date_time = OffsetDateTime::from(self.get_timestamp()).to_offset(self.utc_offset);
        let file_name = format!(
            "{}_{:0>2}-{:0>2}-{:0>2}",
            date_time.date(),
            date_time.hour(),
            date_time.minute(),
            date_time.second()
        );
        if self.get_size_converted() > 15000 {
            let file_name = format!("{}/{}.jpg", self.location_images, file_name);
            if let Err(e) = fs::write(file_name, photo).await {
                error!(%e);
                error!("Error saving to disk");
            }
        }
    }
}

use base64::{engine, Engine};
use futures_util::lock::Mutex;
use futures_util::SinkExt;
use std::process;
use std::sync::Arc;
use std::time::Instant;
use time::OffsetDateTime;
use tracing::{error, trace};

use tokio::sync::Mutex as TokioMutex;

use crate::camera::Camera;
use crate::sysinfo::SysInfo;
use crate::{
    env::AppEnv,
    ws_messages::{to_struct, MessageValues, ParsedMessage, Photo, Response, StructuredResponse},
};

use super::WSWriter;

#[derive(Debug, Clone)]
pub struct WSSender {
    app_envs: AppEnv,
    camera: Arc<TokioMutex<Camera>>,
    connected_instant: Instant,
    writer: Arc<Mutex<WSWriter>>,
}

impl WSSender {
    pub fn new(
        app_envs: AppEnv,
        camera: Arc<TokioMutex<Camera>>,
        connected_instant: Instant,
        writer: Arc<Mutex<WSWriter>>,
    ) -> Self {
        Self {
            app_envs,
            camera,
            connected_instant,
            writer,
        }
    }

    /// Handle text message, in this program they will all be json text
    pub async fn on_text(&mut self, message: String) {
        if let Some(data) = to_struct(&message) {
            match data {
                MessageValues::Invalid(error) => error!("{:?}", error),
                MessageValues::Valid(data, unique) => match data {
                    ParsedMessage::ForceUpdate => {
                        self.camera.lock().await.take_photo().await;
                        let response = self
                            .generate_response(self.camera.lock().await.get_webp().to_owned())
                            .await;
                        self.send_ws_response(response, unique, Some(true)).await;
                    }
                    ParsedMessage::Photo => {
                        let webp = self.camera.lock().await.get_webp().to_owned();
                        let response = self.generate_response(webp).await;
                        self.send_ws_response(response, unique, Some(true)).await;
                    }
                },
            }
        }
    }

    /// Create a photo response, is the only response this app sends (other than pongs)
    async fn generate_response(&self, photo_buffer: Vec<u8>) -> Response {
        let camera = self.camera.lock().await;
        let date_time = OffsetDateTime::from(camera.get_timestamp())
            .to_offset(self.app_envs.timezone.get_offset());
        let (size_converted, size_original) = camera.get_sizes();
        drop(camera);
        let connected_at = self.connected_instant;
        let timestamp = format!(
            "{} {} @ {:0>2}:{:0>2}:{:0>2}",
            date_time.weekday(),
            date_time.date(),
            date_time.hour(),
            date_time.minute(),
            date_time.second()
        );
        let pi_info = SysInfo::new(&self.app_envs, connected_at).await;
        Response::Photo(Photo {
            image: format!(
                "data:image/webp;base64,{}",
                engine::general_purpose::STANDARD.encode(photo_buffer)
            ),
            pi_info,
            timestamp,
            size_converted,
            size_original,
        })
    }

    #[allow(unused)]
    /// restart application by force quitting, assuming running as service or in an auto-restart container
    async fn restart(&mut self) {
        self.close().await;
        process::exit(0);
    }

    /// Send a message to the socket
    async fn send_ws_response(&mut self, response: Response, unique: String, cache: Option<bool>) {
        match self
            .writer
            .lock()
            .await
            .send(StructuredResponse::data(response, unique, cache))
            .await
        {
            Ok(_) => trace!("Message sent"),
            Err(e) => {
                error!("send_ws_response::SEND-ERROR::{:?}", e);
                process::exit(1);
            }
        }
    }

    /// close connection, uses a 2 second timeout
    pub async fn close(&mut self) {
        if let Ok(close) = tokio::time::timeout(
            std::time::Duration::from_secs(2),
            self.writer.lock().await.close(),
        )
        .await
        {
            close.ok();
        }
    }
}

use jiff::civil::DateTime;
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::camera::Camera;

pub struct Croner;

impl Croner {
    /// create a looper and spawn into it's own async thread
    pub fn init(camera: Arc<Mutex<Camera>>) {
        let looper = Self;
        tokio::spawn(async move { looper.init_loop(camera).await });
    }

    fn now_utc() -> DateTime {
        jiff::tz::TimeZone::UTC.to_datetime(jiff::Timestamp::now())
    }

    /// On first loop, need to work out current second, and wait until it's 0, and then start the loop and wait 5 minutes
    /// loop every 60 second,check if its 10am UTC, and send internal file request message, which, if connected to ws, will send a ws message
    async fn init_loop(&self, camera: Arc<Mutex<Camera>>) {
        // wait til now.second is 0
        let wait_for = || {
            std::time::Duration::from_secs(
                60 - u64::try_from(Self::now_utc().second()).unwrap_or_default(),
            )
        };

        tokio::time::sleep(wait_for()).await;
        loop {
            if Self::now_utc().minute() % 5 == 0 {
                let photo = camera.lock().await.take_photo().await;
                camera.lock().await.save_to_disk(photo).await;
            }
            // Should I not just wait 60 seconds here?
            tokio::time::sleep(wait_for()).await;
        }
    }
}

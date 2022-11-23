use std::sync::Arc;
use time::OffsetDateTime;
use tokio::sync::Mutex;

use crate::camera::Camera;

pub struct Croner;

impl Croner {
    /// create a looper and spawn into it's own async thread
    pub async fn init(camera: Arc<Mutex<Camera>>) {
        let looper = Self;
        tokio::spawn(async move { looper.init_loop(camera).await });
    }

    /// On first loop, need to work out current second, and wait until it's 0, and then start the loop and wait 5 minutes
    /// loop every 60 second,check if its 10am UTC, and send internal file request message, which, if connected to ws, will send a ws message
    async fn init_loop(&self, camera: Arc<Mutex<Camera>>) {
        // wait til now.second is 0
        let wait_for =
            || std::time::Duration::from_secs(60 - u64::from(OffsetDateTime::now_utc().second()));

        tokio::time::sleep(wait_for()).await;
        loop {
            let now = OffsetDateTime::now_utc();
            if now.minute() % 5 == 0 {
                let photo = camera.lock().await.take_photo().await;
                camera.lock().await.save_to_disk(photo).await;
            }
            // Should I not just wait 60 seconds here?
            tokio::time::sleep(wait_for()).await;
        }
    }
}

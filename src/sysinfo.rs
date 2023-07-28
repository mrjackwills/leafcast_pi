use std::time::Instant;

use serde::{Deserialize, Serialize};
use tokio::fs::read_to_string;

use crate::{app_error::AppError, app_env::AppEnv};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SysInfo {
    pub uptime: usize,
    pub version: String,
    pub internal_ip: String,
    pub app_uptime: u64,
    pub websocket_uptime: u64,
    pub total_file_size: u64,
    pub number_images: u32,
}

impl SysInfo {
    async fn get_file_info(app_envs: &AppEnv) -> Result<(u64, u32), AppError> {
        let mut total_file_size = 0;
        let mut count = 0;

        let mut entry = tokio::fs::read_dir(&app_envs.location_images).await?;

        while let Some(file) = entry.next_entry().await? {
            if std::path::Path::new(&file.path())
                .extension()
                .map_or(false, |ext| ext.eq_ignore_ascii_case("jpg"))
            {
                {
                    total_file_size += file.metadata().await?.len();
                    count += 1;
                }
            }
        }
        Ok((total_file_size, count))
    }

    async fn get_ip(app_envs: &AppEnv) -> String {
        let na = String::from("N/A");
        let ip = read_to_string(&app_envs.location_ip_address)
            .await
            .unwrap_or_else(|_| na.clone());
        let output = if ip.len() > 1 {
            ip.trim().to_owned()
        } else {
            na
        };
        output
    }

    async fn get_uptime() -> usize {
        let uptime = read_to_string("/proc/uptime").await.unwrap_or_default();
        let (uptime, _) = uptime.split_once('.').unwrap_or_default();
        uptime.parse::<usize>().unwrap_or_default()
    }

    pub async fn new(app_envs: &AppEnv, connected_at: Instant) -> Self {
        let file_info = Self::get_file_info(app_envs).await.unwrap_or((0, 0));

        Self {
            total_file_size: file_info.0,
            number_images: file_info.1,
            internal_ip: Self::get_ip(app_envs).await,
            uptime: Self::get_uptime().await,
            websocket_uptime: connected_at.elapsed().as_secs(),
            app_uptime: std::time::SystemTime::now()
                .duration_since(app_envs.start_time)
                .map_or(0, |value| value.as_secs()),
            version: env!("CARGO_PKG_VERSION").into(),
        }
    }
}

// SysInfo tests
//
/// cargo watch -q -c -w src/ -x 'test sysinfo -- --test-threads=1 --nocapture'
#[cfg(test)]
#[allow(clippy::unwrap_used)]
mod tests {
    use std::time::SystemTime;

    use crate::app_env::{EnvTimeZone, Rotation};

    use super::*;

    fn gen_app_env(location_ip_address: String) -> AppEnv {
        let na = String::from("na");
        AppEnv {
            location_images: String::from("photos"),
            location_ip_address,
            location_log: na.clone(),
            log_level: tracing::Level::INFO,
            rotation: Rotation::Zero,
            start_time: SystemTime::now(),
            timezone: EnvTimeZone::new("America/New_York"),
            ws_address: na.clone(),
            ws_apikey: na.clone(),
            ws_password: na.clone(),
            ws_token_address: na,
        }
    }

    #[tokio::test]
    async fn sysinfo_getuptime_ok() {
        // FIXTURES
        gen_app_env(String::from("ip.addr"));

        // ACTIONS
        let result = SysInfo::get_uptime().await;

        // CHECK
        // Assumes ones computer has been turned on for one minute
        assert!(result > 60);
    }

    #[tokio::test]
    async fn sysinfo_get_ip_na() {
        // FIXTURES
        let app_envs = gen_app_env(String::from("na"));

        // ACTIONS
        let result = SysInfo::get_ip(&app_envs).await;

        // CHECK
        assert_eq!(result, "N/A");
    }

    #[tokio::test]
    async fn sysinfo_get_ip_ok() {
        // FIXTURES
        let app_envs = gen_app_env(String::from("ip.addr"));
        // ACTIONS
        let result = SysInfo::get_ip(&app_envs).await;

        // CHECK
        assert_eq!(result, "127.0.0.1");
    }

    #[tokio::test]
    async fn sysinfo_get_sysinfo_ok() {
        // FIXTURES
        let app_envs = gen_app_env(String::from("ip.addr"));
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;

        let now = Instant::now();

        // ACTIONS
        let result = SysInfo::new(&app_envs, now).await;

        // CHECK
        assert_eq!(result.internal_ip, "127.0.0.1");
        assert_eq!(result.version, env!("CARGO_PKG_VERSION"));
        assert_eq!(result.app_uptime, 1);
        // TODO need to check pi_time with regex?
        // Again assume ones computer has been turned on for one minute
        assert!(result.uptime > 60);
    }
}

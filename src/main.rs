// Only allow when debugging
// #![allow(unused)]

mod app_env;
mod app_error;
mod camera;
mod cron;
mod parse_cli;
mod sysinfo;
mod systemd;
mod word_art;
mod ws;
mod ws_messages;

use app_env::AppEnv;
use app_error::AppError;
use camera::Camera;
use cron::Croner;
use parse_cli::CliArgs;
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing_subscriber::{fmt, prelude::__tracing_subscriber_SubscriberExt};
use word_art::Intro;
use ws::open_connection;

const LOGS_NAME: &str = "leafcast.log";

fn setup_tracing(app_env: &AppEnv) -> Result<(), AppError> {
    let logfile = tracing_appender::rolling::never(&app_env.location_log, LOGS_NAME);

    let log_fmt = fmt::Layer::default()
        .json()
        .flatten_event(true)
        .with_writer(logfile);

    match tracing::subscriber::set_global_default(
        fmt::Subscriber::builder()
            .with_file(true)
            .with_line_number(true)
            .with_max_level(app_env.log_level)
            .finish()
            .with(log_fmt),
    ) {
        Ok(()) => Ok(()),
        Err(e) => {
            println!("{e:?}");
            Err(AppError::Tracing)
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), AppError> {
    let app_envs = AppEnv::get();
    let cli = CliArgs::new();
    setup_tracing(&app_envs)?;
    systemd::check(&cli, &app_envs);
    Intro::new(&app_envs).show();
    let camera = Arc::new(Mutex::new(Camera::init(&app_envs).await));
    Croner::init(Arc::clone(&camera));
    open_connection(app_envs, camera).await;
    Ok(())
}

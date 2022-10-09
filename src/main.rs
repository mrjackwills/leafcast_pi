#![forbid(unsafe_code)]
#![warn(clippy::unused_async, clippy::unwrap_used, clippy::expect_used)]
// Wanring - These are indeed pedantic
// #![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![allow(clippy::module_name_repetitions, clippy::doc_markdown)]
// Only allow when debugging
#![allow(unused)]

mod app_error;
mod camera;
mod cron;
mod env;
mod parse_cli;
mod sysinfo;
mod systemd;
mod word_art;
mod ws;
mod ws_messages;

use camera::Camera;
use cron::Croner;
use env::AppEnv;
use parse_cli::CliArgs;
use tracing::Level;
use word_art::Intro;
use ws::open_connection;

use std::sync::Arc;
use tokio::sync::Mutex;

fn setup_tracing(app_envs: &AppEnv) {
    let level = if app_envs.trace {
        Level::TRACE
    } else if app_envs.debug {
        Level::DEBUG
    } else {
        Level::INFO
    };
    tracing_subscriber::fmt().with_max_level(level).init();
}

#[tokio::main]
async fn main() {
    let app_envs = AppEnv::get();
    let cli = CliArgs::new();
    setup_tracing(&app_envs);
    systemd::check(&cli);
    Intro::new(&app_envs).show();
    let camera = Arc::new(Mutex::new(Camera::init(&app_envs).await));
    Croner::init(Arc::clone(&camera)).await;
    open_connection(app_envs, camera).await;
}

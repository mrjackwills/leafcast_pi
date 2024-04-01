use crate::app_env::AppEnv;
use crate::parse_cli::CliArgs;
use crate::{app_error::AppError, LOGS_NAME};
use std::{env, fs, io::Write, path::Path, process::Command};
use tracing::{error, info};

const SYSTEMCTL: &str = "systemctl";
const APP_NAME: &str = env!("CARGO_PKG_NAME");
const CHOWN: &str = "chown";

enum Code {
    Valid,
    Invalid,
}
fn exit(message: &str, code: &Code) {
    match code {
        Code::Valid => {
            info!(message);
            std::process::exit(0);
        }
        Code::Invalid => {
            error!(message);
            std::process::exit(1);
        }
    }
}

fn check_sudo() {
    match sudo::check() {
        sudo::RunningAs::Root => (),
        _ => exit("not running as sudo", &Code::Invalid),
    }
}

/// Get user name, to check if is sudo
fn get_user_name() -> Option<String> {
    std::env::var("SUDO_USER").map_or(None, |user_name| {
        if user_name == "root" || user_name.is_empty() {
            None
        } else {
            Some(user_name)
        }
    })
}

/// Check if unit file in systemd, and delete if true
#[allow(clippy::cognitive_complexity)]
fn uninstall_service(app_envs: &AppEnv) -> Result<(), AppError> {
    if let Some(user_name) = get_user_name() {
        let service = get_service_name();

        let path = get_dot_service();

        if Path::new(&path).exists() {
            info!("Stopping service");
            Command::new(SYSTEMCTL).args(["stop", &service]).output()?;

            info!("Disabling service");
            Command::new(SYSTEMCTL)
                .args(["disable", &service])
                .output()?;

            info!("Removing service file");
            fs::remove_file(path)?;

            info!("Reload daemon-service");
            Command::new(SYSTEMCTL).arg("daemon-reload").output()?;

            info!("Change logs ownership");
            let logs_path = format!("{}/{}", app_envs.location_log, LOGS_NAME);
            Command::new(CHOWN)
                .args([&format!("{user_name}:{user_name}"), &logs_path])
                .output()?;
        }
    }
    Ok(())
}

/// Get service name for systemd service
fn get_service_name() -> String {
    format!("{APP_NAME}.service")
}

/// Get filename for systemd service file
fn get_dot_service() -> String {
    let service = get_service_name();
    format!("/etc/systemd/system/{service}")
}

/// Create a systemd service file, with correct details
fn create_service_file(user_name: &str) -> Result<String, AppError> {
    let current_dir = env::current_dir()?.display().to_string();
    Ok(format!(
        "[Unit]
Description={APP_NAME}
After=network-online.target
Wants=network-online.target
StartLimitIntervalSec=0

[Service]
ExecStart={current_dir}/{APP_NAME}
WorkingDirectory={current_dir}
SyslogIdentifier={APP_NAME}
User={user_name}
Group={user_name}
Restart=always
RestartSec=5

[Install]
WantedBy=multi-user.target
"
    ))
}
/// If is sudo, and able to get a user name (which isn't root), install leafcast as a service
#[allow(clippy::cognitive_complexity)]
fn install_service(app_envs: &AppEnv) -> Result<(), AppError> {
    if let Some(user_name) = get_user_name() {
        info!("Create service file");
        let mut file = fs::File::create(get_dot_service())?;

        info!("Write unit text to file");
        file.write_all(create_service_file(&user_name)?.as_bytes())?;

        info!("Reload systemctl daemon");
        Command::new(SYSTEMCTL).arg("daemon-reload").output()?;

        let service_name = get_service_name();
        info!("Enable service");
        Command::new(SYSTEMCTL)
            .args(["enable", &service_name])
            .output()?;

        info!("Change logs ownership");
        let logs_path = format!("{}/{}", app_envs.location_log, LOGS_NAME);
        Command::new(CHOWN)
            .args([&format!("{user_name}:{user_name}"), &logs_path])
            .output()?;

        info!("Start service");
        Command::new(SYSTEMCTL)
            .args(["start", &service_name])
            .output()?;
    } else {
        exit("invalid user", &Code::Invalid);
    }
    Ok(())
}

/// if cli argument provided, (un)install service in systemd
pub fn check(cli: &CliArgs, app_envs: &AppEnv) {
    if cli.install {
        check_sudo();
        uninstall_service(app_envs)
            .unwrap_or_else(|_| exit("uninstall old service failure", &Code::Invalid));
        install_service(app_envs).unwrap_or_else(|_| exit("install failure", &Code::Invalid));
        exit("Installed service", &Code::Valid);
    } else if cli.uninstall {
        check_sudo();
        uninstall_service(app_envs)
            .unwrap_or_else(|_| exit("uninstall old service failure", &Code::Invalid));
        exit("Uninstalled service", &Code::Valid);
    }
}

use crate::app_error::AppError;
use crate::parse_cli::CliArgs;
use std::{env, fs, io::Write, os::unix::fs::PermissionsExt, path::Path, process::Command};
use tracing::{error, info};

const SC: &str = "systemctl";
const APP_NAME: &str = env!("CARGO_PKG_NAME");

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
fn get_user_name() -> String {
    let mut user_name: Option<String> = None;
    for i in env::vars() {
        if i.0 == "SUDO_USER" {
            user_name = Some(i.1);
        }
    }
    if user_name.is_none() {
        exit("unable to get username", &Code::Invalid);
    }
    let user_name = user_name.unwrap_or_default();
    if user_name == "ROOT" || user_name.is_empty() {
        exit("invalid username", &Code::Invalid);
    }
    user_name
}

/// Check if unit file in systemd, and delete if true
fn uninstall_service() -> Result<(), AppError> {
    let service = get_service_name();

    let path = get_dot_service();
    if Path::new(&path).exists() {
        info!("Stopping service");
        Command::new(SC).args(["stop", &service]).output()?;

        info!("Disabling service");
        Command::new(SC).args(["disable", &service]).output()?;

        info!("Removing service file");
        fs::remove_file(path)?;

        info!("Reload daemon-service");
        Command::new(SC).arg("daemon-reload").output()?;
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
RestartSec=1

[Install]
WantedBy=multi-user.target
"))
}
/// If is sudo, and able to get a user name (which isn't root), install leafcast as a service
fn install_service() -> Result<(), AppError> {
    let user_name = get_user_name();

    let dot_service = create_service_file(&user_name);

    let path = get_dot_service();

    info!("Create service file");
    let mut file = fs::File::create(path)?;

    info!("Write unit text to file");
    file.write_all(dot_service?.as_bytes())?;

    info!("Set correct service file permissions");
    let metadata = file.metadata()?;
    let mut permissions = metadata.permissions();
    permissions.set_mode(0o644);

    info!("Reload systemctl daemon");
    Command::new(SC).arg("daemon-reload").output()?;

    let service_name = get_service_name();
    info!("Enable service");
    Command::new(SC).args(["enable", &service_name]).output()?;

    info!("Start service");
    Command::new(SC).args(["start", &service_name]).output()?;

    Ok(())
}

/// if cli argument provided, (un)install service in systemd
pub fn check(cli: &CliArgs) {
    if cli.install {
        check_sudo();
        uninstall_service()
            .unwrap_or_else(|_| exit("uninstall old service failure", &Code::Invalid));
        install_service().unwrap_or_else(|_| exit("install failure", &Code::Invalid));
        exit("Installed service", &Code::Valid);
    } else if cli.uninstall {
        check_sudo();
        uninstall_service()
            .unwrap_or_else(|_| exit("uninstall old service failure", &Code::Invalid));
        exit("Uninstalled service", &Code::Valid);
    }
}

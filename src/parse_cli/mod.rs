use clap::Parser;

#[derive(Parser, Debug, Clone)]
#[clap(about, version, author)]
pub struct CliArgs {
    /// Install as a systemd service, need to run as sudo
    #[clap(short = 'i', long = "install", conflicts_with = "uninstall")]
    pub install: bool,

    /// Uninstall the systemd service, need to run as sudo
    #[clap(short = 'u', long = "uninstall", conflicts_with = "install")]
    pub uninstall: bool,
}

impl CliArgs {
    /// Parse cli arguments
    pub fn new() -> Self {
        let args = CliArgs::parse();
        Self {
            install: args.install,
            uninstall: args.uninstall,
        }
    }
}

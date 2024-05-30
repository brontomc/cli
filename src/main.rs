mod macros;
mod versions;
mod install;
mod uninstall;

use clap::{Args, Parser, Subcommand};
use std::path::PathBuf;
use color_eyre::eyre::Result;
use dirs::home_dir;
use colored::Colorize;

use install::install;
use uninstall::uninstall;
use versions::get_latest_release;
use crate::versions::VersionV;

/// CLI structure to parse command-line arguments.
#[derive(Parser)]
#[command(
    version,
    about,
    long_about = None
)]
struct Cli {
    /// Commands.
    #[command(subcommand)]
    command: Commands,
    
}

/// Enum representing different commands.
#[derive(Subcommand)]
enum Commands {
    /// Install Amethyst.
    Install(InstallOptions),
    /// Uninstall Amethyst.
    Uninstall,
    // TODO: Implement These
    Start, 
    Stop,
    Restart,
    Status,
    Update,
    Versions,
    Config, // Example: amethyst config --port 9999
}

#[derive(Args)]
struct InstallOptions {
    /// Sets a custom installation path.
    #[arg(long, value_name = "PATH")]
    path: Option<PathBuf>,

    /// Specifies the version to install.
    #[arg(long, value_name = "VERSION")]
    version: Option<String>,
}

static GITHUB_TEXT: &str = "If you're unable to resolve the issue yourself, you can open an issue at https://github.com/amethyst-core/cli/issues for further assistance and troubleshooting.";

#[tokio::main]
async fn main() -> Result<()> {
    // Parse command-line arguments into the CLI structure.
    let cli = Cli::parse();

    // Match the command and execute corresponding logic.
    match &cli.command {
        Commands::Install(opts) => {

            let install_version: VersionV;

            if let Some(version) = &opts.version {
                install_version = VersionV::from_str(version)?;
            } else {
                install_version = get_latest_release().await?;
            };

            // Install Amethyst Core & Panel.
            let install_path: PathBuf;

            if let Some(path) = &opts.path {
                install_path = home_dir()
                    .expect("Could not find home directory")
                    .join(path)
                    .join(".amethyst");
            } else {
                install_path = home_dir()
                    .expect("Could not find home directory")
                    .join(".amethyst");
            }
            match install(&install_path, install_version) {
                Ok(_) => success!("Installation Completed."),
                Err(e) => {
                    error!("Failed to Install: {}\n", e);
                    info!("{}", GITHUB_TEXT);
                }
            };
        }
        Commands::Uninstall => {
            // Uninstall Amethyst Core & Panel.
            match uninstall() {
                Ok(_) => success!("Uninstalled."),
                Err(e) => {
                    error!("Failed to uninstall: {}\n", e);
                    info!("{}", GITHUB_TEXT);
                },
            };
        }
        _ => {}
    }

    Ok(())
}

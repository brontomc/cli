mod macros;
mod versions;
mod install;
mod uninstall;

use clap::{Args, Parser, Subcommand};
use std::path::PathBuf;
use color_eyre::eyre::Result;
use dirs::home_dir;

use install::install;
use uninstall::uninstall;
use versions::get_latest_release;

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

#[tokio::main]
async fn main() -> Result<()> {
    // Parse command-line arguments into the CLI structure.
    let cli = Cli::parse();

    // Match the command and execute corresponding logic.
    match &cli.command {
        Commands::Install(opts) => {

            if let Some(version) = &opts.version {
                println!("Installing version {}.", version);
            } else {
                let latest_release = get_latest_release().await?;
                println!("Installing latest version {}.", latest_release.0);
            };

            // Install Amethyst Core & Panel.
            if let Some(path) = &opts.path {
                let path = home_dir()
                    .expect("Could not find home directory")
                    .join(path)
                    .join(".amethyst");
                install(&path);
            } else {
                let path = home_dir()
                    .expect("Could not find home directory")
                    .join(".amethyst");
                install(&path);
            }
        }
        Commands::Uninstall => {
            // Uninstall Amethyst Core & Panel.
            uninstall()?;
        }
        _ => {}
    }

    Ok(())
}

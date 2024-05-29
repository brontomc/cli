mod macros;
// mod versions;
mod install;
mod uninstall;

use clap::{Parser, Subcommand};
use std::path::PathBuf;
use color_eyre::eyre::Result;
use dirs::home_dir;

use install::install;
use uninstall::uninstall;
// use versions::get_latest_release;

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
    
    /// Sets a custom installation path.
    #[arg(long, value_name = "PATH")]
    path: Option<PathBuf>,

    /// Flags.
    #[arg(short, long)]
    verbose: bool,
}

/// Enum representing different commands.
#[derive(Subcommand)]
enum Commands {
    /// Install Amethyst.
    Install,
    /// Uninstall Amethyst.
    Uninstall,
}

#[tokio::main]
async fn main() -> Result<()> {
    // Parse command-line arguments into the CLI structure.
    let cli = Cli::parse();

    // Print the specified path, if provided.
    println!("Specified Path: {:?}", cli.path.as_deref());

    // Print verbose mode status.
    // println!("Verbose: {:?}", cli.verbose);

    // Match the command and execute corresponding logic.
    match &cli.command {
        Commands::Install => {
            // Install Amethyst Core & Panel.
            let path = home_dir()
                .expect("Could not find home directory")
                .join(".amethyst");
            install(&path);
        }
        Commands::Uninstall => {
            // Uninstall Amethyst Core & Panel.
            uninstall()?;
        }
    }

    Ok(())
}

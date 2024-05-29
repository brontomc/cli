use std::fs;
use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use color_eyre::eyre::{Result, eyre};

#[derive(Debug, Deserialize, Serialize)]
struct Config {
    version: String,
    install_path: PathBuf,
    service_port: u16,
}

pub fn uninstall() -> Result<()> {
    // Get the path from the configuration file
    let path = get_config_install_path()?;

    // Delete the configuration file
    let config_file_path = dirs::home_dir()
        .ok_or_else(|| eyre!("Failed to determine home directory."))?
        .join("amethyst-config.json");

    if let Err(e) = fs::remove_file(&config_file_path) {
        println!("Error occurred while removing config file: {}", e);
    } else {
        println!("Config file removed successfully.");
    }

    // Attempt to remove the directory and all its contents recursively
    fs::remove_dir_all(&path)
        .map_err(|err| eyre!("Failed to uninstall directory: {}", err))?;

    println!("Uninstall successful.");
    Ok(())
}

// Function to get the install path from the configuration file
fn get_config_install_path() -> Result<PathBuf> {
    // Get the home directory
    let home_dir = dirs::home_dir().ok_or_else(|| eyre!("Failed to determine home directory."))?;

    // Construct the path to the configuration file
    let config_file_path = home_dir.join("amethyst-config.json");
    
    // Read the configuration file
    let config_content = fs::read_to_string(&config_file_path)
        .map_err(|err| eyre!("Failed to read configuration file: {}", err))?;

    // Deserialize the configuration
    let app_config: Config = serde_json::from_str(&config_content)
        .map_err(|err| eyre!("Failed to parse configuration file: {}", err))?;

    Ok(app_config.install_path)
}
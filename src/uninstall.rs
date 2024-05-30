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
    let path = match get_config_install_path() {
        Ok(p) => p,
        Err(e) => {
            return Err(eyre!("The program encountered an error while attempting to retrieve the path from the configuration file. Reason: {}", e));
        }
    };

    // Delete the configuration file
    let config_file_path = match dirs::home_dir() {
        Some(home) => home.join("amethyst-config.json"),
        None => {
            return Err(eyre!("The program encountered an error while attempting to locate the home directory."));
        }
    };

    if let Err(e) = fs::remove_file(&config_file_path) {
        return Err(eyre!("The program encountered an error while attempting to delete the configuration file. Reason: {}", e));
    }

    // Attempt to remove the directory and all its contents recursively
    match fs::remove_dir_all(&path) {
        Ok(_) => {},
        Err(e) => {
            return Err(eyre!("The program encountered an error while attempting to remove the directory and all its contents recursively. Reason: {}", e));
        }
    };

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
        .map_err(|err| eyre!(err))?;

    // Deserialize the configuration
    let app_config: Config = serde_json::from_str(&config_content)
        .map_err(|err| eyre!(err))?;

    Ok(app_config.install_path)
}
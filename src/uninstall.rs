use std::fs;
use std::path::PathBuf;
use serde::{Deserialize, Serialize};
use color_eyre::eyre::{Result, eyre};

// Define a struct to deserialize the configuration file
#[derive(Debug, Deserialize, Serialize)]
struct Config {
    version: String,
    install_path: String,
}

pub fn uninstall() -> Result<()> {
    
    // Get the path from the configuration file
    let path = match get_config_path() {
        Ok(path) => path,
        Err(e) => return Err(e),
    };

    // Construct the path to the configuration file
    let home_dir = match dirs::home_dir() {
        Some(path) => path,
        None => return Err(eyre!("Failed to determine home directory.")),
    };

    let config_file_path = home_dir.join("amethyst-config.json");
    
    // Delete the configuration file
    match fs::remove_file(&config_file_path) {
        Ok(_) => println!("Config file removed successfully."),
        Err(e) => println!("Error occurred while removing config file: {}", e),
    }

    // Attempt to remove the directory and all its contents recursively
    match fs::remove_dir_all(&path) {
        Ok(()) => {
            println!("Uninstall successful.");
            Ok(()) // Return Ok if removal is successful
        },
        Err(err) => Err(eyre!("Failed to uninstall directory: {}", err)), // Return Err with a custom error message if removal fails
    }
}


// Function to get the path from the configuration file
fn get_config_path() -> Result<PathBuf> {
    // Get the home directory
    let home_dir = match dirs::home_dir() {
        Some(path) => path,
        None => return Err(eyre!("Failed to determine home directory.")),
    };

    // Construct the path to the configuration file
    let config_file_path = home_dir.join("amethyst-config.json");

    // Read the configuration file
    let config_content = match fs::read_to_string(&config_file_path) {
        Ok(content) => content,
        Err(err) => return Err(eyre!("Failed to read configuration file: {}", err)),
    };

    // Deserialize the configuration
    let app_config: Config = match serde_json::from_str(&config_content) {
        Ok(config) => config,
        Err(err) => return Err(eyre!("Failed to parse configuration file: {}", err)),
    };

    Ok(PathBuf::from(app_config.install_path))
}
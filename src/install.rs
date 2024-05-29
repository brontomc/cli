use std::fs;
use std::path::{Path, PathBuf};
use serde::Serialize;
use dirs::home_dir;

#[derive(Serialize)]
struct Config {
    version: String,
    install_path: PathBuf,
    service_port: u16,
}

pub fn install<P: AsRef<Path>>(amethyst_path: P) {
    let amethyst_path = amethyst_path.as_ref();

    // Determine the path to the configuration file in the home directory
    let config_path = match home_dir() {
        Some(path) => path.join("amethyst-config.json"),
        None => {
            eprintln!("Could not retrieve home directory.");
            return;
        }
    };

    // Check if the configuration file already exists
    if config_path.exists() {
        println!("Config file already exists at {:?}", config_path);
        return;
    }
    
    // Check if the directory already exists
    if amethyst_path.exists() {
        println!("Amethyst already installed at {:?}", amethyst_path);
        return;
    }

    // Create the directory
    if let Err(err) = fs::create_dir_all(amethyst_path) {
        eprintln!("Could not create directory: {}", err);
        return;
    }

    // Create a configuration struct
    let config = Config {
        version: "0.1.0".to_string(),
        install_path: amethyst_path.to_path_buf(),
        service_port: 9999,
    };

    // Serialize the configuration struct to a JSON string
    let config_data = match serde_json::to_string_pretty(&config) {
        Ok(data) => data,
        Err(err) => {
            eprintln!("Could not serialize config: {}", err);
            return;
        }
    };

    // Write the configuration data to the file
    if let Err(err) = fs::write(&config_path, config_data) {
        eprintln!("Error writing config data: {}", err);
        return;
    }

    println!("Config file created at {:?}", config_path);
    println!("Amethyst installed at {:?}", amethyst_path);
}

use std::fs;
use std::path::{Path, PathBuf};
use std::io::Write;
use serde::Serialize;
use dirs::home_dir;

#[derive(Serialize)]
struct Config {
    version: String,
    install_path: String,
}

pub fn install<P: AsRef<Path>>(amethyst_path: P) {
    // Determine the final path to use
    let amethyst_path: PathBuf = amethyst_path.as_ref().to_path_buf();

    // Check if the path exists
    if amethyst_path.exists() {
        println!("Amethyst already installed at {:?}", amethyst_path);
    } else {
        // Create the directory
        fs::create_dir(&amethyst_path).expect("Could not create directory");

        // Create the config file
        let config: Config = Config {
            version: "0.1.0".to_string(),
            install_path: amethyst_path.to_string_lossy().into_owned(),
        };

        let config_path: PathBuf = match home_dir() {
            Some(path) => path.join("amethyst-config.json"),
            None => {
                eprintln!("Could not retrieve home directory.");
                return;
            }
        };
        
        let config_data = serde_json::to_string_pretty(&config).expect("Could not serialize config");

        let mut file = fs::File::create(&config_path).expect("Could not create file");

        match file.write_all(config_data.as_bytes()) {
            Ok(()) => println!("Config file created at {:?}", config_path),
            Err(err) => eprintln!("Error writing config data: {}", err),
        }

        println!("Amethyst installed at {:?}", amethyst_path);
    }
}
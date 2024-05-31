mod utils;

use std::fs;
use std::path::{Path, PathBuf};
use serde::Serialize;
use dirs::home_dir;
use color_eyre::eyre::{eyre, Result};

use crate::versions::VersionV;

use utils::get_executable_name;
use utils::construct_download_url;
use utils::download_core;

#[derive(Serialize)]
struct Config {
    cli_version: String,
    core_version: String,
    install_path: PathBuf,
    service_port: u16,
}

pub async fn install<P: AsRef<Path>>(amethyst_path: P, version: VersionV, cli_version: semver::Version) -> Result<()> {
    let amethyst_path = amethyst_path.as_ref();

    // Determine the path to the configuration file in the home directory
    let config_path = match home_dir() {
        Some(path) => path.join("amethyst-config.json"),
        None => {
            return Err(eyre!("The program encountered an error while attempting to determine the path to the configuration file in the home directory."));
        }
    };

    // Check if the configuration file already exists
    if config_path.exists() {
        return Err(eyre!("The system encountered an error while attempting to create a configuration file. It appears that a configuration file already exists. Please delete the config file at {:?} manually before creating a fresh installation.", config_path));
    }
    
    // Check if the directory already exists
    if amethyst_path.exists() {
        return Err(eyre!("The system encountered an error while attempting to install the core components. It appears that the core components are already installed on the system at {:?}. To create a new installation, you have to uninstall the previous version first.", amethyst_path));
    }

    // Create the directory
    if let Err(err) = fs::create_dir_all(amethyst_path) {
        return Err(eyre!("The system encountered an error while attempting to install the core components. It appears that the installation process was unable to create a necessary directory. Reason: {}", err));
    }

    // Create a configuration struct
    let config = Config {
        cli_version: cli_version.to_string(),
        core_version: version.to_string(),
        install_path: amethyst_path.to_path_buf(),
        service_port: 9999,
    };

    // Serialize the configuration struct to a JSON string
    let config_data = match serde_json::to_string_pretty(&config) {
        Ok(data) => data,
        Err(err) => {
            return Err(eyre!("The system encountered an error while attempting to serialize the configuration struct to a JSON string. This error typically occurs when the configuration data provided is not valid. Reason: {}", err));
        }
    };

    // Write the configuration data to the file
    if let Err(err) = fs::write(&config_path, config_data) {
        return Err(eyre!("The system encountered an error while attempting to write the configuration data to the file. Reason: {}", err));
    }

    
    // Download the core
    let executable_name = get_executable_name();
    let amethyst_path = PathBuf::from(amethyst_path).join(executable_name.clone());
    let url = construct_download_url(version.to_string(), executable_name, "core");
    if let Err(err) = download_core(url, &amethyst_path).await {
        return Err(eyre!("The program encountered an error while attempting to install the core. Reason: {}", err));
    }

    Ok(())
}
use std::fs;
use std::path::Path;

use color_eyre::eyre::{Result, eyre};

/// Uninstalls a directory and all its contents recursively.
pub fn uninstall(lodestone_path: &Path) -> Result<()> {
    // Attempt to remove the directory and all its contents recursively
    match fs::remove_dir_all(lodestone_path) {
        Ok(()) => Ok(()), // Return Ok if removal is successful
        Err(err) => Err(eyre!("Failed to uninstall directory: {}", err)), // Return Err with a custom error message if removal fails
    }
}

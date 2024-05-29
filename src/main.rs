mod macros;
mod versions;
mod uninstall;

use std::path::Path;
use color_eyre::eyre::Result;

use uninstall::uninstall;
use versions::get_latest_release;

#[tokio::main]
async fn main() -> Result<()> {

    // Get the latest release
    let latest_release = get_latest_release().await?;
    println!("Latest version: v{}", latest_release.0);
    
    // Uninstalling Amethyst Data with path.
    let lodestone_path = Path::new("./.amethyst");
    uninstall(lodestone_path)?;

    Ok(())
}

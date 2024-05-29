mod macros;
mod versions;
mod install;
mod uninstall;

use color_eyre::eyre::Result;
use dirs::home_dir;

use install::install;
use uninstall::uninstall;
use versions::get_latest_release;

#[tokio::main]
async fn main() -> Result<()> {

    // Get the latest release

    let latest_release = get_latest_release().await?;
    println!("Latest version: v{}", latest_release.0);
    
    // Install Amethyst Core & Panel.
    
    let path = home_dir()
        .expect("Could not find home directory")
        .join(".amethyst");
    install(&path);

    // Uninstalling Amethyst Data with path.
    
    uninstall()?;
    Ok(())
}

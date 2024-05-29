use color_eyre::eyre::Result;
use reqwest::{Client, Response};
use semver::Version;
use serde::Deserialize;

// Define the Release struct
#[derive(Debug, Deserialize)]
pub struct Release {
    pub tag_name: String,
}

// Define the VersionV struct
pub struct VersionV(pub Version);

impl VersionV {
    pub fn from_str(s: &str) -> Result<Self, semver::Error> {
        let version = Version::parse(s.trim_start_matches('v'))?;
        Ok(VersionV(version))
    }
}

// Define the asynchronous function to fetch the latest release
pub async fn get_latest_release() -> Result<VersionV> {
    let client = Client::new();
    let release_url: &str = "https://api.github.com/repos/rust-lang/rust/releases/latest";

    let response: Response = client.get(release_url)
        .header("User-Agent", "Amethyst Core")
        .send()
        .await?
        .error_for_status()?;

    let release: Release = response.json().await?;
    
    let version = VersionV::from_str(&release.tag_name)?;

    Ok(version)
}
use std::env;
use std::path::Path;
use std::fs::File;
use color_eyre::eyre::Result;
use indicatif::{ProgressBar, ProgressStyle};
use futures::StreamExt;
use std::io::Write; 

// Function to construct the download URL based on version and executable name
pub fn construct_download_url(version: String, executable_name: String, repo: &str) -> String {
    format!("https://github.com/amethyst-core/{}/releases/download/v{}/amethyst_core_{}", repo, version, executable_name)
}

// Download Amethyst Core
pub async fn download_core(url: String, path: &Path) -> Result<()> {
    let response = reqwest::get(url).await?;
    let total = response.content_length().unwrap_or(0);
    let mut stream = response.bytes_stream();
    let mut dest = File::create(path)?;
    let mut downloaded: u64 = 0;

    // Indicatif setup
    let download_pb = ProgressBar::new(total);
    let progress_style = ProgressStyle::default_bar()
    .template("{msg}\n{spinner:.magenta} [{elapsed_precise}] [{wide_bar:.magenta/magenta}] {bytes}/{total_bytes} ({bytes_per_sec}, {eta})")
    .unwrap(); // Unwrapping the Result to get the ProgressStyle
    download_pb.set_message("Downloading Amethyst Core");
    download_pb.set_style(progress_style.progress_chars("=>-"));

    while let Some(item) = stream.next().await {
        let chunk = item?;
        dest.write_all(&chunk)?;
        downloaded += chunk.len() as u64;
        download_pb.set_position(downloaded);
    }

    Ok(())
}

// Get the executable name based on the architecture and operating system
pub fn get_executable_name() -> String {
    let target_os = env::consts::OS;
    let target_arch = env::consts::ARCH;
    let name: String = match (target_arch, target_os) {
        ("x86_64", "windows") => "windows_x86_64.exe".to_string(),
        ("x86_64", "linux") => "linux_x86_64".to_string(),
        _ => "unknown_executable".to_string(), // Default case or handle other cases as needed
    };

    return name;
}

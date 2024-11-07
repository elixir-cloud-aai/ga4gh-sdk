pub mod configuration;
pub mod transport;
pub mod test_utils;
pub mod extension;
pub mod extension_manager;

pub fn expand_path_with_home_dir(path: &str) -> String {
    let home_dir = dirs::home_dir().expect("Failed to get home directory");
    let full_path = home_dir.join(path);
    full_path.to_str().expect("Failed to convert path to string").to_string()
}

use std::fs::{File, OpenOptions};
use std::io::{copy, Result};

pub fn open_or_create_file(path: &str) -> Result<File> {
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(path)?;
    Ok(file)
}

pub async fn download_file(url: &str, destination: &str) {
    let response = reqwest::get(url).await.unwrap();
    let mut dest = File::create(destination).unwrap();
    let content = response.bytes().await.unwrap();
    copy(&mut content.as_ref(), &mut dest).unwrap();
}


use url::Url;

pub fn extract_filename_from_url(url_str: &str) -> Option<String> {
    Url::parse(url_str).ok()
        .and_then(|url| {
            let segments: Vec<String> = url.path_segments()
                .map(|segments| segments.map(|s| s.to_string()).collect())
                .unwrap_or_else(Vec::new);
            segments.last().cloned()
        })
}

use std::path::Path;
use std::fs;

// use log::debug;

pub fn copy_file_to_folder(src_file_path: &str, dst_folder_path: &str) -> std::io::Result<()> {
    fs::create_dir_all(dst_folder_path)?;
    let file_name = Path::new(src_file_path).file_name().unwrap();
    let destination_file_path = Path::new(dst_folder_path).join(file_name);
    fs::copy(src_file_path, destination_file_path)?;
    Ok(())
}

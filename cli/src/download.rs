use std::path::{Path, PathBuf};
use std::fs;
use reqwest::Client;
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use rusoto_core::Region;
use rusoto_s3::{S3Client, GetObjectRequest, S3};
use tokio::io::AsyncReadExt;

async fn download_http(url: &str, destination: Option<&str>) -> Result<String, Box<dyn std::error::Error>> {
    let response = Client::new().get(url).send().await?;
    let content = response.text().await?;
    
    if let Some(dest) = destination {
        let mut file = File::create(dest).await?;
        file.write_all(content.as_bytes()).await?;
    }
    
    Ok(content)
}

async fn download_s3(url: &str, destination: Option<&str>) -> Result<String, Box<dyn std::error::Error>> {
    let s3_client = S3Client::new(Region::default());

    let url_parts: Vec<&str> = url.trim_start_matches("s3://").splitn(2, '/').collect();
    let bucket = url_parts[0];
    let key = url_parts[1];

    let get_req = GetObjectRequest {
        bucket: bucket.to_string(),
        key: key.to_string(),
        ..Default::default()
    };

    let result = s3_client.get_object(get_req).await?;
    let stream = result.body.unwrap();
    let mut content = Vec::new();
    let mut stream = stream.into_async_read();
    stream.read_to_end(&mut content).await?;

    if let Some(dest) = destination {
        let mut file = File::create(dest).await?;
        file.write_all(&content).await?;
    }

    Ok(String::from_utf8(content)?)
}

pub async fn download_file(url: &str, destination: Option<&str>) -> Result<String, Box<dyn std::error::Error>> {
    if url.starts_with("s3://") {
        download_s3(url, destination).await
    } else {
        download_http(url, destination).await
    }
}
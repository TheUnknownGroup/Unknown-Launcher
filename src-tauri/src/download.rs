use reqwest::Client;
use std::fs::File;
use std::io::copy;
use std::path::Path;

pub async fn download_file(url: &str, destination: &str) -> anyhow::Result<()> {
    let response = Client::new().get(url).send().await?;
    let mut dest = File::create(Path::new(destination))?;
    let mut content = response.bytes().await?;
    copy(&mut content.as_ref(), &mut dest)?;
    Ok(())
}
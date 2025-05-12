/*
src/storage.rs
*/
use anyhow::Result;
use flate2::{write::GzEncoder, Compression};
use serde::{Deserialize, Serialize};
use std::fs::{create_dir_all, File};
use std::io::Write;
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
struct Metadata {
    url: String,
    keywords: Vec<String>,
}

pub fn store_page(dir: &str, _url: &str, html: &str) -> Result<String> {
    let id = Uuid::new_v4().to_string();
    let page_dir = format!("{}/pages", dir);
    create_dir_all(&page_dir)?;
    let path = format!("{}/{}.html.gz", page_dir, &id);
    let f = File::create(&path)?;
    let mut encoder = GzEncoder::new(f, Compression::default());
    encoder.write_all(html.as_bytes())?;
    Ok(id)
}

pub fn store_metadata(dir: &str, id: &str, url: &str, keywords: &[String]) -> Result<()> {
    let meta_dir = format!("{}/meta", dir);
    create_dir_all(&meta_dir)?;
    let path = format!("{}/{}.json", meta_dir, id);
    let meta = Metadata { url: url.to_string(), keywords: keywords.to_vec() };
    let mut f = File::create(&path)?;
    f.write_all(serde_json::to_string(&meta)?.as_bytes())?;
    Ok(())
}
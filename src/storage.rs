// Cargo Packages
use anyhow::Result;
use flate2::{write::GzEncoder, Compression};
use serde::{Deserialize, Serialize};
use std::fs::{create_dir_all, File};
use std::io::Write;
use std::path::Path;
use uuid::Uuid;
use serde_json::Value;

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

pub fn write_to_csv(output_dir: &str, csv_file: &str) -> Result<(), Box<dyn std::error::Error>> {
    let meta_dir = Path::new(output_dir).join("meta");
    let mut csv_writer = csv::Writer::from_path(csv_file)?;

    // Write the header row
    csv_writer.write_record(&["url", "doc_id", "keywords"])?;

    // Iterate through all metadata files
    for entry in std::fs::read_dir(meta_dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.extension().and_then(|ext| ext.to_str()) == Some("json") {
            let file = std::fs::read_to_string(&path)?;
            let json: Value = serde_json::from_str(&file)?;

            let url = json["url"].as_str().unwrap_or("");
            let doc_id = path.file_stem().and_then(|stem| stem.to_str()).unwrap_or("");
            let keywords = json["keywords"]
                .as_array()
                .map(|arr| arr.iter().filter_map(|k| k.as_str()).collect::<Vec<_>>().join(", "))
                .unwrap_or_default();

            // Write the row to the CSV
            csv_writer.write_record(&[url, doc_id, &keywords])?;
        }
    }

    csv_writer.flush()?;
    Ok(())
}
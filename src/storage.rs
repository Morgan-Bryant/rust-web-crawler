// Cargo Packages
use anyhow::Result;
use flate2::{write::GzEncoder, Compression};
use serde::{Deserialize, Serialize};
use std::fs::{create_dir_all, File};
use std::io::Write;
use std::path::Path;
use uuid::Uuid;
use serde_json::Value;

/*
## `store_page`
Compresses and stores the HTML content of a web page in the specified directory.
 
### Arguments
    * `dir` - A string slice representing the base directory where the page will be stored.
    * `_url` - A string slice representing the URL of the page (not used in storage but included for context).
    * `html` - A string slice containing the HTML content of the page.
 
### Returns
    * `Result<String>` - Returns the unique ID of the stored page if successful, or an error if something goes wrong.
 
### Behavior
    1. Generates a unique ID for the page using `Uuid`.
    2. Creates a `pages/` subdirectory in the specified base directory if it doesn't exist.
    3. Compresses the HTML content using gzip and writes it to a file named `<ID>.html.gz`.
*/
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
/*
## `store_metadata`
Stores metadata (URL and keywords) for a web page in the specified directory.
 
### Arguments
    * `dir` - A string slice representing the base directory where the metadata will be stored.
    * `id` - A string slice representing the unique ID of the page.
    * `url` - A string slice representing the URL of the page.
    * `keywords` - A slice of strings representing the keywords extracted from the page.
 
### Returns
    * `Result<()>` - Returns `Ok(())` if the metadata is stored successfully, or an error if something goes wrong.
 
### Behavior
    1. Creates a `meta/` subdirectory in the specified base directory if it doesn't exist.
    2. Serializes the metadata (URL and keywords) into JSON format.
    3. Writes the JSON metadata to a file named `<ID>.json`.
 
# Errors
These functions can return errors in the following cases:
    * Issues with creating directories.
    * Problems with writing files (e.g., permission issues).
    * Errors during compression or JSON serialization.
*/
pub fn store_metadata(dir: &str, id: &str, url: &str, keywords: &[String]) -> Result<()> {
    let meta_dir = format!("{}/meta", dir);
    create_dir_all(&meta_dir)?;
    let path = format!("{}/{}.json", meta_dir, id);
    let meta = Metadata { url: url.to_string(), keywords: keywords.to_vec() };
    let mut f = File::create(&path)?;
    f.write_all(serde_json::to_string(&meta)?.as_bytes())?;
    Ok(())
}

/// Writes all crawled data into a CSV file with columns: url, doc_id, keywords.
///
/// # Arguments
/// * `output_dir` - The directory where the metadata files are stored.
/// * `csv_file` - The path to the CSV file to be created.
///
/// # Returns
/// * `Result<()>` - Returns `Ok(())` if the CSV file is written successfully, or an error if something goes wrong.
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
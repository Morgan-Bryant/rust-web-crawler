/*
src/indexer.rs
*/
use anyhow::Result;
use serde_json::Value;
use std::collections::HashMap;
use std::fs::{File, read_dir};
use std::io::Write;

pub fn build_index(dir: &str) -> Result<()> {
    let meta_dir = format!("{}/meta", dir);
    let mut index: HashMap<String, Vec<String>> = HashMap::new();
    for entry in read_dir(&meta_dir)? {
        let path = entry?.path();
        if path.extension().and_then(|s| s.to_str()) == Some("json") {
            let id = path.file_stem().unwrap().to_string_lossy().to_string();
            let data: Value = serde_json::from_reader(File::open(&path)?)?;
            if let Some(keys) = data.get("keywords").and_then(|k| k.as_array()) {
                for k in keys {
                    if let Some(term) = k.as_str() {
                        index.entry(term.to_string()).or_default().push(id.clone());
                    }
                }
            }
        }
    }
    let idx_path = format!("{}/index.json", dir);
    let mut f = File::create(idx_path)?;
    f.write_all(serde_json::to_string(&index)?.as_bytes())?;
    Ok(())
}
/*
src/query.rs
*/
use anyhow::Result;
use serde_json::Value;
use std::fs::File;
use std::io::Read;

pub fn run_query(dir: &str, term: &str) -> Result<()> {
    let path = format!("{}/index.json", dir);
    let mut data = String::new();
    File::open(&path)?.read_to_string(&mut data)?;
    let idx: Value = serde_json::from_str(&data)?;
    if let Some(arr) = idx.get(term).and_then(|v| v.as_array()) {
        println!("Documents containing '{}':", term);
        for id in arr {
            println!("- {}", id.as_str().unwrap());
        }
    } else {
        println!("No documents found for '{}'.", term);
    }
    Ok(())
}

// Cargo Packages
use anyhow::Result;
use serde_json::Value;
use std::collections::HashMap;
use std::fs::{File, read_dir};
use std::io::Write;
/*
## `build_index`
Builds a search index from metadata files in the specified directory.

/ # Arguments 
    * `dir` - The directory containing the metadata files (`meta/` subdirectory) and where the index will be stored.
 
# Returns 
    * `Result<()>` - Returns `Ok(())` if the index is built successfully, or an error if something goes wrong. 

# Behavior 
    1. Reads all JSON metadata files from the `meta/` subdirectory of the specified directory.
    2. Extracts keywords from each metadata file and maps them to the corresponding page IDs.
    3. Builds a HashMap where each keyword points to a list of page IDs containing that keyword.
    4. Serializes the index into a JSON file (`index.json`) in the specified directory.
 
# Errors 
    This function can return errors in the following cases:
    * Issues with reading the metadata directory or files.
    * Problems with parsing JSON metadata files.
    * Issues with writing the index to the output file.
*/
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
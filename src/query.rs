// Cargo Packages
use anyhow::Result;
use csv::Reader;

pub fn run_query(dir: &str, term: &str) -> Result<()> {
    let path = format!("{}/crawled_data.csv", dir);
    let mut reader = Reader::from_path(&path)?;

    let mut found = false;
    println!("Searching for '{}' in crawled_data.csv...", term);
    let mut count = 0;
    for result in reader.records() {
        let record = result?;
        let url = &record[0];
        let doc_id = &record[1];
        let keywords = &record[2];

        if keywords.to_lowercase().contains(&term.to_lowercase()) {
            if !found {
                println!("Documents containing '{}':", term);
                found = true;
            }
            println!("- URL: {}, Doc ID: {}", url, doc_id);
            count += 1;
        }
    }

    if !found {
        println!("No documents found for '{}'.", term);
    }
    println!("Total number of URLS containing keywords: \"{}\" is {}", term, count);
    Ok(())
}

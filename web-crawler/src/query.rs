// Cargo Packages
use anyhow::Result;
use serde_json::Value;
use std::fs::File;
use std::io::Read;

/*
## `run_query`
Executes a search query on the index and prints the results.
 
# Arguments
    * `dir` - A string slice representing the directory containing the search index (`index.json` file).
    * `term` - A string slice representing the search term to query.
 
# Returns
    * `Result<()>` - Returns `Ok(())` if the query executes successfully, or an error if something goes wrong.
 
# Behavior
    1. Reads the `index.json` file from the specified directory.
    2. Parses the JSON index into a data structure.
    3. Searches for the provided term in the index.
    4. If the term is found:
        - Prints the list of document IDs containing the term.
    5. If the term is not found:
        - Prints a message indicating no documents were found.
 
# Errors 
This function can return errors in the following cases:
    * Issues with reading the `index.json` file.
    * Problems with parsing the JSON index.
*/
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

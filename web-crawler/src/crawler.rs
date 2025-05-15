// Crates 
use anyhow::Result;
use crate::storage;
use crate::url_utils;
use crate::robots;
use crate::parser;
use crate::keywords;
// Cargo Packages
use reqwest::Client;
use std::collections::{HashSet, VecDeque};
use tokio::time::{sleep, Duration};
use tracing::info;
/*
## `run_crawl`
Crawls web pages starting from a seed URL, extracts content,
and stores it in the specified output directory.

# Arguments:
    * `seed` - The starting URL for the crawl.
    * `output_dir` - The directory where crawled pages and metadata will be stored.
    * `max_pages` - The maximum number of pages to crawl.
 
# Returns:
    * `Result<()>` - Returns `Ok(())` if the crawl completes successfully, or an error if something goes wrong.
 
# Behavior:
    1. Initializes an HTTP client with a custom user agent.
    2. Maintains a queue (`frontier`) of URLs to visit and a set (`visited`) of already visited URLs.
    3. Respects `robots.txt` rules to determine if a URL is allowed to be crawled.
    4. Fetches the HTML content of each URL and:
        - Stores the page content.
        - Extracts text and keywords.
        - Stores metadata (e.g., keywords and URL).
    5. Extracts and normalizes links from the HTML content and adds them to the frontier if they haven't been visited.
    6. Enforces a politeness delay of 1 second between requests.
 
# Errors:
    This function can return errors in the following cases:
        * HTTP request failures.
        * Issues with storing page content or metadata.
        * Invalid or malformed URLs.
*/
 pub async fn run_crawl(seed: &str, output_dir: &str, max_pages: usize) -> Result<()> {
    let client = Client::builder()
        .user_agent("FsCrawler/0.1 (+https://www.wikipedia.org/)") // Large Attack Space
        .build()?;

    let mut frontier = VecDeque::new();
    let mut visited = HashSet::new();
    frontier.push_back(seed.to_string());
    visited.insert(seed.to_string());

    while let Some(url) = frontier.pop_front() {
        if visited.len() > max_pages { break; }
        if !robots::is_allowed(&client, &url).await.unwrap_or(true) {
            continue;
        }
        info!("Fetching {}", url);
        if let Ok(html) = client.get(&url).send().await?.text().await {
            let id = storage::store_page(output_dir, &url, &html)?;
            let text = parser::extract_text(&html);
            let keywords = keywords::extract(&text);
            storage::store_metadata(output_dir, &id, &url, &keywords)?;

            for link in parser::extract_links(&html, &url) {
                let normalized = url_utils::normalize(&link);
                if !visited.contains(&normalized) {
                    visited.insert(normalized.clone());
                    frontier.push_back(normalized);
                }
            }
        }
        sleep(Duration::from_secs(1)).await; // politeness delay
    }
    Ok(())
}

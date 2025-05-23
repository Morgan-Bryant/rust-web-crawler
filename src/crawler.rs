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

pub async fn run_crawl(seed: &str, output_dir: &str, max_pages: usize) -> Result<()> {
    let client = Client::builder()
        .user_agent("FsCrawler/0.1 (+https://example.com)") 
        .build()?;

    let mut frontier = VecDeque::new();
    let mut visited = HashSet::new();
    frontier.push_back(seed.to_string());
    visited.insert(seed.to_string());

    while let Some(url) = frontier.pop_front() {
        if visited.len() > max_pages {
            break;
        }
        if !robots::is_allowed(&client, &url).await.unwrap_or(true) {
            continue;
        }
        info!("Fetching {}", url);
        if let Ok(html) = client.get(&url).send().await?.text().await {
            let id = storage::store_page(output_dir, &url, &html)?;
            let text = parser::extract_text(&html);
            let all_keywords = keywords::extract(&text);

            // Store all extracted keywords
            if !all_keywords.is_empty() {
                storage::store_metadata(output_dir, &id, &url, &all_keywords)?;
            }

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
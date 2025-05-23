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
//  pub async fn run_crawl(seed: &str, output_dir: &str, max_pages: usize) -> Result<()> {
//     let client = Client::builder()
//         .user_agent("FsCrawler/0.1 (+https://www.wikipedia.org/)") // Large Attack Space
//         .build()?;

//     let mut frontier = VecDeque::new();
//     let mut visited = HashSet::new();
//     frontier.push_back(seed.to_string());
//     visited.insert(seed.to_string());

//     while let Some(url) = frontier.pop_front() {
//         if visited.len() > max_pages { break; }
//         if !robots::is_allowed(&client, &url).await.unwrap_or(true) {
//             continue;
//         }
//         info!("Fetching {}", url);
//         if let Ok(html) = client.get(&url).send().await?.text().await {
//             let id = storage::store_page(output_dir, &url, &html)?;
//             let text = parser::extract_text(&html);
//             let all_keywords = keywords::extract(&text);

//             // Filter keywords to focus on hospital-related terms
//             let filtered_keywords: Vec<String> = all_keywords
//                 .into_iter()
//                 .filter(|keyword| {
//                     let lower_keyword = keyword.to_lowercase();
//                     lower_keyword.contains("buzz")
//                         || lower_keyword.contains("college of computing")
//                         || lower_keyword.contains("college of engineering")
//                         || lower_keyword.contains("professor")
//                         || lower_keyword.contains("atlanta")
//                         || lower_keyword.contains("research")
//                         || lower_keyword.contains("georgia tech")
//                         || lower_keyword.contains("gt")
//                         || lower_keyword.contains("schedule")
//                         || lower_keyword.contains("contact")
//                         || lower_keyword.contains("people")
//                         || lower_keyword.contains("undergraduate")
//                         || lower_keyword.contains("graduate")
//                         || lower_keyword.contains("schools")
//                         || lower_keyword.contains("ling liu")
//                         || lower_keyword.contains("canvas")
//                         || lower_keyword.contains("computer science undergraduate")
//                         || lower_keyword.contains("ms cs georgia tech")
//                         || lower_keyword.contains("online master’s computer science")
//                         || lower_keyword.contains("phd in computer science")
//                         || lower_keyword.contains("computational media")
//                         || lower_keyword.contains("information security degree")
//                         || lower_keyword.contains("human-computer interaction program")
//                         || lower_keyword.contains("course syllabus georgia tech")
//                         || lower_keyword.contains("course schedule cc georgia tech")
//                         || lower_keyword.contains("artificial intelligence research")
//                         || lower_keyword.contains("machine learning georgia tech")
//                         || lower_keyword.contains("robotics georgia tech")
//                         || lower_keyword.contains("cybersecurity lab")
//                         || lower_keyword.contains("software engineering research")
//                         || lower_keyword.contains("interactive computing")
//                         || lower_keyword.contains("data science research")
//                         || lower_keyword.contains("distributed systems lab")
//                         || lower_keyword.contains("cvpr publications gt")
//                         || lower_keyword.contains("cc.gatech.edu labs")
//                         || lower_keyword.contains("georgia tech cs faculty")
//                         || lower_keyword.contains("faculty homepage")
//                         || lower_keyword.contains("professor ")
//                         || lower_keyword.contains("phd advisor georgia tech")
//                         || lower_keyword.contains("grad student research interests")
//                         || lower_keyword.contains("research group members")
//                         || lower_keyword.contains("georgia tech cs papers")
//                         || lower_keyword.contains("arxiv georgia tech authors")
//                         || lower_keyword.contains("nsf grants georgia tech")
//                         || lower_keyword.contains("cc.gatech.edu publications")
//                         || lower_keyword.contains("student theses georgia tech")
//                         || lower_keyword.contains("github georgia tech projects")
//                         || lower_keyword.contains("research assistant position georgia tech")
//                         || lower_keyword.contains("cs internship georgia tech")
//                         || lower_keyword.contains("phd application requirements")
//                         || lower_keyword.contains("postdoc position gt computing")
//                         || lower_keyword.contains("job opportunities cc.gatech.edu")
//                         || lower_keyword.contains("secure multiparty computation")
//                         || lower_keyword.contains("blockchain research")
//                         || lower_keyword.contains("nlp georgia tech")
//                         || lower_keyword.contains("computer vision gt")
//                         || lower_keyword.contains("edge computing gt")
//                         || lower_keyword.contains("quantum computing research")
//                         || lower_keyword.contains("georgia tech college of computing")
//                         || lower_keyword.contains("georgia tech school of computer science")
//                         || lower_keyword.contains("computing community at gt")
//                         || lower_keyword.contains("diversity in computing gt")
//                         || lower_keyword.contains("student organizations georgia tech")
//                         || lower_keyword.contains("georgia tech clusters")
//                         || lower_keyword.contains("research computing resources gt")
//                         || lower_keyword.contains("data sets georgia tech")
//                         || lower_keyword.contains("open source tools georgia tech")
//                         || lower_keyword.contains("computing seminars gt")
//                         || lower_keyword.contains("gt computing colloquium")
//                         || lower_keyword.contains("georgia tech hackathons")
//                         || lower_keyword.contains("gt computing news")
//                         || lower_keyword.contains("student competitions gt")
//                 })
//                 .collect();

//             if !filtered_keywords.is_empty() {
//                 storage::store_metadata(output_dir, &id, &url, &filtered_keywords)?;
//             }

//             for link in parser::extract_links(&html, &url) {
//                 let normalized = url_utils::normalize(&link);
//                 if !visited.contains(&normalized) {
//                     visited.insert(normalized.clone());
//                     frontier.push_back(normalized);
//                 }
//             }
//         }
//         sleep(Duration::from_secs(1)).await; // politeness delay
//     }
//     Ok(())
// }

pub async fn run_crawl(seed: &str, output_dir: &str, max_pages: usize) -> Result<()> {
    let client = Client::builder()
        .user_agent("FsCrawler/0.1 (+https://www.wikipedia.org/)") // Large Attack Space
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

            // Store all extracted keywords without filtering
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
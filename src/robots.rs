// Cargo Packages
use anyhow::Result;
use reqwest::Client;
use texting_robots::{Robot, get_robots_url};

pub async fn is_allowed(client: &Client, url: &str) -> Result<bool> {
    // Attempt to parse the URL
    let parsed = match url::Url::parse(url) {
        Ok(parsed) => parsed,
        Err(_) => {
            println!("Skipping invalid URL: {}", url);
            return Ok(false); // Skip invalid URLs
        }
    };

    // Construct the robots.txt URL
    let robots_url = match get_robots_url(url) {
        Ok(robots_url) => robots_url,
        Err(_) => {
            println!("Failed to construct robots.txt URL for: {}", url);
            return Ok(false); // Skip if robots.txt URL cannot be constructed
        }
    };

    // Fetch and parse the robots.txt file
    let txt = match client.get(&robots_url).send().await {
        Ok(response) => match response.bytes().await {
            Ok(bytes) => bytes,
            Err(_) => {
                println!("Failed to fetch robots.txt for: {}", robots_url);
                return Ok(false); // Skip if robots.txt cannot be fetched
            }
        },
        Err(_) => {
            println!("Failed to send request for robots.txt: {}", robots_url);
            return Ok(false); // Skip if request fails
        }
    };

    // Check if the URL is allowed
    let robot = match Robot::new("FsCrawler/0.1", &txt) {
        Ok(robot) => robot,
        Err(_) => {
            println!("Failed to parse robots.txt for: {}", robots_url);
            return Ok(false); // Skip if robots.txt cannot be parsed
        }
    };

    Ok(robot.allowed(parsed.path()))
}
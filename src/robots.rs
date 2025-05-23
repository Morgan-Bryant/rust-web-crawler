// Cargo Packages
use anyhow::Result;
use reqwest::Client;
use texting_robots::{Robot, get_robots_url};

/*
##  `is_allowed`
Checks if a given URL is allowed to be crawled based on the site's `robots.txt` rules.
 
# Arguments
    * `client` - A reference to the `reqwest::Client` used to make HTTP requests.
    * `url` - A string slice representing the URL to check against the `robots.txt` rules.
 
# Returns
    * `Result<bool>` - Returns `Ok(true)` if the URL is allowed to be crawled, `Ok(false)` if it is disallowed, or an error if something goes wrong.
 
# Behavior
    1. Constructs the `robots.txt` URL for the given site using the `get_robots_url` function.
    2. Fetches the `robots.txt` file using the provided HTTP client.
    3. Parses the `robots.txt` file using the `texting_robots` crate.
    4. Checks if the given URL is allowed to be crawled based on the rules in the `robots.txt` file.

# Errors 
This function can return errors in the following cases:
    * Issues with parsing the URL.
    * HTTP request failures when fetching the `robots.txt` file.
    * Problems with parsing the `robots.txt` file.
*/
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
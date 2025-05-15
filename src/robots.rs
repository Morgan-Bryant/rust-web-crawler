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
    // parse out just the path for robots.txt rules:
    let parsed = url::Url::parse(url)?;
    let robots_url = get_robots_url(url)?;
    let txt = client.get(&robots_url).send().await?.bytes().await?;
    let robot = Robot::new("FsCrawler/0.1", &txt)?;
    Ok(robot.allowed(parsed.path()))

    // —OR, to completely skip robots.txt during debug:
    // Ok(true)
}

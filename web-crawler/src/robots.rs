/*
src/robots.rs
*/
use anyhow::Result;
use reqwest::Client;
use texting_robots::{Robot, get_robots_url};

pub async fn is_allowed(client: &Client, url: &str) -> Result<bool> {
//     let robots_url = get_robots_url(url)?;
//     let txt = client.get(&robots_url).send().await?.bytes().await?;
//     let robot = Robot::new("FsCrawler/0.1", &txt)?;
//     Ok(robot.allowed(url))
// }
    
    // parse out just the path for robots.txt rules:
    let parsed = url::Url::parse(url)?;
    let robots_url = get_robots_url(url)?;
    let txt = client.get(&robots_url).send().await?.bytes().await?;
    let robot = Robot::new("FsCrawler/0.1", &txt)?;
    Ok(robot.allowed(parsed.path()))

    // —OR, to completely skip robots.txt during debug:
    // Ok(true)
}

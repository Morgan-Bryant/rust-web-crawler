// Cargo Packages
use clap::Parser;
use anyhow::Result;
// Crates
mod crawler;
mod query;
mod storage;
mod parser;
mod url_utils;
mod keywords;
mod robots;

/// CLI args
#[derive(Parser, Debug)]
#[command(author, version, about = "Filesystem-based crawler & search tool")]
struct Opts {
    /// Seed URL to start crawling
    #[arg(long)]
    seed: String,
    /// Directory to store pages and index
    #[arg(long, default_value = "./data")]
    output_dir: String,
    /// Maximum pages to crawl
    #[arg(long, default_value_t = 1000)]
    max_pages: usize,
    /// Query term (skip crawl if provided)
    #[arg(long)]
    query: Option<String>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opts = Opts::parse();

    if opts.query.is_some() {
        // Perform search query
        query::run_query(&opts.output_dir, &opts.query.unwrap())?;
    } else {
        // Run the crawler
        crawler::run_crawl(&opts.seed, &opts.output_dir, opts.max_pages).await?;

        // Write crawled data to CSV
        let csv_file = format!("{}/crawled_data.csv", opts.output_dir);
        storage::write_to_csv(&opts.output_dir, &csv_file)?;
        println!("Crawled data has been written to {}", csv_file);
    }
    Ok(())
}
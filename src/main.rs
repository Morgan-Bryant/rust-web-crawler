// Cargo Packages
use clap::Parser;
use tracing_subscriber;
use anyhow::Result;
// Crates
mod crawler;
mod indexer;
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

/*
Entry point for the filesystem-based web crawler and search tool. 
# Description
This program can either:
    1. Crawl web pages starting from a seed URL, store their content and metadata, and build a search index.
    2. Perform a search query on an existing index if a query term is provided.

# CLI Arguments
The program accepts the following command-line arguments:
    * `--seed` - The seed URL to start crawling from (required).
    * `--output-dir` - The directory to store crawled pages, metadata, and the search index (default: `./data`).
    * `--max-pages` - The maximum number of pages to crawl (default: 1000).
    * `--query` - A search query term. If provided, crawling is skipped, and the program searches the index instead.

# Behavior 
    1. Initializes logging using the `tracing` crate.
    2. Parses CLI arguments using the `clap` crate.
    3. If a query term is provided:
        - Runs the search query using the `query` module.
    4. Otherwise:
        - Crawls web pages using the `crawler` module.
        - Builds a search index from the crawled data using the `indexer` module.
 
# Errors 
This function can return errors in the following cases:
    * Issues with crawling web pages.
    * Problems with building the search index.
    * Errors during query execution.

# Example
To crawl web pages:
```bash
    cargo run -- --seed "https://example.com" --output-dir "./data" --max-pages 100
``` 
To search the index:
```bash
    cargo run -- --output-dir "./data" --query "example"
```
*/
#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    let opts = Opts::parse();

    if let Some(term) = opts.query {
        query::run_query(&opts.output_dir, &term)?;
        return Ok(());
    }

    crawler::run_crawl(&opts.seed, &opts.output_dir, opts.max_pages).await?;
    indexer::build_index(&opts.output_dir)?;
    Ok(())
}
/*
src/main.rs
*/
use clap::Parser;
use tracing_subscriber;
use anyhow::Result;

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
# web-crawler
Collecting workspace informationFiltering to most relevant informationThis project is a **web crawler and search tool** implemented in Rust. It is designed to crawl web pages starting from a seed URL, extract and store their content, and build an index for search functionality. Here's a breakdown of its components and functionality:

### Key Features:
1. **Web Crawling**:
   - The `run_crawl` function in `crawler.rs` is the main entry point for crawling.
   - It uses the `reqwest` library to fetch web pages and follows links extracted from the HTML content.
   - A **frontier** (queue) is maintained to track URLs to visit, and a **visited set** ensures no URL is crawled more than once.
   - The crawler respects the `robots.txt` rules using the `robots` module.

2. **Content Storage**:
   - The `storage` module is used to save the HTML content of pages and their metadata (e.g., keywords) to the specified output directory.

3. **Text and Metadata Extraction**:
   - The `parser` module extracts text and links from HTML pages.
   - The `keywords` module identifies keywords from the extracted text for indexing.

4. **Politeness**:
   - A delay (`tokio::time::sleep`) is added between requests to avoid overwhelming servers.

5. **Search Index**:
   - The project includes an `indexer` module (referenced in `main.rs`) to build a search index from the crawled data.
   - A `query` module allows users to search the indexed data.

6. **Command-Line Interface**:
   - The project uses the `clap` library to define CLI arguments (e.g., seed URL, output directory, max pages to crawl, and query terms).

### Workflow:
1. **Crawling**:
   - Start from a seed URL.
   - Fetch the page, extract links, and add them to the frontier.
   - Store the page content and metadata.

2. **Indexing**:
   - After crawling, build an index of the crawled pages for efficient search.

3. **Searching**:
   - If a query term is provided, skip crawling and directly search the index.

### Project Structure:
- **Source Code**:
  - crawler.rs: Implements the crawling logic.
  - `src/indexer.rs`: Handles indexing of crawled data.
  - `src/query.rs`: Provides search functionality.
  - `src/storage.rs`: Manages storage of pages and metadata.
  - `src/parser.rs`: Extracts text and links from HTML.
  - `src/url_utils.rs`: Normalizes URLs.
  - `src/robots.rs`: Handles `robots.txt` rules.
  - `src/keywords.rs`: Extracts keywords from text.

- **Data Storage**:
  - Crawled pages and metadata are stored in the `data/` directory.

- **Build Artifacts**:
  - Compiled binaries and intermediate files are stored in the `target/` directory.

### Purpose:
This project is a homework to demonstrate skills in:
- Asynchronous programming with Rust (`tokio`).
- Web crawling and parsing.
- Data storage and indexing.
- Command-line application development.

It is a practical example of building a lightweight, filesystem-based web crawler and search engine.

### Usage:
To run:

```bash
./target/release/fs_search_crawler \
  --seed "https://example.com" \
  --output-dir "./data" \
  --max-pages 10
  ```

To Query:

```bash
./target/release/fs_search_crawler \
  --seed "https://example.com" \
  --output-dir "./data" \
  --query "example"
  ```
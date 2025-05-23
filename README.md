# rust-web-crawler
Programmer Information:
- **Name**: Morgan Bryant
- **GT ID**: 903680910
- **Email**: mbryant49@gatech.edu
- **Development Machine** MacBook Pro M4, 16gb RAM

### Key Features:
This project is both a webcrawler and search tool in one. Since it is written in `Rust`, it is as fast as possible on the raw computing side. Additionally, we use the `tokio` async package to crawl multiple web pages at once. It is intended to be ran as a CLI program only in its current state, but could have a UI implemented in the future. The main time cost is from the `politeness` wait call as to not overwhelm the servers. This call waits one second per url crawled.

### Analysis:
#### Pros:
- Host side is fast and effiecent in terms of CPU usage.
- Uses `tokio` async package to crawl multiple web pages at once.
- Uses `serde` to serialize and deserialize data to and from CSV files.
- Fairly lightweight storage and memory foot print (depending on # of crawled URLS).
- Has an indexer/search function if trying to find certain keywords in the crawled data.
- Has a `crawled_data.csv` file that contains the crawled data in a CSV format to imporve utility. Such as viewing all data at once or importing into a spreadsheet software like MS Excel.
- The program is fairly modular and can be easily modified for a specific use case such as filtering out/in URLS or only capturing certain keywords.

#### Cons:
- Requires an understanding of `Rust` and `Cargo` to modify and tailor to a specific use case
- The `politeness` wait call is currently causing an increase in runtime.
   - This can be removed if the user is willing to risk being blocked by the server or is given explicit permission to crawl
- Certain URLS may cause errors and break the crawler.
- Certain websites may be skipped if they have a `robots.txt` file that disallows crawling or if a `robots.txt` file is not found.
- No GUI, as it is Command Line Interface (CLI) use only at this time.
- This is a student project, and may encounter other errors after more strenous use/testing.


### Project Structure:
- **Source Code**:
  - `src/crawler.rs`: Implements the crawling logic.
  - `src/indexer.rs`: Handles indexing of crawled data.
  - `src/query.rs`: Provides search functionality.
  - `src/storage.rs`: Manages storage of pages and metadata.
  - `src/parser.rs`: Extracts text and links from HTML.
  - `src/url_utils.rs`: Normalizes URLs.
  - `src/robots.rs`: Handles `robots.txt` rules.
  - `src/keywords.rs`: Extracts keywords from text.
  - `data/`: Main directory for storing crawled data.
  - `data/meta`: Contains the URLS & Keywords for crawled data.
  - `data/pages`: Contains `.gz` compressed version of the HTML pages
  - `data/crawled_data.csv`: Contains the crawled data in CSV format.
  - `target/release`: Contains the compiled **RELEASE** Binary
  - `target/debug`: Contains the compiled **DEBUG** Binary
  - `Cargo.toml`: Contains all cargo packages & other information used to compile the program.
  - `Cargo.lock`: Contains the lock file for the cargo packages used in the project to ensure that the same versions are used across compilations.

### Usage:
```bash
Filesystem-based crawler & search tool

Usage: fs_search_crawler [OPTIONS] --seed <SEED>

Options:
      --seed <SEED>              Seed URL to start crawling
      --output-dir <OUTPUT_DIR>  Directory to store pages and index [default: ./data]
      --max-pages <MAX_PAGES>    Maximum pages to crawl [default: 1000]
      --query <QUERY>            Query term (skip crawl if provided)
  -h, --help                     Print help
  -V, --version                  Print version
```
#### To Crawl:
```bash
./target/release/fs_search_crawler \
  --seed "https://example.com" \
  --output-dir "./data" \
  --max-pages 10
  ```

#### To Query (after crawling):
```bash
./target/release/fs_search_crawler \
  --seed "" \
  --output-dir "./data" \
  --query "example"
  ```
#### To Query & Export to CSV:
```bash
./target/release/fs_search_crawler \
  --seed "" \
  --output-dir "./data" \
  --query "example" > example.csv
```

### Benchmarks:
```bash
./target/release/fs_search_crawler \
  --seed "https://sandbox.oxylabs.io/h" \
  --output-dir "./data" \
  --max-pages 25000
  ```
Took about 1 hour to crawl 25,000 pages. In my output CSV it has 3099 rows in the format:
```csv
url,doc_id,keywords
```
Storage Output:
- `crawled_data.csv`: 737 KB
- `data/meta`: 12.7 MB
- `data/pages`: 36.9 MB
- TOTAl: 49.6 MB

Approximate time to crawl a page based on the above test run is about
- 1/24 days
- 1 hour
- 52 urls per minute
- 0.86 urls per second.

#### We predict that for 10 million urls:
- 128.20 days
- 3205.13 hours
- 192307.69 minutes
- 11538461.54 seconds
- Total Size: 29.84 GB

#### We predict that for 1 billion urls:
- 12820.51 days
- 320205.13 hours
- 192307.69 minutes
- 11538461.54 seconds
- Total Size: 1.937 TB

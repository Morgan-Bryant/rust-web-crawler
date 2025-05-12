# web-crawler

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
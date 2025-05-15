// Cargo Packages
use url::Url;

/*
## `normalize`
Normalizes a given URL by removing the fragment (if any) and converting it to lowercase.
 
### Arguments
    * `input` - A string slice representing the URL to normalize.
 
### Returns
    * `String` - The normalized URL as a string. If the input is not a valid URL, it returns the input as-is. 

### Behavior 
    1. Attempts to parse the input string as a URL.
    2. If successful:
        - Removes the fragment (e.g., `#section`).
        - Converts the URL to lowercase.
    3. If parsing fails, returns the input string unchanged.
*/
pub fn normalize(input: &str) -> String {
    if let Ok(mut u) = Url::parse(input) {
        u.set_fragment(None);
        u.as_str().to_lowercase()
    } else {
        input.to_string()
    }
}
/*
src/url_utils.rs
*/
use url::Url;

pub fn normalize(input: &str) -> String {
    if let Ok(mut u) = Url::parse(input) {
        u.set_fragment(None);
        u.as_str().to_lowercase()
    } else {
        input.to_string()
    }
}
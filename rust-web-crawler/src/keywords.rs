// Cargo Packages
use stop_words::{get, LANGUAGE};
use std::collections::HashMap;

/*
## `extract`
Extracts the most frequent keywords from a given text, excluding common stop words.
 
# Arguments
    * `text` - A string slice containing the text from which keywords will be extracted.

# Returns 
    * `Vec<String>` - A vector of up to 20 keywords sorted by frequency in descending order.
 
# Behavior
    1. Splits the input text into tokens using non-alphanumeric characters as delimiters.
    2. Converts tokens to lowercase and filters out:
        - Tokens with a length of 2 or fewer characters.
        - Tokens that are common stop words (based on the English language).
    3. Counts the frequency of each remaining token.
    4. Sorts the tokens by frequency in descending order.
    5. Returns the top 20 tokens as keywords.
*/
pub fn extract(text: &str) -> Vec<String> {
    let stops = get(LANGUAGE::English);
    let mut freq = HashMap::new();
    for token in text
        .split(|c: char| !c.is_alphanumeric())
        .map(|w| w.to_lowercase())
        .filter(|w| w.len() > 2 && !stops.contains(w))
    {
        *freq.entry(token.clone()).or_insert(0) += 1;
    }
    let mut keywords: Vec<_> = freq.into_iter().collect();
    keywords.sort_by(|a, b| b.1.cmp(&a.1));
    keywords.into_iter().take(20).map(|(w, _)| w).collect()
}
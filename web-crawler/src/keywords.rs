/*
src/keywords.rs
*/
use stop_words::{get, LANGUAGE};
use std::collections::HashMap;

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
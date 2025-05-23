// Cargo Packages
use scraper::{Html, Selector};
use url::Url;

pub fn extract_text(html: &str) -> String {
    let doc = Html::parse_document(html);
    let selector = Selector::parse("p, h1, h2, h3, h4, h5, h6, li").unwrap();
    doc.select(&selector)
        .flat_map(|el| el.text())
        .collect::<Vec<_>>()
        .join(" ")
}
pub fn extract_links(html: &str, base: &str) -> Vec<String> {
    let doc = Html::parse_document(html);
    let sel = Selector::parse("a[href]").unwrap();
    let base = Url::parse(base).unwrap();
    doc.select(&sel)
        .filter_map(|e| e.value().attr("href"))
        .filter_map(|href| base.join(href).ok())
        .map(|u| u.to_string())
        .collect()
}
// Cargo Packages
use scraper::{Html, Selector};
use url::Url;

/*
## `extract_text`
Extracts visible text content from an HTML document.
 
### Arguments 
    * `html` - A string slice containing the HTML content to parse.
 
### Returns 
    * `String` - A single string containing the concatenated text content of all selected elements. 

### Behavior 
    1. Parses the HTML document.
    2. Selects elements such as paragraphs (`<p>`) and headings (`<h1>` to `<h6>`), as well as list items (`<li>`).
    3. Extracts and concatenates the text content of the selected elements.
*/
pub fn extract_text(html: &str) -> String {
    let doc = Html::parse_document(html);
    let selector = Selector::parse("p, h1, h2, h3, h4, h5, h6, li").unwrap();
    doc.select(&selector)
        .flat_map(|el| el.text())
        .collect::<Vec<_>>()
        .join(" ")
}
/*
## `extract_links`
Extracts all hyperlinks (`<a href="...">`) from an HTML document and resolves them relative to a base URL.
 
### Arguments
    * `html` - A string slice containing the HTML content to parse.
    * `base` - A string slice representing the base URL for resolving relative links.
 
### Returns 
    * `Vec<String>` - A vector of fully resolved URLs as strings.
 
### Behavior 
    1. Parses the HTML document.
    2. Selects all anchor (`<a>`) elements with an `href` attribute.
    3. Resolves relative URLs against the provided base URL.
    4. Returns a list of fully resolved URLs.
*/
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
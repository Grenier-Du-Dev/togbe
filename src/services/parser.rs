use select::document::Document;
use select::predicate::{Attr, Name, Predicate};
use url::Url;
use crate::entities::link::Link;

use crate::data::queries::insert_page_content;

pub async fn extract_links(html: &str) -> Vec<Link> {

    let document = Document::from_read(html.as_bytes()).unwrap();
    document
        .find(Name("a"))
        .filter_map(|node| node.attr("href"))
        .flat_map(|url| {
            // Parse and normalize the URL
            let parsed_url = url.parse::<url::Url>();
            let normalized_url = parsed_url.map_or_else(|_| None, |u| Some(u));
            normalized_url.map(|u| Link {
                url: u,
                depth: 0,
            })
        })
        .collect()
}

pub fn extract_title(document: &Document) -> Option<String> {
    
    if let Some(title) = document.find(Name("title")).next().map(|n| n.text()) {
        Some(title)
    } else {
        None
    }
}

pub fn extract_language(document: &Document) -> Option<String> {
    document.find(Name("html")).next().and_then(|n| n.attr("lang").map(|lang| lang.to_string()))
}

pub fn extract_description(document: &Document) -> Option<String> {
    if let Some(node) = document.find(Name("meta").and(Attr("name", "description"))).next() {
        node.attr("content").map(|content| content.to_string())
    } else {
        None
    }
}

// Function to extract author from the HTML document
pub fn extract_author(document: &Document) -> Option<String> {
    
    if let Some(author_node) = document.find(Name("meta").and(Attr("name", "author"))).next() {
        // Extract the content attribute value if it exists
        return author_node.attr("content").map(|content| content.to_string());
    }

    // If the author information is not found, return None
    None
}

pub fn extract_site_name(document: &Document) -> Option<String> {
    
    if let Some(node) = document.find(Name("meta").and(Attr("property", "og:site_name"))).next() {
        // If the node is found, extract the content attribute, otherwise, return None
        node.attr("content").map(|content| content.to_string())
    } else {
        None
    }
}
pub fn extract_domain(url: &str) -> String {
    match Url::parse(url) {
        Ok(parsed_url) => parsed_url.domain().unwrap_or_default().to_string(),
        Err(_) => "Failed to extract domain".to_string(), 
    }
}


pub fn extract_page_size(html_content: &str) -> i32 {
    // Convert HTML content to bytes
    let html_bytes = html_content.as_bytes();

    // Get the length of the byte slice
    let size = html_bytes.len() as i32;

    size
}

pub fn extract_content(document: &Document) -> Option<String> {
    let mut content = String::new();

    // Extract content from h1, h2, h3, h4, p, and span elements
    for selector in ["h1", "h2", "h3", "p"].iter() {
        for node in document.find(Name(*selector)) {
            content.push_str(&node.text());
            content.push_str(" "); // Add a space between elements for separation
        }
    }

    // Check if any content was extracted
    if content.is_empty() {
        None
    } else {
        Some(content.trim().to_string())
    }
}

pub async fn process_link(link: &Link, body:String) {
    insert_page_content(&link.url.to_string(), link.depth as i32, &body).await;
}
#[allow(dead_code)]
use url::Url;

pub fn resolve_url(base_url: &str, href: &str) -> String {
    // Resolve relative URLs
    url::Url::parse(href)
        .or_else(|_| url::Url::parse(&format!("{}/{}", base_url, href)))
        .map(|url| url.into_string())
        .unwrap_or_else(|_| href.to_string())
}

use serde::{Serialize, Deserialize};


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WebPageContent {
    pub title: Option<String>,
    pub description: Option<String>,
    pub site_name: Option<String>,
    pub author: Option<String>,
    pub url: String,
    pub domain: String,
    pub language: Option<String>,
    pub page_size: i32,
    pub content: Option<String>,
}

use select::document::Document;
use std::fs;
use std::fs::OpenOptions;
use std::env;
use chrono::prelude::*;
use crate::entities::page_content;
use crate::services::parser::*;



pub async fn insert_page_content(url: &str, _depth: i32, content: &str) {
    let document = Document::from_read(content.as_bytes()).unwrap();
    let web_page = page_content::WebPageContent{
        title:extract_title(&document),
        description:extract_description(&document),
        site_name:extract_site_name(&document),
        author:extract_author(&document),
        url:url.to_string(),
        domain:extract_domain(url),
        language:extract_language(&document),
        content:extract_content(&document),
        page_size:extract_page_size(content)
    };

    let json_web_page = serde_json::to_string(&web_page).expect("Failed to serialize");
    let s1 = String::from("web_page_");
    let s2 = generate_unique_string();
    let s3 = ".json";
    let file_name = s1 + &s2 + &s3;

    if let Ok(current_dir) = env::current_dir() {
        let data_dir_name = "src/outpout";
        let data_dir_path = current_dir.join(data_dir_name);
        if !data_dir_path.exists() {
            fs::create_dir(&data_dir_path).expect("Failed to create data directory");
        }
        let file_path = data_dir_path.join(file_name.clone());
        let _created_file_name = OpenOptions::new()
                                                .write(true)
                                                .create(true)
                                                .open(file_path.clone()).expect("Failed to create json file");

        fs::write(file_path, json_web_page).expect("Unable to write to file");
    } else {
        println!("Failed to get current directory");
    }
}


fn generate_unique_string() -> String {
    let current_time = Utc::now();
    let timestamp = current_time.format("%Y%m%d%H%M%S").to_string();
    let nanos = current_time.timestamp_subsec_nanos();
    let random_component = rand::random::<u16>() % 10000;
    format!("{}{:04}{:04}", timestamp, nanos, random_component)
}

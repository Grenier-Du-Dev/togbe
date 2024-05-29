
use togbe::crawl;


#[tokio::main]
async fn main() {
   

    // initial urls to fetch pages
    let seed_urls = vec!["https://en.wikipedia.org/wiki/Rust_(programming_language)"];
    let crawl_depth = 3;
    let concurrent_requests = 10;
    crawl(seed_urls, crawl_depth, concurrent_requests).await;
}

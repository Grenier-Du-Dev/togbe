
use togbe::crawl;


#[tokio::main]
async fn main() {
   

    // initial urls to fetch pages
    let seed_urls = vec!["https://www.grenierdudev.com/"];
    let crawl_depth = 3;
    let concurrent_requests = 10;
    crawl(seed_urls, crawl_depth, concurrent_requests).await;
}

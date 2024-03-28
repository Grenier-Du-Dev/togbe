
mod data;
mod services;
mod entities;
use services::crawler::crawl;

#[tokio::main]
async fn main() {
   
   let seed_urls = vec![
    "https://www.togofirst.com/fr",
    "https://www.republiquetogolaise.com/",
    "https://www.newworldtv.com/",
    "https://burkina24.com/",
    "https://icilome.com/",
    "https://www.sidwaya.info/",
    "https://made-in-togo.com/accueil",
    "https://www.maliweb.net/",
    "https://www.lesahel.org/"
   ];

   let crawl_depth = 3;
   let concurrent_requests = 10;

   crawl(seed_urls, crawl_depth, concurrent_requests).await;
}

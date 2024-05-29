# Togbe
Togbe is a very experimental web crawler library in rust.  
Documentation coming soon.


# How to use
Install rust ([From rust-lang.org](https://www.rust-lang.org/tools/install)).


Download the project. In your `Cargo.toml` file:
```
[dependencies]
tokio = { version = "1.36.0", features = ["full"] }
togbe = {git = "https://github.com/Grenier-Du-Dev/togbe" ,branch = "main"}
```

In your `main.rs` file
```
use togbe::crawl;


#[tokio::main]
async fn main() {
   
    // initial urls to fetch pages
    let seed_urls = vec!["https://www.grenierdudev.com/"];
    let crawl_depth = 3;
    let concurrent_requests = 10;
    crawl(seed_urls, crawl_depth, concurrent_requests).await;
}
 
```

The result will be in `output` folder.
See the example folder for more use case.

# Project goals and non-goals:
Coming soon.

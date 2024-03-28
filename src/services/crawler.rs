use async_recursion::async_recursion;
use std::collections::{HashSet, VecDeque};
use url::Url;
use std::sync::Arc;
use tokio::sync::{mpsc, Mutex};


use crate::entities::link::Link;

use super::{
    fetcher::fetch, 
    parser::{
        extract_links,
        process_link
    },
};    


#[async_recursion]
pub async fn crawl(seed_urls: Vec<&'static str>, max_depth: usize, concurrent_requests: usize) {
    let visited_urls = Arc::new(Mutex::new(HashSet::new()));
    let queue = Arc::new(Mutex::new(VecDeque::new()));
    for seed_url in seed_urls {
        let link = Link {
            url: Url::parse(seed_url).expect("Invalid seed URL"),
            depth: 0,
        };
        queue.lock().await.push_back(link);
    }
    
    let (tx, mut rx) = mpsc::channel::<(String, Link)>(concurrent_requests);
    let mut handles = vec![];
    for _i in 0..concurrent_requests {
        let tx = tx.clone();
        let queue = Arc::clone(&queue);
        let visited_urls = Arc::clone(&visited_urls);
        let handle = tokio::spawn(async move {

            let mut queue_lock = queue.lock().await;
            while let Some(link) = queue_lock.pop_front() {
                let mut visited_urls = visited_urls.lock().await;
                if visited_urls.contains(link.url.as_str()) {
                    println!("WARNING :  {} already visited", link.url);
                    continue;
                }

                //sleep(Duration::from_secs(5)).await;
                match fetch(&link.url.to_string()).await {
                    Ok(body) => {
                        println!("[FETCH] : {}", link.url);   
                        let links = extract_links(&body).await;
                        for  (_i, mut new_link) in links.into_iter().enumerate() {
                            new_link.depth = link.depth + 1;
                            if new_link.depth <= max_depth {
                                queue_lock.push_back(new_link.clone());
                            }
                        }
                        let res = tx.send((body.clone(), link.clone())).await;
                        match res {
                            Ok(_response) =>{},
                            Err(_error) =>{
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("Failed to fetch {}: {}", link.url, e);
                    }
                }
                visited_urls.insert(link.url.to_string());
            }
        });
        handles.push(handle);
    }

    tokio::spawn(async move {
        while let Some((body, new_link)) = rx.recv().await {
            process_link(&new_link, body.clone()).await;
        }
    });

    for handle in handles {
        let res = handle.await;
        match res {
            Ok(_response) =>{},
            Err(error) =>{
                println!("ERROR : {:?}", error);
            }
        }
    }
}


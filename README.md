# Togbe
Togbe is a very experimental web crawler build in rust. Use it at your own risk.

# How it works 
There is a vec of initial url in the `main.rs` file to crawl. The result of each page is processed and parsed and output the json file in ouput folder. 
You can then do whatever you want with the json data, either send to another service or save in database.

# How to run
Clone or fork and clone the project.
Install rust ([From rust-lang.org](https://github.com/Speykious/cve-rs/blob/main/LICENSE)).
From the root directory, run 
```
cargo run
```

# Project goals and non-goals:
N/A

# Licence
[GLWTSPL](https://github.com/Speykious/cve-rs/blob/main/LICENSE)
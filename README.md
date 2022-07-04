# SerpApi Search in Rust
[![CI](https://github.com/serpapi/serpapi-search-rust/actions/workflows/ci.yml/badge.svg)](https://github.com/serpapi/serpapi-search-rust/actions/workflows/ci.yml) [![serpapi-search-rust](https://img.shields.io/crates/v/serpapi-search-rust.svg)](https://crates.io/crates/serpapi-search-rust)


This Rust package enables to scrape and parse search results from Google, Bing, Baidu, Yandex, Yahoo, Ebay, Apple, Youtube, Naver, Home depot and more. It's powered by [SerpApi](https://serpapi.com) which delivered a consistent JSON format accross search engines.
SerpApi.com enables to do localized search, leverage advanced search engine features and a lot more...
A completed documentation is available at [SerpApi](https://serpapi.com).

To install in your rust application, update Cargo.toml
```sh
serpapi-search-rust="0.1.0"
```

Basic application.
```rust
use serpapi_search_rust::serp_api_search::SerpApiSearch;
use std::collections::HashMap;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // read secret api key from environment variable
    // To get the key simply copy/paste from https://serpapi.com/dashboard.
    let api_key = match env::var_os("API_KEY") {
        Some(v) => v.into_string().unwrap(),
        None => panic!("$API_KEY is not set"),
    };

    println!("let's search about coffee on google");
    let mut params = HashMap::<String, String>::new();
    params.insert("q".to_string(), "coffee".to_string());
    params.insert("location".to_string(), "Austin, TX, Texas, United States".to_string());

    // initialize the search engine
    let search = SerpApiSearch::google(params, api_key);

    // search returns a JSON as serde_json::Value which can be accessed like a HashMap.
    println!("waiting...");
    let results = search.json().await?;
    let organic_results = results["organic_results"].as_array().unwrap();
    println!("results received");
    println!("--- JSON ---");
    println!(" - number of organic results: {}", organic_results.len());
    println!(" - organic_results first result description: {}", results["organic_results"][0]["about_this_result"]["source"]["description"]);
    let places = results["local_results"]["places"].as_array().unwrap();
    println!("number of local_results: {}", places.len());
    println!(" - local_results first address: {}", places[0]["address"]);

    // search returns text
    println!("--- HTML search ---");
    let raw = search.html().await?;
    print!(" - raw HTML size {} bytes\n", raw.len());
    print!(" - async search completed with {}\n", results["search_parameters"]["engine"]);

    // // edit the location in the search
    // println!("--- JSON search with a different location ---");
    // params = HashMap::<String, String>::new();
    // params.insert("location".to_string(), "Destin, Florida, United States".to_string());
    // search = SerpApiSearch::google(params, api_key);
    // let results = search.json().await?;
    // println!(">> search_parameters: {}", results["search_parameters"]);
    // let places = results["local_results"]["places"].as_array().unwrap();
    // println!("number of local_results: {}\n", places.len());
    // println!("local_results first address: {}\n", places[0]["address"]);

    print!("ok");
    Ok(())
}
```

To run an example:
```sh
cargo build --example google_search_example
```
file: (examples/google_search_example.rs)

The keyword google can be replaced by any supported search engine:
- google
- baidu
- bing
- duckduckgo
- yahoo
- yandex
- ebay
- youtube
- walmart
- home_depot
- apple_app_store
- naver

To run test.
```sh
cargo test
```

For more information how to build a paramaters HashMap see [serpapi.com documentation](https://serpapi.com/search-api)

### Technical features
- Dynamic JSON decoding using Serde JSON
- Asyncronous HTTP request handle method using tokio and reqwest
- Async tests using Tokio

## Reference
 * https://www.lpalmieri.com/posts/how-to-write-a-rest-client-in-rust-with-reqwest-and-wiremock/
 
### TODO
 - [ ] more test to close code coverage (each search engine)
 - [ ] add more examples
 - [ ] better documentation
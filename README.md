
# SerpApi Search in Rust
[![serpapi-rust](https://github.com/serpapi/serpapi-rust/actions/workflows/ci.yml/badge.svg)](https://github.com/serpapi/serpapi-rust/actions/workflows/ci.yml) [![serpapi-rust](https://img.shields.io/crates/v/serpapi.svg)](https://crates.io/crates/serpapi)

This Rust package enables to scrape and parse search results from Google, Bing, Baidu, Yandex, Yahoo, Ebay, Apple, Youtube, Naver, Home depot and more. It's powered by [SerpApi](https://serpapi.com) which delivered a consistent JSON format accross search engines.
SerpApi.com enables to do localized search, leverage advanced search engine features and a lot more...
A completed documentation is available at [SerpApi](https://serpapi.com).

## Installation

To install in your rust application, update Cargo.toml
```sh
serpapi="1.1.0"
```

## Usage

Let's start by searching for Coffee on Google:

```rust
// search example for google
//
use serpapi::serpapi::Client;
use std::collections::HashMap;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Read your private API Key from an environment variable.
    // Copy/paste from [https://serpapi.com/dashboard] to your shell:
    // ```bash
    // export API_key="paste_your_private_api_key"
    // ```
    let api_key = match env::var_os("SERPAPI_KEY") {
        Some(v) => v.into_string().unwrap(),
        None => panic!("$SERPAPI_KEY environment variable is not set!"),
    };

    println!("let's initiliaze the client to search on google");
    let mut default = HashMap::new();
    default.insert("api_key".to_string(), api_key);
    default.insert("engine".to_string(), "google".to_string());
    // initialize the search engine
    let client = Client::new(default).unwrap();

    // let's search for coffee in Austin, TX
    let mut parameter = HashMap::new();
    parameter.insert("q".to_string(), "coffee".to_string());
    parameter.insert("engine".to_string(), "google".to_string());
    // copy search parameter for the html search
    let mut html_parameter = HashMap::new();
    html_parameter.clone_from(&parameter);

    // search returns a JSON as serde_json::Value which can be accessed like a HashMap.
    println!("waiting...");
    let results = client.search(parameter).await?;
    let organic_results = results["organic_results"].as_array().unwrap();
    println!("results received");
    println!("--- JSON ---");
    let status = &results["search_metadata"]["status"];
    if status != "Success" {
        println!("search failed with status: {}", status);
    } else {
        println!("search is successfull");

        println!(" - number of organic_results: {}", organic_results.len());
        println!(
            " - organic_results first result description: {}",
            results["organic_results"][0]
        );

        // search returns text
        println!("--- HTML search ---");
        let raw = client.html(html_parameter).await.expect("html content");
        println!(" - raw HTML size {} bytes\n", raw.len());
        println!(
            " - async search completed with {}\n",
            results["search_parameters"]["engine"]
        );
    }

    print!("ok");
    Ok(())
}
```

[Google search documentation](https://serpapi.com/search-api).
More hands on examples are available below.

### Response formats

Use `search` for structured results decoded into a `serde_json::Value`:

```rust
let results = client.search(parameter).await?;
```

Use `md` for a token-efficient Markdown `String` optimized for LLMs and AI agents.
It returns clean headings, links and tables behind a YAML frontmatter while using
roughly half the tokens of the JSON output:

```rust
let markdown = client.md(parameter).await?;
```

Use `html` when you need the raw response from the search engine:

```rust
let raw_html = client.html(parameter).await?;
```

`md` and `html` always force the output format, so any `output` search parameter is ignored.
Archived results are also available as Markdown with `client.search_archive_md(&id).await?`.

Learn more about [SerpApi Markdown output](https://serpapi.com/markdown-output).

#### Documentations

 * [Full documentation on SerpApi.com](https://serpapi.com)
 * [API health status](https://serpapi.com/status)

For more information how to build a paramaters HashMap see [serpapi.com documentation](https://serpapi.com/search-api)

### Location API

```rust
let default = HashMap::<String, String>::new();
let client = Client::new(default).unwrap();
let mut parameter = HashMap::<String, String>::new();
parameter.insert("q".to_string(), "Austin".to_string());
let data = client.location(parameter).await.expect("request");
let locations = data.as_array().unwrap();
```

It returns the first 3 locations matching Austin (Texas, Texas, Rochester)


### Search Archive API

```rust
let mut default = HashMap::<String, String>::new();
default.insert("engine".to_string(), "google".to_string());
default.insert("api_key".to_string(), "your_secret_key".to_string());
let client = Client::new(default).unwrap();

// initialize the search engine
let mut parameter = HashMap::<String, String>::new();
parameter.insert("q".to_string(), "coffee".to_string());
parameter.insert(
    "location".to_string(),
    "Austin, TX, Texas, United States".to_string(),
);
let initial_results = client.search(parameter).await.expect("request");
let mut id = initial_results["search_metadata"]["id"].to_string();
// remove extra quote " from string convertion
id = id.replace("\"", "");

println!("{}", initial_results["search_metadata"]);
assert_ne!(id, "");

// search in archive
let archived_results = client.search_archive(&id).await.expect("request");
let archive_id = archived_results["search_metadata"]["id"].as_str();
let search_id = initial_results["search_metadata"]["id"].as_str();
println!("{}", archived_results);
assert_eq!(archive_id, search_id);
```

The same archived search is available as Markdown:

```rust
let markdown = client.search_archive_md(&id).await.expect("request");
```

### Account API
```rust
let client = Client::new(HashMap::<String, String>::new());
let mut parameter = HashMap::<String, String>::new();
parameter.insert("api_key".to_string(), "your_secret_key".to_string());
let account = client.account(parameter).await.expect("request");
```

It returns your account information.

### Technical features
- Search results as JSON with `search`, Markdown with `md`, or raw search engine HTML with `html`
- Dynamic JSON decoding using Serde JSON
- Asyncronous HTTP request handle method using tokio and reqwest
- Async tests using Tokio

### Changes log
- Unreleased:
  - Add Markdown output for LLMs and AI agents with `client.md(parameter)` and `client.search_archive_md(&id)`.
  - `client.html(parameter)` now calls the search endpoint with `output=html` which returns the raw search engine HTML.
- 1.1.0: Always reuse the same client object instead of creating a new one for each search.
  - This is a breaking change for the API because the client must be unwrapped in the main function.
    ```rust
    # now: 1.1
    let client = Client::new(default).unwrap();
    # old: 1.0
    let client = Client::new(default);
    ```
- 1.0.0: Initial release

### References
 * https://www.lpalmieri.com/posts/how-to-write-a-rest-client-in-rust-with-reqwest-and-wiremock/
 *  Serdes JSON

## Examples in rust

To run an example:
```sh
cargo build --example google_search
```
file: (examples/google_search.rs)

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

More example to come to match all search engines supported by [SerpApi.com.](https://serpapi.com/search-engine-apis)

### Search bing
```rust
let mut default = HashMap::new();
default.insert("api_key".to_string(), "your_secret_api_key".to_string());
default.insert("engine".to_string(), "bing".to_string());
// initialize the search engine
let client = Client::new(default).unwrap();

// let's search for coffee in Austin, TX
let mut parameter = HashMap::new();
parameter.insert("q".to_string(), "coffee".to_string());
// copy search parameter for the html search
let mut html_parameter = HashMap::new();
html_parameter.clone_from(&parameter);

// search returns a JSON as serde_json::Value which can be accessed like a HashMap.
println!("waiting...");
let results = client.search(parameter).await?;
let organic_results = results["organic_results"].as_array().unwrap();
println!("results received");
println!("--- JSON ---");
let status = &results["search_metadata"]["status"];
if status != "Success" {
    println!("search failed with status: {}", status);
} else {
    println!("search is successfull");

    println!(" - number of organic_results: {}", organic_results.len());
    println!(
        " - organic_results first result description: {}",
        results["organic_results"][0]
    );

    // search returns text
    println!("--- HTML search ---");
    println!(" - raw HTML size {} bytes\n", raw.len());
    println!(
        " - async search completed with {}\n",
        results["search_parameters"]["engine"]
    );
}

print!("ok");
Ok(())

```

 * source code: [examples/bing_search.rs](https://github.com/serpapi/serpapi-rust/blob/master/examples/bing_search.rs)
see: [https://serpapi.com/bing-search-api](https://serpapi.com/bing-search-api)

### Search baidu
```rust
let mut default = HashMap::new();
default.insert("api_key".to_string(), "your_secret_api_key".to_string());
default.insert("engine".to_string(), "baidu".to_string());
// initialize the search engine
let client = Client::new(default).unwrap();

// let's search for coffee in Austin, TX
let mut parameter = HashMap::new();
parameter.insert("q".to_string(), "coffee".to_string());
// copy search parameter for the html search
let mut html_parameter = HashMap::new();
html_parameter.clone_from(&parameter);

// search returns a JSON as serde_json::Value which can be accessed like a HashMap.
println!("waiting...");
let results = client.search(parameter).await?;
let organic_results = results["organic_results"].as_array().unwrap();
println!("results received");
println!("--- JSON ---");
let status = &results["search_metadata"]["status"];
if status != "Success" {
    println!("search failed with status: {}", status);
} else {
    println!("search is successfull");

    println!(" - number of organic_results: {}", organic_results.len());
    println!(
        " - organic_results first result description: {}",
        results["organic_results"][0]
    );

    // search returns text
    println!("--- HTML search ---");
    println!(" - raw HTML size {} bytes\n", raw.len());
    println!(
        " - async search completed with {}\n",
        results["search_parameters"]["engine"]
    );
}

print!("ok");
Ok(())

```

 * source code: [examples/baidu_search.rs](https://github.com/serpapi/serpapi-rust/blob/master/examples/baidu_search.rs)
see: [https://serpapi.com/baidu-search-api](https://serpapi.com/baidu-search-api)

### Search yahoo
```rust
let mut default = HashMap::new();
default.insert("api_key".to_string(), "your_secret_api_key".to_string());
default.insert("engine".to_string(), "yahoo".to_string());
// initialize the search engine
let client = Client::new(default).unwrap();

// let's search for coffee in Austin, TX
let mut parameter = HashMap::new();
parameter.insert("p".to_string(), "coffee".to_string());
// copy search parameter for the html search
let mut html_parameter = HashMap::new();
html_parameter.clone_from(&parameter);

// search returns a JSON as serde_json::Value which can be accessed like a HashMap.
println!("waiting...");
let results = client.search(parameter).await?;
let organic_results = results["organic_results"].as_array().unwrap();
println!("results received");
println!("--- JSON ---");
let status = &results["search_metadata"]["status"];
if status != "Success" {
    println!("search failed with status: {}", status);
} else {
    println!("search is successfull");

    println!(" - number of organic_results: {}", organic_results.len());
    println!(
        " - organic_results first result description: {}",
        results["organic_results"][0]
    );

    // search returns text
    println!("--- HTML search ---");
    println!(" - raw HTML size {} bytes\n", raw.len());
    println!(
        " - async search completed with {}\n",
        results["search_parameters"]["engine"]
    );
}

print!("ok");
Ok(())

```

 * source code: [examples/yahoo_search.rs](https://github.com/serpapi/serpapi-rust/blob/master/examples/yahoo_search.rs)
see: [https://serpapi.com/yahoo-search-api](https://serpapi.com/yahoo-search-api)

### Search youtube
```rust
let mut default = HashMap::new();
default.insert("api_key".to_string(), "your_secret_api_key".to_string());
default.insert("engine".to_string(), "youtube".to_string());
// initialize the search engine
let client = Client::new(default).unwrap();

// let's search for coffee in Austin, TX
let mut parameter = HashMap::new();
parameter.insert("search_query".to_string(), "coffee".to_string());
// copy search parameter for the html search
let mut html_parameter = HashMap::new();
html_parameter.clone_from(&parameter);

// search returns a JSON as serde_json::Value which can be accessed like a HashMap.
println!("waiting...");
let results = client.search(parameter).await?;
let video_results = results["video_results"].as_array().unwrap();
println!("results received");
println!("--- JSON ---");
let status = &results["search_metadata"]["status"];
if status != "Success" {
    println!("search failed with status: {}", status);
} else {
    println!("search is successfull");

    println!(" - number of video_results: {}", video_results.len());
    println!(
        " - video_results first result description: {}",
        results["video_results"][0]
    );

    // search returns text
    println!("--- HTML search ---");
    println!(" - raw HTML size {} bytes\n", raw.len());
    println!(
        " - async search completed with {}\n",
        results["search_parameters"]["engine"]
    );
}

print!("ok");
Ok(())

```

 * source code: [examples/youtube_search.rs](https://github.com/serpapi/serpapi-rust/blob/master/examples/youtube_search.rs)
see: [https://serpapi.com/youtube-search-api](https://serpapi.com/youtube-search-api)

### Search walmart
```rust
let mut default = HashMap::new();
default.insert("api_key".to_string(), "your_secret_api_key".to_string());
default.insert("engine".to_string(), "walmart".to_string());
// initialize the search engine
let client = Client::new(default).unwrap();

// let's search for coffee in Austin, TX
let mut parameter = HashMap::new();
parameter.insert("query".to_string(), "coffee".to_string());
// copy search parameter for the html search
let mut html_parameter = HashMap::new();
html_parameter.clone_from(&parameter);

// search returns a JSON as serde_json::Value which can be accessed like a HashMap.
println!("waiting...");
let results = client.search(parameter).await?;
let organic_results = results["organic_results"].as_array().unwrap();
println!("results received");
println!("--- JSON ---");
let status = &results["search_metadata"]["status"];
if status != "Success" {
    println!("search failed with status: {}", status);
} else {
    println!("search is successfull");

    println!(" - number of organic_results: {}", organic_results.len());
    println!(
        " - organic_results first result description: {}",
        results["organic_results"][0]
    );

    // search returns text
    println!("--- HTML search ---");
    println!(" - raw HTML size {} bytes\n", raw.len());
    println!(
        " - async search completed with {}\n",
        results["search_parameters"]["engine"]
    );
}

print!("ok");
Ok(())

```

 * source code: [examples/walmart_search.rs](https://github.com/serpapi/serpapi-rust/blob/master/examples/walmart_search.rs)
see: [https://serpapi.com/walmart-search-api](https://serpapi.com/walmart-search-api)

### Search ebay
```rust
let mut default = HashMap::new();
default.insert("api_key".to_string(), "your_secret_api_key".to_string());
default.insert("engine".to_string(), "ebay".to_string());
// initialize the search engine
let client = Client::new(default).unwrap();

// let's search for coffee in Austin, TX
let mut parameter = HashMap::new();
parameter.insert("_nkw".to_string(), "coffee".to_string());
// copy search parameter for the html search
let mut html_parameter = HashMap::new();
html_parameter.clone_from(&parameter);

// search returns a JSON as serde_json::Value which can be accessed like a HashMap.
println!("waiting...");
let results = client.search(parameter).await?;
let organic_results = results["organic_results"].as_array().unwrap();
println!("results received");
println!("--- JSON ---");
let status = &results["search_metadata"]["status"];
if status != "Success" {
    println!("search failed with status: {}", status);
} else {
    println!("search is successfull");

    println!(" - number of organic_results: {}", organic_results.len());
    println!(
        " - organic_results first result description: {}",
        results["organic_results"][0]
    );

    // search returns text
    println!("--- HTML search ---");
    println!(" - raw HTML size {} bytes\n", raw.len());
    println!(
        " - async search completed with {}\n",
        results["search_parameters"]["engine"]
    );
}

print!("ok");
Ok(())

```

 * source code: [examples/ebay_search.rs](https://github.com/serpapi/serpapi-rust/blob/master/examples/ebay_search.rs)
see: [https://serpapi.com/ebay-search-api](https://serpapi.com/ebay-search-api)

### Search naver
```rust
let mut default = HashMap::new();
default.insert("api_key".to_string(), "your_secret_api_key".to_string());
default.insert("engine".to_string(), "naver".to_string());
// initialize the search engine
let client = Client::new(default).unwrap();

// let's search for coffee in Austin, TX
let mut parameter = HashMap::new();
parameter.insert("query".to_string(), "coffee".to_string());
// copy search parameter for the html search
let mut html_parameter = HashMap::new();
html_parameter.clone_from(&parameter);

// search returns a JSON as serde_json::Value which can be accessed like a HashMap.
println!("waiting...");
let results = client.search(parameter).await?;
let ads_results = results["ads_results"].as_array().unwrap();
println!("results received");
println!("--- JSON ---");
let status = &results["search_metadata"]["status"];
if status != "Success" {
    println!("search failed with status: {}", status);
} else {
    println!("search is successfull");

    println!(" - number of ads_results: {}", ads_results.len());
    println!(
        " - ads_results first result description: {}",
        results["ads_results"][0]
    );

    // search returns text
    println!("--- HTML search ---");
    println!(" - raw HTML size {} bytes\n", raw.len());
    println!(
        " - async search completed with {}\n",
        results["search_parameters"]["engine"]
    );
}

print!("ok");
Ok(())

```

 * source code: [examples/naver_search.rs](https://github.com/serpapi/serpapi-rust/blob/master/examples/naver_search.rs)
see: [https://serpapi.com/naver-search-api](https://serpapi.com/naver-search-api)

### Search home depot
```rust
let mut default = HashMap::new();
default.insert("api_key".to_string(), "your_secret_api_key".to_string());
default.insert("engine".to_string(), "home_depot".to_string());
// initialize the search engine
let client = Client::new(default).unwrap();

// let's search for coffee in Austin, TX
let mut parameter = HashMap::new();
parameter.insert("q".to_string(), "table".to_string());
// copy search parameter for the html search
let mut html_parameter = HashMap::new();
html_parameter.clone_from(&parameter);

// search returns a JSON as serde_json::Value which can be accessed like a HashMap.
println!("waiting...");
let results = client.search(parameter).await?;
let products = results["products"].as_array().unwrap();
println!("results received");
println!("--- JSON ---");
let status = &results["search_metadata"]["status"];
if status != "Success" {
    println!("search failed with status: {}", status);
} else {
    println!("search is successfull");

    println!(" - number of products: {}", products.len());
    println!(
        " - products first result description: {}",
        results["products"][0]
    );

    // search returns text
    println!("--- HTML search ---");
    println!(" - raw HTML size {} bytes\n", raw.len());
    println!(
        " - async search completed with {}\n",
        results["search_parameters"]["engine"]
    );
}

print!("ok");
Ok(())

```

 * source code: [examples/home_depot_search.rs](https://github.com/serpapi/serpapi-rust/blob/master/examples/home_depot_search.rs)
see: [https://serpapi.com/home-depot-search-api](https://serpapi.com/home-depot-search-api)

### Search apple app store
```rust
let mut default = HashMap::new();
default.insert("api_key".to_string(), "your_secret_api_key".to_string());
default.insert("engine".to_string(), "apple_app_store".to_string());
// initialize the search engine
let client = Client::new(default).unwrap();

// let's search for coffee in Austin, TX
let mut parameter = HashMap::new();
parameter.insert("term".to_string(), "coffee".to_string());
// copy search parameter for the html search
let mut html_parameter = HashMap::new();
html_parameter.clone_from(&parameter);

// search returns a JSON as serde_json::Value which can be accessed like a HashMap.
println!("waiting...");
let results = client.search(parameter).await?;
let organic_results = results["organic_results"].as_array().unwrap();
println!("results received");
println!("--- JSON ---");
let status = &results["search_metadata"]["status"];
if status != "Success" {
    println!("search failed with status: {}", status);
} else {
    println!("search is successfull");

    println!(" - number of organic_results: {}", organic_results.len());
    println!(
        " - organic_results first result description: {}",
        results["organic_results"][0]
    );

    // search returns text
    println!("--- HTML search ---");
    println!(" - raw HTML size {} bytes\n", raw.len());
    println!(
        " - async search completed with {}\n",
        results["search_parameters"]["engine"]
    );
}

print!("ok");
Ok(())

```

 * source code: [examples/apple_app_store_search.rs](https://github.com/serpapi/serpapi-rust/blob/master/examples/apple_app_store_search.rs)
see: [https://serpapi.com/apple-app-store](https://serpapi.com/apple-app-store)

### Search duckduckgo
```rust
let mut default = HashMap::new();
default.insert("api_key".to_string(), "your_secret_api_key".to_string());
default.insert("engine".to_string(), "duckduckgo".to_string());
// initialize the search engine
let client = Client::new(default).unwrap();

// let's search for coffee in Austin, TX
let mut parameter = HashMap::new();
parameter.insert("q".to_string(), "coffee".to_string());
// copy search parameter for the html search
let mut html_parameter = HashMap::new();
html_parameter.clone_from(&parameter);

// search returns a JSON as serde_json::Value which can be accessed like a HashMap.
println!("waiting...");
let results = client.search(parameter).await?;
let organic_results = results["organic_results"].as_array().unwrap();
println!("results received");
println!("--- JSON ---");
let status = &results["search_metadata"]["status"];
if status != "Success" {
    println!("search failed with status: {}", status);
} else {
    println!("search is successfull");

    println!(" - number of organic_results: {}", organic_results.len());
    println!(
        " - organic_results first result description: {}",
        results["organic_results"][0]
    );

    // search returns text
    println!("--- HTML search ---");
    println!(" - raw HTML size {} bytes\n", raw.len());
    println!(
        " - async search completed with {}\n",
        results["search_parameters"]["engine"]
    );
}

print!("ok");
Ok(())

```

 * source code: [examples/duckduckgo_search.rs](https://github.com/serpapi/serpapi-rust/blob/master/examples/duckduckgo_search.rs)
see: [https://serpapi.com/duckduckgo-search-api](https://serpapi.com/duckduckgo-search-api)

### Search google
```rust
let mut default = HashMap::new();
default.insert("api_key".to_string(), "your_secret_api_key".to_string());
default.insert("engine".to_string(), "google".to_string());
// initialize the search engine
let client = Client::new(default).unwrap();

// let's search for coffee in Austin, TX
let mut parameter = HashMap::new();
parameter.insert("q".to_string(), "coffee".to_string());
parameter.insert("engine".to_string(), "google".to_string());
// copy search parameter for the html search
let mut html_parameter = HashMap::new();
html_parameter.clone_from(&parameter);

// search returns a JSON as serde_json::Value which can be accessed like a HashMap.
println!("waiting...");
let results = client.search(parameter).await?;
let organic_results = results["organic_results"].as_array().unwrap();
println!("results received");
println!("--- JSON ---");
let status = &results["search_metadata"]["status"];
if status != "Success" {
    println!("search failed with status: {}", status);
} else {
    println!("search is successfull");

    println!(" - number of organic_results: {}", organic_results.len());
    println!(
        " - organic_results first result description: {}",
        results["organic_results"][0]
    );

    // search returns text
    println!("--- HTML search ---");
    println!(" - raw HTML size {} bytes\n", raw.len());
    println!(
        " - async search completed with {}\n",
        results["search_parameters"]["engine"]
    );
}

print!("ok");
Ok(())

```

 * source code: [examples/google_search.rs](https://github.com/serpapi/serpapi-rust/blob/master/examples/google_search.rs)
see: [https://serpapi.com/search-api](https://serpapi.com/search-api)

### Search google scholar
```rust
let mut default = HashMap::new();
default.insert("api_key".to_string(), "your_secret_api_key".to_string());
default.insert("engine".to_string(), "google_scholar".to_string());
// initialize the search engine
let client = Client::new(default).unwrap();

// let's search for coffee in Austin, TX
let mut parameter = HashMap::new();
parameter.insert("q".to_string(), "coffee".to_string());
// copy search parameter for the html search
let mut html_parameter = HashMap::new();
html_parameter.clone_from(&parameter);

// search returns a JSON as serde_json::Value which can be accessed like a HashMap.
println!("waiting...");
let results = client.search(parameter).await?;
let organic_results = results["organic_results"].as_array().unwrap();
println!("results received");
println!("--- JSON ---");
let status = &results["search_metadata"]["status"];
if status != "Success" {
    println!("search failed with status: {}", status);
} else {
    println!("search is successfull");

    println!(" - number of organic_results: {}", organic_results.len());
    println!(
        " - organic_results first result description: {}",
        results["organic_results"][0]
    );

    // search returns text
    println!("--- HTML search ---");
    println!(" - raw HTML size {} bytes\n", raw.len());
    println!(
        " - async search completed with {}\n",
        results["search_parameters"]["engine"]
    );
}

print!("ok");
Ok(())

```

 * source code: [examples/google_scholar_search.rs](https://github.com/serpapi/serpapi-rust/blob/master/examples/google_scholar_search.rs)
see: [https://serpapi.com/google-scholar-api](https://serpapi.com/google-scholar-api)

### Search google autocomplete
```rust
let mut default = HashMap::new();
default.insert("api_key".to_string(), "your_secret_api_key".to_string());
default.insert("engine".to_string(), "google_autocomplete".to_string());
// initialize the search engine
let client = Client::new(default).unwrap();

// let's search for coffee in Austin, TX
let mut parameter = HashMap::new();
parameter.insert("q".to_string(), "coffee".to_string());
// copy search parameter for the html search
let mut html_parameter = HashMap::new();
html_parameter.clone_from(&parameter);

// search returns a JSON as serde_json::Value which can be accessed like a HashMap.
println!("waiting...");
let results = client.search(parameter).await?;
let suggestions = results["suggestions"].as_array().unwrap();
println!("results received");
println!("--- JSON ---");
let status = &results["search_metadata"]["status"];
if status != "Success" {
    println!("search failed with status: {}", status);
} else {
    println!("search is successfull");

    println!(" - number of suggestions: {}", suggestions.len());
    println!(
        " - suggestions first result description: {}",
        results["suggestions"][0]
    );

    // search returns text
    println!("--- HTML search ---");
    println!(" - raw HTML size {} bytes\n", raw.len());
    println!(
        " - async search completed with {}\n",
        results["search_parameters"]["engine"]
    );
}

print!("ok");
Ok(())

```

 * source code: [examples/google_autocomplete_search.rs](https://github.com/serpapi/serpapi-rust/blob/master/examples/google_autocomplete_search.rs)
see: [https://serpapi.com/google-autocomplete-api](https://serpapi.com/google-autocomplete-api)

### Search google product
```rust
let mut default = HashMap::new();
default.insert("api_key".to_string(), "your_secret_api_key".to_string());
default.insert("engine".to_string(), "google_product".to_string());
// initialize the search engine
let client = Client::new(default).unwrap();

// let's search for coffee in Austin, TX
let mut parameter = HashMap::new();
parameter.insert("q".to_string(), "coffee".to_string());
parameter.insert("product_id".to_string(), "4887235756540435899".to_string());
// copy search parameter for the html search
let mut html_parameter = HashMap::new();
html_parameter.clone_from(&parameter);

// search returns a JSON as serde_json::Value which can be accessed like a HashMap.
println!("waiting...");
let results = client.search(parameter).await?;
let product_results = results["product_results"].as_array().unwrap();
println!("results received");
println!("--- JSON ---");
let status = &results["search_metadata"]["status"];
if status != "Success" {
    println!("search failed with status: {}", status);
} else {
    println!("search is successfull");

    println!(" - number of product_results: {}", product_results.len());
    println!(
        " - product_results first result description: {}",
        results["product_results"][0]
    );

    // search returns text
    println!("--- HTML search ---");
    println!(" - raw HTML size {} bytes\n", raw.len());
    println!(
        " - async search completed with {}\n",
        results["search_parameters"]["engine"]
    );
}

print!("ok");
Ok(())

```

 * source code: [examples/google_product_search.rs](https://github.com/serpapi/serpapi-rust/blob/master/examples/google_product_search.rs)
see: [https://serpapi.com/google-product-api](https://serpapi.com/google-product-api)

### Search google reverse image
```rust
let mut default = HashMap::new();
default.insert("api_key".to_string(), "your_secret_api_key".to_string());
default.insert("engine".to_string(), "google_reverse_image".to_string());
// initialize the search engine
let client = Client::new(default).unwrap();

// let's search for coffee in Austin, TX
let mut parameter = HashMap::new();
parameter.insert(
    "image_url".to_string(),
    "https://i.imgur.com/5bGzZi7.jpg".to_string(),
);
parameter.insert("max_results".to_string(), "1".to_string());
// copy search parameter for the html search
let mut html_parameter = HashMap::new();
html_parameter.clone_from(&parameter);

// search returns a JSON as serde_json::Value which can be accessed like a HashMap.
println!("waiting...");
let results = client.search(parameter).await?;
let image_sizes = results["image_sizes"].as_array().unwrap();
println!("results received");
println!("--- JSON ---");
let status = &results["search_metadata"]["status"];
if status != "Success" {
    println!("search failed with status: {}", status);
} else {
    println!("search is successfull");

    println!(" - number of image_sizes: {}", image_sizes.len());
    println!(
        " - image_sizes first result description: {}",
        results["image_sizes"][0]
    );

    // search returns text
    println!("--- HTML search ---");
    println!(" - raw HTML size {} bytes\n", raw.len());
    println!(
        " - async search completed with {}\n",
        results["search_parameters"]["engine"]
    );
}

print!("ok");
Ok(())

```

 * source code: [examples/google_reverse_image_search.rs](https://github.com/serpapi/serpapi-rust/blob/master/examples/google_reverse_image_search.rs)
see: [https://serpapi.com/google-reverse-image](https://serpapi.com/google-reverse-image)

### Search google events
```rust
let mut default = HashMap::new();
default.insert("api_key".to_string(), "your_secret_api_key".to_string());
default.insert("engine".to_string(), "google_events".to_string());
// initialize the search engine
let client = Client::new(default).unwrap();

// let's search for coffee in Austin, TX
let mut parameter = HashMap::new();
parameter.insert("q".to_string(), "coffee".to_string());
// copy search parameter for the html search
let mut html_parameter = HashMap::new();
html_parameter.clone_from(&parameter);

// search returns a JSON as serde_json::Value which can be accessed like a HashMap.
println!("waiting...");
let results = client.search(parameter).await?;
let events_results = results["events_results"].as_array().unwrap();
println!("results received");
println!("--- JSON ---");
let status = &results["search_metadata"]["status"];
if status != "Success" {
    println!("search failed with status: {}", status);
} else {
    println!("search is successfull");

    println!(" - number of events_results: {}", events_results.len());
    println!(
        " - events_results first result description: {}",
        results["events_results"][0]
    );

    // search returns text
    println!("--- HTML search ---");
    println!(" - raw HTML size {} bytes\n", raw.len());
    println!(
        " - async search completed with {}\n",
        results["search_parameters"]["engine"]
    );
}

print!("ok");
Ok(())

```

 * source code: [examples/google_events_search.rs](https://github.com/serpapi/serpapi-rust/blob/master/examples/google_events_search.rs)
see: [https://serpapi.com/google-events-api](https://serpapi.com/google-events-api)

### Search google local services
```rust
let mut default = HashMap::new();
default.insert("api_key".to_string(), "your_secret_api_key".to_string());
default.insert("engine".to_string(), "google_local_services".to_string());
// initialize the search engine
let client = Client::new(default).unwrap();

// let's search for coffee in Austin, TX
let mut parameter = HashMap::new();
parameter.insert("q".to_string(), "electrician".to_string());
parameter.insert("data_cid".to_string(), "6745062158417646970".to_string());
// copy search parameter for the html search
let mut html_parameter = HashMap::new();
html_parameter.clone_from(&parameter);

// search returns a JSON as serde_json::Value which can be accessed like a HashMap.
println!("waiting...");
let results = client.search(parameter).await?;
let local_ads = results["local_ads"].as_array().unwrap();
println!("results received");
println!("--- JSON ---");
let status = &results["search_metadata"]["status"];
if status != "Success" {
    println!("search failed with status: {}", status);
} else {
    println!("search is successfull");

    println!(" - number of local_ads: {}", local_ads.len());
    println!(
        " - local_ads first result description: {}",
        results["local_ads"][0]
    );

    // search returns text
    println!("--- HTML search ---");
    println!(" - raw HTML size {} bytes\n", raw.len());
    println!(
        " - async search completed with {}\n",
        results["search_parameters"]["engine"]
    );
}

print!("ok");
Ok(())

```

 * source code: [examples/google_local_services_search.rs](https://github.com/serpapi/serpapi-rust/blob/master/examples/google_local_services_search.rs)
see: [https://serpapi.com/google-local-services-api](https://serpapi.com/google-local-services-api)

### Search google maps
```rust
let mut default = HashMap::new();
default.insert("api_key".to_string(), "your_secret_api_key".to_string());
default.insert("engine".to_string(), "google_maps".to_string());
// initialize the search engine
let client = Client::new(default).unwrap();

// let's search for coffee in Austin, TX
let mut parameter = HashMap::new();
parameter.insert("q".to_string(), "pizza".to_string());
parameter.insert(
    "ll".to_string(),
    "@40.7455096,-74.0083012,15.1z".to_string(),
);
parameter.insert("type".to_string(), "search".to_string());
// copy search parameter for the html search
let mut html_parameter = HashMap::new();
html_parameter.clone_from(&parameter);

// search returns a JSON as serde_json::Value which can be accessed like a HashMap.
println!("waiting...");
let results = client.search(parameter).await?;
let local_results = results["local_results"].as_array().unwrap();
println!("results received");
println!("--- JSON ---");
let status = &results["search_metadata"]["status"];
if status != "Success" {
    println!("search failed with status: {}", status);
} else {
    println!("search is successfull");

    println!(" - number of local_results: {}", local_results.len());
    println!(
        " - local_results first result description: {}",
        results["local_results"][0]
    );

    // search returns text
    println!("--- HTML search ---");
    println!(" - raw HTML size {} bytes\n", raw.len());
    println!(
        " - async search completed with {}\n",
        results["search_parameters"]["engine"]
    );
}

print!("ok");
Ok(())

```

 * source code: [examples/google_maps_search.rs](https://github.com/serpapi/serpapi-rust/blob/master/examples/google_maps_search.rs)
see: [https://serpapi.com/google-maps-api](https://serpapi.com/google-maps-api)

### Search google jobs
```rust
let mut default = HashMap::new();
default.insert("api_key".to_string(), "your_secret_api_key".to_string());
default.insert("engine".to_string(), "google_jobs".to_string());
// initialize the search engine
let client = Client::new(default).unwrap();

// let's search for coffee in Austin, TX
let mut parameter = HashMap::new();
parameter.insert("q".to_string(), "coffee".to_string());
// copy search parameter for the html search
let mut html_parameter = HashMap::new();
html_parameter.clone_from(&parameter);

// search returns a JSON as serde_json::Value which can be accessed like a HashMap.
println!("waiting...");
let results = client.search(parameter).await?;
let jobs_results = results["jobs_results"].as_array().unwrap();
println!("results received");
println!("--- JSON ---");
let status = &results["search_metadata"]["status"];
if status != "Success" {
    println!("search failed with status: {}", status);
} else {
    println!("search is successfull");

    println!(" - number of jobs_results: {}", jobs_results.len());
    println!(
        " - jobs_results first result description: {}",
        results["jobs_results"][0]
    );

    // search returns text
    println!("--- HTML search ---");
    println!(" - raw HTML size {} bytes\n", raw.len());
    println!(
        " - async search completed with {}\n",
        results["search_parameters"]["engine"]
    );
}

print!("ok");
Ok(())

```

 * source code: [examples/google_jobs_search.rs](https://github.com/serpapi/serpapi-rust/blob/master/examples/google_jobs_search.rs)
see: [https://serpapi.com/google-jobs-api](https://serpapi.com/google-jobs-api)

### Search google play
```rust
let mut default = HashMap::new();
default.insert("api_key".to_string(), "your_secret_api_key".to_string());
default.insert("engine".to_string(), "google_play".to_string());
// initialize the search engine
let client = Client::new(default).unwrap();

// let's search for coffee in Austin, TX
let mut parameter = HashMap::new();
parameter.insert("q".to_string(), "kite".to_string());
parameter.insert("store".to_string(), "apps".to_string());
parameter.insert("max_results".to_string(), "2".to_string());
// copy search parameter for the html search
let mut html_parameter = HashMap::new();
html_parameter.clone_from(&parameter);

// search returns a JSON as serde_json::Value which can be accessed like a HashMap.
println!("waiting...");
let results = client.search(parameter).await?;
let organic_results = results["organic_results"].as_array().unwrap();
println!("results received");
println!("--- JSON ---");
let status = &results["search_metadata"]["status"];
if status != "Success" {
    println!("search failed with status: {}", status);
} else {
    println!("search is successfull");

    println!(" - number of organic_results: {}", organic_results.len());
    println!(
        " - organic_results first result description: {}",
        results["organic_results"][0]
    );

    // search returns text
    println!("--- HTML search ---");
    println!(" - raw HTML size {} bytes\n", raw.len());
    println!(
        " - async search completed with {}\n",
        results["search_parameters"]["engine"]
    );
}

print!("ok");
Ok(())

```

 * source code: [examples/google_play_search.rs](https://github.com/serpapi/serpapi-rust/blob/master/examples/google_play_search.rs)
see: [https://serpapi.com/google-play-api](https://serpapi.com/google-play-api)

### Search google images
```rust
let mut default = HashMap::new();
default.insert("api_key".to_string(), "your_secret_api_key".to_string());
default.insert("engine".to_string(), "google_images".to_string());
// initialize the search engine
let client = Client::new(default).unwrap();

// let's search for coffee in Austin, TX
let mut parameter = HashMap::new();
parameter.insert("engine".to_string(), "google_images".to_string());
parameter.insert("tbm".to_string(), "isch".to_string());
parameter.insert("q".to_string(), "coffee".to_string());
// copy search parameter for the html search
let mut html_parameter = HashMap::new();
html_parameter.clone_from(&parameter);

// search returns a JSON as serde_json::Value which can be accessed like a HashMap.
println!("waiting...");
let results = client.search(parameter).await?;
let images_results = results["images_results"].as_array().unwrap();
println!("results received");
println!("--- JSON ---");
let status = &results["search_metadata"]["status"];
if status != "Success" {
    println!("search failed with status: {}", status);
} else {
    println!("search is successfull");

    println!(" - number of images_results: {}", images_results.len());
    println!(
        " - images_results first result description: {}",
        results["images_results"][0]
    );

    // search returns text
    println!("--- HTML search ---");
    println!(" - raw HTML size {} bytes\n", raw.len());
    println!(
        " - async search completed with {}\n",
        results["search_parameters"]["engine"]
    );
}

print!("ok");
Ok(())

```

 * source code: [examples/google_images_search.rs](https://github.com/serpapi/serpapi-rust/blob/master/examples/google_images_search.rs)
see: [https://serpapi.com/images-results](https://serpapi.com/images-results)

## License

MIT License

## Continuous integration
We love "true open source" and "continuous integration", and Test Drive Development (TDD).
 We are using RSpec to test [our infrastructure around the clock]) using Github Action to achieve the best QoS (Quality Of Service).

The directory spec/ includes specification which serves the dual purposes of examples and functional tests.

Set your secret API key in your shell before running a test.
```bash
export API_KEY="your_secret_key"
cargo test
```

Contributions are welcome. Feel to submit a pull request!

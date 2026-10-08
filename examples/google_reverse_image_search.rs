// search example for google_reverse_image
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

    println!("let's initiliaze the client to search on google_reverse_image");
    let mut default = HashMap::new();
    default.insert("api_key".to_string(), api_key);
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

        // pagination: fetch up to 2 more pages
        println!("--- Pagination ---");
        let mut next = results["serpapi_pagination"]["next"]
            .as_str()
            .map(String::from);
        for page in 2..=3 {
            let Some(url) = next else { break };
            let page_parameter: HashMap<String, String> = reqwest::Url::parse(&url)?
                .query_pairs()
                .into_owned()
                .collect();
            let page_results = client.search(page_parameter).await?;
            let count = page_results["image_sizes"]
                .as_array()
                .map_or(0, |r| r.len());
            println!(" - page {}: {} image_sizes", page, count);
            next = page_results["serpapi_pagination"]["next"]
                .as_str()
                .map(String::from);
        }

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

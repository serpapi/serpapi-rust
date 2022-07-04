use serpapi::serpapi::SerpApiClient;
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
    let mut default = HashMap::new();
    default.insert("api_key".to_string(), api_key);
    default.insert("q".to_string(), "coffee".to_string());
    // initialize the search engine
    let client = SerpApiClient::new(default);
    
    let mut parameter = HashMap::new();
    parameter.insert(
        "location".to_string(),
        "Austin, TX, Texas, United States".to_string(),
    );
    let mut html_parameter = HashMap::new();
    html_parameter.clone_from(&parameter);

    // search returns a JSON as serde_json::Value which can be accessed like a HashMap.
    println!("waiting...");
    let results = client.search(parameter).await?;
    let organic_results = results["organic_results"].as_array().unwrap();
    println!("results received");
    println!("--- JSON ---");
    println!(" - number of organic results: {}", organic_results.len());
    println!(
        " - organic_results first result description: {}",
        results["organic_results"][0]["about_this_result"]["source"]["description"]
    );
    let places = results["local_results"]["places"].as_array().unwrap();
    println!("number of local_results: {}", places.len());
    println!(" - local_results first address: {}", places[0]["address"]);

    // search returns text
    println!("--- HTML search ---");
    let raw = client.html(html_parameter).await?;
    print!(" - raw HTML size {} bytes\n", raw.len());
    print!(
        " - async search completed with {}\n",
        results["search_parameters"]["engine"]
    );

    // // edit the location in the search
    // println!("--- JSON search with a different location ---");
    // parameter = HashMap::<String, String>::new();
    // parameter.insert("location".to_string(), "Destin, Florida, United States".to_string());
    // search = SerpApiSearch::google(parameter, api_key);
    // let results = search.json().await?;
    // println!(">> search_parameters: {}", results["search_parameters"]);
    // let places = results["local_results"]["places"].as_array().unwrap();
    // println!("number of local_results: {}\n", places.len());
    // println!("local_results first address: {}\n", places[0]["address"]);

    print!("ok");
    Ok(())
}

// reference example google search
use serpapi::serpapi::Client;
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
    default.insert("engine".to_string(), "youtube".to_string());
    // initialize the search engine
    let client = Client::new(default);

    let mut parameter = HashMap::new();
    parameter.insert("search_query".to_string(), "coffee".to_string());

    let mut html_parameter = HashMap::new();
    println!("{:?}", parameter);
    html_parameter.clone_from(&parameter);
    println!("{:?}", html_parameter);

    // search returns a JSON as serde_json::Value which can be accessed like a HashMap.
    println!("waiting...");
    let results = client.search(parameter).await?;
    let video_results = results["video_results"].as_array().unwrap();
    println!("results received");
    println!("--- JSON ---");
    println!(" - number of video results: {}", video_results.len());
    println!(
        " - video_results first result description: {}",
        results["video_results"][0]["about_this_result"]["source"]["description"]
    );
    print!(
        " - async search completed with {:?}\n",
        results["search_parameters"]
    );

    // search returns text
    println!("--- HTML search ---");
    println!("{:?}", html_parameter);
    let raw = client.html(html_parameter).await?;
    println!(">> {:?}", raw);
    print!(" - raw HTML size {} bytes\n", raw.len());

    // // edit the location in the search
    // println!("--- JSON search with a different location ---");
    // parameter = HashMap::<String, String>::new();
    // parameter.insert("location".to_string(), "Destin, Florida, United States".to_string());
    // client = Client::google(parameter, api_key);
    // let results = client.json().await?;
    // println!(">> search_parameters: {}", results["search_parameters"]);
    // let places = results["local_results"]["places"].as_array().unwrap();
    // println!("number of local_results: {}\n", places.len());
    // println!("local_results first address: {}\n", places[0]["address"]);

    print!("ok");
    Ok(())
}

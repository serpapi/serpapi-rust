// search example for google_lens with an uploaded image
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

    println!("let's initiliaze the client to search on google_lens");
    let mut default = HashMap::new();
    default.insert("api_key".to_string(), api_key);
    default.insert("engine".to_string(), "google_lens".to_string());
    // initialize the search engine
    let client = Client::new(default).unwrap();

    // upload the image given on the command line,
    //  or download a sample image when no path is provided:
    //  cargo run --example google_lens_upload_image -- ./image.jpg
    println!("uploading...");
    let upload = match env::args().nth(1) {
        Some(path) => client.upload_image(&path, HashMap::new()).await?,
        None => {
            let image = reqwest::get("https://i.imgur.com/5bGzZi7.jpg")
                .await?
                .bytes()
                .await?
                .to_vec();
            client
                .upload_image_bytes(image, "image.jpg", HashMap::new())
                .await?
        }
    };
    let image_id = upload["image_id"].as_str().unwrap();
    println!(" - image uploaded with id: {}", image_id);

    // let's search with google lens using the uploaded image
    let mut parameter = HashMap::new();
    parameter.insert("image_id".to_string(), image_id.to_string());

    // search returns a JSON as serde_json::Value which can be accessed like a HashMap.
    println!("waiting...");
    let results = client.search(parameter).await?;
    println!("results received");
    println!("--- JSON ---");
    let status = &results["search_metadata"]["status"];
    if status != "Success" {
        println!("search failed with status: {}", status);
    } else {
        println!("search is successfull");
        let visual_matches = results["visual_matches"].as_array().unwrap();
        println!(" - number of visual_matches: {}", visual_matches.len());
        println!(
            " - visual_matches first result description: {}",
            results["visual_matches"][0]
        );
        println!(
            " - async search completed with {}\n",
            results["search_parameters"]["engine"]
        );
    }

    print!("ok");
    Ok(())
}

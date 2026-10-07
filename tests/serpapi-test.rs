//#[cfg(test)]
#![cfg(not(target_arch = "wasm32"))]

use serpapi::serpapi::Client;
use std::collections::HashMap;

fn api_key() -> String {
    let api_key = match std::env::var_os("SERPAPI_KEY") {
        Some(v) => v.into_string().unwrap(),
        None => panic!("$SERPAPI_KEY is not set"),
    };
    return api_key;
}

#[tokio::test]
async fn search() {
    let mut default = std::collections::HashMap::<String, String>::new();
    default.insert("engine".to_string(), "google".to_string());
    default.insert("api_key".to_string(), api_key());

    // initialize the search engine
    let client = Client::new(default).unwrap();

    let mut parameter = HashMap::<String, String>::new();
    parameter.insert("q".to_string(), "coffee".to_string());
    parameter.insert(
        "location".to_string(),
        "Austin, TX, Texas, United States".to_string(),
    );

    // search returns a JSON as serde_json::Value which can be accessed like a HashMap.
    let results = client.search(parameter).await.expect("request");
    let organic_results = results["organic_results"].as_array().unwrap();
    assert!(organic_results.len() > 1);

    let places = results["local_results"]["places"].as_array().unwrap();
    assert!(places.len() > 0);
}

#[tokio::test]
async fn html() {
    let mut default = HashMap::<String, String>::new();
    default.insert("engine".to_string(), "google".to_string());
    default.insert("api_key".to_string(), api_key());

    // initialize the search engine
    let client = Client::new(default).unwrap();

    let mut parameter = HashMap::<String, String>::new();
    parameter.insert("q".to_string(), "coffee".to_string());
    parameter.insert(
        "location".to_string(),
        "Austin, TX, Texas, United States".to_string(),
    );
    let html = client.html(parameter).await.expect("request");
    assert!(html.len() > 100);
}

#[tokio::test]
async fn markdown() {
    let mut default = HashMap::<String, String>::new();
    default.insert("engine".to_string(), "google".to_string());
    default.insert("api_key".to_string(), api_key());

    // initialize the search engine
    let client = Client::new(default).unwrap();

    let mut parameter = HashMap::<String, String>::new();
    parameter.insert("q".to_string(), "coffee".to_string());
    parameter.insert(
        "location".to_string(),
        "Austin, TX, Texas, United States".to_string(),
    );
    // md returns the search results as a Markdown String.
    let markdown = client.md(parameter).await.expect("request");
    // the Markdown output starts with a YAML frontmatter
    assert!(markdown.starts_with("---"));
    assert!(markdown.contains("coffee"));
}

#[tokio::test]
async fn markdown_ignores_the_output_parameter() {
    let mut default = HashMap::<String, String>::new();
    default.insert("engine".to_string(), "google".to_string());
    default.insert("api_key".to_string(), api_key());
    let client = Client::new(default).unwrap();

    let mut parameter = HashMap::<String, String>::new();
    parameter.insert("q".to_string(), "coffee".to_string());
    parameter.insert("output".to_string(), "json".to_string());
    let markdown = client.md(parameter).await.expect("request");
    assert!(markdown.starts_with("---"));
}

#[tokio::test]
async fn location() {
    let default = HashMap::<String, String>::new();
    let client = Client::new(default).unwrap();
    let mut parameter = HashMap::<String, String>::new();
    parameter.insert("q".to_string(), "Austin".to_string());
    let data = client.location(parameter).await.expect("request");
    let locations = data.as_array().unwrap();
    assert!(locations.len() > 3);
    assert!(locations[0]["name"].as_str().unwrap().contains("Austin"));
}

// smallest valid 1x1 transparent PNG
const PNG_1X1: [u8; 67] = [
    0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A, 0x00, 0x00, 0x00, 0x0D, 0x49, 0x48, 0x44, 0x52,
    0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x01, 0x08, 0x06, 0x00, 0x00, 0x00, 0x1F, 0x15, 0xC4,
    0x89, 0x00, 0x00, 0x00, 0x0A, 0x49, 0x44, 0x41, 0x54, 0x78, 0x9C, 0x63, 0x00, 0x01, 0x00, 0x00,
    0x05, 0x00, 0x01, 0x0D, 0x0A, 0x2D, 0xB4, 0x00, 0x00, 0x00, 0x00, 0x49, 0x45, 0x4E, 0x44, 0xAE,
    0x42, 0x60, 0x82,
];

#[tokio::test]
async fn upload_image_from_file() {
    let mut default = HashMap::<String, String>::new();
    default.insert("api_key".to_string(), api_key());
    let client = Client::new(default).unwrap();

    let path = std::env::temp_dir().join("serpapi-rust-upload.png");
    std::fs::write(&path, PNG_1X1).expect("write image");

    let upload = client
        .upload_image(&path, HashMap::new())
        .await
        .expect("upload");
    std::fs::remove_file(&path).ok();

    assert_eq!(upload["message"], "Image uploaded successfully.");
    assert!(!upload["image_id"].as_str().expect("image id").is_empty());
}

#[tokio::test]
async fn upload_image_from_bytes() {
    let client = Client::new(HashMap::<String, String>::new()).unwrap();
    // the api_key is provided as a request parameter instead of a client default
    let mut parameter = HashMap::<String, String>::new();
    parameter.insert("api_key".to_string(), api_key());

    let upload = client
        .upload_image_bytes(PNG_1X1.to_vec(), "image.png", parameter)
        .await
        .expect("upload");
    assert!(!upload["image_id"].as_str().expect("image id").is_empty());
}

#[tokio::test]
async fn upload_image_rejects_invalid_image() {
    let mut default = HashMap::<String, String>::new();
    default.insert("api_key".to_string(), api_key());
    let client = Client::new(default).unwrap();

    let error = client
        .upload_image_bytes(
            b"invalid image data".to_vec(),
            "invalid.txt",
            HashMap::new(),
        )
        .await
        .expect_err("invalid image");
    assert!(error.to_string().contains("Invalid image format"));
}

#[tokio::test]
async fn upload_image_missing_file() {
    let client = Client::new(HashMap::<String, String>::new()).unwrap();
    let result = client
        .upload_image("/does/not/exist.png", HashMap::new())
        .await;
    assert!(result.is_err());
}

#[tokio::test]
async fn account() {
    let client = Client::new(HashMap::<String, String>::new()).unwrap();
    let mut parameter = HashMap::<String, String>::new();
    parameter.insert("api_key".to_string(), api_key());
    let account = client.account(parameter).await.expect("request");
    assert_eq!(account["api_key"], api_key());
    assert_ne!(account["account_email"], "");
}

#[tokio::test]
async fn search_archive() {
    let mut default = HashMap::<String, String>::new();
    default.insert("engine".to_string(), "google".to_string());
    default.insert("api_key".to_string(), api_key());
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
}

#[tokio::test]
async fn search_archive_md() {
    let mut default = HashMap::<String, String>::new();
    default.insert("engine".to_string(), "google".to_string());
    default.insert("api_key".to_string(), api_key());
    let client = Client::new(default).unwrap();

    let mut parameter = HashMap::<String, String>::new();
    parameter.insert("q".to_string(), "coffee".to_string());
    let initial_results = client.search(parameter).await.expect("request");
    let id = initial_results["search_metadata"]["id"]
        .as_str()
        .expect("search id");

    // search in archive as Markdown
    let markdown = client.search_archive_md(id).await.expect("request");
    assert!(markdown.starts_with("---"));
    assert!(markdown.contains(id));
}

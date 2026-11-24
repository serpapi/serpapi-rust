#![cfg(not(target_arch = "wasm32"))]

use serpapi::serpapi::{Client, Error};
use std::collections::HashMap;
use std::sync::Arc;

fn offline_client() -> Client {
    std::env::set_var("HTTPS_PROXY", "http://127.0.0.1:9");
    std::env::set_var("HTTP_PROXY", "http://127.0.0.1:9");
    std::env::remove_var("NO_PROXY");
    std::env::remove_var("no_proxy");
    let mut default = HashMap::<String, String>::new();
    default.insert("engine".to_string(), "google".to_string());
    default.insert("api_key".to_string(), "fake_key".to_string());
    Client::new(default).unwrap()
}

fn query() -> HashMap<String, String> {
    let mut parameter = HashMap::<String, String>::new();
    parameter.insert("q".to_string(), "coffee".to_string());
    parameter
}

fn assert_send_sync<T: Send + Sync + 'static>(_: &T) {}

#[tokio::test]
async fn search_runs_in_spawned_task() {
    let client = Arc::new(offline_client());
    let task = {
        let client = Arc::clone(&client);
        tokio::spawn(async move { client.search(query()).await })
    };
    let error = task.await.unwrap().unwrap_err();
    assert_send_sync(&error);
}

#[tokio::test]
async fn error_converts_into_send_sync_box() {
    async fn run(client: &Client) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        client.md(query()).await?;
        client.html(query()).await?;
        Ok(())
    }
    assert!(run(&offline_client()).await.is_err());
}

#[tokio::test]
async fn error_converts_into_plain_box() {
    async fn run(client: &Client) -> Result<(), Box<dyn std::error::Error>> {
        client.search(query()).await?;
        Ok(())
    }
    assert!(run(&offline_client()).await.is_err());
}

#[tokio::test]
async fn underlying_error_can_be_downcast() {
    let error: Error = offline_client().search(query()).await.unwrap_err();
    assert!(error.get_ref().downcast_ref::<reqwest::Error>().is_some());
}

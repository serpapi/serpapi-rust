// Offline regression tests: the API key must never appear in an error (spec F13).
//
// Every request is routed to an unreachable proxy, so no network access
//  nor SERPAPI_KEY is required. This file is compiled as its own test binary,
//  so the proxy environment does not leak into the live tests.
#![cfg(not(target_arch = "wasm32"))]

use serpapi::serpapi::Client;
use std::collections::HashMap;
use std::error::Error;

const FAKE_KEY: &str = "fake0secret0api0key0do0not0leak";

// route all requests to a closed local port
fn offline() {
    std::env::set_var("HTTPS_PROXY", "http://127.0.0.1:9");
    std::env::set_var("HTTP_PROXY", "http://127.0.0.1:9");
    std::env::remove_var("NO_PROXY");
    std::env::remove_var("no_proxy");
}

fn client_with_key() -> Client {
    offline();
    let mut default = HashMap::<String, String>::new();
    default.insert("engine".to_string(), "google".to_string());
    default.insert("api_key".to_string(), FAKE_KEY.to_string());
    Client::new(default).unwrap()
}

fn client_without_key() -> Client {
    offline();
    Client::new(HashMap::new()).unwrap()
}

fn key_parameter() -> HashMap<String, String> {
    let mut parameter = HashMap::<String, String>::new();
    parameter.insert("api_key".to_string(), FAKE_KEY.to_string());
    parameter.insert("q".to_string(), "coffee".to_string());
    parameter
}

// check Display and Debug of the error and of its whole source chain
fn assert_redacted(error: Box<dyn Error>) {
    let mut cause: Option<&dyn Error> = Some(error.as_ref());
    while let Some(e) = cause {
        let display = e.to_string();
        let debug = format!("{:?}", e);
        assert!(!display.contains(FAKE_KEY), "api_key leaked: {}", display);
        assert!(!debug.contains(FAKE_KEY), "api_key leaked in Debug");
        cause = e.source();
    }
}

#[tokio::test]
async fn search_error_redacts_default_api_key() {
    let mut parameter = HashMap::<String, String>::new();
    parameter.insert("q".to_string(), "coffee".to_string());
    let error = client_with_key()
        .search(parameter)
        .await
        .expect_err("unreachable proxy");
    // the redacted URL is kept to help debugging
    assert!(error.to_string().contains("serpapi.com/search"));
    assert!(error.to_string().contains("q=coffee"));
    assert_redacted(error);
}

#[tokio::test]
async fn search_error_redacts_request_api_key() {
    let error = client_without_key()
        .search(key_parameter())
        .await
        .expect_err("unreachable proxy");
    assert_redacted(error);
}

#[tokio::test]
async fn html_error_redacts_api_key() {
    let error = client_with_key()
        .html(key_parameter())
        .await
        .expect_err("unreachable proxy");
    assert_redacted(error);
}

#[tokio::test]
async fn md_error_redacts_api_key() {
    let error = client_with_key()
        .md(key_parameter())
        .await
        .expect_err("unreachable proxy");
    assert_redacted(error);
}

#[tokio::test]
async fn location_error_redacts_api_key() {
    let error = client_with_key()
        .location(key_parameter())
        .await
        .expect_err("unreachable proxy");
    assert_redacted(error);
}

#[tokio::test]
async fn account_error_redacts_api_key() {
    let error = client_without_key()
        .account(key_parameter())
        .await
        .expect_err("unreachable proxy");
    assert_redacted(error);
}

#[tokio::test]
async fn search_archive_error_redacts_api_key() {
    let client = client_with_key();
    let error = client
        .search_archive("5df0da5a7ac4f5ad2e5b2ed0")
        .await
        .expect_err("unreachable proxy");
    assert_redacted(error);
    let error = client
        .search_archive_md("5df0da5a7ac4f5ad2e5b2ed0")
        .await
        .expect_err("unreachable proxy");
    assert_redacted(error);
}

#[tokio::test]
async fn upload_image_error_redacts_api_key() {
    let error = client_with_key()
        .upload_image_bytes(b"\x89PNG\r\n\x1a\n".to_vec(), "image.png", key_parameter())
        .await
        .expect_err("unreachable proxy");
    assert_redacted(error);
}

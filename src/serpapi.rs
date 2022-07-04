#![allow(warnings)]
#![allow(dead_code)]

//! SerpApiClientClient wraps a custom HTTP client for SerpApi.com
//!
//!

use std::collections::HashMap;

// model serpapi client
//  because of Rust designed we propose to create a new search everytime
//   as opose of modifying the same search object over and over.
//  I noticed thar updating a HashMap is difficult in Rust. (I know).
//  I guess it's cheaper to create a new object in the stack
//   than updating a mutable object in the heap.
pub struct SerpApiClient {
    // search parameter like: q=coffee for google
    pub parameter: HashMap<String, String>
}

impl SerpApiClient {

    pub fn new(parameter: HashMap<String, String>) -> SerpApiClient {
        SerpApiClient {
            parameter: parameter,
        }
    }

    pub async fn search(&self, parameter: HashMap<String, String>) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let results = self.json("/search", parameter).await?;
        Ok(results)
    }

    pub async fn html(&self, parameter: HashMap<String, String>) -> Result<String, Box<dyn std::error::Error>> {
        let body = self.get("/html", parameter).await?;
        Ok(body)
    }

    // Get location using Location API
    pub async fn location(&self, parameter: HashMap<String, String>) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let results = self.json("/locations.json", parameter).await?;
        Ok(results)
    }

    // Retrieve search result from the Search Archive API
    pub async fn search_archive(
        &self,
        search_id: &str
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let mut endpoint = "/searches/".to_string();
        endpoint.push_str(search_id);
        endpoint.push_str(".json");
        println!(">> {}", endpoint);
        let results = self.json(&endpoint, HashMap::new()).await?;
        Ok(results)
    }

    // Get account information using Account API
    pub async fn account(&self, parameter: HashMap<String, String>) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let results = self.json("/account", parameter).await?;
        Ok(results)
    }

    pub async fn json(
        &self,
        endpoint: &str,
        parameter: HashMap<String, String>
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let body = self.get(endpoint, parameter).await?;
        println!("Body:\n{}", body);
        let value: serde_json::Value = serde_json::from_str(&body).unwrap();
        Ok(value)
    }

    pub async fn get(&self, endpoint: &str, parameter: HashMap<String, String>) -> Result<String, Box<dyn std::error::Error>> {
        let mut query = HashMap::<String, String>::new();
        query.insert("source".to_string(), "rust".to_string());
        for (key, value) in self.parameter.iter() {
            if !parameter.contains_key(key) {
                query.insert(key.to_string(), value.to_string());
            }
        }
        for (key, value) in parameter.iter() {
            query.insert(key.to_string(), value.to_string());
        }

        let mut url = "http://serpapi.com".to_string();
        url.push_str(endpoint);
        let client = reqwest::Client::builder().build()?;
        let res = client.get(url)
            .query(&query)
            .send()
            .await?;
        let body = res.text().await?;
        Ok(body)
    }
}

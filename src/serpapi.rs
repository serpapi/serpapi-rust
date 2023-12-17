#![allow(warnings)]
#![allow(dead_code)]
//! Client wraps a custom HTTP client designed for SerpApi.com
//!
use std::collections::HashMap;
use std::borrow::Borrow;
use std::ops::Deref;

use serde::{de::DeserializeOwned, Serialize};

// model serpapi client
//  because of Rust designed we propose to create a new search everytime
//   as opose of modifying the same search object over and over.
//  I noticed thar updating a HashMap is difficult in Rust. (I know).
//  I guess it's cheaper to create a new object in the stack
//   than updating a mutable object in the heap.
pub struct Client {
    // search parameter like: q=coffee for google
    pub parameter: HashMap<String, String>,
    pub verbose: bool,
}

impl Client {
    /// initialize a serp api client with default parameters.
    /// # Arguments
    /// * `parameter` allows to set default parameter like: api_key or engine for the client.
    pub fn new<T, K, V>(parameter: T) -> Client
    where
        T: IntoIterator,
        T::Item: Borrow<(K, V)>,
        K: ToString,
        V: ToString,
    {
        let parameter = parameter
            .into_iter()
            .map(|kv| (kv.borrow().0.to_string(), kv.borrow().1.to_string()))
            .collect();
        Client {
            parameter,
            verbose: false,
        }
    }

    /// Set this client to log verbose information about requests at TRACE level
    pub fn verbose(&mut self, verbose: bool) {
        self.verbose = verbose;
    }

    /// execute a search on serpapi.com
    ///  and return the JSON results as a serd_json::Value
    /// # Arguments
    ///  * `parameter` search parameter
    ///
    pub async fn search<P, T>(&self, parameter: P) -> Result<T, Box<dyn std::error::Error>>
    where
        P: Serialize  + Borrow<P>,
        T: DeserializeOwned,
    {
        self.json("/search.json", parameter).await
    }

    // execute a search and return the result as raw HTML formatted as String
    /// # Arguments
    /// * `parameter` html search parameter
    /// # Examples:
    /// ```
    /// use std::collections::HashMap;
    /// use serpapi::Client;
    ///
    /// #[tokio::main]
    /// async fn main() {
    /// let mut default = HashMap::<String, String>::new();
    /// default.insert("engine".to_string(), "google".to_string());
    /// default.insert("api_key".to_string(), env!("SERPAPI_KEY").to_string());
    /// // initialize the search engine
    /// let client = Client::new(default);
    /// let mut parameter = HashMap::<String, String>::new();
    /// parameter.insert("q".to_string(), "coffee".to_string());
    /// parameter.insert("location".to_string(), "Austin, TX, Texas, United States".to_string());
    /// let html = client.html(parameter).await.expect("request");
    /// assert!(html.len() > 100);
    /// }
    /// ```
    pub async fn html<P>(&self, parameter: P) -> Result<String, Box<dyn std::error::Error>>
    where
        P: Serialize  + Borrow<P>,
    {
        self.get("/search", parameter, false).await
    }

    /// Get location using Location API
    /// # Arguments
    /// * `parameter` html search parameter
    /// # Examples
    /// ```
    /// use std::collections::HashMap;
    /// use serpapi::Client;
    ///
    /// #[tokio::main]
    /// async fn main() {
    /// let client = Client::new(&[("api_key", env!("SERPAPI_KEY"))]);
    /// let mut parameter = HashMap::<String, String>::new();
    /// parameter.insert("q".to_string(), "Austin".to_string());
    /// let data: serde_json::Value = client.location(parameter).await.expect("request");
    /// let locations = data.as_array().unwrap();
    /// assert!(locations.len() > 3);
    /// }
    /// ```
    pub async fn location<P, T>(&self, parameter: P) -> Result<T, Box<dyn std::error::Error>>
    where
        P: Serialize  + Borrow<P>,
        T: DeserializeOwned,
    {
        self.json("/locations.json", parameter).await
    }

    // Retrieve search result from the Search Archive API
    pub async fn search_archive<T>(&self, search_id: &str) -> Result<T, Box<dyn std::error::Error>>
    where
        T: DeserializeOwned,
    {
        self.json(&format!("/searches/{}.json", search_id), &()).await
    }

    // Get account information using Account API
    pub async fn account<P, T>(&self, parameter: P) -> Result<T, Box<dyn std::error::Error>>
    where
        T: DeserializeOwned,
        P: Serialize  + Borrow<P>,
    {
        self.json("/account", parameter).await
    }

    pub async fn json<P, T>(
        &self,
        endpoint: &str,
        parameter: P,
    ) -> Result<T, Box<dyn std::error::Error>>
    where
        P: Serialize  + Borrow<P>,
        T: DeserializeOwned,
    {
        let body = self.get(endpoint, parameter, true).await?;
        let value = serde_json::from_str(&body)?;
        Ok(value)
    }

    async fn get<P>(
        &self,
        endpoint: &str,
        parameter: P,
        json: bool
    ) -> Result<String, Box<dyn std::error::Error>>
    where
        P: Serialize  + Borrow<P>,
    {
        let mut url = "https://serpapi.com".to_string();
        url.push_str(endpoint);
        let client = reqwest::Client::builder()
            .connection_verbose(self.verbose)
            .build()?;
        let res = client
            .get(url)
            .query(&[("output", if json { "json" } else { "html" })])
            .query(&[("source", concat!("serpapi-rust:", env!("CARGO_PKG_VERSION")))])
            .query(&self.parameter)
            .query(&parameter)
            .send()
            .await?;
        let body = res.text().await?;
        Ok(body)
    }
}

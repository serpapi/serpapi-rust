#![allow(warnings)]
#![allow(dead_code)]

//! Client wraps a custom HTTP client designed for SerpApi.com
//!
use std::collections::HashMap;
use std::path::Path;

// model serpapi client
//  because of Rust designed we propose to create a new search everytime
//   as opose of modifying the same search object over and over.
//  I noticed thar updating a HashMap is difficult in Rust. (I know).
//  I guess it's cheaper to create a new object in the stack
//   than updating a mutable object in the heap.
pub struct Client {
    // search parameter like: q=coffee for google
    pub parameter: HashMap<String, String>,
    pub http: reqwest::Client,
}

// https is required: a plain http request is redirected with a 301
//  which turns the Image API multipart POST into a GET.
const HOST: &str = "https://serpapi.com";

impl Client {
    /// initialize a serp api client with default parameters.
    /// # Arguments
    /// * `parameter` allows to set default parameter like: api_key or engine for the client.
    pub fn new(parameter: HashMap<String, String>) -> Result<Client, Box<dyn std::error::Error>> {
        let http = reqwest::Client::builder().build()?;
        let client = Client { parameter, http };

        Ok(client)
    }

    /// execute a search on serpapi.com
    ///  and return the JSON results as a serd_json::Value
    /// # Arguments
    ///  * `parameter` search parameter
    ///
    /// # Examples:
    /// ```
    /// use std::collections::HashMap;
    /// use serpapi::serpapi::Client;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///  let mut default = std::collections::HashMap::<String, String>::new();
    ///  default.insert("engine".to_string(), "google".to_string());
    ///  default.insert("api_key".to_string(), "secret_api_key".to_string());
    ///  // initialize the serpapi client
    ///  let client = Client::new(default).unwrap();
    ///  let mut parameter = HashMap::<String, String>::new();
    ///  parameter.insert("q".to_string(), "coffee".to_string());
    ///  parameter.insert(
    ///      "location".to_string(),
    ///      "Austin, TX, Texas, United States".to_string(),
    ///  );
    ///  // search returns a JSON as serde_json::Value which can be accessed like a HashMap.
    ///  let results = client.search(parameter).await.expect("request");
    ///  // let organic_results = results["organic_results"].as_array().unwrap();
    ///  // assert!(organic_results.len() > 1);
    /// }
    /// ```
    pub async fn search(
        &self,
        parameter: HashMap<String, String>,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let results = self.json("/search", parameter).await?;
        Ok(results)
    }

    /// execute a search on serpapi.com
    ///  and return the results as Markdown formatted as String.
    ///  The Markdown output is optimized for LLMs and AI agents.
    ///  It holds a YAML frontmatter followed by headings, links and tables
    ///  using about half the tokens of the JSON output.
    ///  see: https://serpapi.com/markdown-output
    /// # Arguments
    ///  * `parameter` search parameter, the output is always set to md.
    ///
    /// # Examples:
    /// ```no_run
    /// use std::collections::HashMap;
    /// use serpapi::serpapi::Client;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///  let mut default = HashMap::<String, String>::new();
    ///  default.insert("engine".to_string(), "google".to_string());
    ///  default.insert("api_key".to_string(), "secret_api_key".to_string());
    ///  // initialize the serpapi client
    ///  let client = Client::new(default).unwrap();
    ///  let mut parameter = HashMap::<String, String>::new();
    ///  parameter.insert("q".to_string(), "coffee".to_string());
    ///  // md returns the search results as a Markdown String.
    ///  let markdown = client.md(parameter).await.expect("request");
    ///  assert!(markdown.starts_with("---"));
    /// }
    /// ```
    pub async fn md(
        &self,
        parameter: HashMap<String, String>,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let body = self.text("/search", force_output(parameter, "md")).await?;
        Ok(body)
    }

    // execute a search and return the result as raw HTML formatted as String
    /// # Arguments
    /// * `parameter` html search parameter, the output is always set to html.
    /// # Examples:
    /// ```no_run
    /// use std::collections::HashMap;
    /// use serpapi::serpapi::Client;
    ///
    /// #[tokio::main]
    /// async fn main() {
    /// let mut default = HashMap::<String, String>::new();
    /// default.insert("engine".to_string(), "google".to_string());
    /// default.insert("api_key".to_string(), "secret_api_key".to_string());
    /// // initialize the search engine
    /// let client = Client::new(default).unwrap();
    /// let mut parameter = HashMap::<String, String>::new();
    /// parameter.insert("q".to_string(), "coffee".to_string());
    /// parameter.insert("location".to_string(), "Austin, TX, Texas, United States".to_string());
    /// let html = client.html(parameter).await.expect("request");
    /// assert!(html.len() > 100);
    /// }
    /// ```
    pub async fn html(
        &self,
        parameter: HashMap<String, String>,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let body = self
            .text("/search", force_output(parameter, "html"))
            .await?;
        Ok(body)
    }

    /// Get location using Location API
    /// # Arguments
    /// * `parameter` html search parameter
    /// # Examples
    /// ```
    /// use std::collections::HashMap;
    /// use serpapi::serpapi::Client;
    ///
    /// #[tokio::main]
    /// async fn main() {
    /// let client = Client::new(HashMap::<String, String>::new()).unwrap();
    /// let mut parameter = HashMap::<String, String>::new();
    /// parameter.insert("q".to_string(), "Austin".to_string());
    /// let data = client.location(parameter).await.expect("request");
    /// let locations = data.as_array().unwrap();
    /// assert!(locations.len() > 3);
    /// }
    /// ```
    pub async fn location(
        &self,
        parameter: HashMap<String, String>,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let results = self.json("/locations.json", parameter).await?;
        Ok(results)
    }

    /// Upload an image using the Image API.
    ///  The returned `image_id` can be supplied to Search API engines
    ///  supporting uploaded images, such as Google Lens.
    ///  Supported formats: jpg, jpeg, png and webp up to 500 KB.
    ///  The `image_id` expires after 10 minutes.
    ///  see: https://serpapi.com/image-api
    /// # Arguments
    /// * `path` image file path
    /// * `parameter` request parameter, such as an `api_key` overriding the client default
    /// # Examples
    /// ```no_run
    /// use std::collections::HashMap;
    /// use serpapi::serpapi::Client;
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///  let mut default = HashMap::<String, String>::new();
    ///  default.insert("api_key".to_string(), "secret_api_key".to_string());
    ///  let client = Client::new(default).unwrap();
    ///  let upload = client.upload_image("./image.jpg", HashMap::new()).await.expect("upload");
    ///  let image_id = upload["image_id"].as_str().expect("image id");
    ///  // search with google lens using the uploaded image
    ///  let mut parameter = HashMap::<String, String>::new();
    ///  parameter.insert("engine".to_string(), "google_lens".to_string());
    ///  parameter.insert("image_id".to_string(), image_id.to_string());
    ///  let results = client.search(parameter).await.expect("request");
    ///  // let visual_matches = results["visual_matches"].as_array().unwrap();
    /// }
    /// ```
    pub async fn upload_image<P: AsRef<Path>>(
        &self,
        path: P,
        parameter: HashMap<String, String>,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let path = path.as_ref();
        let file_name = path
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("image");
        let image = tokio::fs::read(path).await?;
        self.upload_image_bytes(image, file_name, parameter).await
    }

    /// Upload an in-memory image using the Image API.
    ///  see: Client::upload_image
    /// # Arguments
    /// * `image` binary content of the image
    /// * `file_name` file name including the extension, such as "image.png"
    /// * `parameter` request parameter, such as an `api_key` overriding the client default
    pub async fn upload_image_bytes(
        &self,
        image: Vec<u8>,
        file_name: &str,
        parameter: HashMap<String, String>,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let part = reqwest::multipart::Part::bytes(image)
            .file_name(file_name.to_string())
            .mime_str(image_mime_type(file_name))?;
        let form = reqwest::multipart::Form::new().part("image", part);
        let results = self.post_multipart("/image", parameter, form).await?;
        if let Some(error) = results["error"].as_str() {
            return Err(format!("image upload failed with: {}", error).into());
        }
        Ok(results)
    }

    // Retrieve search result from the Search Archive API
    pub async fn search_archive(
        &self,
        search_id: &str,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let mut endpoint = "/searches/".to_string();
        endpoint.push_str(search_id);
        endpoint.push_str(".json");
        println!(">> {}", endpoint);
        let results = self.json(&endpoint, HashMap::new()).await?;
        Ok(results)
    }

    /// Retrieve a search result from the Search Archive API as Markdown.
    ///  see: https://serpapi.com/markdown-output
    /// # Arguments
    /// * `search_id` from the original search: `results["search_metadata"]["id"]`
    pub async fn search_archive_md(
        &self,
        search_id: &str,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let mut endpoint = "/searches/".to_string();
        endpoint.push_str(search_id);
        endpoint.push_str(".md");
        let body = self.text(&endpoint, HashMap::new()).await?;
        Ok(body)
    }

    // Get account information using Account API
    pub async fn account(
        &self,
        parameter: HashMap<String, String>,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let results = self.json("/account", parameter).await?;
        Ok(results)
    }

    pub async fn json(
        &self,
        endpoint: &str,
        parameter: HashMap<String, String>,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let body = self.get(endpoint, parameter).await?;
        //debug: println!("Body:\n{}", body);
        //  a non JSON body means the output is html or md, see: Client::html and Client::md
        let value: serde_json::Value = serde_json::from_str(&body)?;
        Ok(value)
    }

    /// execute a request and return the body as String.
    ///  SerpApi reports errors as JSON even when html or md output is requested,
    ///  so a JSON response to a text request is reported as an error.
    /// # Arguments
    /// * `endpoint` HTTP service URI
    /// * `parameter` search parameter
    pub async fn text(
        &self,
        endpoint: &str,
        parameter: HashMap<String, String>,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let (content_type, body) = self.raw(endpoint, parameter).await?;
        if content_type.starts_with("application/json") {
            return Err(format!("search failed on {} with: {}", endpoint, body).into());
        }
        Ok(body)
    }

    pub async fn get(
        &self,
        endpoint: &str,
        parameter: HashMap<String, String>,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let (_content_type, body) = self.raw(endpoint, parameter).await?;
        Ok(body)
    }

    /// execute a request and return the content type along with the body.
    async fn raw(
        &self,
        endpoint: &str,
        parameter: HashMap<String, String>,
    ) -> Result<(String, String), Box<dyn std::error::Error>> {
        let query = self.query(parameter);
        let mut url = HOST.to_string();
        url.push_str(endpoint);
        let res = self.http.get(url).query(&query).send().await?;
        let content_type = res
            .headers()
            .get(reqwest::header::CONTENT_TYPE)
            .and_then(|value| value.to_str().ok())
            .unwrap_or("")
            .to_lowercase();
        let body = res.text().await?;
        Ok((content_type, body))
    }

    /// execute a multipart/form-data POST request and decode the JSON response.
    ///  the request parameters are sent as form fields alongside the given form parts.
    async fn post_multipart(
        &self,
        endpoint: &str,
        parameter: HashMap<String, String>,
        mut form: reqwest::multipart::Form,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        for (key, value) in self.query(parameter) {
            form = form.text(key, value);
        }
        let mut url = HOST.to_string();
        url.push_str(endpoint);
        let res = self.http.post(url).multipart(form).send().await?;
        let body = res.text().await?;
        let value: serde_json::Value = serde_json::from_str(&body)?;
        Ok(value)
    }

    /// merge the client default parameter with the request parameter.
    ///  the request parameter takes precedence over the client default.
    fn query(&self, parameter: HashMap<String, String>) -> HashMap<String, String> {
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
        query
    }
}

/// force the output format whatever the caller provides.
///  the format drives the return type: json -> serde_json::Value, html / md -> String.
fn force_output(mut parameter: HashMap<String, String>, format: &str) -> HashMap<String, String> {
    parameter.insert("output".to_string(), format.to_string());
    parameter
}

/// guess the image MIME type from the file extension.
///  the Image API supports jpg, jpeg, png and webp.
fn image_mime_type(file_name: &str) -> &'static str {
    let extension = Path::new(file_name)
        .extension()
        .and_then(|ext| ext.to_str())
        .map(|ext| ext.to_lowercase());
    match extension.as_deref() {
        Some("jpg") | Some("jpeg") => "image/jpeg",
        Some("png") => "image/png",
        Some("webp") => "image/webp",
        _ => "application/octet-stream",
    }
}

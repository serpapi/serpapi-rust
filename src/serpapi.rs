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
    ///  default.insert("api_key".to_string(), std::env::var("SERPAPI_KEY").expect("$SERPAPI_KEY is not set"));
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
        // fail fast before loading an oversized image in memory
        check_image_size(tokio::fs::metadata(path).await?.len())?;
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
        check_image_size(image.len() as u64)?;
        let mime_type = image_mime_type(&image, file_name);
        let part = reqwest::multipart::Part::bytes(image)
            .file_name(file_name.to_string())
            .mime_str(mime_type)?;
        // the Image API only takes the api_key,
        //  search parameter from the client default such as engine or output are not forwarded.
        let mut field = self.query(parameter);
        field.retain(|key, _| key == "api_key" || key == "source");
        let form = reqwest::multipart::Form::new().part("image", part);
        let api_key = field.get("api_key").cloned();
        let results = self.post_multipart("/image", field, form).await?;
        if let Some(error) = results["error"].as_str() {
            let message = format!("image upload failed with: {}", error);
            return Err(redact(message, api_key.as_deref()).into());
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
        let api_key = self.api_key(&parameter);
        let (content_type, body) = self.raw(endpoint, parameter).await?;
        if content_type.starts_with("application/json") {
            let message = format!("search failed on {} with: {}", endpoint, body);
            return Err(redact(message, api_key.as_deref()).into());
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
        let api_key = query.get("api_key").map(String::as_str);
        self.send(endpoint, self.http.get(url).query(&query), api_key)
            .await
    }

    /// send a request and return the content type along with the body.
    ///  a non-2xx HTTP status is reported as an error including the response body,
    ///  which holds the SerpApi error message or a non JSON error page from a proxy.
    ///  the `api_key` is removed from every error, see: redact_error and redact.
    async fn send(
        &self,
        endpoint: &str,
        request: reqwest::RequestBuilder,
        api_key: Option<&str>,
    ) -> Result<(String, String), Box<dyn std::error::Error>> {
        let res = request.send().await.map_err(redact_error)?;
        let status = res.status();
        let content_type = res
            .headers()
            .get(reqwest::header::CONTENT_TYPE)
            .and_then(|value| value.to_str().ok())
            .unwrap_or("")
            .to_lowercase();
        let body = res.text().await.map_err(redact_error)?;
        if !status.is_success() {
            let message = format!("{} failed with HTTP {}: {}", endpoint, status, body);
            return Err(redact(message, api_key).into());
        }
        Ok((content_type, body))
    }

    /// execute a multipart/form-data POST request and decode the JSON response.
    ///  the fields are sent as is alongside the given form parts,
    ///  the client default parameter are not merged.
    async fn post_multipart(
        &self,
        endpoint: &str,
        field: HashMap<String, String>,
        mut form: reqwest::multipart::Form,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
        let api_key = field.get("api_key").cloned();
        for (key, value) in field {
            form = form.text(key, value);
        }
        let mut url = HOST.to_string();
        url.push_str(endpoint);
        let request = self.http.post(url).multipart(form);
        let (_content_type, body) = self.send(endpoint, request, api_key.as_deref()).await?;
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

    /// the api_key sent with the request parameter, or the client default.
    fn api_key(&self, parameter: &HashMap<String, String>) -> Option<String> {
        parameter
            .get("api_key")
            .or_else(|| self.parameter.get("api_key"))
            .cloned()
    }
}

/// remove the api_key from the URL carried by a reqwest error.
///  reqwest displays the full request URL in its errors,
///  which would expose the secret api_key in logs or panics.
fn redact_error(mut error: reqwest::Error) -> reqwest::Error {
    if let Some(url) = error.url_mut() {
        redact_url(url);
    }
    error
}

/// remove the api_key from the URL query, the other parameter are kept to ease debugging.
fn redact_url(url: &mut reqwest::Url) {
    if !url.query_pairs().any(|(key, _)| key == "api_key") {
        return;
    }
    let pairs: Vec<(String, String)> = url
        .query_pairs()
        .filter(|(key, _)| key != "api_key")
        .map(|(key, value)| (key.into_owned(), value.into_owned()))
        .collect();
    if pairs.is_empty() {
        url.set_query(None);
    } else {
        url.query_pairs_mut().clear().extend_pairs(pairs);
    }
}

/// replace any occurrence of the api_key in an error message,
///  such as a proxy error page echoing the request URL.
fn redact(message: String, api_key: Option<&str>) -> String {
    match api_key {
        Some(key) if !key.is_empty() => message.replace(key, "[REDACTED]"),
        _ => message,
    }
}

/// Image API upload size limit: 500 KB.
const IMAGE_MAX_SIZE: u64 = 500 * 1024;

/// reject an image larger than the Image API limit before uploading it.
fn check_image_size(size: u64) -> Result<(), Box<dyn std::error::Error>> {
    if size > IMAGE_MAX_SIZE {
        return Err(format!(
            "image is too large: {} bytes, the Image API limit is {} bytes",
            size, IMAGE_MAX_SIZE
        )
        .into());
    }
    Ok(())
}

/// guess the image MIME type from the content signature,
///  or from the file extension when the signature is unknown.
///  the Image API supports jpg, jpeg, png and webp.
fn image_mime_type(image: &[u8], file_name: &str) -> &'static str {
    if image.starts_with(&[0xFF, 0xD8, 0xFF]) {
        return "image/jpeg";
    }
    if image.starts_with(b"\x89PNG\r\n\x1a\n") {
        return "image/png";
    }
    if image.len() >= 12 && image.starts_with(b"RIFF") && &image[8..12] == b"WEBP" {
        return "image/webp";
    }
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

/// force the output format whatever the caller provides.
///  the format drives the return type: json -> serde_json::Value, html / md -> String.
fn force_output(mut parameter: HashMap<String, String>, format: &str) -> HashMap<String, String> {
    parameter.insert("output".to_string(), format.to_string());
    parameter
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn redact_url_removes_api_key_only() {
        let mut url =
            reqwest::Url::parse("https://serpapi.com/search?q=a%20b&api_key=secret&source=rust")
                .unwrap();
        redact_url(&mut url);
        assert_eq!(url.as_str(), "https://serpapi.com/search?q=a+b&source=rust");
    }

    #[test]
    fn redact_url_without_other_parameter() {
        let mut url = reqwest::Url::parse("https://serpapi.com/account?api_key=secret").unwrap();
        redact_url(&mut url);
        assert_eq!(url.as_str(), "https://serpapi.com/account");
    }

    #[test]
    fn redact_url_keeps_url_without_api_key() {
        let raw = "https://serpapi.com/searches/123.json?source=rust&q=%C3%A9";
        let mut url = reqwest::Url::parse(raw).unwrap();
        redact_url(&mut url);
        assert_eq!(url.as_str(), raw);
    }

    #[test]
    fn redact_replaces_api_key_in_message() {
        let message = "proxy error for https://serpapi.com/search?api_key=secret".to_string();
        assert_eq!(
            redact(message, Some("secret")),
            "proxy error for https://serpapi.com/search?api_key=[REDACTED]"
        );
    }

    #[test]
    fn redact_ignores_missing_or_empty_api_key() {
        assert_eq!(redact("message".to_string(), None), "message");
        assert_eq!(redact("message".to_string(), Some("")), "message");
    }
}

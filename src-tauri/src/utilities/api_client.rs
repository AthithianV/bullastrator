use reqwest::{Client, Method, header};
use serde::{Deserialize, Serialize};
use std::time::Duration;

pub struct ApiClient {
    client: Client,
    base_url: String,
    token: Option<String>,
}

#[derive(serde::Deserialize, Debug)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: T,
}

impl ApiClient {
    pub fn new(base_url: String, token: Option<String>) -> Self {
        let mut headers = header::HeaderMap::new();
        headers.insert(
            header::CONTENT_TYPE,
            header::HeaderValue::from_static("application/json"),
        );

        Self {
            client: Client::builder()
                .default_headers(headers)
                .timeout(Duration::from_secs(10))
                .build()
                .expect("Failed to initialize API Client"),
            base_url,
            token,
        }
    }

    /// Generic request wrapper
    async fn request<T, R>(&self, method: Method, url: &str, body: Option<T>) -> Result<R, String>
    where
        T: Serialize,
        R: for<'de> Deserialize<'de> + std::fmt::Debug,
    {
        let full_url = format!("{}{}", self.base_url, url);

        if self.token.is_none() {
            return Err("UNAUTHORIZED".to_string());
        }

        let mut request_builder = self.client.request(method, &full_url);

        if let Some(ref t) = self.token {
            request_builder = request_builder.bearer_auth(t);
        }

        if let Some(b) = body {
            request_builder = request_builder.json(&b);
        }

        let response = request_builder
            .send()
            .await
            .map_err(|e| format!("[API Error] Network failure: {}", e))?;

        if response.status() == reqwest::StatusCode::UNAUTHORIZED {
            return Err("UNAUTHORIZED".to_string());
        }

        if !response.status().is_success() {
            let status = response.status();
            let err_body = response.text().await.unwrap_or_default();
            return Err(format!("[API Error] {} - {}", status, err_body));
        }

        let raw_body = response
            .text()
            .await
            .map_err(|e| format!("Could not read body: {}", e))?;

        let data: ApiResponse<R> = serde_json::from_str(&raw_body)
            .map_err(|e| format!("[JSON Error] Match failure: {}. Body was: {}", e, raw_body))?;

        Ok(data.data)
    }

    pub async fn get<R>(&self, url: &str) -> Result<R, String>
    where
        R: for<'de> Deserialize<'de> + std::fmt::Debug,
    {
        self.request::<(), R>(Method::GET, url, None).await
    }

    pub async fn post<T, R>(&self, url: &str, body: T) -> Result<R, String>
    where
        T: Serialize,
        R: for<'de> Deserialize<'de> + std::fmt::Debug,
    {
        self.request(Method::POST, url, Some(body)).await
    }
}

//! Onionoo API Client
//!
//! This module provides the main client for making HTTP requests to the Onionoo API.
//! The client handles request construction, response processing, and error handling.

use reqwest::Client as HttpClient;
use serde::de::DeserializeOwned;
use std::fmt;

use crate::parameters::QueryParameters;

/// Custom error type for API client operations
#[derive(Debug)]
pub enum ClientError {
    /// HTTP request failed (network issues, DNS, etc.)
    Request(reqwest::Error),
    /// HTTP response returned an error status code
    StatusCode(u16, String),
    /// JSON deserialization failed
    Deserialization(String),
    /// Invalid URL construction
    UrlConstruction(String),
    /// Generic client error with message
    Other(String),
}

impl fmt::Display for ClientError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ClientError::Request(e) => write!(f, "HTTP request failed: {}", e),
            ClientError::StatusCode(code, msg) => write!(f, "HTTP {}: {}", code, msg),
            ClientError::Deserialization(msg) => write!(f, "JSON deserialization failed: {}", msg),
            ClientError::UrlConstruction(msg) => write!(f, "URL construction error: {}", msg),
            ClientError::Other(msg) => write!(f, "Client error: {}", msg),
        }
    }
}

impl std::error::Error for ClientError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            ClientError::Request(e) => Some(e),
            _ => None,
        }
    }
}

impl From<reqwest::Error> for ClientError {
    fn from(error: reqwest::Error) -> Self {
        ClientError::Request(error)
    }
}

impl From<serde_json::Error> for ClientError {
    fn from(error: serde_json::Error) -> Self {
        ClientError::Deserialization(error.to_string())
    }
}

/// Main client for the Onionoo API
#[derive(Debug)]
pub struct Client {
    /// Internal HTTP client for making requests
    http_client: HttpClient,
    /// Base URL for the Onionoo API
    base_url: &'static str,
}

impl Client {
    /// Create a new Onionoo API client
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use onionoo::Client;
    /// let client = Client::new();
    /// ```
    pub fn new() -> Self {
        Self {
            http_client: HttpClient::new(),
            base_url: "https://onionoo.torproject.org",
        }
    }

    /// Create a new client with a custom base URL
    ///
    /// This is useful for testing or using a different Onionoo instance.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use onionoo::Client;
    /// let client = Client::new_with_base_url("https://custom-onionoo.example.com");
    /// ```
    pub fn new_with_base_url(base_url: &'static str) -> Self {
        Self {
            http_client: HttpClient::new(),
            base_url,
        }
    }

    /// Make a generic GET request to an endpoint
    ///
    /// # Type Parameters
    ///
    /// * `T` - The type to deserialize the response into. Must implement `serde::de::DeserializeOwned`.
    ///
    /// # Arguments
    ///
    /// * `endpoint` - The API endpoint path (e.g., "/summary", "/details")
    /// * `params` - Query parameters to include in the request
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing either:
    /// * `Ok(response)` - The successfully deserialized response
    /// * `Err(ClientError)` - An error occurred during the request or deserialization
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use onionoo::{Client, QueryParameters, selection::TypeValue, models::SummaryResponse};
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = Client::new();
    /// let params = QueryParameters::new()
    ///     .type_param(TypeValue::Relay)
    ///     .limit(10);
    ///
    /// let response: SummaryResponse = client.get("/summary", params).await?;
    /// println!("Found {} relays", response.relays.len());
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get<T>(&self, endpoint: &str, params: QueryParameters) -> Result<T, ClientError>
    where
        T: DeserializeOwned,
    {
        let endpoint = if endpoint.starts_with('/') {
            endpoint
        } else {
            return Err(ClientError::UrlConstruction(
                "Endpoint must start with '/'".to_string(),
            ));
        };

        let mut url = format!("{}{}", self.base_url, endpoint);

        let query_string = params.to_query_string();
        if !query_string.is_empty() {
            url.push_str(&query_string);
        }

        let response = self.http_client.get(&url).send().await?;

        if !response.status().is_success() {
            let status_code = response.status().as_u16();
            let error_message = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(ClientError::StatusCode(status_code, error_message));
        }

        let response_text = response.text().await?;
        let deserialized: T = serde_json::from_str(&response_text)?;

        Ok(deserialized)
    }

    /// Make a GET request to a specific endpoint using the Endpoint enum
    ///
    /// # Type Parameters
    ///
    /// * `T` - The type to deserialize the response into. Must implement `serde::de::DeserializeOwned`.
    ///
    /// # Arguments
    ///
    /// * `endpoint` - The endpoint to request
    /// * `params` - Query parameters to include in the request
    ///
    /// # Returns
    ///
    /// Returns a `Result` containing either:
    /// * `Ok(response)` - The successfully deserialized response
    /// * `Err(ClientError)` - An error occurred during the request or deserialization
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use onionoo::{Client, QueryParameters, endpoints::Endpoint, selection::TypeValue, models::SummaryResponse};
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = Client::new();
    /// let params = QueryParameters::new()
    ///     .type_param(TypeValue::Relay)
    ///     .limit(10);
    ///
    /// let response: SummaryResponse = client.get_endpoint(Endpoint::Summary, params).await?;
    /// println!("Found {} relays", response.relays.len());
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_endpoint<T>(
        &self,
        endpoint: crate::endpoints::Endpoint,
        params: QueryParameters,
    ) -> Result<T, ClientError>
    where
        T: DeserializeOwned,
    {
        self.get(endpoint.path(), params).await
    }

    /// Get the base URL being used by this client
    pub fn base_url(&self) -> &'static str {
        self.base_url
    }

    /// Check if the client is configured to use the default Onionoo URL
    pub fn is_default(&self) -> bool {
        self.base_url == "https://onionoo.torproject.org"
    }
}

impl Default for Client {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::parameters::{
        QueryParameters,
        selection::{BoolValue, TypeValue},
    };

    #[test]
    fn test_client_default_creation() {
        let client = Client::new();
        assert_eq!(client.base_url(), "https://onionoo.torproject.org");
        assert!(client.is_default());
    }

    #[test]
    fn test_client_custom_base_url() {
        let custom_url = "https://custom.example.com";
        let client = Client::new_with_base_url(custom_url);
        assert_eq!(client.base_url(), custom_url);
        assert!(!client.is_default());
    }

    #[test]
    fn test_url_construction() {
        let client = Client::new();
        let params = QueryParameters::new()
            .type_param(TypeValue::Relay)
            .limit(10);

        // The actual URL construction happens in the get() method
        // We can test that invalid endpoints are rejected
        // Note: This test would need to be async in a real scenario
        // For now, we just test the constructor methods work
        assert_eq!(client.base_url(), "https://onionoo.torproject.org");
    }

    #[test]
    fn test_error_display() {
        let error = ClientError::Other("test error".to_string());
        assert_eq!(error.to_string(), "Client error: test error");

        let error = ClientError::StatusCode(404, "Not Found".to_string());
        assert_eq!(error.to_string(), "HTTP 404: Not Found");
    }
}

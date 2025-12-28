//! HTTP client module for API communication
//!
//! This module provides HTTP client functionality for communicating
//! with the ChainUp API.

use reqwest::blocking::Client;
use std::collections::HashMap;
use std::time::Duration;

use crate::error::{NetworkError, Result};

/// Base HTTP client for API requests
pub struct HttpClient {
    client: Client,
    content_type: String,
    debug: bool,
}

impl HttpClient {
    /// Creates a new HTTP client
    ///
    /// # Arguments
    /// * `content_type` - Content-Type header value
    /// * `debug` - Enable debug logging
    pub fn new(content_type: &str, debug: bool) -> Result<Self> {
        let client = Client::builder()
            .timeout(Duration::from_secs(30))
            .build()
            .map_err(|e| NetworkError::with_source("Failed to create HTTP client", e))?;

        Ok(Self {
            client,
            content_type: content_type.to_string(),
            debug,
        })
    }

    /// Creates a new HTTP client with form URL-encoded content type
    pub fn new_form_client(debug: bool) -> Result<Self> {
        Self::new("application/x-www-form-urlencoded", debug)
    }

    /// Creates a new HTTP client with JSON content type
    pub fn new_json_client(debug: bool) -> Result<Self> {
        Self::new("application/json", debug)
    }

    /// Executes a POST request
    ///
    /// # Arguments
    /// * `url` - Full URL to request
    /// * `data` - Request data
    ///
    /// # Returns
    /// Response body as string
    pub fn post(&self, url: &str, data: &HashMap<String, String>) -> Result<String> {
        if self.debug {
            log::debug!("[HTTP Request]: POST {}", url);
            log::debug!("[HTTP Data]: {:?}", data);
        }

        let response = self
            .client
            .post(url)
            .header("Content-Type", &self.content_type)
            .form(data)
            .send()
            .map_err(|e| NetworkError::with_source("POST request failed", e))?;

        let status = response.status();
        let body = response
            .text()
            .map_err(|e| NetworkError::with_source("Failed to read response body", e))?;

        if self.debug {
            log::debug!("[HTTP Response Status]: {}", status);
            log::debug!("[HTTP Response Body]: {}", body);
        }

        if !status.is_success() {
            return Err(NetworkError::new(format!(
                "HTTP request failed with status {}: {}",
                status, body
            ))
            .into());
        }

        Ok(body)
    }

    /// Executes a GET request
    ///
    /// # Arguments
    /// * `url` - Full URL to request
    /// * `data` - Query parameters
    ///
    /// # Returns
    /// Response body as string
    pub fn get(&self, url: &str, data: &HashMap<String, String>) -> Result<String> {
        if self.debug {
            log::debug!("[HTTP Request]: GET {}", url);
            log::debug!("[HTTP Query]: {:?}", data);
        }

        let response = self
            .client
            .get(url)
            .header("Content-Type", &self.content_type)
            .query(data)
            .send()
            .map_err(|e| NetworkError::with_source("GET request failed", e))?;

        let status = response.status();
        let body = response
            .text()
            .map_err(|e| NetworkError::with_source("Failed to read response body", e))?;

        if self.debug {
            log::debug!("[HTTP Response Status]: {}", status);
            log::debug!("[HTTP Response Body]: {}", body);
        }

        if !status.is_success() {
            return Err(NetworkError::new(format!(
                "HTTP request failed with status {}: {}",
                status, body
            ))
            .into());
        }

        Ok(body)
    }
}

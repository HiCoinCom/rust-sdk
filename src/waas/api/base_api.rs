//! Base API for WaaS
//!
//! Provides common functionality for all WaaS API implementations.

use std::collections::HashMap;
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};

use serde::de::DeserializeOwned;
use serde_json::Value;

use crate::crypto::CryptoProvider;
use crate::error::{ApiError, Result};
use crate::http_client::HttpClient;
use crate::waas::config::WaasConfig;

/// Base API class for WaaS
///
/// Provides common functionality for all WaaS API implementations.
/// Implements the same encryption flow as Java SDK.
pub struct BaseApi {
    pub(crate) config: WaasConfig,
    pub(crate) http_client: HttpClient,
    pub(crate) crypto_provider: Arc<dyn CryptoProvider>,
}

impl BaseApi {
    /// Creates a new BaseApi instance
    pub fn new(config: WaasConfig, crypto_provider: Arc<dyn CryptoProvider>) -> Self {
        let http_client =
            HttpClient::new_form_client(config.debug).expect("Failed to create HTTP client");

        Self {
            config,
            http_client,
            crypto_provider,
        }
    }

    /// Builds request args JSON with common parameters.
    /// Matches Python SDK: args = {**data, "time": milliseconds, "charset": "utf-8"}
    ///
    /// # Arguments
    /// * `data` - API-specific request data
    ///
    /// # Returns
    /// JSON string of request args
    fn build_request_args_json(&self, data: Option<&HashMap<String, Value>>) -> String {
        let mut args: HashMap<String, Value> = data.cloned().unwrap_or_default();

        // Add timestamp in milliseconds (matches Python SDK: int(time.time() * 1000))
        let time_ms = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64;
        args.insert("time".to_string(), Value::Number(time_ms.into()));

        // Add charset
        args.insert(
            "charset".to_string(),
            Value::String(self.config.charset.clone()),
        );

        serde_json::to_string(&args).unwrap_or_default()
    }

    /// Executes an API request with encryption.
    ///
    /// Flow matches Python SDK _execute_request():
    /// 1. Serialize params to JSON with time and charset
    /// 2. Encrypt with private key
    /// 3. Send only app_id and encrypted data
    /// 4. Decrypt response data with public key
    pub(crate) fn execute_request(
        &self,
        method: &str,
        path: &str,
        data: Option<&HashMap<String, Value>>,
    ) -> Result<Value> {
        // Step 1: Build request args JSON
        let raw_json = self.build_request_args_json(data);

        if self.config.debug {
            println!("[WaaS Request args]: {}", raw_json);
        }

        // Step 2: Encrypt with private key
        let encrypted_data = self.crypto_provider.encrypt_with_private_key(&raw_json)?;

        if self.config.debug {
            println!(
                "[WaaS Encrypted data]: {}...",
                &encrypted_data[..100.min(encrypted_data.len())]
            );
        }

        // Step 3: Send request with only app_id and data
        let mut request_data = HashMap::new();
        request_data.insert("app_id".to_string(), self.config.app_id.clone());
        request_data.insert("data".to_string(), encrypted_data);

        let url = self.config.get_url(path);

        let response_body = match method {
            "GET" => self.http_client.get(&url, &request_data)?,
            _ => self.http_client.post(&url, &request_data)?,
        };

        if self.config.debug {
            println!("[WaaS Response]: {}", response_body);
        }

        let response: Value = serde_json::from_str(&response_body)?;

        // Step 4: Check if response has encrypted data field and decrypt
        if let Some(Value::String(encrypted_response_data)) = response.get("data") {
            // Decrypt the data field
            match self
                .crypto_provider
                .decrypt_with_public_key(encrypted_response_data)
            {
                Ok(decrypted) => {
                    if self.config.debug {
                        println!("[WaaS Decrypted]: {}", decrypted);
                    }
                    // Parse decrypted JSON and return the complete decrypted response
                    match serde_json::from_str::<Value>(&decrypted) {
                        Ok(decrypted_data) => return Ok(decrypted_data),
                        Err(_) => {
                            // If parsing fails, return the original response
                            if self.config.debug {
                                println!("[WaaS] Failed to parse decrypted data as JSON");
                            }
                        }
                    }
                }
                Err(e) => {
                    if self.config.debug {
                        println!("[WaaS Decrypt error]: {}", e);
                    }
                    // If decryption fails, might be an error response, return as-is
                }
            }
        }

        Ok(response)
    }

    /// Executes a POST request
    ///
    /// # Arguments
    /// * `path` - API path
    /// * `data` - Request data
    ///
    /// # Returns
    /// API response as JSON Value
    pub fn post(&self, path: &str, data: Option<&HashMap<String, Value>>) -> Result<Value> {
        self.execute_request("POST", path, data)
    }

    /// Executes a GET request
    ///
    /// # Arguments
    /// * `path` - API path
    /// * `data` - Request data
    ///
    /// # Returns
    /// API response as JSON Value
    pub fn get(&self, path: &str, data: Option<&HashMap<String, Value>>) -> Result<Value> {
        self.execute_request("GET", path, data)
    }

    /// Validates API response and extracts data
    ///
    /// The response has already been decrypted in execute_request.
    /// This method checks the code and extracts the data field.
    ///
    /// # Arguments
    /// * `response` - API response JSON (already decrypted)
    ///
    /// # Returns
    /// Parsed response data
    pub fn validate_response<T: DeserializeOwned>(&self, response: Value) -> Result<T> {
        // Check response code (can be int or string)
        let code = match response.get("code") {
            Some(Value::Number(n)) => n.as_i64().unwrap_or(-1) as i32,
            Some(Value::String(s)) => s.parse::<i32>().unwrap_or(-1),
            _ => -1,
        };

        if code != 0 {
            let message = response
                .get("msg")
                .and_then(|v| v.as_str())
                .unwrap_or("Unknown error")
                .to_string();

            return Err(ApiError::new(code, message).into());
        }

        // Get data field (already decrypted)
        let data = response.get("data");

        match data {
            Some(value) => {
                // Parse data directly
                let parsed: T = serde_json::from_value(value.clone())?;
                Ok(parsed)
            }
            None => {
                // No data field - try empty array first (for list responses)
                // If that fails, try null, otherwise the full response
                if let Ok(parsed) = serde_json::from_value::<T>(Value::Array(vec![])) {
                    return Ok(parsed);
                }
                if let Ok(parsed) = serde_json::from_value::<T>(Value::Null) {
                    return Ok(parsed);
                }
                let parsed: T = serde_json::from_value(response)?;
                Ok(parsed)
            }
        }
    }

    /// Validates API response and returns raw Value
    pub fn validate_response_raw(&self, response: Value) -> Result<Value> {
        // Check response code (can be int or string)
        let code = match response.get("code") {
            Some(Value::Number(n)) => n.as_i64().unwrap_or(-1) as i32,
            Some(Value::String(s)) => s.parse::<i32>().unwrap_or(-1),
            _ => -1,
        };

        if code != 0 {
            let message = response
                .get("msg")
                .and_then(|v| v.as_str())
                .unwrap_or("Unknown error")
                .to_string();

            return Err(ApiError::new(code, message).into());
        }

        // Get data field (already decrypted)
        match response.get("data") {
            Some(value) => Ok(value.clone()),
            None => Ok(response),
        }
    }
}

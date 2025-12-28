//! WaaS Configuration
//!
//! Stores configuration parameters for WaaS API client.

use crate::crypto::CryptoProvider;
use crate::error::{ConfigError, Result};
use std::sync::Arc;

/// WaaS Configuration
///
/// Stores configuration parameters for WaaS API client.
///
/// # Example
/// ```ignore
/// let config = WaasConfig::new(
///     "your-app-id",
///     "your-private-key",
///     "chainup-public-key",
/// );
/// ```
#[derive(Clone)]
pub struct WaasConfig {
    /// API host URL
    pub host: String,
    /// Application ID
    pub app_id: String,
    /// RSA private key for signing requests
    pub private_key: String,
    /// ChainUp public key for verifying responses
    pub public_key: String,
    /// Custom crypto provider implementation
    pub crypto_provider: Option<Arc<dyn CryptoProvider>>,
    /// API version
    pub version: String,
    /// Request charset encoding
    pub charset: String,
    /// Enable debug mode
    pub debug: bool,
}

impl WaasConfig {
    /// Creates a new WaaS configuration
    ///
    /// # Arguments
    /// * `app_id` - Application ID
    /// * `private_key` - RSA private key
    /// * `public_key` - ChainUp public key
    pub fn new(app_id: impl Into<String>, private_key: impl Into<String>, public_key: impl Into<String>) -> Self {
        Self {
            host: "https://openapi.chainup.com/".to_string(),
            app_id: app_id.into(),
            private_key: private_key.into(),
            public_key: public_key.into(),
            crypto_provider: None,
            version: "v2".to_string(),
            charset: "UTF-8".to_string(),
            debug: false,
        }
    }

    /// Creates a configuration with custom crypto provider
    pub fn with_crypto_provider(app_id: impl Into<String>, crypto_provider: Arc<dyn CryptoProvider>) -> Self {
        Self {
            host: "https://openapi.chainup.com/".to_string(),
            app_id: app_id.into(),
            private_key: String::new(),
            public_key: String::new(),
            crypto_provider: Some(crypto_provider),
            version: "v2".to_string(),
            charset: "UTF-8".to_string(),
            debug: false,
        }
    }

    /// Validates the configuration
    ///
    /// # Returns
    /// Ok(()) if configuration is valid
    ///
    /// # Errors
    /// Returns ConfigError if required fields are missing
    pub fn validate(&self) -> Result<()> {
        if self.host.is_empty() {
            return Err(ConfigError::new("WaasConfig: host is required").into());
        }
        if self.app_id.is_empty() {
            return Err(ConfigError::new("WaasConfig: app_id is required").into());
        }

        // Either crypto_provider or private_key/public_key must be provided
        if self.crypto_provider.is_none() {
            if self.private_key.is_empty() {
                return Err(ConfigError::new(
                    "WaasConfig: private_key is required (or provide crypto_provider)",
                )
                .into());
            }
            if self.public_key.is_empty() {
                return Err(ConfigError::new(
                    "WaasConfig: public_key is required (or provide crypto_provider)",
                )
                .into());
            }
        }

        Ok(())
    }

    /// Gets the full API URL
    ///
    /// # Arguments
    /// * `path` - API path (without leading slash for v2 prefix)
    ///
    /// # Returns
    /// Full API URL
    pub fn get_url(&self, path: &str) -> String {
        let host = if self.host.ends_with('/') {
            &self.host[..self.host.len() - 1]
        } else {
            &self.host
        };

        let path = path.trim_start_matches('/');
        format!("{}/{}/{}", host, self.version, path)
    }
}

impl Default for WaasConfig {
    fn default() -> Self {
        Self {
            host: "https://openapi.chainup.com/".to_string(),
            app_id: String::new(),
            private_key: String::new(),
            public_key: String::new(),
            crypto_provider: None,
            version: "v2".to_string(),
            charset: "UTF-8".to_string(),
            debug: false,
        }
    }
}

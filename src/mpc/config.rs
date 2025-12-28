//! MPC Configuration
//!
//! Stores configuration parameters for MPC API client.

use crate::crypto::CryptoProvider;
use crate::error::{ConfigError, Result};
use std::sync::Arc;

/// MPC Configuration
///
/// Stores configuration parameters for MPC API client.
///
/// # Example
/// ```ignore
/// let config = MpcConfig::new(
///     "your-app-id",
///     "your-rsa-private-key",
///     "waas-public-key",
/// );
/// ```
#[derive(Clone)]
pub struct MpcConfig {
    /// Application ID
    pub app_id: String,
    /// RSA private key for encrypting requests
    pub rsa_private_key: String,
    /// WaaS server public key for decrypting responses
    pub waas_public_key: String,
    /// RSA private key for transaction signing
    pub sign_private_key: String,
    /// API domain URL
    pub domain: String,
    /// API key for authentication
    pub api_key: String,
    /// Custom crypto provider implementation
    pub crypto_provider: Option<Arc<dyn CryptoProvider>>,
    /// Enable debug mode
    pub debug: bool,
}

impl MpcConfig {
    /// Creates a new MPC configuration
    ///
    /// # Arguments
    /// * `app_id` - Application ID
    /// * `rsa_private_key` - RSA private key
    /// * `waas_public_key` - WaaS public key
    pub fn new(
        app_id: impl Into<String>,
        rsa_private_key: impl Into<String>,
        waas_public_key: impl Into<String>,
    ) -> Self {
        Self {
            app_id: app_id.into(),
            rsa_private_key: rsa_private_key.into(),
            waas_public_key: waas_public_key.into(),
            sign_private_key: String::new(),
            domain: "https://openapi.chainup.com/".to_string(),
            api_key: String::new(),
            crypto_provider: None,
            debug: false,
        }
    }

    /// Creates a configuration with custom crypto provider
    pub fn with_crypto_provider(app_id: impl Into<String>, crypto_provider: Arc<dyn CryptoProvider>) -> Self {
        Self {
            app_id: app_id.into(),
            rsa_private_key: String::new(),
            waas_public_key: String::new(),
            sign_private_key: String::new(),
            domain: "https://openapi.chainup.com/".to_string(),
            api_key: String::new(),
            crypto_provider: Some(crypto_provider),
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
        if self.domain.is_empty() {
            return Err(ConfigError::new("MpcConfig: domain is required").into());
        }
        if self.app_id.is_empty() {
            return Err(ConfigError::new("MpcConfig: app_id is required").into());
        }

        // Either crypto_provider or rsa_private_key must be provided
        if self.crypto_provider.is_none() && self.rsa_private_key.is_empty() {
            return Err(ConfigError::new(
                "MpcConfig: rsa_private_key is required (or provide crypto_provider)",
            )
            .into());
        }

        Ok(())
    }

    /// Gets the full API URL
    ///
    /// # Arguments
    /// * `path` - API path
    ///
    /// # Returns
    /// Full API URL
    pub fn get_url(&self, path: &str) -> String {
        let domain = if self.domain.ends_with('/') {
            &self.domain[..self.domain.len() - 1]
        } else {
            &self.domain
        };

        let path = path.trim_start_matches('/');
        format!("{}/{}", domain, path)
    }
}

impl Default for MpcConfig {
    fn default() -> Self {
        Self {
            app_id: String::new(),
            rsa_private_key: String::new(),
            waas_public_key: String::new(),
            sign_private_key: String::new(),
            domain: "https://openapi.chainup.com/".to_string(),
            api_key: String::new(),
            crypto_provider: None,
            debug: false,
        }
    }
}

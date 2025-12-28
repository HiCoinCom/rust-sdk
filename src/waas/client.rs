//! WaaS Client
//!
//! Main entry point for WaaS API operations.

use std::sync::Arc;

use crate::crypto::{CryptoProvider, RsaCryptoProvider};
use crate::error::Result;
use crate::waas::api::{AccountApi, AsyncNotifyApi, BillingApi, CoinApi, TransferApi, UserApi};
use crate::waas::config::WaasConfig;

/// WaaS Client - Main entry point for WaaS API operations
///
/// Provides factory methods for creating API instances.
///
/// # Example
/// ```ignore
/// let client = WaasClient::builder()
///     .set_app_id("your-app-id")
///     .set_private_key("your-private-key")
///     .set_public_key("chainup-public-key")
///     .build()?;
///
/// let user_api = client.get_user_api();
/// ```
#[derive(Clone)]
pub struct WaasClient {
    config: WaasConfig,
    crypto_provider: Arc<dyn CryptoProvider>,
}

impl WaasClient {
    /// Creates a new WaasClient instance
    ///
    /// # Arguments
    /// * `config` - WaaS configuration
    ///
    /// # Note
    /// Prefer using `WaasClient::builder()` for construction
    pub fn new(config: WaasConfig) -> Result<Self> {
        config.validate()?;

        let crypto_provider: Arc<dyn CryptoProvider> =
            if let Some(ref provider) = config.crypto_provider {
                provider.clone()
            } else {
                Arc::new(RsaCryptoProvider::with_keys(
                    &config.private_key,
                    &config.public_key,
                )?)
            };

        Ok(Self {
            config,
            crypto_provider,
        })
    }

    /// Creates a new Builder instance for configuring WaasClient
    ///
    /// # Returns
    /// Builder instance
    ///
    /// # Example
    /// ```ignore
    /// let client = WaasClient::builder()
    ///     .set_app_id("your-app-id")
    ///     .set_private_key("your-private-key")
    ///     .set_public_key("chainup-public-key")
    ///     .build()?;
    /// ```
    pub fn builder() -> WaasClientBuilder {
        WaasClientBuilder::new()
    }

    /// Alias for builder()
    pub fn new_builder() -> WaasClientBuilder {
        Self::builder()
    }

    /// Gets UserApi instance for user-related operations
    ///
    /// # Returns
    /// UserApi instance
    pub fn get_user_api(&self) -> UserApi {
        UserApi::new(self.config.clone(), self.crypto_provider.clone())
    }

    /// Gets AccountApi instance for account-related operations
    ///
    /// # Returns
    /// AccountApi instance
    pub fn get_account_api(&self) -> AccountApi {
        AccountApi::new(self.config.clone(), self.crypto_provider.clone())
    }

    /// Gets BillingApi instance for billing and transaction operations
    ///
    /// # Returns
    /// BillingApi instance
    pub fn get_billing_api(&self) -> BillingApi {
        BillingApi::new(self.config.clone(), self.crypto_provider.clone())
    }

    /// Gets CoinApi instance for coin-related operations
    ///
    /// # Returns
    /// CoinApi instance
    pub fn get_coin_api(&self) -> CoinApi {
        CoinApi::new(self.config.clone(), self.crypto_provider.clone())
    }

    /// Gets TransferApi instance for internal transfer operations
    ///
    /// # Returns
    /// TransferApi instance
    pub fn get_transfer_api(&self) -> TransferApi {
        TransferApi::new(self.config.clone(), self.crypto_provider.clone())
    }

    /// Gets AsyncNotifyApi instance for notification operations
    ///
    /// # Returns
    /// AsyncNotifyApi instance
    pub fn get_async_notify_api(&self) -> AsyncNotifyApi {
        AsyncNotifyApi::new(self.config.clone(), self.crypto_provider.clone())
    }

    /// Gets the crypto provider for direct cryptographic operations
    ///
    /// # Returns
    /// Arc reference to the CryptoProvider
    pub fn get_crypto_provider(&self) -> Arc<dyn CryptoProvider> {
        self.crypto_provider.clone()
    }

    /// Gets the configuration
    pub fn config(&self) -> &WaasConfig {
        &self.config
    }
}

/// Builder for WaasClient
///
/// Provides a fluent interface for configuring and creating WaasClient instances.
#[derive(Default)]
pub struct WaasClientBuilder {
    app_id: Option<String>,
    private_key: Option<String>,
    public_key: Option<String>,
    host: Option<String>,
    version: Option<String>,
    crypto_provider: Option<Arc<dyn CryptoProvider>>,
    debug: bool,
}

impl WaasClientBuilder {
    /// Creates a new builder instance
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the application ID
    pub fn set_app_id(mut self, app_id: impl Into<String>) -> Self {
        self.app_id = Some(app_id.into());
        self
    }

    /// Sets the RSA private key
    pub fn set_private_key(mut self, private_key: impl Into<String>) -> Self {
        self.private_key = Some(private_key.into());
        self
    }

    /// Sets the ChainUp public key
    pub fn set_public_key(mut self, public_key: impl Into<String>) -> Self {
        self.public_key = Some(public_key.into());
        self
    }

    /// Sets the API host URL
    pub fn set_host(mut self, host: impl Into<String>) -> Self {
        self.host = Some(host.into());
        self
    }

    /// Sets the API version
    pub fn set_version(mut self, version: impl Into<String>) -> Self {
        self.version = Some(version.into());
        self
    }

    /// Sets a custom crypto provider
    pub fn set_crypto_provider(mut self, crypto_provider: Arc<dyn CryptoProvider>) -> Self {
        self.crypto_provider = Some(crypto_provider);
        self
    }

    /// Enables or disables debug mode
    pub fn set_debug(mut self, debug: bool) -> Self {
        self.debug = debug;
        self
    }

    /// Builds the WaasClient instance
    ///
    /// # Returns
    /// Configured WaasClient instance
    ///
    /// # Errors
    /// Returns ConfigError if required fields are missing
    pub fn build(self) -> Result<WaasClient> {
        let mut config = WaasConfig {
            app_id: self.app_id.unwrap_or_default(),
            private_key: self.private_key.unwrap_or_default(),
            public_key: self.public_key.unwrap_or_default(),
            host: self
                .host
                .unwrap_or_else(|| "https://openapi.chainup.com/".to_string()),
            version: self.version.unwrap_or_else(|| "v2".to_string()),
            crypto_provider: self.crypto_provider,
            charset: "UTF-8".to_string(),
            debug: self.debug,
        };

        // Normalize host URL
        if !config.host.ends_with('/') {
            config.host.push('/');
        }

        WaasClient::new(config)
    }
}

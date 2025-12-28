//! MPC Client
//!
//! Main entry point for MPC API operations.

use std::sync::Arc;

use crate::crypto::{CryptoProvider, RsaCryptoProvider};
use crate::error::Result;
use crate::mpc::api::{
    AutoSweepApi, DepositApi, NotifyApi, TronResourceApi, WalletApi, Web3Api,
    WithdrawApi, WorkspaceApi,
};
use crate::mpc::config::MpcConfig;

/// MPC Client - Main entry point for MPC API operations
///
/// Provides factory methods for creating API instances.
///
/// # Example
/// ```ignore
/// let client = MpcClient::builder()
///     .set_app_id("your-app-id")
///     .set_rsa_private_key("your-private-key")
///     .set_waas_public_key("waas-public-key")
///     .build()?;
///
/// let wallet_api = client.get_wallet_api();
/// ```
#[derive(Clone)]
pub struct MpcClient {
    config: MpcConfig,
    crypto_provider: Arc<dyn CryptoProvider>,
}

impl MpcClient {
    /// Creates a new MpcClient instance
    ///
    /// # Arguments
    /// * `config` - MPC configuration
    ///
    /// # Note
    /// Prefer using `MpcClient::builder()` for construction
    pub fn new(config: MpcConfig) -> Result<Self> {
        config.validate()?;

        let crypto_provider: Arc<dyn CryptoProvider> = if let Some(ref provider) = config.crypto_provider {
            provider.clone()
        } else {
            Arc::new(RsaCryptoProvider::new(
                Some(&config.rsa_private_key),
                if config.waas_public_key.is_empty() {
                    None
                } else {
                    Some(&config.waas_public_key)
                },
                if config.sign_private_key.is_empty() {
                    None
                } else {
                    Some(&config.sign_private_key)
                },
            )?)
        };

        Ok(Self {
            config,
            crypto_provider,
        })
    }

    /// Creates a new Builder instance for configuring MpcClient
    ///
    /// # Returns
    /// Builder instance
    ///
    /// # Example
    /// ```ignore
    /// let client = MpcClient::builder()
    ///     .set_app_id("your-app-id")
    ///     .set_rsa_private_key("your-private-key")
    ///     .set_waas_public_key("waas-public-key")
    ///     .build()?;
    /// ```
    pub fn builder() -> MpcClientBuilder {
        MpcClientBuilder::new()
    }

    /// Alias for builder()
    pub fn new_builder() -> MpcClientBuilder {
        Self::builder()
    }

    /// Gets WalletApi instance for wallet management operations
    ///
    /// # Returns
    /// WalletApi instance
    pub fn get_wallet_api(&self) -> WalletApi {
        WalletApi::new(self.config.clone(), self.crypto_provider.clone())
    }

    /// Gets DepositApi instance for deposit operations
    ///
    /// # Returns
    /// DepositApi instance
    pub fn get_deposit_api(&self) -> DepositApi {
        DepositApi::new(self.config.clone(), self.crypto_provider.clone())
    }

    /// Gets WithdrawApi instance for withdrawal operations
    ///
    /// # Returns
    /// WithdrawApi instance
    pub fn get_withdraw_api(&self) -> WithdrawApi {
        WithdrawApi::new(self.config.clone(), self.crypto_provider.clone())
    }

    /// Gets Web3Api instance for Web3 operations
    ///
    /// # Returns
    /// Web3Api instance
    pub fn get_web3_api(&self) -> Web3Api {
        Web3Api::new(self.config.clone(), self.crypto_provider.clone())
    }

    /// Gets AutoSweepApi instance for auto-sweep operations
    ///
    /// # Returns
    /// AutoSweepApi instance
    pub fn get_auto_sweep_api(&self) -> AutoSweepApi {
        AutoSweepApi::new(self.config.clone(), self.crypto_provider.clone())
    }

    /// Gets NotifyApi instance for notification operations
    ///
    /// # Returns
    /// NotifyApi instance
    pub fn get_notify_api(&self) -> NotifyApi {
        NotifyApi::new(self.config.clone(), self.crypto_provider.clone())
    }

    /// Gets WorkspaceApi instance for workspace operations
    ///
    /// # Returns
    /// WorkspaceApi instance
    pub fn get_workspace_api(&self) -> WorkspaceApi {
        WorkspaceApi::new(self.config.clone(), self.crypto_provider.clone())
    }

    /// Gets TronResourceApi instance for TRON resource operations
    ///
    /// # Returns
    /// TronResourceApi instance
    pub fn get_tron_resource_api(&self) -> TronResourceApi {
        TronResourceApi::new(self.config.clone(), self.crypto_provider.clone())
    }

    /// Gets the configuration
    pub fn config(&self) -> &MpcConfig {
        &self.config
    }
}

/// Builder for MpcClient
///
/// Provides a fluent interface for configuring and creating MpcClient instances.
#[derive(Default)]
pub struct MpcClientBuilder {
    app_id: Option<String>,
    rsa_private_key: Option<String>,
    waas_public_key: Option<String>,
    sign_private_key: Option<String>,
    domain: Option<String>,
    api_key: Option<String>,
    crypto_provider: Option<Arc<dyn CryptoProvider>>,
    debug: bool,
}

impl MpcClientBuilder {
    /// Creates a new builder instance
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets the application ID
    pub fn set_app_id(mut self, app_id: impl Into<String>) -> Self {
        self.app_id = Some(app_id.into());
        self
    }

    /// Sets the RSA private key for encrypting requests
    pub fn set_rsa_private_key(mut self, rsa_private_key: impl Into<String>) -> Self {
        self.rsa_private_key = Some(rsa_private_key.into());
        self
    }

    /// Sets the WaaS public key for decrypting responses
    pub fn set_waas_public_key(mut self, waas_public_key: impl Into<String>) -> Self {
        self.waas_public_key = Some(waas_public_key.into());
        self
    }

    /// Sets the signing private key for transaction signing
    pub fn set_sign_private_key(mut self, sign_private_key: impl Into<String>) -> Self {
        self.sign_private_key = Some(sign_private_key.into());
        self
    }

    /// Sets the API domain URL
    pub fn set_host(mut self, domain: impl Into<String>) -> Self {
        self.domain = Some(domain.into());
        self
    }

    /// Sets the API key
    pub fn set_api_key(mut self, api_key: impl Into<String>) -> Self {
        self.api_key = Some(api_key.into());
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

    /// Builds the MpcClient instance
    ///
    /// # Returns
    /// Configured MpcClient instance
    ///
    /// # Errors
    /// Returns ConfigError if required fields are missing
    pub fn build(self) -> Result<MpcClient> {
        let mut config = MpcConfig {
            app_id: self.app_id.unwrap_or_default(),
            rsa_private_key: self.rsa_private_key.unwrap_or_default(),
            waas_public_key: self.waas_public_key.unwrap_or_default(),
            sign_private_key: self.sign_private_key.unwrap_or_default(),
            domain: self.domain.unwrap_or_else(|| "https://openapi.chainup.com/".to_string()),
            api_key: self.api_key.unwrap_or_default(),
            crypto_provider: self.crypto_provider,
            debug: self.debug,
        };

        // Normalize domain URL
        if !config.domain.ends_with('/') {
            config.domain.push('/');
        }

        MpcClient::new(config)
    }
}

//! Async Notify API - Asynchronous notification management
//!
//! Provides methods for decrypting and managing webhook notifications.

use std::sync::Arc;

use serde::{Deserialize, Serialize};

use rust_decimal::Decimal;

use crate::crypto::CryptoProvider;
use crate::error::{CryptoError, Result};
use crate::utils::serde_helpers::{deserialize_optional_i32, deserialize_optional_i64};
use crate::waas::api::base_api::BaseApi;
use crate::waas::api::billing_api::WithdrawParams;
use crate::waas::config::WaasConfig;

// ============================================================================
// Types
// ============================================================================

/// Notification data from webhook
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NotifyData {
    /// Transaction side: "deposit" or "withdraw"
    #[serde(default)]
    pub side: Option<String>,
    /// Transaction ID
    #[serde(default, deserialize_with = "deserialize_optional_i64")]
    pub id: Option<i64>,
    /// User ID
    #[serde(default, deserialize_with = "deserialize_optional_i64")]
    pub uid: Option<i64>,
    /// User email
    #[serde(default)]
    pub email: Option<String>,
    /// Cryptocurrency symbol
    #[serde(default)]
    pub symbol: Option<String>,
    /// Base symbol (main chain)
    #[serde(default)]
    pub base_symbol: Option<String>,
    /// Transaction amount
    #[serde(default, with = "rust_decimal::serde::str_option")]
    pub amount: Option<Decimal>,
    /// Destination address
    #[serde(default)]
    pub address_to: Option<String>,
    /// Source address
    #[serde(default)]
    pub address_from: Option<String>,
    /// Transaction hash
    #[serde(default)]
    pub txid: Option<String>,
    /// Transaction ID type (0: on-chain, 1: internal)
    #[serde(default)]
    pub txid_type: Option<String>,
    /// Confirmations
    #[serde(default, deserialize_with = "deserialize_optional_i32")]
    pub confirmations: Option<i32>,
    /// Contract address (for tokens)
    #[serde(default)]
    pub contract_address: Option<String>,
    /// Status
    #[serde(default, deserialize_with = "deserialize_optional_i32")]
    pub status: Option<i32>,
    /// SaaS status (for withdrawals)
    #[serde(default, deserialize_with = "deserialize_optional_i32")]
    pub saas_status: Option<i32>,
    /// Company status (for withdrawals)
    #[serde(default, deserialize_with = "deserialize_optional_i32")]
    pub company_status: Option<i32>,
    /// Request ID (for withdrawals)
    #[serde(default)]
    pub request_id: Option<String>,
    /// Withdrawal fee (for withdrawals)
    #[serde(default, with = "rust_decimal::serde::str_option")]
    pub withdraw_fee: Option<Decimal>,
    /// Withdrawal fee symbol
    #[serde(default)]
    pub withdraw_fee_symbol: Option<String>,
    /// Platform fee
    #[serde(default, with = "rust_decimal::serde::str_option")]
    pub fee: Option<Decimal>,
    /// Fee symbol
    #[serde(default)]
    pub fee_symbol: Option<String>,
    /// Real fee (actual miner fee)
    #[serde(default, with = "rust_decimal::serde::str_option")]
    pub real_fee: Option<Decimal>,
    /// Is mining reward (for deposits)
    #[serde(default, deserialize_with = "deserialize_optional_i32")]
    pub is_mining: Option<i32>,
    /// Creation time (milliseconds timestamp)
    #[serde(default, deserialize_with = "deserialize_optional_i64")]
    pub created_at: Option<i64>,
    /// Update time (milliseconds timestamp)
    #[serde(default, deserialize_with = "deserialize_optional_i64")]
    pub updated_at: Option<i64>,
}

// ============================================================================
// Async Notify API Implementation
// ============================================================================

/// Async Notify API - Asynchronous notification management
///
/// Provides methods for decrypting and managing webhook notifications.
pub struct AsyncNotifyApi {
    #[allow(dead_code)]
    base: BaseApi,
    crypto_provider: Arc<dyn CryptoProvider>,
    debug: bool,
}

impl AsyncNotifyApi {
    /// Creates a new AsyncNotifyApi instance
    pub fn new(config: WaasConfig, crypto_provider: Arc<dyn CryptoProvider>) -> Self {
        let debug = config.debug;
        Self {
            base: BaseApi::new(config, crypto_provider.clone()),
            crypto_provider,
            debug,
        }
    }

    /// Decrypts deposit and withdrawal notification parameters
    ///
    /// Used to decrypt encrypted notification data received from WaaS callbacks.
    ///
    /// # Arguments
    /// * `cipher` - Encrypted notification data
    ///
    /// # Returns
    /// Decrypted notification arguments, or error if decryption fails
    ///
    /// # Example
    /// ```ignore
    /// let notify_data = async_notify_api.notify_request(encrypted_data)?;
    /// println!("Notify type: {:?}", notify_data.side);
    /// ```
    pub fn notify_request(&self, cipher: &str) -> Result<NotifyData> {
        if cipher.is_empty() {
            if self.debug {
                log::debug!("[AsyncNotify] Cipher cannot be empty");
            }
            return Err(CryptoError::new("Cipher cannot be empty").into());
        }

        // Decrypt the cipher text using public key
        let raw = self.crypto_provider.decrypt_with_public_key(cipher)?;

        if self.debug {
            log::debug!("[AsyncNotify] Decrypted data: {}", raw);
        }

        // Parse JSON to notification arguments
        let notify: NotifyData = serde_json::from_str(&raw)?;
        Ok(notify)
    }

    /// Decrypts withdrawal secondary verification request parameters
    ///
    /// Used to decrypt verification request data for withdrawal operations
    /// that require additional confirmation.
    ///
    /// # Arguments
    /// * `cipher` - Encrypted verification request data (encrypted WithdrawParams)
    ///
    /// # Returns
    /// Decrypted withdrawal parameters, or error if decryption fails
    ///
    /// # Example
    /// ```ignore
    /// let withdraw_params = async_notify_api.verify_request(encrypted_data)?;
    /// println!("Withdraw request_id: {}", withdraw_params.request_id);
    /// ```
    pub fn verify_request(&self, cipher: &str) -> Result<WithdrawParams> {
        if cipher.is_empty() {
            if self.debug {
                log::debug!("[AsyncNotify] VerifyRequest cipher cannot be empty");
            }
            return Err(CryptoError::new("VerifyRequest cipher cannot be empty").into());
        }

        // Decrypt the cipher text
        let raw = self.crypto_provider.decrypt_with_public_key(cipher)?;

        if self.debug {
            log::debug!("[AsyncNotify] VerifyRequest decrypted data: {}", raw);
        }

        // Parse JSON to withdrawal parameters
        let withdraw: WithdrawParams = serde_json::from_str(&raw)?;
        Ok(withdraw)
    }

    /// Encrypts the secondary verification withdrawal response data
    ///
    /// Used to encrypt the response data when confirming or rejecting
    /// a withdrawal that requires secondary verification.
    ///
    /// # Arguments
    /// * `params` - Withdrawal parameters to encrypt
    ///
    /// # Returns
    /// Encrypted response data
    ///
    /// # Example
    /// ```ignore
    /// let response_data = async_notify_api.verify_response(withdraw_params)?;
    /// ```
    pub fn verify_response(&self, params: WithdrawParams) -> Result<String> {
        // Convert to JSON string
        let response_json = serde_json::to_string(&params)?;

        // Encrypt with private key
        let encrypted = self
            .crypto_provider
            .encrypt_with_private_key(&response_json)?;
        Ok(encrypted)
    }
}

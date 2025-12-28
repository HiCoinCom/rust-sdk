//! Notify API - MPC webhook notification handling
//!
//! Provides methods for decrypting and verifying webhook notifications.

use std::collections::HashMap;
use std::sync::Arc;

use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::crypto::CryptoProvider;
use crate::error::Result;
use crate::mpc::config::MpcConfig;
use crate::utils::serde_helpers::{
    deserialize_optional_bool, deserialize_optional_i32, deserialize_optional_i64,
};

// ============================================================================
// Notification types
// ============================================================================

/// MPC notification data structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MpcNotifyData {
    /// Record ID
    #[serde(default, deserialize_with = "deserialize_optional_i64")]
    pub id: Option<i64>,
    /// Notification side (deposit/withdraw)
    #[serde(default)]
    pub side: Option<String>,
    /// Notification type
    #[serde(default)]
    pub notify_type: Option<String>,
    /// Request ID
    #[serde(default)]
    pub request_id: Option<String>,
    /// Sub-wallet ID
    #[serde(default, deserialize_with = "deserialize_optional_i64")]
    pub sub_wallet_id: Option<i64>,
    /// App ID
    #[serde(default)]
    pub app_id: Option<String>,
    /// Main chain symbol / Base symbol
    #[serde(default)]
    pub main_chain_symbol: Option<String>,
    /// Base symbol
    #[serde(default)]
    pub base_symbol: Option<String>,
    /// Symbol
    #[serde(default)]
    pub symbol: Option<String>,
    /// Contract address (for tokens)
    #[serde(default)]
    pub contract_address: Option<String>,
    /// Amount
    #[serde(default, with = "rust_decimal::serde::str_option")]
    pub amount: Option<Decimal>,
    /// Fee
    #[serde(default, with = "rust_decimal::serde::str_option")]
    pub fee: Option<Decimal>,
    /// Real fee
    #[serde(default, with = "rust_decimal::serde::str_option")]
    pub real_fee: Option<Decimal>,
    /// Fee symbol
    #[serde(default)]
    pub fee_symbol: Option<String>,
    /// Refund amount
    #[serde(default, with = "rust_decimal::serde::str_option")]
    pub refund_amount: Option<Decimal>,
    /// TRON delegate fee
    #[serde(default, with = "rust_decimal::serde::str_option")]
    pub delegate_fee: Option<Decimal>,
    /// Transaction hash
    #[serde(default)]
    pub txid: Option<String>,
    /// Transaction height
    #[serde(default, deserialize_with = "deserialize_optional_i64")]
    pub tx_height: Option<i64>,
    /// Block height
    #[serde(default, deserialize_with = "deserialize_optional_i64")]
    pub block_height: Option<i64>,
    /// Block time
    #[serde(default, deserialize_with = "deserialize_optional_i64")]
    pub block_time: Option<i64>,
    /// Confirmations count
    #[serde(default, deserialize_with = "deserialize_optional_i32")]
    pub confirmations: Option<i32>,
    /// From address
    #[serde(default)]
    pub from: Option<String>,
    /// To address
    #[serde(default)]
    pub to: Option<String>,
    /// Memo
    #[serde(default)]
    pub memo: Option<String>,
    /// Status
    #[serde(default, deserialize_with = "deserialize_optional_i32")]
    pub status: Option<i32>,
    /// Address from
    #[serde(default)]
    pub address_from: Option<String>,
    /// Address to
    #[serde(default)]
    pub address_to: Option<String>,
    /// Confirmation count (alias)
    #[serde(default, deserialize_with = "deserialize_optional_i32")]
    pub confirm: Option<i32>,
    /// Safe confirmation count
    #[serde(default, deserialize_with = "deserialize_optional_i32")]
    pub safe_confirm: Option<i32>,
    /// Is mining reward
    #[serde(default, deserialize_with = "deserialize_optional_i32")]
    pub is_mining: Option<i32>,
    /// Transaction type
    #[serde(default, deserialize_with = "deserialize_optional_i32")]
    pub trans_type: Option<i32>,
    /// Withdraw source
    #[serde(default)]
    pub withdraw_source: Option<String>,
    /// KYT status
    #[serde(default, deserialize_with = "deserialize_optional_bool")]
    pub kyt_status: Option<bool>,
    /// Interactive contract address (Web3)
    #[serde(default)]
    pub interactive_contract: Option<String>,
    /// Input data (Web3)
    #[serde(default)]
    pub input_data: Option<String>,
    /// Dapp image URL (Web3)
    #[serde(default)]
    pub dapp_img: Option<String>,
    /// Dapp name (Web3)
    #[serde(default)]
    pub dapp_name: Option<String>,
    /// Dapp URL (Web3)
    #[serde(default)]
    pub dapp_url: Option<String>,
    /// Charset
    #[serde(default)]
    pub charset: Option<String>,
    /// Sign
    #[serde(default)]
    pub sign: Option<String>,
    /// Notify time
    #[serde(default)]
    pub notify_time: Option<String>,
    /// Creation time
    #[serde(default)]
    pub created_at: Option<String>,
    /// Update time
    #[serde(default)]
    pub updated_at: Option<String>,
    /// Raw data for additional fields
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

// ============================================================================
// Notify API Implementation
// ============================================================================

/// Notify API - MPC webhook notification handling
///
/// Provides methods for decrypting and verifying webhook notifications.
pub struct NotifyApi {
    #[allow(dead_code)]
    config: MpcConfig,
    crypto_provider: Arc<dyn CryptoProvider>,
}

impl NotifyApi {
    /// Creates a new NotifyApi instance
    pub fn new(config: MpcConfig, crypto_provider: Arc<dyn CryptoProvider>) -> Self {
        Self {
            config,
            crypto_provider,
        }
    }

    /// Decrypts webhook notification data
    ///
    /// # Arguments
    /// * `encrypted_data` - Encrypted notification data
    ///
    /// # Returns
    /// Decrypted notification data as MpcNotifyData
    ///
    /// # Example
    /// ```ignore
    /// let data = notify_api.decrypt_notification("encrypted_string")?;
    /// println!("Notification type: {:?}", data.notify_type);
    /// ```
    pub fn decrypt_notification(&self, encrypted_data: &str) -> Result<MpcNotifyData> {
        let decrypted = self
            .crypto_provider
            .decrypt_with_public_key(encrypted_data)?;
        let data: MpcNotifyData = serde_json::from_str(&decrypted)?;
        Ok(data)
    }
}

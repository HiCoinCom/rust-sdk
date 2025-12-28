//! Coin API - Cryptocurrency information operations
//!
//! Provides methods for querying supported cryptocurrencies.

use std::collections::HashMap;
use std::sync::Arc;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use rust_decimal::Decimal;

use crate::crypto::CryptoProvider;
use crate::error::Result;
use crate::utils::serde_helpers::{deserialize_optional_bool, deserialize_optional_i32};
use crate::waas::api::base_api::BaseApi;
use crate::waas::config::WaasConfig;

// ============================================================================
// Response types
// ============================================================================

/// Coin information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoinInfo {
    /// Coin network name
    #[serde(default)]
    pub coin_net: Option<String>,
    /// Coin symbol
    #[serde(default)]
    pub symbol: Option<String>,
    /// Icon URL
    #[serde(default)]
    pub icon: Option<String>,
    /// Real symbol
    #[serde(default)]
    pub real_symbol: Option<String>,
    /// Symbol alias
    #[serde(default)]
    pub symbol_alias: Option<String>,
    /// Base chain symbol
    #[serde(default)]
    pub base_symbol: Option<String>,
    /// Merge address symbol
    #[serde(default)]
    pub merge_address_symbol: Option<String>,
    /// Margin symbol
    #[serde(default)]
    pub margin_symbol: Option<String>,
    /// Decimal places (API returns as string)
    #[serde(default, deserialize_with = "deserialize_optional_i32")]
    pub decimals: Option<i32>,
    /// Token contract address (if applicable)
    #[serde(default)]
    pub contract_address: Option<String>,
    /// Required confirmations for deposit (API returns as string)
    #[serde(default, deserialize_with = "deserialize_optional_i32")]
    pub deposit_confirmation: Option<i32>,
    /// Required confirmations for withdraw (API returns as string)
    #[serde(default, deserialize_with = "deserialize_optional_i32")]
    pub withdraw_confirmation: Option<i32>,
    /// Whether memo/tag is supported (API returns as 0/1)
    #[serde(default, deserialize_with = "deserialize_optional_bool")]
    pub support_memo: Option<bool>,
    /// Whether tokens are supported (API returns as 0/1)
    #[serde(default, deserialize_with = "deserialize_optional_bool")]
    pub support_token: Option<bool>,
    /// Address validation regex
    #[serde(default)]
    pub address_regex: Option<String>,
    /// Memo/tag validation regex
    #[serde(default)]
    pub address_tag_regex: Option<String>,
    /// Minimum deposit amount
    #[serde(default, with = "rust_decimal::serde::str_option")]
    pub min_deposit: Option<Decimal>,
    /// Transaction ID link
    #[serde(default)]
    pub txid_link: Option<String>,
    /// Explorer URL
    #[serde(default)]
    pub explorer: Option<String>,
    /// Address link
    #[serde(default)]
    pub address_link: Option<String>,
}

/// Response for get coin list
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetCoinListResponse {
    /// List of coins
    #[serde(default)]
    pub list: Vec<CoinInfo>,
}

// ============================================================================
// Coin API Implementation
// ============================================================================

/// Coin API - Cryptocurrency information operations
///
/// Provides methods for querying supported cryptocurrencies.
pub struct CoinApi {
    base: BaseApi,
}

impl CoinApi {
    /// Creates a new CoinApi instance
    pub fn new(config: WaasConfig, crypto_provider: Arc<dyn CryptoProvider>) -> Self {
        Self {
            base: BaseApi::new(config, crypto_provider),
        }
    }

    /// Gets supported coin list
    ///
    /// Retrieves information about all cryptocurrencies supported by the platform.
    ///
    /// # Returns
    /// List of supported coins
    ///
    /// # Example
    /// ```ignore
    /// let coin_list = coin_api.get_coin_list()?;
    /// ```
    pub fn get_coin_list(&self) -> Result<Vec<CoinInfo>> {
        let data: HashMap<String, Value> = HashMap::new();
        let response = self.base.post("/user/getCoinList", Some(&data))?;
        self.base.validate_response(response)
    }
}

//! Workspace API - MPC workspace operations
//!
//! Provides methods for querying coin details and blockchain information.

use std::collections::HashMap;
use std::sync::Arc;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::crypto::CryptoProvider;
use crate::error::{Result, ValidationError};
use crate::mpc::api::base_api::MpcBaseApi;
use crate::mpc::config::MpcConfig;
use crate::utils::serde_helpers::deserialize_optional_i32;

// ============================================================================
// Request parameter types
// ============================================================================

/// Parameters for getting coin details
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetCoinDetailsParams {
    /// Coin symbol (required)
    pub symbol: String,
    /// Main chain symbol (optional, required for tokens)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub main_chain_symbol: Option<String>,
}

impl GetCoinDetailsParams {
    /// Creates new parameters with just symbol
    pub fn new(symbol: impl Into<String>) -> Self {
        Self {
            symbol: symbol.into(),
            main_chain_symbol: None,
        }
    }

    /// Creates new parameters with symbol and main chain symbol
    pub fn with_main_chain(
        symbol: impl Into<String>,
        main_chain_symbol: impl Into<String>,
    ) -> Self {
        Self {
            symbol: symbol.into(),
            main_chain_symbol: Some(main_chain_symbol.into()),
        }
    }

    fn to_map(&self) -> HashMap<String, Value> {
        let mut map = HashMap::new();
        map.insert("symbol".to_string(), Value::String(self.symbol.clone()));
        if let Some(ref main_chain_symbol) = self.main_chain_symbol {
            map.insert(
                "main_chain_symbol".to_string(),
                Value::String(main_chain_symbol.clone()),
            );
        }
        map
    }
}

/// Parameters for getting last block height
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetLastBlockHeightParams {
    /// Base symbol - the main chain symbol (e.g., "ETH", "BTC") (required)
    pub base_symbol: String,
}

impl GetLastBlockHeightParams {
    /// Creates new parameters
    pub fn new(base_symbol: impl Into<String>) -> Self {
        Self {
            base_symbol: base_symbol.into(),
        }
    }

    fn to_map(&self) -> HashMap<String, Value> {
        let mut map = HashMap::new();
        map.insert(
            "base_symbol".to_string(),
            Value::String(self.base_symbol.clone()),
        );
        map
    }
}

// ============================================================================
// Response types
// ============================================================================

/// Coin details information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoinDetails {
    /// Record ID
    #[serde(default)]
    pub id: Option<i64>,
    /// Network name (e.g., "Ethereum")
    #[serde(default)]
    pub coin_net: Option<String>,
    /// Coin symbol
    #[serde(default)]
    pub symbol: Option<String>,
    /// Real symbol
    #[serde(default)]
    pub real_symbol: Option<String>,
    /// Symbol alias
    #[serde(default)]
    pub symbol_alias: Option<String>,
    /// Base/Main chain symbol
    #[serde(default)]
    pub base_symbol: Option<String>,
    /// Merge address symbol
    #[serde(default)]
    pub merge_address_symbol: Option<String>,
    /// Coin decimals (API returns string like "18")
    #[serde(default, deserialize_with = "deserialize_optional_i32")]
    pub decimals: Option<i32>,
    /// Token contract address
    #[serde(default)]
    pub contract_address: Option<String>,
    /// Coin type (0 = native, 1 = token)
    #[serde(default)]
    pub coin_type: Option<i32>,
    /// Whether memo is supported ("0" = no, "1" = yes)
    #[serde(default)]
    pub support_memo: Option<String>,
    /// Whether token is supported ("0" = no, "1" = yes)
    #[serde(default)]
    pub support_token: Option<String>,
    /// Whether multiple addresses are supported
    #[serde(default)]
    pub support_multi_addr: Option<bool>,
    /// Whether acceleration is supported
    #[serde(default)]
    pub support_acceleration: Option<bool>,
    /// Whether the chain is open
    #[serde(default)]
    pub if_open_chain: Option<bool>,
    /// Coin icon URL
    #[serde(default)]
    pub icon: Option<String>,
    /// Address regex pattern
    #[serde(default)]
    pub address_regex: Option<String>,
    /// Address tag regex pattern
    #[serde(default)]
    pub address_tag_regex: Option<String>,
    /// Address explorer link
    #[serde(default)]
    pub address_link: Option<String>,
    /// Transaction explorer link
    #[serde(default)]
    pub txid_link: Option<String>,
    /// Minimum deposit amount
    #[serde(default)]
    pub min_deposit: Option<String>,
    /// Minimum withdraw amount
    #[serde(default)]
    pub min_withdraw: Option<String>,
    /// Deposit confirmation count
    #[serde(default)]
    pub deposit_confirmation: Option<String>,
    /// Withdraw confirmation count
    #[serde(default)]
    pub withdraw_confirmation: Option<String>,
}

/// Block height information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockHeightInfo {
    /// Latest block height (API returns "height")
    #[serde(default, alias = "height")]
    pub block_height: Option<i64>,
}

/// Supported coin/chain information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SupportedCoin {
    /// Network name (e.g., "Ethereum", "Bitcoin", "TRON")
    #[serde(default)]
    pub coin_net: Option<String>,
    /// Coin symbol (e.g., "ETH", "BTC", "TRX")
    #[serde(default)]
    pub symbol: Option<String>,
    /// Whether memo is supported (0 = no, 1 = yes)
    #[serde(default)]
    pub is_support_memo: Option<i32>,
    /// Chain ID (e.g., "1" for Ethereum mainnet, "56" for BSC)
    #[serde(default)]
    pub chain_id: Option<String>,
    /// Whether withdrawal is enabled
    #[serde(default)]
    pub enable_withdraw: Option<bool>,
    /// Whether deposit is enabled
    #[serde(default)]
    pub enable_deposit: Option<bool>,
    /// Whether acceleration is supported
    #[serde(default)]
    pub support_acceleration: Option<bool>,
    /// Whether payment is required
    #[serde(default)]
    pub need_payment: Option<bool>,
    /// Whether the chain is open (only in support_main_chain)
    #[serde(default)]
    pub if_open_chain: Option<bool>,
    /// Real symbol
    #[serde(default)]
    pub real_symbol: Option<String>,
    /// Symbol alias
    #[serde(default)]
    pub symbol_alias: Option<String>,
    /// Display order (only in support_main_chain)
    #[serde(default)]
    pub display_order: Option<i32>,
}

/// Response for get supported coins
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetSupportedCoinsResponse {
    /// List of opened main chains
    #[serde(default)]
    pub open_main_chain: Vec<SupportedCoin>,
    /// List of supported main chains
    #[serde(default)]
    pub support_main_chain: Vec<SupportedCoin>,
}

// ============================================================================
// Workspace API Implementation
// ============================================================================

/// Workspace API - MPC workspace operations
///
/// Provides methods for querying coin details and blockchain information.
pub struct WorkspaceApi {
    base: MpcBaseApi,
}

impl WorkspaceApi {
    /// Creates a new WorkspaceApi instance
    pub fn new(config: MpcConfig, crypto_provider: Arc<dyn CryptoProvider>) -> Self {
        Self {
            base: MpcBaseApi::new(config, crypto_provider),
        }
    }

    /// Gets details for a specific coin
    ///
    /// # Arguments
    /// * `params` - Coin detail parameters
    ///
    /// # Returns
    /// List of coin details matching the query
    ///
    /// # Example
    /// ```ignore
    /// let details = workspace_api.get_coin_details(GetCoinDetailsParams::new("ETH"))?;
    /// for coin in details {
    ///     println!("Symbol: {:?}, Decimals: {:?}", coin.symbol, coin.decimals);
    /// }
    /// ```
    pub fn get_coin_details(&self, params: GetCoinDetailsParams) -> Result<Vec<CoinDetails>> {
        if params.symbol.is_empty() {
            return Err(ValidationError::new("Parameter 'symbol' is required").into());
        }

        let data = params.to_map();
        let response = self.base.get("/api/mpc/coin_list", Some(&data))?;
        self.base.validate_response(response)
    }

    /// Gets the latest block height for a blockchain
    ///
    /// # Arguments
    /// * `params` - Block height parameters
    ///
    /// # Returns
    /// Block height information
    ///
    /// # Example
    /// ```ignore
    /// let info = workspace_api.get_last_block_height(GetLastBlockHeightParams::new("ETH"))?;
    /// println!("Block height: {:?}", info.block_height);
    /// ```
    pub fn get_last_block_height(
        &self,
        params: GetLastBlockHeightParams,
    ) -> Result<BlockHeightInfo> {
        if params.base_symbol.is_empty() {
            return Err(ValidationError::new("Parameter 'base_symbol' is required").into());
        }

        let data = params.to_map();
        let response = self.base.get("/api/mpc/chain_height", Some(&data))?;
        self.base.validate_response(response)
    }

    /// Gets list of supported coins
    ///
    /// # Returns
    /// List of supported coins
    ///
    /// # Example
    /// ```ignore
    /// let coins = workspace_api.get_supported_coins()?;
    /// for coin in coins.list {
    ///     println!("Symbol: {:?}", coin.symbol);
    /// }
    /// ```
    pub fn get_supported_coins(&self) -> Result<GetSupportedCoinsResponse> {
        let response = self.base.get("/api/mpc/wallet/open_coin", None)?;
        self.base.validate_response(response)
    }
}

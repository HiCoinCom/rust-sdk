//! Wallet API - MPC wallet management operations
//!
//! Provides methods for creating and managing MPC wallets.

use std::collections::HashMap;
use std::sync::Arc;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use rust_decimal::Decimal;

use crate::crypto::CryptoProvider;
use crate::error::{Result, ValidationError};
use crate::mpc::api::base_api::MpcBaseApi;
use crate::mpc::config::MpcConfig;

// ============================================================================
// Request parameter types
// ============================================================================

/// Parameters for creating a wallet
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateWalletParams {
    /// Wallet name (max 50 characters)
    pub sub_wallet_name: String,
    /// Display status: 1 (show), 2 (hide, default)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_show_status: Option<i32>,
}

impl CreateWalletParams {
    /// Creates new parameters
    pub fn new(sub_wallet_name: impl Into<String>) -> Self {
        Self {
            sub_wallet_name: sub_wallet_name.into(),
            app_show_status: None,
        }
    }

    /// Sets the display status
    pub fn with_show_status(mut self, status: i32) -> Self {
        self.app_show_status = Some(status);
        self
    }

    fn to_map(&self) -> HashMap<String, Value> {
        let mut map = HashMap::new();
        map.insert(
            "sub_wallet_name".to_string(),
            Value::String(self.sub_wallet_name.clone()),
        );
        if let Some(status) = self.app_show_status {
            map.insert("app_show_status".to_string(), Value::Number(status.into()));
        }
        map
    }
}

/// Parameters for creating a wallet address
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateWalletAddressParams {
    /// Wallet ID
    pub sub_wallet_id: i64,
    /// Unique identifier for the coin (e.g., "ETH")
    pub symbol: String,
}

impl CreateWalletAddressParams {
    /// Creates new parameters
    pub fn new(sub_wallet_id: i64, symbol: impl Into<String>) -> Self {
        Self {
            sub_wallet_id,
            symbol: symbol.into(),
        }
    }

    fn to_map(&self) -> HashMap<String, Value> {
        let mut map = HashMap::new();
        map.insert(
            "sub_wallet_id".to_string(),
            Value::Number(self.sub_wallet_id.into()),
        );
        map.insert("symbol".to_string(), Value::String(self.symbol.clone()));
        map
    }
}

/// Parameters for querying wallet addresses
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryWalletAddressParams {
    /// Wallet ID
    pub sub_wallet_id: i64,
    /// Unique identifier for the coin (e.g., "ETH")
    pub symbol: String,
    /// Starting address ID (optional, default: 0)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_id: Option<i64>,
}

impl QueryWalletAddressParams {
    /// Creates new parameters
    pub fn new(sub_wallet_id: i64, symbol: impl Into<String>) -> Self {
        Self {
            sub_wallet_id,
            symbol: symbol.into(),
            max_id: None,
        }
    }

    /// Sets the max ID for pagination
    pub fn with_max_id(mut self, max_id: i64) -> Self {
        self.max_id = Some(max_id);
        self
    }

    fn to_map(&self) -> HashMap<String, Value> {
        let mut map = HashMap::new();
        map.insert(
            "sub_wallet_id".to_string(),
            Value::Number(self.sub_wallet_id.into()),
        );
        map.insert("symbol".to_string(), Value::String(self.symbol.clone()));
        if let Some(max_id) = self.max_id {
            map.insert("max_id".to_string(), Value::Number(max_id.into()));
        }
        map
    }
}

/// Parameters for getting wallet assets
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetWalletAssetsParams {
    /// Wallet ID
    pub sub_wallet_id: i64,
    /// Unique identifier for the coin (e.g., "ETH")
    pub symbol: String,
}

impl GetWalletAssetsParams {
    /// Creates new parameters
    pub fn new(sub_wallet_id: i64, symbol: impl Into<String>) -> Self {
        Self {
            sub_wallet_id,
            symbol: symbol.into(),
        }
    }

    fn to_map(&self) -> HashMap<String, Value> {
        let mut map = HashMap::new();
        map.insert(
            "sub_wallet_id".to_string(),
            Value::Number(self.sub_wallet_id.into()),
        );
        map.insert("symbol".to_string(), Value::String(self.symbol.clone()));
        map
    }
}

/// Parameters for changing wallet display status
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChangeWalletShowStatusParams {
    /// Wallet IDs (comma-separated string, e.g., "123,456")
    pub sub_wallet_ids: String,
    /// Display status: 1 (show), 2 (hide)
    pub app_show_status: i32,
}

impl ChangeWalletShowStatusParams {
    /// Creates new parameters
    pub fn new(sub_wallet_ids: impl Into<String>, app_show_status: i32) -> Self {
        Self {
            sub_wallet_ids: sub_wallet_ids.into(),
            app_show_status,
        }
    }

    /// Creates parameters from a list of wallet IDs
    pub fn from_ids(ids: &[i64], app_show_status: i32) -> Self {
        let ids_str = ids
            .iter()
            .map(|id| id.to_string())
            .collect::<Vec<_>>()
            .join(",");
        Self {
            sub_wallet_ids: ids_str,
            app_show_status,
        }
    }

    fn to_map(&self) -> HashMap<String, Value> {
        let mut map = HashMap::new();
        map.insert(
            "sub_wallet_ids".to_string(),
            Value::String(self.sub_wallet_ids.clone()),
        );
        map.insert(
            "app_show_status".to_string(),
            Value::Number(self.app_show_status.into()),
        );
        map
    }
}

/// Parameters for querying wallet address info
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WalletAddressInfoParams {
    /// Any address to query
    pub address: String,
    /// Memo/tag (optional, required for memo-type addresses)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memo: Option<String>,
}

impl WalletAddressInfoParams {
    /// Creates new parameters
    pub fn new(address: impl Into<String>) -> Self {
        Self {
            address: address.into(),
            memo: None,
        }
    }

    /// Sets the memo/tag
    pub fn with_memo(mut self, memo: impl Into<String>) -> Self {
        self.memo = Some(memo.into());
        self
    }

    fn to_map(&self) -> HashMap<String, Value> {
        let mut map = HashMap::new();
        map.insert("address".to_string(), Value::String(self.address.clone()));
        if let Some(memo) = &self.memo {
            map.insert("memo".to_string(), Value::String(memo.clone()));
        }
        map
    }
}

// ============================================================================
// Response types
// ============================================================================

/// Wallet information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WalletInfo {
    /// Wallet ID
    #[serde(default)]
    pub sub_wallet_id: Option<i64>,
    /// Wallet name
    #[serde(default)]
    pub sub_wallet_name: Option<String>,
    /// Display status
    #[serde(default)]
    pub app_show_status: Option<i32>,
    /// Creation time
    #[serde(default)]
    pub created_at: Option<String>,
}

/// Wallet address information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WalletAddressInfo {
    /// Address ID
    #[serde(default)]
    pub id: Option<i64>,
    /// Address type (1: deposit address, 2: main address)
    #[serde(default)]
    pub addr_type: Option<i32>,
    /// Deposit address
    #[serde(default)]
    pub address: Option<String>,
    /// Address memo/tag
    #[serde(default)]
    pub memo: Option<String>,
}

/// Wallet asset information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WalletAssetInfo {
    /// Normal/available balance (API returns "normal_balance")
    #[serde(default, with = "rust_decimal::serde::str_option")]
    pub normal_balance: Option<Decimal>,
    /// Collecting balance (in collection process)
    #[serde(default, with = "rust_decimal::serde::str_option")]
    pub collecting_balance: Option<Decimal>,
    /// Locked balance
    #[serde(default, with = "rust_decimal::serde::str_option")]
    pub lock_balance: Option<Decimal>,
}

/// Wallet address info response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WalletAddressInfoResponse {
    /// Address type (1: deposit address, 2: main address)
    #[serde(default)]
    pub addr_type: Option<i32>,
    /// Wallet ID (if address belongs to this account)
    #[serde(default)]
    pub sub_wallet_id: Option<i64>,
    /// Merge address symbol (main chain symbol for address aggregation)
    #[serde(default)]
    pub merge_address_symbol: Option<String>,
}

// ============================================================================
// Wallet API Implementation
// ============================================================================

/// Wallet API - MPC wallet management operations
///
/// Provides methods for creating and managing MPC wallets.
pub struct WalletApi {
    base: MpcBaseApi,
}

impl WalletApi {
    /// Creates a new WalletApi instance
    pub fn new(config: MpcConfig, crypto_provider: Arc<dyn CryptoProvider>) -> Self {
        Self {
            base: MpcBaseApi::new(config, crypto_provider),
        }
    }

    /// Creates a new wallet
    ///
    /// # Arguments
    /// * `params` - Wallet creation parameters
    ///
    /// # Returns
    /// Created wallet information
    ///
    /// # Example
    /// ```ignore
    /// let wallet = wallet_api.create_wallet(CreateWalletParams::new("My Wallet"))?;
    /// ```
    pub fn create_wallet(&self, params: CreateWalletParams) -> Result<WalletInfo> {
        if params.sub_wallet_name.is_empty() {
            return Err(ValidationError::new("Parameter 'sub_wallet_name' is required").into());
        }
        if params.sub_wallet_name.len() > 50 {
            return Err(
                ValidationError::new("Wallet name cannot be longer than 50 characters").into(),
            );
        }

        let data = params.to_map();
        let response = self.base.post("/api/mpc/sub_wallet/create", Some(&data))?;
        self.base.validate_response(response)
    }

    /// Creates a wallet address
    ///
    /// # Arguments
    /// * `params` - Address creation parameters
    ///
    /// # Returns
    /// Created address information
    ///
    /// # Example
    /// ```ignore
    /// let address = wallet_api.create_wallet_address(CreateWalletAddressParams::new(123, "ETH"))?;
    /// ```
    pub fn create_wallet_address(
        &self,
        params: CreateWalletAddressParams,
    ) -> Result<WalletAddressInfo> {
        if params.symbol.is_empty() {
            return Err(ValidationError::new("Parameter 'symbol' is required").into());
        }

        let data = params.to_map();
        let response = self
            .base
            .post("/api/mpc/sub_wallet/create/address", Some(&data))?;
        self.base.validate_response(response)
    }

    /// Queries wallet address list
    ///
    /// # Arguments
    /// * `params` - Query parameters
    ///
    /// # Returns
    /// Wallet address list
    ///
    /// # Example
    /// ```ignore
    /// let addresses = wallet_api.query_wallet_address(QueryWalletAddressParams::new(123, "ETH"))?;
    /// ```
    pub fn query_wallet_address(
        &self,
        params: QueryWalletAddressParams,
    ) -> Result<Vec<WalletAddressInfo>> {
        if params.symbol.is_empty() {
            return Err(ValidationError::new("Parameter 'symbol' is required").into());
        }

        let data = params.to_map();
        let response = self
            .base
            .post("/api/mpc/sub_wallet/get/address/list", Some(&data))?;
        self.base.validate_response(response)
    }

    /// Gets wallet assets
    ///
    /// # Arguments
    /// * `params` - Query parameters
    ///
    /// # Returns
    /// Wallet asset information
    ///
    /// # Example
    /// ```ignore
    /// let assets = wallet_api.get_wallet_assets(GetWalletAssetsParams::new(123, "ETH"))?;
    /// ```
    pub fn get_wallet_assets(&self, params: GetWalletAssetsParams) -> Result<WalletAssetInfo> {
        if params.symbol.is_empty() {
            return Err(ValidationError::new("Parameter 'symbol' is required").into());
        }

        let data = params.to_map();
        let response = self.base.get("/api/mpc/sub_wallet/assets", Some(&data))?;
        self.base.validate_response(response)
    }

    /// Modifies the wallet display status
    ///
    /// # Arguments
    /// * `params` - Update parameters
    ///
    /// # Returns
    /// True if successful
    ///
    /// # Example
    /// ```ignore
    /// let result = wallet_api.change_wallet_show_status(
    ///     ChangeWalletShowStatusParams::from_ids(&[123, 456], 1)
    /// )?;
    /// ```
    pub fn change_wallet_show_status(&self, params: ChangeWalletShowStatusParams) -> Result<bool> {
        if params.sub_wallet_ids.is_empty() {
            return Err(ValidationError::new("Parameter 'sub_wallet_ids' is required").into());
        }
        if params.app_show_status != 1 && params.app_show_status != 2 {
            return Err(ValidationError::new("Parameter 'app_show_status' must be 1 or 2").into());
        }

        let data = params.to_map();
        let response = self
            .base
            .post("/api/mpc/sub_wallet/change_show_status", Some(&data))?;

        // 这个 API 的解密响应格式是 {"code":"0","msg":"success"}，没有额外的 data 字段
        // 直接检查响应的 code 字段来判断成功与否
        let code = response
            .get("code")
            .and_then(|v| v.as_str())
            .unwrap_or("-1");

        Ok(code == "0")
    }

    /// Queries wallet address information
    ///
    /// Verifies address information. Input a specific address and get the response
    /// of the corresponding custody user and currency information.
    ///
    /// # Arguments
    /// * `params` - Query parameters
    ///
    /// # Returns
    /// Address information including whether it belongs to this custody account
    ///
    /// # Example
    /// ```ignore
    /// let info = wallet_api.wallet_address_info(
    ///     WalletAddressInfoParams::new("0x123...")
    /// )?;
    /// if info.is_belong == Some(true) {
    ///     println!("Address belongs to wallet: {:?}", info.sub_wallet_id);
    /// }
    /// ```
    pub fn wallet_address_info(
        &self,
        params: WalletAddressInfoParams,
    ) -> Result<WalletAddressInfoResponse> {
        if params.address.is_empty() {
            return Err(ValidationError::new("Parameter 'address' is required").into());
        }

        let data = params.to_map();
        let response = self
            .base
            .get("/api/mpc/sub_wallet/address/info", Some(&data))?;
        self.base.validate_response(response)
    }
}

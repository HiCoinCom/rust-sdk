//! Account API - Account and balance management operations
//!
//! Provides methods for querying account balances and deposit addresses.

use std::collections::HashMap;
use std::sync::Arc;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::crypto::CryptoProvider;
use crate::error::Result;
use crate::waas::api::base_api::BaseApi;
use crate::waas::config::WaasConfig;

// ============================================================================
// Request parameter types
// ============================================================================

/// Parameters for getting user account balance
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetUserAccountParams {
    /// User ID
    pub uid: i64,
    /// Cryptocurrency symbol (e.g., "ETH", "BTC")
    pub symbol: String,
}

impl GetUserAccountParams {
    /// Creates new parameters
    pub fn new(uid: i64, symbol: impl Into<String>) -> Self {
        Self {
            uid,
            symbol: symbol.into(),
        }
    }

    fn to_map(&self) -> HashMap<String, Value> {
        let mut map = HashMap::new();
        map.insert("uid".to_string(), Value::Number(self.uid.into()));
        map.insert("symbol".to_string(), Value::String(self.symbol.clone()));
        map
    }
}

/// Parameters for getting user deposit address
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetUserAddressParams {
    /// User ID
    pub uid: i64,
    /// Cryptocurrency symbol (e.g., "ETH", "BTC")
    pub symbol: String,
}

impl GetUserAddressParams {
    /// Creates new parameters
    pub fn new(uid: i64, symbol: impl Into<String>) -> Self {
        Self {
            uid,
            symbol: symbol.into(),
        }
    }

    fn to_map(&self) -> HashMap<String, Value> {
        let mut map = HashMap::new();
        map.insert("uid".to_string(), Value::Number(self.uid.into()));
        map.insert("symbol".to_string(), Value::String(self.symbol.clone()));
        map
    }
}

/// Parameters for getting user address info by address
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetUserAddressInfoParams {
    /// Deposit address
    pub address: String,
}

impl GetUserAddressInfoParams {
    /// Creates new parameters
    pub fn new(address: impl Into<String>) -> Self {
        Self {
            address: address.into(),
        }
    }

    fn to_map(&self) -> HashMap<String, Value> {
        let mut map = HashMap::new();
        map.insert("address".to_string(), Value::String(self.address.clone()));
        map
    }
}

/// Parameters for getting company account balance
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetCompanyAccountParams {
    /// Cryptocurrency symbol (e.g., "ETH", "BTC")
    pub symbol: String,
}

impl GetCompanyAccountParams {
    /// Creates new parameters
    pub fn new(symbol: impl Into<String>) -> Self {
        Self {
            symbol: symbol.into(),
        }
    }

    fn to_map(&self) -> HashMap<String, Value> {
        let mut map = HashMap::new();
        map.insert("symbol".to_string(), Value::String(self.symbol.clone()));
        map
    }
}

// ============================================================================
// Response types
// ============================================================================

use rust_decimal::Decimal;

use crate::utils::serde_helpers::deserialize_optional_i64;

/// User account information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserAccountInfo {
    /// User ID
    #[serde(default, deserialize_with = "deserialize_optional_i64")]
    pub uid: Option<i64>,
    /// Cryptocurrency symbol
    #[serde(default)]
    pub symbol: Option<String>,
    /// Available balance
    #[serde(default, with = "rust_decimal::serde::str_option")]
    pub balance: Option<Decimal>,
    /// Frozen balance
    #[serde(default, with = "rust_decimal::serde::str_option")]
    pub frozen: Option<Decimal>,
    /// Address ID (from sync list)
    #[serde(default, deserialize_with = "deserialize_optional_i64")]
    pub id: Option<i64>,
}

/// User deposit address information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserAddressInfo {
    /// Address ID (from sync list)
    #[serde(default, deserialize_with = "deserialize_optional_i64")]
    pub id: Option<i64>,
    /// User ID
    #[serde(default, deserialize_with = "deserialize_optional_i64")]
    pub uid: Option<i64>,
    /// Cryptocurrency symbol
    #[serde(default)]
    pub symbol: Option<String>,
    /// Deposit address
    #[serde(default)]
    pub address: Option<String>,
}

/// Company account information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompanyAccountInfo {
    /// Cryptocurrency symbol
    #[serde(default)]
    pub symbol: Option<String>,
    /// Available balance
    #[serde(default, with = "rust_decimal::serde::str_option")]
    pub balance: Option<Decimal>,
    /// Frozen balance
    #[serde(default, with = "rust_decimal::serde::str_option")]
    pub frozen: Option<Decimal>,
}

/// Response for sync user address list
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncUserAddressListResponse {
    /// List of user addresses
    #[serde(default)]
    pub list: Vec<UserAddressInfo>,
}

// ============================================================================
// Account API Implementation
// ============================================================================

/// Account API - Account and balance management operations
///
/// Provides methods for querying account balances and deposit addresses.
pub struct AccountApi {
    base: BaseApi,
}

impl AccountApi {
    /// Creates a new AccountApi instance
    pub fn new(config: WaasConfig, crypto_provider: Arc<dyn CryptoProvider>) -> Self {
        Self {
            base: BaseApi::new(config, crypto_provider),
        }
    }

    /// Gets user account balance
    ///
    /// # Arguments
    /// * `params` - Query parameters
    ///
    /// # Returns
    /// User account information
    ///
    /// # Example
    /// ```ignore
    /// let balance = account_api.get_user_account(GetUserAccountParams::new(12345, "ETH"))?;
    /// ```
    pub fn get_user_account(&self, params: GetUserAccountParams) -> Result<UserAccountInfo> {
        let data = params.to_map();
        let response = self.base.post("/account/getByUidAndSymbol", Some(&data))?;
        self.base.validate_response(response)
    }

    /// Gets user deposit address
    ///
    /// # Arguments
    /// * `params` - Query parameters
    ///
    /// # Returns
    /// User address information
    ///
    /// # Example
    /// ```ignore
    /// let address = account_api.get_user_address(GetUserAddressParams::new(12345, "ETH"))?;
    /// ```
    pub fn get_user_address(&self, params: GetUserAddressParams) -> Result<UserAddressInfo> {
        let data = params.to_map();
        let response = self.base.post("/account/getDepositAddress", Some(&data))?;
        self.base.validate_response(response)
    }

    /// Gets user address info by deposit address
    ///
    /// # Arguments
    /// * `params` - Query parameters
    ///
    /// # Returns
    /// User address information
    ///
    /// # Example
    /// ```ignore
    /// let info = account_api.get_user_address_info(GetUserAddressInfoParams::new("0x1234..."))?;
    /// ```
    pub fn get_user_address_info(
        &self,
        params: GetUserAddressInfoParams,
    ) -> Result<UserAddressInfo> {
        let data = params.to_map();
        let response = self
            .base
            .post("/account/getDepositAddressInfo", Some(&data))?;
        self.base.validate_response(response)
    }

    /// Gets company account balance
    ///
    /// # Arguments
    /// * `params` - Query parameters
    ///
    /// # Returns
    /// Company account information
    ///
    /// # Example
    /// ```ignore
    /// let balance = account_api.get_company_account(GetCompanyAccountParams::new("ETH"))?;
    /// ```
    pub fn get_company_account(
        &self,
        params: GetCompanyAccountParams,
    ) -> Result<CompanyAccountInfo> {
        let data = params.to_map();
        let response = self.base.post("/account/getCompanyBySymbol", Some(&data))?;
        self.base.validate_response(response)
    }

    /// Syncs user address list by max ID (pagination)
    ///
    /// # Arguments
    /// * `max_id` - Maximum address ID for pagination (0 for first sync)
    ///
    /// # Returns
    /// Synced address list
    ///
    /// # Example
    /// ```ignore
    /// let addresses = account_api.sync_user_address_list(0)?;
    /// ```
    pub fn sync_user_address_list(&self, max_id: i64) -> Result<Vec<UserAddressInfo>> {
        let mut data = HashMap::new();
        data.insert("max_id".to_string(), Value::Number(max_id.into()));

        let response = self.base.post("/address/syncList", Some(&data))?;
        self.base.validate_response(response)
    }
}

//! Billing API - Deposit, withdrawal and miner fee operations
//!
//! Provides methods for withdraw requests and querying deposit/withdrawal records.

use std::collections::HashMap;
use std::sync::Arc;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use rust_decimal::Decimal;

use crate::crypto::CryptoProvider;
use crate::error::Result;
use crate::waas::api::base_api::BaseApi;
use crate::waas::config::WaasConfig;

// ============================================================================
// Request parameter types
// ============================================================================

/// Parameters for creating a withdrawal request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WithdrawParams {
    /// Unique request ID (merchant generated)
    pub request_id: String,
    /// Source user ID
    pub from_uid: i64,
    /// Destination address or (address_memo for XRP)
    pub to_address: String,
    /// Withdrawal amount
    pub amount: String,
    /// Cryptocurrency symbol (e.g., "BTC", "ETH")
    pub symbol: String,
    /// Withdrawal check_sum, for callback (provided by ChainUp) (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub check_sum: Option<String>,
}

impl WithdrawParams {
    /// Creates new parameters
    pub fn new(
        request_id: impl Into<String>,
        from_uid: i64,
        to_address: impl Into<String>,
        amount: impl Into<String>,
        symbol: impl Into<String>,
    ) -> Self {
        Self {
            request_id: request_id.into(),
            from_uid,
            to_address: to_address.into(),
            amount: amount.into(),
            symbol: symbol.into(),
            check_sum: None,
        }
    }

    /// Sets the check_sum
    pub fn with_check_sum(mut self, check_sum: impl Into<String>) -> Self {
        self.check_sum = Some(check_sum.into());
        self
    }

    fn to_map(&self) -> HashMap<String, Value> {
        let mut map = HashMap::new();
        map.insert(
            "request_id".to_string(),
            Value::String(self.request_id.clone()),
        );
        map.insert("from_uid".to_string(), Value::Number(self.from_uid.into()));
        map.insert(
            "to_address".to_string(),
            Value::String(self.to_address.clone()),
        );
        map.insert("amount".to_string(), Value::String(self.amount.clone()));
        map.insert("symbol".to_string(), Value::String(self.symbol.clone()));
        map
    }
}

// ============================================================================
// Response types
// ============================================================================

/// Withdrawal response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WithdrawResponse {
    /// Withdrawal ID
    #[serde(default)]
    pub id: Option<i64>,
    /// Request ID
    #[serde(default)]
    pub request_id: Option<String>,
}

/// Withdrawal record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WithdrawRecord {
    /// Record ID
    #[serde(default)]
    pub id: Option<i64>,
    /// Request ID
    #[serde(default)]
    pub request_id: Option<String>,
    /// User ID
    #[serde(default)]
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
    /// Withdrawal amount
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
    #[serde(default)]
    pub confirmations: Option<i32>,
    /// Contract address (for tokens)
    #[serde(default)]
    pub contract_address: Option<String>,
    /// Status
    #[serde(default)]
    pub status: Option<i32>,
    /// SaaS status
    #[serde(default)]
    pub saas_status: Option<i32>,
    /// Company status
    #[serde(default)]
    pub company_status: Option<i32>,
    /// Withdrawal fee
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
    /// Creation time (milliseconds timestamp)
    #[serde(default)]
    pub created_at: Option<i64>,
    /// Update time (milliseconds timestamp)
    #[serde(default)]
    pub updated_at: Option<i64>,
}

/// Deposit record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DepositRecord {
    /// Record ID
    #[serde(default)]
    pub id: Option<i64>,
    /// User ID
    #[serde(default)]
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
    /// Deposit amount
    #[serde(default, with = "rust_decimal::serde::str_option")]
    pub amount: Option<Decimal>,
    /// Deposit address
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
    #[serde(default)]
    pub confirmations: Option<i32>,
    /// Contract address (for tokens)
    #[serde(default)]
    pub contract_address: Option<String>,
    /// Is mining reward
    #[serde(default)]
    pub is_mining: Option<i32>,
    /// Status
    #[serde(default)]
    pub status: Option<i32>,
    /// Creation time (milliseconds timestamp)
    #[serde(default)]
    pub created_at: Option<i64>,
    /// Update time (milliseconds timestamp)
    #[serde(default)]
    pub updated_at: Option<i64>,
}

/// Miner fee record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MinerFeeRecord {
    /// Record ID
    #[serde(default)]
    pub id: Option<i64>,
    /// Cryptocurrency symbol
    #[serde(default)]
    pub symbol: Option<String>,
    /// Miner fee amount
    #[serde(default, with = "rust_decimal::serde::str_option")]
    pub amount: Option<Decimal>,
    /// Miner fee symbol
    #[serde(default)]
    pub fee_symbol: Option<String>,
    /// Transaction hash
    #[serde(default)]
    pub txid: Option<String>,
    /// Status
    #[serde(default)]
    pub status: Option<i32>,
    /// Creation time (milliseconds timestamp)
    #[serde(default)]
    pub created_at: Option<i64>,
}

/// Response for sync withdraw list
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncWithdrawListResponse {
    /// List of withdrawal records
    #[serde(default)]
    pub list: Vec<WithdrawRecord>,
}

/// Response for sync deposit list
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncDepositListResponse {
    /// List of deposit records
    #[serde(default)]
    pub list: Vec<DepositRecord>,
}

/// Response for sync miner fee list
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncMinerFeeListResponse {
    /// List of miner fee records
    #[serde(default)]
    pub list: Vec<MinerFeeRecord>,
}

// ============================================================================
// Billing API Implementation
// ============================================================================

/// Billing API - Deposit, withdrawal and miner fee operations
///
/// Provides methods for withdraw requests and querying deposit/withdrawal records.
pub struct BillingApi {
    base: BaseApi,
}

impl BillingApi {
    /// Creates a new BillingApi instance
    pub fn new(config: WaasConfig, crypto_provider: Arc<dyn CryptoProvider>) -> Self {
        Self {
            base: BaseApi::new(config, crypto_provider),
        }
    }

    /// Creates a withdrawal request
    ///
    /// # Arguments
    /// * `params` - Withdrawal parameters
    ///
    /// # Returns
    /// Withdrawal result
    ///
    /// # Example
    /// ```ignore
    /// let result = billing_api.withdraw(WithdrawParams::new(
    ///     "withdraw_001",
    ///     12345,
    ///     "0x1234...",
    ///     "1.5",
    ///     "ETH",
    /// ))?;
    /// ```
    pub fn withdraw(&self, params: WithdrawParams) -> Result<WithdrawResponse> {
        let data = params.to_map();
        let response = self.base.post("/billing/withdraw", Some(&data))?;
        self.base.validate_response(response)
    }

    /// Gets withdrawal records by request IDs
    ///
    /// # Arguments
    /// * `ids` - List of request IDs
    ///
    /// # Returns
    /// Withdrawal records
    ///
    /// # Example
    /// ```ignore
    /// let withdrawals = billing_api.withdraw_list(&["withdraw_001", "withdraw_002"])?;
    /// ```
    pub fn withdraw_list(&self, ids: &[&str]) -> Result<Vec<WithdrawRecord>> {
        let mut data = HashMap::new();
        data.insert("ids".to_string(), Value::String(ids.join(",")));

        let response = self.base.post("/billing/withdrawList", Some(&data))?;
        self.base.validate_response(response)
    }

    /// Syncs withdrawal records by max ID (pagination)
    ///
    /// # Arguments
    /// * `max_id` - Maximum transaction ID for pagination
    ///
    /// # Returns
    /// Synced withdrawal records
    ///
    /// # Example
    /// ```ignore
    /// let withdrawals = billing_api.sync_withdraw_list(0)?;
    /// ```
    pub fn sync_withdraw_list(&self, max_id: i64) -> Result<Vec<WithdrawRecord>> {
        let mut data = HashMap::new();
        data.insert("max_id".to_string(), Value::Number(max_id.into()));

        let response = self.base.post("/billing/syncWithdrawList", Some(&data))?;
        self.base.validate_response(response)
    }

    /// Gets deposit records by WaaS IDs
    ///
    /// # Arguments
    /// * `ids` - List of WaaS deposit IDs
    ///
    /// # Returns
    /// Deposit records
    ///
    /// # Example
    /// ```ignore
    /// let deposits = billing_api.deposit_list(&["123", "456"])?;
    /// ```
    pub fn deposit_list(&self, ids: &[&str]) -> Result<Vec<DepositRecord>> {
        let mut data = HashMap::new();
        data.insert("ids".to_string(), Value::String(ids.join(",")));

        let response = self.base.post("/billing/depositList", Some(&data))?;
        self.base.validate_response(response)
    }

    /// Syncs deposit records by max ID (pagination)
    ///
    /// # Arguments
    /// * `max_id` - Maximum transaction ID for pagination
    ///
    /// # Returns
    /// Synced deposit records
    ///
    /// # Example
    /// ```ignore
    /// let deposits = billing_api.sync_deposit_list(0)?;
    /// ```
    pub fn sync_deposit_list(&self, max_id: i64) -> Result<Vec<DepositRecord>> {
        let mut data = HashMap::new();
        data.insert("max_id".to_string(), Value::Number(max_id.into()));

        let response = self.base.post("/billing/syncDepositList", Some(&data))?;
        self.base.validate_response(response)
    }

    /// Gets miner fee records by WaaS IDs
    ///
    /// # Arguments
    /// * `ids` - List of WaaS transaction IDs
    ///
    /// # Returns
    /// Miner fee records
    ///
    /// # Example
    /// ```ignore
    /// let fees = billing_api.miner_fee_list(&["123", "456"])?;
    /// ```
    pub fn miner_fee_list(&self, ids: &[&str]) -> Result<Vec<MinerFeeRecord>> {
        let mut data = HashMap::new();
        data.insert("ids".to_string(), Value::String(ids.join(",")));

        let response = self.base.post("/billing/minerFeeList", Some(&data))?;
        self.base.validate_response(response)
    }

    /// Syncs miner fee records by max ID (pagination)
    ///
    /// # Arguments
    /// * `max_id` - Maximum transaction ID for pagination
    ///
    /// # Returns
    /// Synced miner fee records
    ///
    /// # Example
    /// ```ignore
    /// let fees = billing_api.sync_miner_fee_list(0)?;
    /// ```
    pub fn sync_miner_fee_list(&self, max_id: i64) -> Result<Vec<MinerFeeRecord>> {
        let mut data = HashMap::new();
        data.insert("max_id".to_string(), Value::Number(max_id.into()));

        let response = self.base.post("/billing/syncMinerFeeList", Some(&data))?;
        self.base.validate_response(response)
    }
}

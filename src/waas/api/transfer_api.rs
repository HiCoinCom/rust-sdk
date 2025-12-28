//! Transfer API - Internal account transfer operations
//!
//! Provides methods for transferring funds between merchant accounts.

use std::collections::HashMap;
use std::sync::Arc;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use rust_decimal::Decimal;

use crate::crypto::CryptoProvider;
use crate::enums::QueryIdType;
use crate::error::Result;
use crate::waas::api::base_api::BaseApi;
use crate::waas::config::WaasConfig;

// ============================================================================
// Request parameter types
// ============================================================================

/// Parameters for internal account transfer
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountTransferParams {
    /// Unique request ID (merchant generated)
    pub request_id: String,
    /// Cryptocurrency symbol (e.g., "BTC", "ETH")
    pub symbol: String,
    /// Transfer amount
    pub amount: String,
    /// Source user ID (as string)
    pub from: String,
    /// Destination user ID (as string)
    pub to: String,
    /// Transfer remark (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remark: Option<String>,
}

impl AccountTransferParams {
    /// Creates new parameters
    pub fn new(
        request_id: impl Into<String>,
        symbol: impl Into<String>,
        amount: impl Into<String>,
        from: impl Into<String>,
        to: impl Into<String>,
    ) -> Self {
        Self {
            request_id: request_id.into(),
            symbol: symbol.into(),
            amount: amount.into(),
            from: from.into(),
            to: to.into(),
            remark: None,
        }
    }

    /// Sets the remark
    pub fn with_remark(mut self, remark: impl Into<String>) -> Self {
        self.remark = Some(remark.into());
        self
    }

    fn to_map(&self) -> HashMap<String, Value> {
        let mut map = HashMap::new();
        map.insert(
            "request_id".to_string(),
            Value::String(self.request_id.clone()),
        );
        map.insert("symbol".to_string(), Value::String(self.symbol.clone()));
        map.insert("amount".to_string(), Value::String(self.amount.clone()));
        map.insert("from".to_string(), Value::String(self.from.clone()));
        map.insert("to".to_string(), Value::String(self.to.clone()));

        if let Some(ref remark) = self.remark {
            map.insert("remark".to_string(), Value::String(remark.clone()));
        }
        map
    }
}

/// Parameters for getting transfer list
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetAccountTransferListParams {
    /// Comma-separated list of IDs to query
    pub ids: String,
    /// Type of IDs: "request_id" or "receipt"
    pub ids_type: String,
}

impl GetAccountTransferListParams {
    /// Creates new parameters
    pub fn new(ids: impl Into<String>, ids_type: QueryIdType) -> Self {
        Self {
            ids: ids.into(),
            ids_type: ids_type.as_str().to_string(),
        }
    }

    /// Creates parameters for request ID query
    pub fn by_request_id(ids: &[&str]) -> Self {
        Self {
            ids: ids.join(","),
            ids_type: "request_id".to_string(),
        }
    }

    /// Creates parameters for receipt query
    pub fn by_receipt(ids: &[&str]) -> Self {
        Self {
            ids: ids.join(","),
            ids_type: "receipt".to_string(),
        }
    }

    fn to_map(&self) -> HashMap<String, Value> {
        let mut map = HashMap::new();
        map.insert("ids".to_string(), Value::String(self.ids.clone()));
        map.insert("ids_type".to_string(), Value::String(self.ids_type.clone()));
        map
    }
}

// ============================================================================
// Response types
// ============================================================================

/// Transfer record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferRecord {
    /// Record ID
    #[serde(default)]
    pub id: Option<i64>,
    /// Request ID
    #[serde(default)]
    pub request_id: Option<String>,
    /// Receipt number
    #[serde(default)]
    pub receipt: Option<String>,
    /// Cryptocurrency symbol
    #[serde(default)]
    pub symbol: Option<String>,
    /// Transfer amount
    #[serde(default, with = "rust_decimal::serde::str_option")]
    pub amount: Option<Decimal>,
    /// Source user ID
    #[serde(default)]
    pub from: Option<String>,
    /// Destination user ID
    #[serde(default)]
    pub to: Option<String>,
    /// Status
    #[serde(default)]
    pub status: Option<i32>,
    /// Remark
    #[serde(default)]
    pub remark: Option<String>,
    /// Creation time (milliseconds timestamp)
    #[serde(default)]
    pub created_at: Option<i64>,
}

/// Response for sync account transfer list
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncAccountTransferListResponse {
    /// List of transfer records
    #[serde(default)]
    pub list: Vec<TransferRecord>,
}

// ============================================================================
// Transfer API Implementation
// ============================================================================

/// Transfer API - Internal account transfer operations
///
/// Provides methods for transferring funds between merchant accounts.
pub struct TransferApi {
    base: BaseApi,
}

impl TransferApi {
    /// Creates a new TransferApi instance
    pub fn new(config: WaasConfig, crypto_provider: Arc<dyn CryptoProvider>) -> Self {
        Self {
            base: BaseApi::new(config, crypto_provider),
        }
    }

    /// Internal transfer between merchant accounts
    ///
    /// # Arguments
    /// * `params` - Transfer parameters
    ///
    /// # Returns
    /// Transfer result
    ///
    /// # Example
    /// ```ignore
    /// let result = transfer_api.account_transfer(AccountTransferParams::new(
    ///     "transfer_001",
    ///     "USDT",
    ///     "100.5",
    ///     "123",
    ///     "456",
    /// ))?;
    /// ```
    pub fn account_transfer(&self, params: AccountTransferParams) -> Result<TransferRecord> {
        let data = params.to_map();
        let response = self.base.post("/account/transfer", Some(&data))?;
        self.base.validate_response(response)
    }

    /// Gets transfer records by request IDs or receipts
    ///
    /// # Arguments
    /// * `params` - Query parameters
    ///
    /// # Returns
    /// Transfer records
    ///
    /// # Example
    /// ```ignore
    /// let transfers = transfer_api.get_account_transfer_list(
    ///     GetAccountTransferListParams::by_request_id(&["transfer_001", "transfer_002"])
    /// )?;
    /// ```
    pub fn get_account_transfer_list(
        &self,
        params: GetAccountTransferListParams,
    ) -> Result<Vec<TransferRecord>> {
        let data = params.to_map();
        let response = self.base.post("/account/transferList", Some(&data))?;
        self.base.validate_response(response)
    }

    /// Syncs transfer records by max ID (pagination)
    ///
    /// # Arguments
    /// * `max_id` - Maximum transaction ID for pagination
    ///
    /// # Returns
    /// Synced transfer records
    ///
    /// # Example
    /// ```ignore
    /// let transfers = transfer_api.sync_account_transfer_list(0)?;
    /// ```
    pub fn sync_account_transfer_list(&self, max_id: i64) -> Result<Vec<TransferRecord>> {
        let mut data = HashMap::new();
        data.insert("max_id".to_string(), Value::Number(max_id.into()));

        let response = self.base.post("/account/syncTransferList", Some(&data))?;
        self.base.validate_response(response)
    }
}

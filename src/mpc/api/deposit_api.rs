//! Deposit API - MPC deposit management operations
//!
//! Provides methods for querying deposit records.

use std::collections::HashMap;
use std::sync::Arc;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use rust_decimal::Decimal;

use crate::crypto::CryptoProvider;
use crate::error::{Result, ValidationError};
use crate::mpc::api::base_api::MpcBaseApi;
use crate::mpc::config::MpcConfig;
use crate::utils::serde_helpers::deserialize_optional_i64;

// ============================================================================
// Request parameter types
// ============================================================================

/// Parameters for getting deposit records
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetDepositRecordsParams {
    /// Receiving IDs (list of integers, up to 100)
    pub ids: Vec<i64>,
}

impl GetDepositRecordsParams {
    /// Creates new parameters
    pub fn new(ids: Vec<i64>) -> Self {
        Self { ids }
    }

    fn to_map(&self) -> HashMap<String, Value> {
        let mut map = HashMap::new();
        let ids_str = self
            .ids
            .iter()
            .map(|id| id.to_string())
            .collect::<Vec<_>>()
            .join(",");
        map.insert("ids".to_string(), Value::String(ids_str));
        map
    }
}

// ============================================================================
// Response types
// ============================================================================

/// Deposit record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DepositRecord {
    /// Record ID
    #[serde(default)]
    pub id: Option<i64>,
    /// Sub-wallet ID
    #[serde(default)]
    pub sub_wallet_id: Option<i64>,
    /// Cryptocurrency symbol
    #[serde(default)]
    pub symbol: Option<String>,
    /// Base chain symbol
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
    /// Address memo/tag
    #[serde(default)]
    pub memo: Option<String>,
    /// Transaction hash
    #[serde(default)]
    pub txid: Option<String>,
    /// Confirmations
    #[serde(default)]
    pub confirmations: Option<i32>,
    /// Transaction block height
    #[serde(default, deserialize_with = "deserialize_optional_i64")]
    pub tx_height: Option<i64>,
    /// Contract address (for token transfers)
    #[serde(default)]
    pub contract_address: Option<String>,
    /// Status
    #[serde(default)]
    pub status: Option<i32>,
    /// Deposit type (1: normal token deposit, 2: web3 transaction deposit, 10: collection, 11: collection miner fee)
    #[serde(default)]
    pub deposit_type: Option<i32>,
    /// Refund amount
    #[serde(default, with = "rust_decimal::serde::str_option")]
    pub refund_amount: Option<Decimal>,
    /// KYT status
    #[serde(default)]
    pub kyt_status: Option<String>,
    /// Remark
    #[serde(default)]
    pub remark: Option<String>,
    /// Creation time (timestamp in milliseconds)
    #[serde(default, deserialize_with = "deserialize_optional_i64")]
    pub created_at: Option<i64>,
    /// Update time (timestamp in milliseconds)
    #[serde(default, deserialize_with = "deserialize_optional_i64")]
    pub updated_at: Option<i64>,
}

/// Response for sync deposit records
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncDepositRecordsResponse {
    /// List of deposit records
    #[serde(default)]
    pub list: Vec<DepositRecord>,
}

// ============================================================================
// Deposit API Implementation
// ============================================================================

/// Deposit API - MPC deposit management operations
///
/// Provides methods for querying deposit records.
pub struct DepositApi {
    base: MpcBaseApi,
}

impl DepositApi {
    /// Creates a new DepositApi instance
    pub fn new(config: MpcConfig, crypto_provider: Arc<dyn CryptoProvider>) -> Self {
        Self {
            base: MpcBaseApi::new(config, crypto_provider),
        }
    }

    /// Gets receiving records
    ///
    /// # Arguments
    /// * `params` - Query parameters
    ///
    /// # Returns
    /// Deposit records
    ///
    /// # Example
    /// ```ignore
    /// let deposits = deposit_api.get_deposit_records(GetDepositRecordsParams::new(vec![123, 456, 789]))?;
    /// ```
    pub fn get_deposit_records(
        &self,
        params: GetDepositRecordsParams,
    ) -> Result<Vec<DepositRecord>> {
        if params.ids.is_empty() {
            return Err(ValidationError::new(
                "Parameter 'ids' is required and must be a non-empty list",
            )
            .into());
        }

        let data = params.to_map();
        let response = self
            .base
            .get("/api/mpc/billing/deposit_list", Some(&data))?;
        self.base.validate_response(response)
    }

    /// Synchronizes transfer (deposit) records
    ///
    /// # Arguments
    /// * `max_id` - Receiving record initial ID (default: 0)
    ///
    /// # Returns
    /// Synchronized deposit records response containing list
    ///
    /// # Example
    /// ```ignore
    /// let response = deposit_api.sync_deposit_records(0)?;
    /// for deposit in response.list {
    ///     println!("Deposit: {:?}", deposit);
    /// }
    /// ```
    pub fn sync_deposit_records(&self, max_id: i64) -> Result<Vec<DepositRecord>> {
        let mut data = HashMap::new();
        data.insert("max_id".to_string(), Value::Number(max_id.into()));

        let response = self
            .base
            .get("/api/mpc/billing/sync_deposit_list", Some(&data))?;
        self.base.validate_response(response)
    }
}

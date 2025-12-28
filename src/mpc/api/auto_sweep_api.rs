//! Auto Sweep API - MPC auto collection operations
//!
//! Provides methods for auto-sweeping funds from sub-wallets.

use std::collections::HashMap;
use std::sync::Arc;

use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::crypto::CryptoProvider;
use crate::error::{Result, ValidationError};
use crate::mpc::api::base_api::MpcBaseApi;
use crate::mpc::config::MpcConfig;

// ============================================================================
// Request parameter types
// ============================================================================

/// Parameters for getting auto-sweep wallets
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutoCollectSubWalletsParams {
    /// Unique identifier for the coin (e.g., "USDTERC20")
    pub symbol: String,
}

impl AutoCollectSubWalletsParams {
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

/// Parameters for setting auto-collection symbol configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetAutoCollectSymbolParams {
    /// Unique identifier for the coin (e.g., "USDTERC20")
    pub symbol: String,
    /// Minimum amount for auto-sweep (up to 6 decimal places)
    pub collect_min: String,
    /// Maximum miner fee amount for auto-sweep refueling (up to 6 decimal places)
    pub fueling_limit: String,
}

impl SetAutoCollectSymbolParams {
    /// Creates new parameters
    pub fn new(
        symbol: impl Into<String>,
        collect_min: impl Into<String>,
        fueling_limit: impl Into<String>,
    ) -> Self {
        Self {
            symbol: symbol.into(),
            collect_min: collect_min.into(),
            fueling_limit: fueling_limit.into(),
        }
    }

    fn to_map(&self) -> HashMap<String, Value> {
        let mut map = HashMap::new();
        map.insert("symbol".to_string(), Value::String(self.symbol.clone()));
        map.insert(
            "collect_min".to_string(),
            Value::String(self.collect_min.clone()),
        );
        map.insert(
            "fueling_limit".to_string(),
            Value::String(self.fueling_limit.clone()),
        );
        map
    }
}

// ============================================================================
// Response types
// ============================================================================

/// Auto collection sub wallets result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutoCollectResult {
    /// Fueling sub wallet ID (for fee refueling)
    #[serde(default)]
    pub fueling_sub_wallet_id: Option<i64>,
    /// Collect sub wallet ID (for collection destination)
    #[serde(default)]
    pub collect_sub_wallet_id: Option<i64>,
}

/// Auto collection record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutoCollectRecord {
    /// Record ID
    #[serde(default)]
    pub id: i64,
    /// Sub-wallet ID
    #[serde(default)]
    pub sub_wallet_id: i64,
    /// Coin symbol
    #[serde(default)]
    pub symbol: Option<String>,
    /// Collection amount
    #[serde(default, with = "rust_decimal::serde::str_option")]
    pub amount: Option<Decimal>,
    /// Estimated fee
    #[serde(default, with = "rust_decimal::serde::str_option")]
    pub fee: Option<Decimal>,
    /// Actual fee
    #[serde(default, with = "rust_decimal::serde::str_option")]
    pub real_fee: Option<Decimal>,
    /// Fee coin symbol
    #[serde(default)]
    pub fee_symbol: Option<String>,
    /// Sender address
    #[serde(default)]
    pub address_from: Option<String>,
    /// Receiver address
    #[serde(default)]
    pub address_to: Option<String>,
    /// Contract address
    #[serde(default)]
    pub contract_address: Option<String>,
    /// Transaction hash
    #[serde(default)]
    pub txid: Option<String>,
    /// Address memo/tag
    #[serde(default)]
    pub memo: Option<String>,
    /// Remark
    #[serde(default)]
    pub remark: Option<String>,
    /// Confirmation count
    #[serde(default)]
    pub confirmations: Option<i64>,
    /// Transaction block height
    #[serde(default)]
    pub tx_height: Option<i64>,
    /// Base chain symbol
    #[serde(default)]
    pub base_symbol: Option<String>,
    /// Collection status
    #[serde(default)]
    pub status: Option<i64>,
    /// Transaction type (10: collect)
    #[serde(default)]
    pub trans_type: Option<i64>,
    /// Created timestamp (milliseconds)
    #[serde(default)]
    pub created_at: Option<i64>,
    /// Updated timestamp (milliseconds)
    #[serde(default)]
    pub updated_at: Option<i64>,
}

// ============================================================================
// Auto Sweep API Implementation
// ============================================================================

/// Auto Sweep API - MPC auto collection operations
///
/// Provides methods for auto-sweeping funds from sub-wallets.
pub struct AutoSweepApi {
    base: MpcBaseApi,
}

impl AutoSweepApi {
    /// Creates a new AutoSweepApi instance
    pub fn new(config: MpcConfig, crypto_provider: Arc<dyn CryptoProvider>) -> Self {
        Self {
            base: MpcBaseApi::new(config, crypto_provider),
        }
    }

    /// Gets auto-sweep wallets for a specific coin
    ///
    /// Retrieve the auto-sweep wallet and auto fueling wallet for a specific coin.
    ///
    /// # Arguments
    /// * `params` - Query parameters containing symbol
    ///
    /// # Returns
    /// Auto-sweep wallet information
    ///
    /// # Example
    /// ```ignore
    /// let wallets = auto_sweep_api.auto_collect_sub_wallets(
    ///     AutoCollectSubWalletsParams::new("USDTERC20")
    /// )?;
    /// ```
    pub fn auto_collect_sub_wallets(
        &self,
        params: AutoCollectSubWalletsParams,
    ) -> Result<AutoCollectResult> {
        if params.symbol.is_empty() {
            return Err(ValidationError::new("Parameter 'symbol' is required").into());
        }

        let data = params.to_map();
        let response = self
            .base
            .get("/api/mpc/auto_collect/sub_wallets", Some(&data))?;
        self.base.validate_response(response)
    }

    /// Configures auto-sweep for a coin
    ///
    /// Set the minimum auto-sweep amount and the maximum miner fee for refueling.
    ///
    /// # Arguments
    /// * `params` - Configuration parameters
    ///
    /// # Returns
    /// Ok(()) if successful, or an error
    ///
    /// # Example
    /// ```ignore
    /// auto_sweep_api.set_auto_collect_symbol(SetAutoCollectSymbolParams::new(
    ///     "USDTERC20",
    ///     "100",      // collect_min
    ///     "0.01",     // fueling_limit
    /// ))?;
    /// ```
    pub fn set_auto_collect_symbol(&self, params: SetAutoCollectSymbolParams) -> Result<()> {
        if params.symbol.is_empty() {
            return Err(ValidationError::new("Parameter 'symbol' is required").into());
        }
        if params.collect_min.is_empty() {
            return Err(ValidationError::new("Parameter 'collect_min' is required").into());
        }
        if params.fueling_limit.is_empty() {
            return Err(ValidationError::new("Parameter 'fueling_limit' is required").into());
        }

        let data = params.to_map();
        let response = self
            .base
            .post("/api/mpc/auto_collect/symbol/set", Some(&data))?;
        let _: serde_json::Value = self.base.validate_response(response)?;
        Ok(())
    }

    /// Syncs auto-collection records by max ID (pagination)
    ///
    /// Retrieve up to 100 sweeping records for all wallets under a workspace.
    ///
    /// # Arguments
    /// * `max_id` - Starting ID for sweeping records (default: 0)
    ///
    /// # Returns
    /// Synced auto-collection records
    ///
    /// # Example
    /// ```ignore
    /// let records = auto_sweep_api.sync_auto_collect_records(0)?;
    /// ```
    pub fn sync_auto_collect_records(&self, max_id: i64) -> Result<Vec<AutoCollectRecord>> {
        let mut data = HashMap::new();
        data.insert("max_id".to_string(), Value::Number(max_id.into()));

        let response = self
            .base
            .get("/api/mpc/billing/sync_auto_collect_list", Some(&data))?;
        self.base.validate_response(response)
    }
}

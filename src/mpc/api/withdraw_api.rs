//! Withdraw API - MPC withdrawal management operations
//!
//! Provides methods for initiating withdrawals and querying withdrawal records.

use std::collections::HashMap;
use std::sync::Arc;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use rust_decimal::Decimal;

use crate::crypto::CryptoProvider;
use crate::error::{Result, ValidationError};
use crate::mpc::api::base_api::MpcBaseApi;
use crate::mpc::config::MpcConfig;
use crate::mpc::sign_util::{MpcSignUtil, WithdrawSignParams};
use crate::utils::serde_helpers::deserialize_optional_i64;

// ============================================================================
// Request parameter types
// ============================================================================

/// Parameters for withdrawal
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WithdrawParams {
    /// Unique request ID
    pub request_id: String,
    /// Sub-wallet ID
    pub sub_wallet_id: i64,
    /// Coin symbol (e.g., "USDTERC20")
    pub symbol: String,
    /// Withdrawal amount
    pub amount: String,
    /// Destination address
    pub address_to: String,
    /// Specify the transfer coin address (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<String>,
    /// Address memo (for coins that require it) (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memo: Option<String>,
    /// Withdrawal remark (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remark: Option<String>,
    /// UTXO outputs (for BTC-like coins) (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outputs: Option<String>,
    /// Whether to sign the transaction (optional, default: false)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub need_transaction_sign: Option<bool>,
}

impl WithdrawParams {
    /// Creates new parameters
    pub fn new(
        request_id: impl Into<String>,
        sub_wallet_id: i64,
        symbol: impl Into<String>,
        amount: impl Into<String>,
        address_to: impl Into<String>,
    ) -> Self {
        Self {
            request_id: request_id.into(),
            sub_wallet_id,
            symbol: symbol.into(),
            amount: amount.into(),
            address_to: address_to.into(),
            from: None,
            memo: None,
            remark: None,
            outputs: None,
            need_transaction_sign: None,
        }
    }

    /// Sets the from address
    pub fn with_from(mut self, from: impl Into<String>) -> Self {
        self.from = Some(from.into());
        self
    }

    /// Sets the memo
    pub fn with_memo(mut self, memo: impl Into<String>) -> Self {
        self.memo = Some(memo.into());
        self
    }

    /// Sets the remark
    pub fn with_remark(mut self, remark: impl Into<String>) -> Self {
        self.remark = Some(remark.into());
        self
    }

    /// Sets the UTXO outputs
    pub fn with_outputs(mut self, outputs: impl Into<String>) -> Self {
        self.outputs = Some(outputs.into());
        self
    }

    /// Enables transaction signing
    pub fn with_transaction_sign(mut self) -> Self {
        self.need_transaction_sign = Some(true);
        self
    }

    fn to_map(&self) -> HashMap<String, Value> {
        let mut map = HashMap::new();
        map.insert(
            "request_id".to_string(),
            Value::String(self.request_id.clone()),
        );
        map.insert(
            "sub_wallet_id".to_string(),
            Value::Number(self.sub_wallet_id.into()),
        );
        map.insert("symbol".to_string(), Value::String(self.symbol.clone()));
        map.insert("amount".to_string(), Value::String(self.amount.clone()));
        map.insert(
            "address_to".to_string(),
            Value::String(self.address_to.clone()),
        );

        if let Some(ref from) = self.from {
            map.insert("from".to_string(), Value::String(from.clone()));
        }
        if let Some(ref memo) = self.memo {
            map.insert("memo".to_string(), Value::String(memo.clone()));
        }
        if let Some(ref remark) = self.remark {
            map.insert("remark".to_string(), Value::String(remark.clone()));
        }
        if let Some(ref outputs) = self.outputs {
            map.insert("outputs".to_string(), Value::String(outputs.clone()));
        }
        map
    }

    fn to_sign_params(&self) -> WithdrawSignParams {
        WithdrawSignParams {
            request_id: self.request_id.clone(),
            sub_wallet_id: self.sub_wallet_id,
            symbol: self.symbol.clone(),
            address_to: self.address_to.clone(),
            amount: self.amount.clone(),
            memo: self.memo.clone(),
            outputs: self.outputs.clone(),
        }
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
    pub withdraw_id: Option<i64>,
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
    /// Sub-wallet ID
    #[serde(default)]
    pub sub_wallet_id: Option<i64>,
    /// Cryptocurrency symbol
    #[serde(default)]
    pub symbol: Option<String>,
    /// Base cryptocurrency symbol
    #[serde(default)]
    pub base_symbol: Option<String>,
    /// Withdrawal amount
    #[serde(default, with = "rust_decimal::serde::str_option")]
    pub amount: Option<Decimal>,
    /// Source address
    #[serde(default)]
    pub address_from: Option<String>,
    /// Destination address
    #[serde(default)]
    pub address_to: Option<String>,
    /// Address memo/tag
    #[serde(default)]
    pub memo: Option<String>,
    /// Transaction hash
    #[serde(default)]
    pub txid: Option<String>,
    /// Status
    #[serde(default)]
    pub status: Option<i32>,
    /// Creation time (timestamp in milliseconds)
    #[serde(default, deserialize_with = "deserialize_optional_i64")]
    pub created_at: Option<i64>,
    /// Update time (timestamp in milliseconds)
    #[serde(default, deserialize_with = "deserialize_optional_i64")]
    pub updated_at: Option<i64>,
    /// Estimated miner fee
    #[serde(default, with = "rust_decimal::serde::str_option")]
    pub fee: Option<Decimal>,
    /// Actual miner fee
    #[serde(default, with = "rust_decimal::serde::str_option")]
    pub real_fee: Option<Decimal>,
    /// Miner fee symbol
    #[serde(default)]
    pub fee_symbol: Option<String>,
    /// Number of confirmations
    #[serde(default)]
    pub confirmations: Option<i32>,
    /// Transaction block height
    #[serde(default)]
    pub tx_height: Option<i64>,
    /// Contract address (for token transfers)
    #[serde(default)]
    pub contract_address: Option<String>,
    /// Withdrawal remark
    #[serde(default)]
    pub remark: Option<String>,
    /// Withdrawal source (1: app, 2: openapi, 3: web, 10: collect, 11: collect-fee)
    #[serde(default)]
    pub withdraw_source: Option<i32>,
}

/// Response for sync withdraw records
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncWithdrawRecordsResponse {
    /// List of withdrawal records
    #[serde(default)]
    pub list: Vec<WithdrawRecord>,
}

// ============================================================================
// Withdraw API Implementation
// ============================================================================

/// Withdraw API - MPC withdrawal management operations
///
/// Provides methods for initiating withdrawals and querying withdrawal records.
pub struct WithdrawApi {
    base: MpcBaseApi,
    crypto_provider: Arc<dyn CryptoProvider>,
    sign_private_key_available: bool,
}

impl WithdrawApi {
    /// Creates a new WithdrawApi instance
    pub fn new(config: MpcConfig, crypto_provider: Arc<dyn CryptoProvider>) -> Self {
        // Sign uses sign_private_key, or falls back to rsa_private_key
        let sign_private_key_available =
            !config.sign_private_key.is_empty() || !config.rsa_private_key.is_empty();
        Self {
            base: MpcBaseApi::new(config, crypto_provider.clone()),
            crypto_provider,
            sign_private_key_available,
        }
    }

    /// Initiates a transfer (withdrawal)
    ///
    /// # Arguments
    /// * `params` - Withdrawal parameters
    ///
    /// # Returns
    /// Withdrawal result with withdraw_id
    ///
    /// # Example
    /// ```ignore
    /// let result = withdraw_api.withdraw(WithdrawParams::new(
    ///     "unique-id",
    ///     123,
    ///     "ETH",
    ///     "0.1",
    ///     "0x123...",
    /// ))?;
    /// ```
    pub fn withdraw(&self, params: WithdrawParams) -> Result<WithdrawResponse> {
        // Validate required fields
        if params.request_id.is_empty() {
            return Err(ValidationError::new("Parameter 'request_id' is required").into());
        }
        if params.symbol.is_empty() {
            return Err(ValidationError::new("Parameter 'symbol' is required").into());
        }
        if params.amount.is_empty() {
            return Err(ValidationError::new("Parameter 'amount' is required").into());
        }
        if params.address_to.is_empty() {
            return Err(ValidationError::new("Parameter 'address_to' is required").into());
        }

        let need_sign = params.need_transaction_sign.unwrap_or(false);

        // Check if signing is available when needed
        if need_sign && !self.sign_private_key_available {
            return Err(ValidationError::new(
                "MPC withdrawal requires sign_private_key in config when need_transaction_sign is true"
            ).into());
        }

        let mut data = params.to_map();

        // Generate signature if needed
        if need_sign {
            let sign_params = params.to_sign_params();
            let signature =
                MpcSignUtil::generate_withdraw_sign(&sign_params, self.crypto_provider.as_ref())?;

            data.insert("sign".to_string(), Value::String(signature));
        }

        let response = self.base.post("/api/mpc/billing/withdraw", Some(&data))?;
        self.base.validate_response(response)
    }

    /// Gets transfer records
    ///
    /// # Arguments
    /// * `request_ids` - Request IDs (list of strings, up to 100)
    ///
    /// # Returns
    /// Withdrawal records
    ///
    /// # Example
    /// ```ignore
    /// let records = withdraw_api.get_withdraw_records(&["req-1", "req-2"])?;
    /// ```
    pub fn get_withdraw_records(&self, request_ids: &[&str]) -> Result<Vec<WithdrawRecord>> {
        if request_ids.is_empty() {
            return Err(ValidationError::new(
                "Parameter 'request_ids' is required and must be a non-empty list",
            )
            .into());
        }

        let mut data = HashMap::new();
        data.insert("ids".to_string(), Value::String(request_ids.join(",")));

        let response = self
            .base
            .get("/api/mpc/billing/withdraw_list", Some(&data))?;
        self.base.validate_response(response)
    }

    /// Synchronizes transfer (withdraw) records
    ///
    /// # Arguments
    /// * `max_id` - Starting ID of withdraw records (default: 0)
    ///
    /// # Returns
    /// List of synchronized withdrawal records
    ///
    /// # Example
    /// ```ignore
    /// let records = withdraw_api.sync_withdraw_records(0)?;
    /// for record in records {
    ///     println!("Withdraw: {:?}", record);
    /// }
    /// ```
    pub fn sync_withdraw_records(&self, max_id: i64) -> Result<Vec<WithdrawRecord>> {
        let mut data = HashMap::new();
        data.insert("max_id".to_string(), Value::Number(max_id.into()));

        let response = self
            .base
            .get("/api/mpc/billing/sync_withdraw_list", Some(&data))?;
        self.base.validate_response(response)
    }
}

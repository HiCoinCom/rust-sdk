//! Web3 API - MPC Web3 transaction operations
//!
//! Provides methods for creating, accelerating, and querying Web3 transactions.

use std::collections::HashMap;
use std::sync::Arc;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use rust_decimal::Decimal;

use crate::crypto::CryptoProvider;
use crate::error::{Result, ValidationError};
use crate::mpc::api::base_api::MpcBaseApi;
use crate::mpc::config::MpcConfig;
use crate::mpc::sign_util::{MpcSignUtil, Web3SignParams};
use crate::utils::serde_helpers::{deserialize_optional_i32, deserialize_optional_i64};

// ============================================================================
// Request parameter types
// ============================================================================

/// Parameters for creating a Web3 transaction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateWeb3TransParams {
    /// Unique request ID (required)
    pub request_id: String,
    /// Sub-wallet ID (required)
    pub sub_wallet_id: i64,
    /// Main chain coin symbol, e.g. ETH (required)
    pub main_chain_symbol: String,
    /// Interactive contract address (required)
    pub interactive_contract: String,
    /// Transfer amount (required)
    pub amount: String,
    /// Gas price in Gwei (required)
    pub gas_price: String,
    /// Gas limit (required)
    pub gas_limit: String,
    /// Hexadecimal data for contract transaction (required)
    pub input_data: String,
    /// Transaction type: 0=Authorization, 1=Other (required)
    pub trans_type: String,
    /// Transaction initiation address (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<String>,
    /// Dapp name (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dapp_name: Option<String>,
    /// Dapp URL (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dapp_url: Option<String>,
    /// Dapp image (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dapp_img: Option<String>,
    /// Whether transaction signature is required (optional, default: false)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub need_transaction_sign: Option<bool>,
}

impl CreateWeb3TransParams {
    /// Creates new parameters
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        request_id: impl Into<String>,
        sub_wallet_id: i64,
        main_chain_symbol: impl Into<String>,
        interactive_contract: impl Into<String>,
        amount: impl Into<String>,
        gas_price: impl Into<String>,
        gas_limit: impl Into<String>,
        input_data: impl Into<String>,
        trans_type: impl Into<String>,
    ) -> Self {
        Self {
            request_id: request_id.into(),
            sub_wallet_id,
            main_chain_symbol: main_chain_symbol.into(),
            interactive_contract: interactive_contract.into(),
            amount: amount.into(),
            gas_price: gas_price.into(),
            gas_limit: gas_limit.into(),
            input_data: input_data.into(),
            trans_type: trans_type.into(),
            from: None,
            dapp_name: None,
            dapp_url: None,
            dapp_img: None,
            need_transaction_sign: None,
        }
    }

    /// Sets the from address
    pub fn with_from(mut self, from: impl Into<String>) -> Self {
        self.from = Some(from.into());
        self
    }

    /// Sets the dapp name
    pub fn with_dapp_name(mut self, dapp_name: impl Into<String>) -> Self {
        self.dapp_name = Some(dapp_name.into());
        self
    }

    /// Sets the dapp URL
    pub fn with_dapp_url(mut self, dapp_url: impl Into<String>) -> Self {
        self.dapp_url = Some(dapp_url.into());
        self
    }

    /// Sets the dapp image
    pub fn with_dapp_img(mut self, dapp_img: impl Into<String>) -> Self {
        self.dapp_img = Some(dapp_img.into());
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
        map.insert(
            "main_chain_symbol".to_string(),
            Value::String(self.main_chain_symbol.clone()),
        );
        map.insert(
            "interactive_contract".to_string(),
            Value::String(self.interactive_contract.clone()),
        );
        map.insert("amount".to_string(), Value::String(self.amount.clone()));
        map.insert(
            "gas_price".to_string(),
            Value::String(self.gas_price.clone()),
        );
        map.insert(
            "gas_limit".to_string(),
            Value::String(self.gas_limit.clone()),
        );
        map.insert(
            "input_data".to_string(),
            Value::String(self.input_data.clone()),
        );
        map.insert(
            "trans_type".to_string(),
            Value::String(self.trans_type.clone()),
        );

        if let Some(ref from) = self.from {
            map.insert("from".to_string(), Value::String(from.clone()));
        }
        if let Some(ref dapp_name) = self.dapp_name {
            map.insert("dapp_name".to_string(), Value::String(dapp_name.clone()));
        }
        if let Some(ref dapp_url) = self.dapp_url {
            map.insert("dapp_url".to_string(), Value::String(dapp_url.clone()));
        }
        if let Some(ref dapp_img) = self.dapp_img {
            map.insert("dapp_img".to_string(), Value::String(dapp_img.clone()));
        }
        map
    }

    fn to_sign_params(&self) -> Web3SignParams {
        Web3SignParams {
            request_id: self.request_id.clone(),
            sub_wallet_id: self.sub_wallet_id,
            main_chain_symbol: self.main_chain_symbol.clone(),
            interactive_contract: self.interactive_contract.clone(),
            amount: self.amount.clone(),
            input_data: self.input_data.clone(),
        }
    }
}

/// Parameters for accelerating a Web3 transaction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccelerateWeb3TransParams {
    /// Web3 transaction ID (required)
    pub trans_id: i64,
    /// New gas price in Gwei (required)
    pub gas_price: String,
    /// New gas limit (required)
    pub gas_limit: String,
}

impl AccelerateWeb3TransParams {
    /// Creates new parameters
    pub fn new(trans_id: i64, gas_price: impl Into<String>, gas_limit: impl Into<String>) -> Self {
        Self {
            trans_id,
            gas_price: gas_price.into(),
            gas_limit: gas_limit.into(),
        }
    }

    fn to_map(&self) -> HashMap<String, Value> {
        let mut map = HashMap::new();
        map.insert("trans_id".to_string(), Value::Number(self.trans_id.into()));
        map.insert(
            "gas_price".to_string(),
            Value::String(self.gas_price.clone()),
        );
        map.insert(
            "gas_limit".to_string(),
            Value::String(self.gas_limit.clone()),
        );
        map
    }
}

// ============================================================================
// Response types
// ============================================================================

/// Web3 transaction record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Web3TransRecord {
    /// Record ID
    #[serde(default)]
    pub id: Option<i64>,
    /// Request ID
    #[serde(default)]
    pub request_id: Option<String>,
    /// Sub-wallet ID
    #[serde(default)]
    pub sub_wallet_id: Option<i64>,
    /// Main chain symbol
    #[serde(default)]
    pub main_chain_symbol: Option<String>,
    /// Token symbol
    #[serde(default)]
    pub symbol: Option<String>,
    /// Interactive contract
    #[serde(default)]
    pub interactive_contract: Option<String>,
    /// Amount
    #[serde(default, with = "rust_decimal::serde::str_option")]
    pub amount: Option<Decimal>,
    /// Gas price
    #[serde(default, with = "rust_decimal::serde::str_option")]
    pub gas_price: Option<Decimal>,
    /// Gas limit
    #[serde(default, with = "rust_decimal::serde::str_option")]
    pub gas_limit: Option<Decimal>,
    /// Gas used
    #[serde(default, with = "rust_decimal::serde::str_option")]
    pub gas_used: Option<Decimal>,
    /// Transaction hash
    #[serde(default)]
    pub txid: Option<String>,
    /// From address
    #[serde(default)]
    pub address_from: Option<String>,
    /// To address
    #[serde(default)]
    pub address_to: Option<String>,
    /// Estimated fee
    #[serde(default, with = "rust_decimal::serde::str_option")]
    pub fee: Option<Decimal>,
    /// Actual fee
    #[serde(default, with = "rust_decimal::serde::str_option")]
    pub real_fee: Option<Decimal>,
    /// Fee token symbol
    #[serde(default)]
    pub fee_symbol: Option<String>,
    /// Status
    #[serde(default)]
    pub status: Option<i32>,
    /// Transaction type
    #[serde(default, deserialize_with = "deserialize_optional_i32")]
    pub trans_type: Option<i32>,
    /// Transaction source (1: web app, 2: open-api)
    #[serde(default, deserialize_with = "deserialize_optional_i32")]
    pub trans_source: Option<i32>,
    /// Number of confirmations
    #[serde(default, deserialize_with = "deserialize_optional_i32")]
    pub confirmations: Option<i32>,
    /// Transaction block height
    #[serde(default, deserialize_with = "deserialize_optional_i64")]
    pub tx_height: Option<i64>,
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

/// Response for sync Web3 transaction records
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncWeb3TransRecordsResponse {
    /// List of Web3 transaction records
    #[serde(default)]
    pub list: Vec<Web3TransRecord>,
}

// ============================================================================
// Web3 API Implementation
// ============================================================================

/// Web3 API - MPC Web3 transaction operations
///
/// Provides methods for creating, accelerating, and querying Web3 transactions.
pub struct Web3Api {
    base: MpcBaseApi,
    crypto_provider: Arc<dyn CryptoProvider>,
    sign_private_key_available: bool,
}

impl Web3Api {
    /// Creates a new Web3Api instance
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

    /// Creates a Web3 transaction
    ///
    /// # Arguments
    /// * `params` - Transaction parameters
    ///
    /// # Returns
    /// Created transaction result
    ///
    /// # Example
    /// ```ignore
    /// let result = web3_api.create_web3_trans(CreateWeb3TransParams::new(
    ///     "unique-id",
    ///     123,
    ///     "ETH",
    ///     "0x123...",
    ///     "1000000000000000000",
    ///     "20",
    ///     "21000",
    ///     "0x",
    ///     "1",
    /// ))?;
    /// ```
    pub fn create_web3_trans(&self, params: CreateWeb3TransParams) -> Result<Web3TransRecord> {
        // Validate required fields
        let required_fields = [
            ("request_id", &params.request_id),
            ("main_chain_symbol", &params.main_chain_symbol),
            ("interactive_contract", &params.interactive_contract),
            ("amount", &params.amount),
            ("gas_price", &params.gas_price),
            ("gas_limit", &params.gas_limit),
            ("input_data", &params.input_data),
            ("trans_type", &params.trans_type),
        ];

        for (name, value) in required_fields {
            if value.is_empty() {
                return Err(
                    ValidationError::new(format!("Parameter '{}' is required", name)).into(),
                );
            }
        }

        let need_sign = params.need_transaction_sign.unwrap_or(false);

        // Check if signing is available when needed
        if need_sign && !self.sign_private_key_available {
            return Err(ValidationError::new(
                "MPC Web3 transaction requires sign_private_key in config when need_transaction_sign is true"
            ).into());
        }

        let mut data = params.to_map();

        // Generate signature if needed
        if need_sign {
            let sign_params = params.to_sign_params();
            let signature =
                MpcSignUtil::generate_web3_sign(&sign_params, self.crypto_provider.as_ref())?;
            data.insert("sign".to_string(), Value::String(signature));
        }

        let response = self.base.post("/api/mpc/web3/trans/create", Some(&data))?;
        self.base.validate_response(response)
    }

    /// Accelerates a Web3 transaction
    ///
    /// # Arguments
    /// * `params` - Acceleration parameters
    ///
    /// # Returns
    /// Acceleration result
    ///
    /// # Example
    /// ```ignore
    /// let result = web3_api.accelerate_web3_trans(AccelerateWeb3TransParams::new(
    ///     12345678,  // trans_id: Web3 transaction ID
    ///     "50",      // gas_price in Gwei
    ///     "30000",   // gas_limit
    /// ))?;
    /// ```
    pub fn accelerate_web3_trans(
        &self,
        params: AccelerateWeb3TransParams,
    ) -> Result<Web3TransRecord> {
        if params.trans_id <= 0 {
            return Err(ValidationError::new(
                "Parameter 'trans_id' is required and must be positive",
            )
            .into());
        }
        if params.gas_price.is_empty() {
            return Err(ValidationError::new("Parameter 'gas_price' is required").into());
        }
        if params.gas_limit.is_empty() {
            return Err(ValidationError::new("Parameter 'gas_limit' is required").into());
        }

        let data = params.to_map();
        let response = self.base.post("/api/mpc/web3/pending", Some(&data))?;
        self.base.validate_response(response)
    }

    /// Gets Web3 transaction records
    ///
    /// # Arguments
    /// * `request_ids` - Request IDs (list of strings, up to 100)
    ///
    /// # Returns
    /// Web3 transaction records
    ///
    /// # Example
    /// ```ignore
    /// let records = web3_api.get_web3_trans_records(&["req-1", "req-2"])?;
    /// ```
    pub fn get_web3_trans_records(&self, request_ids: &[&str]) -> Result<Vec<Web3TransRecord>> {
        if request_ids.is_empty() {
            return Err(ValidationError::new(
                "Parameter 'request_ids' is required and must be a non-empty list",
            )
            .into());
        }

        let mut data = HashMap::new();
        data.insert("ids".to_string(), Value::String(request_ids.join(",")));

        let response = self.base.get("/api/mpc/web3/trans_list", Some(&data))?;
        self.base.validate_response(response)
    }

    /// Synchronizes Web3 transaction records
    ///
    /// # Arguments
    /// * `max_id` - Starting ID of Web3 records (default: 0)
    ///
    /// # Returns
    /// List of synchronized Web3 transaction records
    ///
    /// # Example
    /// ```ignore
    /// let records = web3_api.sync_web3_trans_records(0)?;
    /// for record in records {
    ///     println!("Web3 tx: {:?}", record);
    /// }
    /// ```
    pub fn sync_web3_trans_records(&self, max_id: i64) -> Result<Vec<Web3TransRecord>> {
        let mut data = HashMap::new();
        data.insert("max_id".to_string(), Value::Number(max_id.into()));

        let response = self
            .base
            .get("/api/mpc/web3/sync_trans_list", Some(&data))?;
        self.base.validate_response(response)
    }
}

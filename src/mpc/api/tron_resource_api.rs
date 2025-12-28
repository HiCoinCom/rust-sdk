//! Tron Resource API - TRON resource delegation operations
//!
//! Provides methods for buying and querying TRON network resources (Energy/Bandwidth).

use std::collections::HashMap;
use std::sync::Arc;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::crypto::CryptoProvider;
use crate::error::{Result, ValidationError};
use crate::mpc::api::base_api::MpcBaseApi;
use crate::mpc::config::MpcConfig;

// ============================================================================
// Request parameter types
// ============================================================================

/// Parameters for creating a Tron delegate (Buy Tron Resource)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateTronDelegateParams {
    /// Unique request ID (required)
    pub request_id: String,
    /// Buy type: 0=System, 1=Manual (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buy_type: Option<i32>,
    /// Resource type: 0=Energy, 1=Bandwidth (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resource_type: Option<i32>,
    /// Service charge type (required): "10010"=10min, "20001"=1hour, "30001"=1day
    pub service_charge_type: String,
    /// Energy amount to purchase (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub energy_num: Option<i64>,
    /// Bandwidth amount to purchase (optional)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub net_num: Option<i64>,
    /// Address paying for resources (required)
    pub address_from: String,
    /// Address to receive resources (optional, required for buy_type 0 or 2)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_to: Option<String>,
    /// Contract address (optional, required for buy_type 0 or 2)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contract_address: Option<String>,
}

impl CreateTronDelegateParams {
    /// Creates new parameters with required fields
    pub fn new(
        request_id: impl Into<String>,
        address_from: impl Into<String>,
        service_charge_type: impl Into<String>,
    ) -> Self {
        Self {
            request_id: request_id.into(),
            buy_type: None,
            resource_type: None,
            service_charge_type: service_charge_type.into(),
            energy_num: None,
            net_num: None,
            address_from: address_from.into(),
            address_to: None,
            contract_address: None,
        }
    }

    /// Sets the buy type
    pub fn with_buy_type(mut self, buy_type: i32) -> Self {
        self.buy_type = Some(buy_type);
        self
    }

    /// Sets the resource type
    pub fn with_resource_type(mut self, resource_type: i32) -> Self {
        self.resource_type = Some(resource_type);
        self
    }

    /// Sets the energy amount
    pub fn with_energy_num(mut self, energy_num: i64) -> Self {
        self.energy_num = Some(energy_num);
        self
    }

    /// Sets the bandwidth amount
    pub fn with_net_num(mut self, net_num: i64) -> Self {
        self.net_num = Some(net_num);
        self
    }

    /// Sets the receiving address
    pub fn with_address_to(mut self, address_to: impl Into<String>) -> Self {
        self.address_to = Some(address_to.into());
        self
    }

    /// Sets the contract address
    pub fn with_contract_address(mut self, contract_address: impl Into<String>) -> Self {
        self.contract_address = Some(contract_address.into());
        self
    }

    fn to_map(&self) -> HashMap<String, Value> {
        let mut map = HashMap::new();
        map.insert(
            "request_id".to_string(),
            Value::String(self.request_id.clone()),
        );
        map.insert(
            "service_charge_type".to_string(),
            Value::String(self.service_charge_type.clone()),
        );
        map.insert(
            "address_from".to_string(),
            Value::String(self.address_from.clone()),
        );

        if let Some(buy_type) = self.buy_type {
            map.insert("buy_type".to_string(), Value::Number(buy_type.into()));
        }
        if let Some(resource_type) = self.resource_type {
            map.insert(
                "resource_type".to_string(),
                Value::Number(resource_type.into()),
            );
        }
        if let Some(energy_num) = self.energy_num {
            map.insert("energy_num".to_string(), Value::Number(energy_num.into()));
        }
        if let Some(net_num) = self.net_num {
            map.insert("net_num".to_string(), Value::Number(net_num.into()));
        }
        if let Some(ref address_to) = self.address_to {
            map.insert("address_to".to_string(), Value::String(address_to.clone()));
        }
        if let Some(ref contract_address) = self.contract_address {
            map.insert(
                "contract_address".to_string(),
                Value::String(contract_address.clone()),
            );
        }
        map
    }
}

// ============================================================================
// Response types
// ============================================================================

/// Tron delegate transaction result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TronDelegateResult {
    /// Transaction ID
    #[serde(default)]
    pub trans_id: Option<String>,
    /// Request ID
    #[serde(default)]
    pub request_id: Option<String>,
    /// Raw data for additional fields
    #[serde(flatten)]
    pub extra: HashMap<String, Value>,
}

/// Tron resource record (buy resource record)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TronResourceRecord {
    /// Record ID
    #[serde(default)]
    pub id: Option<i64>,
    /// Request ID
    #[serde(default)]
    pub request_id: Option<String>,
    /// Buy type: 0=System estimate, 1=Customer specified quantity
    #[serde(default)]
    pub buy_type: Option<i32>,
    /// Resource type: 0=Energy and Bandwidth, 1=Energy only
    #[serde(default)]
    pub resource_type: Option<i32>,
    /// Service charge rate
    #[serde(default)]
    pub service_charge_rate: Option<String>,
    /// Service charge amount
    #[serde(default)]
    pub service_charge: Option<String>,
    /// Energy amount
    #[serde(default)]
    pub energy_num: Option<i64>,
    /// Bandwidth amount
    #[serde(default)]
    pub net_num: Option<i64>,
    /// From address
    #[serde(default)]
    pub address_from: Option<String>,
    /// To address
    #[serde(default)]
    pub address_to: Option<String>,
    /// Contract address
    #[serde(default)]
    pub contract_address: Option<String>,
    /// Energy transaction hash
    #[serde(default)]
    pub energy_txid: Option<String>,
    /// Net/Bandwidth transaction hash
    #[serde(default)]
    pub net_txid: Option<String>,
    /// Reclaim energy transaction hash
    #[serde(default)]
    pub reclaim_energy_txid: Option<String>,
    /// Reclaim net transaction hash
    #[serde(default)]
    pub reclaim_net_txid: Option<String>,
    /// Energy delegation time (timestamp in ms)
    #[serde(default)]
    pub energy_time: Option<i64>,
    /// Net delegation time (timestamp in ms)
    #[serde(default)]
    pub net_time: Option<i64>,
    /// Energy reclaim time (timestamp in ms)
    #[serde(default)]
    pub reclaim_energy_time: Option<i64>,
    /// Net reclaim time (timestamp in ms)
    #[serde(default)]
    pub reclaim_net_time: Option<i64>,
    /// Energy price
    #[serde(default)]
    pub energy_price: Option<String>,
    /// Net/Bandwidth price
    #[serde(default)]
    pub net_price: Option<String>,
    /// Status
    #[serde(default)]
    pub status: Option<i32>,
}

/// Response for sync buy resource records
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncBuyResourceRecordsResponse {
    /// List of Tron resource records
    #[serde(default)]
    pub list: Vec<TronResourceRecord>,
}

// ============================================================================
// Tron Resource API Implementation
// ============================================================================

/// Tron Resource API - TRON resource delegation operations
///
/// Provides methods for buying and querying TRON network resources (Energy/Bandwidth).
pub struct TronResourceApi {
    base: MpcBaseApi,
}

impl TronResourceApi {
    /// Creates a new TronResourceApi instance
    pub fn new(config: MpcConfig, crypto_provider: Arc<dyn CryptoProvider>) -> Self {
        Self {
            base: MpcBaseApi::new(config, crypto_provider),
        }
    }

    /// Creates a Tron delegate (Buy TRON Resource)
    ///
    /// Purchase TRON network energy or bandwidth for a specific address.
    ///
    /// # Arguments
    /// * `params` - Delegate parameters
    ///
    /// # Returns
    /// Delegation result with trans_id
    ///
    /// # Example
    /// ```ignore
    /// let result = tron_api.create_tron_delegate(
    ///     CreateTronDelegateParams::new("unique-id", "TXxxx...", "10010")
    ///         .with_resource_type(0)  // Energy
    ///         .with_energy_num(32000)
    ///         .with_address_to("TRxxx...")
    ///         .with_contract_address("TEDxxx...")
    /// )?;
    /// ```
    pub fn create_tron_delegate(
        &self,
        params: CreateTronDelegateParams,
    ) -> Result<TronDelegateResult> {
        if params.request_id.is_empty() {
            return Err(ValidationError::new("Parameter 'request_id' is required").into());
        }
        if params.address_from.is_empty() {
            return Err(ValidationError::new("Parameter 'address_from' is required").into());
        }
        if params.service_charge_type.is_empty() {
            return Err(ValidationError::new("Parameter 'service_charge_type' is required").into());
        }

        // Additional validation for buy_type 0 or 2
        if let Some(buy_type) = params.buy_type {
            if buy_type == 0 || buy_type == 2 {
                if params.address_to.is_none() || params.contract_address.is_none() {
                    return Err(ValidationError::new(
                        "For buy_type 0 or 2, address_to and contract_address are required",
                    )
                    .into());
                }
            }
        }

        let data = params.to_map();
        let response = self.base.post("/api/mpc/tron/delegate", Some(&data))?;
        self.base.validate_response(response)
    }

    /// Gets buy resource records by request IDs
    ///
    /// Get delegation records by request IDs.
    ///
    /// # Arguments
    /// * `request_ids` - Request IDs (list of strings, up to 100)
    ///
    /// # Returns
    /// Tron resource records
    ///
    /// # Example
    /// ```ignore
    /// let records = tron_api.get_buy_resource_records(&["req-1", "req-2"])?;
    /// ```
    pub fn get_buy_resource_records(
        &self,
        request_ids: &[&str],
    ) -> Result<Vec<TronResourceRecord>> {
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
            .post("/api/mpc/tron/delegate/trans_list", Some(&data))?;
        self.base.validate_response(response)
    }

    /// Synchronizes buy resource records
    ///
    /// Get all delegation records, maximum of 100 records.
    ///
    /// # Arguments
    /// * `max_id` - Starting ID of delegation records (default: 0)
    ///
    /// # Returns
    /// List of synchronized delegation records
    ///
    /// # Example
    /// ```ignore
    /// let records = tron_api.sync_buy_resource_records(0)?;
    /// for record in records {
    ///     println!("Record: {:?}", record.request_id);
    /// }
    /// ```
    pub fn sync_buy_resource_records(&self, max_id: i64) -> Result<Vec<TronResourceRecord>> {
        let mut data = HashMap::new();
        data.insert("max_id".to_string(), Value::Number(max_id.into()));

        let response = self
            .base
            .post("/api/mpc/tron/delegate/sync_trans_list", Some(&data))?;
        self.base.validate_response(response)
    }
}

//! MPC Sign Utility
//!
//! Provides signature generation for MPC transactions.

use std::collections::BTreeMap;

use crate::crypto::CryptoProvider;
use crate::error::Result;

/// MPC Sign Utility
///
/// Provides signature generation for MPC transactions.
pub struct MpcSignUtil;

impl MpcSignUtil {
    /// Generates signature for withdrawal transaction
    ///
    /// # Arguments
    /// * `params` - Transaction parameters
    /// * `crypto_provider` - Crypto provider for signing
    ///
    /// # Returns
    /// Base64 encoded signature
    pub fn generate_withdraw_sign(
        params: &WithdrawSignParams,
        crypto_provider: &dyn CryptoProvider,
    ) -> Result<String> {
        // Build sign string from params in sorted order (BTreeMap auto-sorts by key ASCII ascending)
        // Empty values will be filtered out and not participate in signing
        let mut sign_map: BTreeMap<&str, String> = BTreeMap::new();
        sign_map.insert("request_id", params.request_id.clone());
        sign_map.insert("sub_wallet_id", params.sub_wallet_id.to_string());
        sign_map.insert("symbol", params.symbol.clone());
        sign_map.insert("address_to", params.address_to.clone());
        sign_map.insert("amount", params.amount.clone());

        if let Some(ref memo) = params.memo {
            sign_map.insert("memo", memo.clone());
        }
        if let Some(ref outputs) = params.outputs {
            sign_map.insert("outputs", outputs.clone());
        }

        // Build sign string
        let sign_string = Self::build_sign_string(&sign_map).to_lowercase();

        // Sign the string
        crypto_provider.sign(&sign_string)
    }

    /// Generates signature for Web3 transaction
    ///
    /// # Arguments
    /// * `params` - Transaction parameters
    /// * `crypto_provider` - Crypto provider for signing
    ///
    /// # Returns
    /// Base64 encoded signature
    pub fn generate_web3_sign(
        params: &Web3SignParams,
        crypto_provider: &dyn CryptoProvider,
    ) -> Result<String> {
        // Build sign string from params in sorted order (BTreeMap auto-sorts by key ASCII ascending)
        // Empty values will be filtered out and not participate in signing
        let mut sign_map: BTreeMap<&str, String> = BTreeMap::new();
        sign_map.insert("request_id", params.request_id.clone());
        sign_map.insert("sub_wallet_id", params.sub_wallet_id.to_string());
        sign_map.insert("main_chain_symbol", params.main_chain_symbol.clone());
        sign_map.insert("interactive_contract", params.interactive_contract.clone());
        sign_map.insert("amount", params.amount.clone());
        sign_map.insert("input_data", params.input_data.clone());

        // Build sign string
        let sign_string = Self::build_sign_string(&sign_map).to_lowercase();

        // Sign the string
        crypto_provider.sign(&sign_string)
    }

    /// Builds a sign string from sorted parameters
    ///
    /// - Parameters are sorted by key in ASCII ascending order (via BTreeMap)
    /// - Empty values are filtered out and do not participate in signing
    fn build_sign_string(params: &BTreeMap<&str, String>) -> String {
        params
            .iter()
            .filter(|(_, v)| !v.is_empty()) // Empty values do not participate in signing
            .map(|(k, v)| format!("{}={}", k, v))
            .collect::<Vec<_>>()
            .join("&")
    }
}

/// Parameters for withdrawal signing
#[derive(Debug, Clone)]
pub struct WithdrawSignParams {
    /// Request ID
    pub request_id: String,
    /// Sub-wallet ID
    pub sub_wallet_id: i64,
    /// Coin symbol
    pub symbol: String,
    /// Destination address
    pub address_to: String,
    /// Amount
    pub amount: String,
    /// Memo (optional)
    pub memo: Option<String>,
    /// UTXO outputs (optional)
    pub outputs: Option<String>,
}

/// Parameters for Web3 signing
#[derive(Debug, Clone)]
pub struct Web3SignParams {
    /// Request ID
    pub request_id: String,
    /// Sub-wallet ID
    pub sub_wallet_id: i64,
    /// Main chain symbol
    pub main_chain_symbol: String,
    /// Interactive contract address
    pub interactive_contract: String,
    /// Amount
    pub amount: String,
    /// Input data
    pub input_data: String,
}

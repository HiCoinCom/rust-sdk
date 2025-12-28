//! ChainUp Custody Rust SDK
//!
//! This SDK provides a Rust interface for the ChainUp Custody WaaS and MPC services.
//!
//! # Features
//!
//! - **WaaS (Wallet-as-a-Service)**: User management, account operations, billing, transfers, and coin information
//! - **MPC (Multi-Party Computation)**: Wallet management, deposits, withdrawals, Web3 transactions, and Tron resources
//!
//! # Quick Start
//!
//! ## WaaS Example
//!
//! ```rust,ignore
//! use chainup_sdk::waas::{WaasClient, WaasConfig};
//! use chainup_sdk::crypto::RsaCryptoProvider;
//! use std::sync::Arc;
//!
//! let config = WaasConfig::new(
//!     "https://api.chainup.com",
//!     "your-app-id",
//!     "your-rsa-private-key",
//!     "chainup-rsa-public-key",
//! );
//!
//! let crypto_provider = Arc::new(RsaCryptoProvider::new(
//!     &config.rsa_private_key,
//!     &config.rsa_public_key,
//! )?);
//!
//! let client = WaasClient::builder()
//!     .config(config)
//!     .crypto_provider(crypto_provider)
//!     .build()?;
//!
//! // Get coin list
//! let coins = client.coin().get_coin_list()?;
//! ```
//!
//! ## MPC Example
//!
//! ```rust,ignore
//! use chainup_sdk::mpc::{MpcClient, MpcConfig};
//! use chainup_sdk::crypto::RsaCryptoProvider;
//! use std::sync::Arc;
//!
//! let config = MpcConfig::new(
//!     "https://mpc-api.chainup.com",
//!     "your-app-id",
//!     "your-rsa-private-key",
//!     "chainup-rsa-public-key",
//!     "your-sign-private-key",
//! );
//!
//! let crypto_provider = Arc::new(RsaCryptoProvider::new(
//!     &config.rsa_private_key,
//!     &config.rsa_public_key,
//! )?);
//!
//! let client = MpcClient::builder()
//!     .config(config)
//!     .crypto_provider(crypto_provider)
//!     .build()?;
//!
//! // Get wallet list
//! let wallets = client.wallet().get_wallet_list(
//!     chainup_sdk::mpc::api::types::GetWalletListParams::new(100, 0)
//! )?;
//! ```

#![warn(missing_docs)]
#![warn(rust_2018_idioms)]

pub mod crypto;
pub mod enums;
pub mod error;
pub mod http_client;
pub mod mpc;
pub mod utils;
pub mod waas;

// Re-export Decimal type for convenience
pub use rust_decimal::Decimal;

// Re-export commonly used types at crate root
pub use crypto::{CryptoProvider, RsaCryptoProvider};
pub use enums::*;
pub use error::{ChainUpError, Result};

// Re-export WaaS types
pub use waas::{WaasClient, WaasClientBuilder, WaasConfig};
pub use waas::api::types as waas_types;

// Re-export MPC types
pub use mpc::{MpcClient, MpcClientBuilder, MpcConfig, MpcSignUtil};
pub use mpc::api::types as mpc_types;

/// SDK version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// SDK name
pub const SDK_NAME: &str = "chainup-custody-rust-sdk";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version() {
        assert!(!VERSION.is_empty());
    }

    #[test]
    fn test_sdk_name() {
        assert_eq!(SDK_NAME, "chainup-custody-rust-sdk");
    }
}

//! MPC API modules
//!
//! This module exports all MPC API implementations.

mod auto_sweep_api;
mod base_api;
mod deposit_api;
mod notify_api;
mod tron_resource_api;
mod wallet_api;
mod web3_api;
mod withdraw_api;
mod workspace_api;

pub use auto_sweep_api::AutoSweepApi;
pub use base_api::MpcBaseApi;
pub use deposit_api::DepositApi;
pub use notify_api::NotifyApi;
pub use tron_resource_api::TronResourceApi;
pub use wallet_api::WalletApi;
pub use web3_api::Web3Api;
pub use withdraw_api::WithdrawApi;
pub use workspace_api::WorkspaceApi;

/// Re-exported request and response types for MPC APIs
pub mod types {
    pub use super::auto_sweep_api::{
        AutoCollectRecord, AutoCollectResult, AutoCollectSubWalletsParams,
        SetAutoCollectSymbolParams,
    };
    pub use super::deposit_api::{
        DepositRecord, GetDepositRecordsParams, SyncDepositRecordsResponse,
    };
    pub use super::notify_api::MpcNotifyData;
    pub use super::tron_resource_api::{
        CreateTronDelegateParams, TronDelegateResult, TronResourceRecord,
    };
    pub use super::wallet_api::{
        ChangeWalletShowStatusParams, CreateWalletAddressParams, CreateWalletParams,
        GetWalletAssetsParams, QueryWalletAddressParams, WalletAddressInfo,
        WalletAddressInfoParams, WalletAddressInfoResponse, WalletAssetInfo, WalletInfo,
    };
    pub use super::web3_api::{
        AccelerateWeb3TransParams, CreateWeb3TransParams, SyncWeb3TransRecordsResponse,
        Web3TransRecord,
    };
    pub use super::withdraw_api::{
        SyncWithdrawRecordsResponse, WithdrawParams, WithdrawRecord, WithdrawResponse,
    };
    pub use super::workspace_api::{
        BlockHeightInfo, CoinDetails, GetCoinDetailsParams, GetLastBlockHeightParams,
        GetSupportedCoinsResponse, SupportedCoin,
    };
}

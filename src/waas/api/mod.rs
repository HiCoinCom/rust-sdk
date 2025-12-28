//! WaaS API modules
//!
//! This module exports all WaaS API implementations.

mod account_api;
mod async_notify_api;
mod base_api;
mod billing_api;
mod coin_api;
mod transfer_api;
mod user_api;

pub use account_api::AccountApi;
pub use async_notify_api::AsyncNotifyApi;
pub use base_api::BaseApi;
pub use billing_api::BillingApi;
pub use coin_api::CoinApi;
pub use transfer_api::TransferApi;
pub use user_api::UserApi;

/// Re-exported request and response types for WaaS APIs
pub mod types {
    pub use super::account_api::{
        CompanyAccountInfo, GetCompanyAccountParams, GetUserAccountParams,
        GetUserAddressInfoParams, GetUserAddressParams, SyncUserAddressListResponse,
        UserAccountInfo, UserAddressInfo,
    };
    pub use super::async_notify_api::NotifyData;
    pub use super::billing_api::{
        DepositRecord, MinerFeeRecord, SyncDepositListResponse, SyncMinerFeeListResponse,
        SyncWithdrawListResponse, WithdrawParams, WithdrawRecord, WithdrawResponse,
    };
    pub use super::coin_api::{CoinInfo, GetCoinListResponse};
    pub use super::transfer_api::{
        AccountTransferParams, GetAccountTransferListParams, SyncAccountTransferListResponse,
        TransferRecord,
    };
    pub use super::user_api::{
        GetEmailUserParams, GetMobileUserParams, RegisterEmailUserParams, RegisterMobileUserParams,
        UserInfo,
    };
}

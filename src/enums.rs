//! Enumerations for ChainUp Custody SDK
//!
//! This module defines enums matching the Python SDK's enums.

use serde::{Deserialize, Serialize};

/// API response codes
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum ApiCode {
    /// Success
    Success = 0,

    // System errors
    /// System error
    SystemError = 100001,
    /// Invalid request parameters
    ParamInvalid = 100004,
    /// Signature verification failed
    SignError = 100005,
    /// IP address not allowed
    IpForbidden = 100007,
    /// Invalid merchant ID
    MerchantIdInvalid = 100015,
    /// Merchant information expired
    MerchantExpired = 100016,

    // User-related errors
    /// User is frozen, withdrawal not allowed
    UserFrozen = 110004,
    /// Mobile number already registered
    MobileRegistered = 110023,
    /// Withdrawal address has risk
    WithdrawAddressRisk = 110037,
    /// Invalid withdrawal address
    WithdrawAddressError = 110055,
    /// User does not exist
    UserNotExist = 110065,
    /// Withdrawal or transfer amount below minimum
    AmountBelowMin = 110078,
    /// Withdrawal or transfer amount exceeds maximum
    AmountExceedMax = 110087,
    /// Duplicate request
    DuplicateRequest = 110088,
    /// Invalid mobile number for registration
    MobileInvalid = 110089,
    /// User registration failed
    RegisterFailed = 110101,
    /// Withdrawal precision exceeded maximum supported
    PrecisionExceeded = 110161,

    // Coin/transaction-related errors
    /// Coin not supported
    CoinNotSupported = 120202,
    /// Withdrawal confirmation failed
    ConfirmFailed = 120206,
    /// Insufficient balance for withdrawal or transfer
    BalanceInsufficient = 120402,
    /// Insufficient balance for withdrawal fee
    FeeInsufficient = 120403,
    /// Withdrawal or transfer amount too small
    AmountLessThanFee = 120404,

    // Risk control errors
    /// User has risk, withdrawal forbidden
    UserRiskForbidden = 900006,

    // Transfer-related errors
    /// Cannot transfer to self
    SelfTransferForbidden = 3040006,
}

impl ApiCode {
    /// Check if the code represents success
    pub fn is_success(&self) -> bool {
        matches!(self, ApiCode::Success)
    }

    /// Get the numeric value of the code
    pub fn code(&self) -> i32 {
        *self as i32
    }

    /// Try to convert from an i32 value
    pub fn from_code(code: i32) -> Option<Self> {
        match code {
            0 => Some(ApiCode::Success),
            100001 => Some(ApiCode::SystemError),
            100004 => Some(ApiCode::ParamInvalid),
            100005 => Some(ApiCode::SignError),
            100007 => Some(ApiCode::IpForbidden),
            100015 => Some(ApiCode::MerchantIdInvalid),
            100016 => Some(ApiCode::MerchantExpired),
            110004 => Some(ApiCode::UserFrozen),
            110023 => Some(ApiCode::MobileRegistered),
            110037 => Some(ApiCode::WithdrawAddressRisk),
            110055 => Some(ApiCode::WithdrawAddressError),
            110065 => Some(ApiCode::UserNotExist),
            110078 => Some(ApiCode::AmountBelowMin),
            110087 => Some(ApiCode::AmountExceedMax),
            110088 => Some(ApiCode::DuplicateRequest),
            110089 => Some(ApiCode::MobileInvalid),
            110101 => Some(ApiCode::RegisterFailed),
            110161 => Some(ApiCode::PrecisionExceeded),
            120202 => Some(ApiCode::CoinNotSupported),
            120206 => Some(ApiCode::ConfirmFailed),
            120402 => Some(ApiCode::BalanceInsufficient),
            120403 => Some(ApiCode::FeeInsufficient),
            120404 => Some(ApiCode::AmountLessThanFee),
            900006 => Some(ApiCode::UserRiskForbidden),
            3040006 => Some(ApiCode::SelfTransferForbidden),
            _ => None,
        }
    }
}

/// HTTP methods
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HttpMethod {
    /// GET request
    Get,
    /// POST request
    Post,
    /// PUT request
    Put,
    /// DELETE request
    Delete,
    /// PATCH request
    Patch,
}

/// MPC Deposit status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[repr(i32)]
pub enum MpcDepositStatus {
    /// Confirming
    Confirming = 1900,
    /// Success
    Success = 2000,
    /// Failed
    Failed = 2400,
}

impl MpcDepositStatus {
    /// Try to convert from an i32 value
    pub fn from_code(code: i32) -> Option<Self> {
        match code {
            1900 => Some(MpcDepositStatus::Confirming),
            2000 => Some(MpcDepositStatus::Success),
            2400 => Some(MpcDepositStatus::Failed),
            _ => None,
        }
    }
}

/// MPC Withdraw status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[repr(i32)]
pub enum MpcWithdrawStatus {
    /// Pending audit
    PendingAudit = 1000,
    /// Audit passed
    AuditPassed = 1100,
    /// Processing
    Processing = 1200,
    /// Cancelled
    Cancelled = 2200,
    /// Audit rejected
    AuditRejected = 2300,
    /// Success
    Success = 2000,
    /// Failed
    Failed = 2400,
}

impl MpcWithdrawStatus {
    /// Try to convert from an i32 value
    pub fn from_code(code: i32) -> Option<Self> {
        match code {
            1000 => Some(MpcWithdrawStatus::PendingAudit),
            1100 => Some(MpcWithdrawStatus::AuditPassed),
            1200 => Some(MpcWithdrawStatus::Processing),
            2000 => Some(MpcWithdrawStatus::Success),
            2200 => Some(MpcWithdrawStatus::Cancelled),
            2300 => Some(MpcWithdrawStatus::AuditRejected),
            2400 => Some(MpcWithdrawStatus::Failed),
            _ => None,
        }
    }
}

/// MPC Web3 transaction type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[repr(i32)]
pub enum MpcWeb3TransType {
    /// Approve
    Approve = 0,
    /// Transaction
    Transaction = 1,
    /// TRON permission approve
    TronPermissionApprove = 22,
    /// TRON approved transfer
    TronApprovedTransfer = 23,
}

/// TRON resource type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[repr(i32)]
pub enum TronResourceType {
    /// Bandwidth and Energy
    BandwidthAndEnergy = 0,
    /// Energy only
    Energy = 1,
}

/// TRON resource service duration types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TronServiceType {
    /// 10 minutes
    TenMin,
    /// 1 hour
    OneHour,
    /// 1 day
    OneDay,
}

impl TronServiceType {
    /// Get the string representation
    pub fn as_str(&self) -> &'static str {
        match self {
            TronServiceType::TenMin => "10010",
            TronServiceType::OneHour => "20001",
            TronServiceType::OneDay => "30001",
        }
    }
}

/// TRON resource buy types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[repr(i32)]
pub enum TronBuyType {
    /// System
    System = 0,
    /// Manual
    Manual = 1,
}

/// Wallet display status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[repr(i32)]
pub enum WalletShowStatus {
    /// Visible
    Visible = 1,
    /// Hidden
    Hidden = 2,
}

/// Auto-collection status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[repr(i32)]
pub enum AutoCollectStatus {
    /// Disabled
    Disabled = 0,
    /// Enabled
    Enabled = 1,
}

/// Query ID type for records
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum QueryIdType {
    /// Query by request ID
    RequestId,
    /// Query by receipt
    Receipt,
    /// Query by WaaS ID
    WaasId,
}

impl QueryIdType {
    /// Get the string representation
    pub fn as_str(&self) -> &'static str {
        match self {
            QueryIdType::RequestId => "request_id",
            QueryIdType::Receipt => "receipt",
            QueryIdType::WaasId => "id",
        }
    }
}

/// Cryptocurrency type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CoinType {
    /// Main chain coin
    MainCoin,
    /// Token
    Token,
}

impl CoinType {
    /// Get the string representation
    pub fn as_str(&self) -> &'static str {
        match self {
            CoinType::MainCoin => "main",
            CoinType::Token => "token",
        }
    }
}

/// Blockchain network type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NetworkType {
    /// Mainnet
    Mainnet,
    /// Testnet
    Testnet,
}

impl NetworkType {
    /// Get the string representation
    pub fn as_str(&self) -> &'static str {
        match self {
            NetworkType::Mainnet => "mainnet",
            NetworkType::Testnet => "testnet",
        }
    }
}

/// HTTP Content-Type headers
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ContentType {
    /// Form URL encoded
    Form,
    /// JSON
    Json,
    /// Multipart form data
    Multipart,
}

impl ContentType {
    /// Get the string representation
    pub fn as_str(&self) -> &'static str {
        match self {
            ContentType::Form => "application/x-www-form-urlencoded",
            ContentType::Json => "application/json",
            ContentType::Multipart => "multipart/form-data",
        }
    }
}

/// Character encoding
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Charset {
    /// UTF-8
    Utf8,
    /// ASCII
    Ascii,
    /// Latin-1 (ISO-8859-1)
    Latin1,
}

impl Charset {
    /// Get the string representation
    pub fn as_str(&self) -> &'static str {
        match self {
            Charset::Utf8 => "UTF-8",
            Charset::Ascii => "ASCII",
            Charset::Latin1 => "ISO-8859-1",
        }
    }
}

/// Transaction side (deposit or withdraw)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TransactionSide {
    /// Deposit transaction
    Deposit,
    /// Withdrawal transaction
    Withdraw,
}

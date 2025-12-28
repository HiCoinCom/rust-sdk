//! Error types for ChainUp Custody SDK
//!
//! This module defines the error hierarchy for the SDK,
//! matching the Python SDK's exception structure.

use std::fmt;
use thiserror::Error;

/// Base error type for all ChainUp SDK errors
#[derive(Error, Debug)]
pub enum ChainUpError {
    /// API request errors
    #[error("{0}")]
    Api(ApiError),

    /// Configuration errors
    #[error("Configuration error: {0}")]
    Config(ConfigError),

    /// Encryption/decryption errors
    #[error("Crypto error: {0}")]
    Crypto(CryptoError),

    /// Network connectivity errors
    #[error("Network error: {0}")]
    Network(NetworkError),

    /// Input validation errors
    #[error("Validation error: {0}")]
    Validation(ValidationError),

    /// Signature verification errors
    #[error("Signature error: {0}")]
    Signature(SignatureError),

    /// Authentication errors
    #[error("Authentication error: {0}")]
    Authentication(AuthenticationError),

    /// Rate limiting errors
    #[error("Rate limit error: {0}")]
    RateLimit(RateLimitError),
}

/// API request error with code and message
#[derive(Debug, Clone)]
pub struct ApiError {
    /// Error code from API response
    pub code: i32,
    /// Error message from API response
    pub message: String,
    /// Optional additional data
    pub data: Option<serde_json::Value>,
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "API error [{}]: {}", self.code, self.message)
    }
}

impl std::error::Error for ApiError {}

impl ApiError {
    /// Creates a new API error
    pub fn new(code: i32, message: impl Into<String>) -> Self {
        Self {
            code,
            message: message.into(),
            data: None,
        }
    }

    /// Creates a new API error with data
    pub fn with_data(code: i32, message: impl Into<String>, data: serde_json::Value) -> Self {
        Self {
            code,
            message: message.into(),
            data: Some(data),
        }
    }
}

/// Configuration error
#[derive(Error, Debug, Clone)]
#[error("{message}")]
pub struct ConfigError {
    /// Error message
    pub message: String,
}

impl ConfigError {
    /// Creates a new configuration error
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

/// Crypto error for encryption/decryption failures
#[derive(Error, Debug, Clone)]
#[error("{message}")]
pub struct CryptoError {
    /// Error message
    pub message: String,
}

impl CryptoError {
    /// Creates a new crypto error
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

/// Network connectivity error
#[derive(Error, Debug)]
#[error("{message}")]
pub struct NetworkError {
    /// Error message
    pub message: String,
    /// Underlying source error
    #[source]
    pub source: Option<Box<dyn std::error::Error + Send + Sync>>,
}

impl NetworkError {
    /// Creates a new network error
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            source: None,
        }
    }

    /// Creates a new network error with a source error
    pub fn with_source(
        message: impl Into<String>,
        source: impl std::error::Error + Send + Sync + 'static,
    ) -> Self {
        Self {
            message: message.into(),
            source: Some(Box::new(source)),
        }
    }
}

/// Input validation error
#[derive(Error, Debug, Clone)]
#[error("{message}")]
pub struct ValidationError {
    /// Error message
    pub message: String,
}

impl ValidationError {
    /// Creates a new validation error
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

/// Signature verification error
#[derive(Error, Debug, Clone)]
#[error("{message}")]
pub struct SignatureError {
    /// Error message
    pub message: String,
}

impl SignatureError {
    /// Creates a new signature error
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

/// Authentication error
#[derive(Error, Debug, Clone)]
#[error("{message}")]
pub struct AuthenticationError {
    /// Error message
    pub message: String,
}

impl AuthenticationError {
    /// Creates a new authentication error
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

/// Rate limit error
#[derive(Error, Debug, Clone)]
#[error("{message}")]
pub struct RateLimitError {
    /// Error message
    pub message: String,
}

impl RateLimitError {
    /// Creates a new rate limit error
    pub fn new(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
        }
    }
}

/// Result type alias for ChainUp SDK operations
pub type Result<T> = std::result::Result<T, ChainUpError>;

// Convenient From implementations for error conversion

impl From<ApiError> for ChainUpError {
    fn from(err: ApiError) -> Self {
        ChainUpError::Api(err)
    }
}

impl From<ConfigError> for ChainUpError {
    fn from(err: ConfigError) -> Self {
        ChainUpError::Config(err)
    }
}

impl From<CryptoError> for ChainUpError {
    fn from(err: CryptoError) -> Self {
        ChainUpError::Crypto(err)
    }
}

impl From<NetworkError> for ChainUpError {
    fn from(err: NetworkError) -> Self {
        ChainUpError::Network(err)
    }
}

impl From<ValidationError> for ChainUpError {
    fn from(err: ValidationError) -> Self {
        ChainUpError::Validation(err)
    }
}

impl From<SignatureError> for ChainUpError {
    fn from(err: SignatureError) -> Self {
        ChainUpError::Signature(err)
    }
}

impl From<AuthenticationError> for ChainUpError {
    fn from(err: AuthenticationError) -> Self {
        ChainUpError::Authentication(err)
    }
}

impl From<RateLimitError> for ChainUpError {
    fn from(err: RateLimitError) -> Self {
        ChainUpError::RateLimit(err)
    }
}

impl From<reqwest::Error> for ChainUpError {
    fn from(err: reqwest::Error) -> Self {
        ChainUpError::Network(NetworkError::with_source("HTTP request failed", err))
    }
}

impl From<serde_json::Error> for ChainUpError {
    fn from(err: serde_json::Error) -> Self {
        ChainUpError::Validation(ValidationError::new(format!("JSON parse error: {}", err)))
    }
}

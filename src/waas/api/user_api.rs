//! User API - User management and registration operations
//!
//! Provides methods for user registration, information retrieval, and user list queries.

use std::collections::HashMap;
use std::sync::Arc;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::crypto::CryptoProvider;
use crate::error::Result;
use crate::waas::api::base_api::BaseApi;
use crate::waas::config::WaasConfig;

// ============================================================================
// Request parameter types
// ============================================================================

/// Parameters for registering a user with mobile phone
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegisterMobileUserParams {
    /// Country code (e.g., "86")
    pub country: String,
    /// Mobile phone number
    pub mobile: String,
}

impl RegisterMobileUserParams {
    /// Creates new parameters
    pub fn new(country: impl Into<String>, mobile: impl Into<String>) -> Self {
        Self {
            country: country.into(),
            mobile: mobile.into(),
        }
    }

    fn to_map(&self) -> HashMap<String, Value> {
        let mut map = HashMap::new();
        map.insert("country".to_string(), Value::String(self.country.clone()));
        map.insert("mobile".to_string(), Value::String(self.mobile.clone()));
        map
    }
}

/// Parameters for registering a user with email
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegisterEmailUserParams {
    /// Email address
    pub email: String,
}

impl RegisterEmailUserParams {
    /// Creates new parameters
    pub fn new(email: impl Into<String>) -> Self {
        Self {
            email: email.into(),
        }
    }

    fn to_map(&self) -> HashMap<String, Value> {
        let mut map = HashMap::new();
        map.insert("email".to_string(), Value::String(self.email.clone()));
        map
    }
}

/// Parameters for getting user by mobile phone
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetMobileUserParams {
    /// Country code (e.g., "86")
    pub country: String,
    /// Mobile phone number
    pub mobile: String,
}

impl GetMobileUserParams {
    /// Creates new parameters
    pub fn new(country: impl Into<String>, mobile: impl Into<String>) -> Self {
        Self {
            country: country.into(),
            mobile: mobile.into(),
        }
    }

    fn to_map(&self) -> HashMap<String, Value> {
        let mut map = HashMap::new();
        map.insert("country".to_string(), Value::String(self.country.clone()));
        map.insert("mobile".to_string(), Value::String(self.mobile.clone()));
        map
    }
}

/// Parameters for getting user by email
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetEmailUserParams {
    /// Email address
    pub email: String,
}

impl GetEmailUserParams {
    /// Creates new parameters
    pub fn new(email: impl Into<String>) -> Self {
        Self {
            email: email.into(),
        }
    }

    fn to_map(&self) -> HashMap<String, Value> {
        let mut map = HashMap::new();
        map.insert("email".to_string(), Value::String(self.email.clone()));
        map
    }
}

// ============================================================================
// Response types
// ============================================================================

/// User information response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserInfo {
    /// User ID
    pub uid: i64,
    /// Authentication level
    #[serde(default)]
    pub auth_level: Option<i32>,
    /// User nickname
    #[serde(default)]
    pub nickname: Option<String>,
    /// Real name
    #[serde(default)]
    pub real_name: Option<String>,
    /// Invite code
    #[serde(default)]
    pub invite_code: Option<String>,
    /// Country code (for mobile users, in some APIs)
    #[serde(default)]
    pub country: Option<String>,
    /// Mobile number (for mobile users, in some APIs)
    #[serde(default)]
    pub mobile: Option<String>,
    /// Email (for email users, in some APIs)
    #[serde(default)]
    pub email: Option<String>,
}

// ============================================================================
// User API Implementation
// ============================================================================

/// User API - User management and registration operations
///
/// Provides methods for user registration, information retrieval, and coin list queries.
pub struct UserApi {
    base: BaseApi,
}

impl UserApi {
    /// Creates a new UserApi instance
    pub fn new(config: WaasConfig, crypto_provider: Arc<dyn CryptoProvider>) -> Self {
        Self {
            base: BaseApi::new(config, crypto_provider),
        }
    }

    /// Registers a new user using mobile phone
    ///
    /// # Arguments
    /// * `params` - Registration parameters
    ///
    /// # Returns
    /// User registration result containing uid
    ///
    /// # Example
    /// ```ignore
    /// let result = user_api.register_mobile_user(RegisterMobileUserParams::new("86", "13800000000"))?;
    /// ```
    pub fn register_mobile_user(&self, params: RegisterMobileUserParams) -> Result<UserInfo> {
        let data = params.to_map();
        let response = self.base.post("/user/createUser", Some(&data))?;
        self.base.validate_response(response)
    }

    /// Registers a new user using email
    ///
    /// # Arguments
    /// * `params` - Registration parameters
    ///
    /// # Returns
    /// User registration result containing uid
    ///
    /// # Example
    /// ```ignore
    /// let result = user_api.register_email_user(RegisterEmailUserParams::new("user@example.com"))?;
    /// ```
    pub fn register_email_user(&self, params: RegisterEmailUserParams) -> Result<UserInfo> {
        let data = params.to_map();
        let response = self.base.post("/user/registerEmail", Some(&data))?;
        self.base.validate_response(response)
    }

    /// Gets user information by mobile phone
    ///
    /// # Arguments
    /// * `params` - Query parameters
    ///
    /// # Returns
    /// User information
    ///
    /// # Example
    /// ```ignore
    /// let user_info = user_api.get_mobile_user(GetMobileUserParams::new("86", "13800000000"))?;
    /// ```
    pub fn get_mobile_user(&self, params: GetMobileUserParams) -> Result<UserInfo> {
        let data = params.to_map();
        let response = self.base.post("/user/info", Some(&data))?;
        self.base.validate_response(response)
    }

    /// Gets user information by email
    ///
    /// # Arguments
    /// * `params` - Query parameters
    ///
    /// # Returns
    /// User information
    ///
    /// # Example
    /// ```ignore
    /// let user_info = user_api.get_email_user(GetEmailUserParams::new("user@example.com"))?;
    /// ```
    pub fn get_email_user(&self, params: GetEmailUserParams) -> Result<UserInfo> {
        let data = params.to_map();
        let response = self.base.post("/user/info", Some(&data))?;
        self.base.validate_response(response)
    }

    /// Syncs user list by max ID (pagination)
    ///
    /// # Arguments
    /// * `max_id` - Maximum user ID for pagination (0 for first sync)
    ///
    /// # Returns
    /// Synced user list
    ///
    /// # Example
    /// ```ignore
    /// let users = user_api.sync_user_list(0)?;
    /// ```
    pub fn sync_user_list(&self, max_id: i64) -> Result<Vec<UserInfo>> {
        let mut data = HashMap::new();
        data.insert("max_id".to_string(), Value::Number(max_id.into()));

        let response = self.base.post("/user/syncList", Some(&data))?;
        self.base.validate_response(response)
    }
}

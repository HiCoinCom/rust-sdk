//! Utility modules for the SDK

/// Serde deserializers for API response handling
pub mod serde_helpers {
    use serde::{Deserialize, Deserializer};

    /// Deserialize a string or number as Option<i32>
    pub fn deserialize_optional_i32<'de, D>(deserializer: D) -> Result<Option<i32>, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(untagged)]
        enum StringOrInt {
            String(String),
            Int(i32),
        }

        Option::<StringOrInt>::deserialize(deserializer).map(|opt| {
            opt.and_then(|v| match v {
                StringOrInt::String(s) => s.parse().ok(),
                StringOrInt::Int(i) => Some(i),
            })
        })
    }

    /// Deserialize a string or number as Option<i64>
    pub fn deserialize_optional_i64<'de, D>(deserializer: D) -> Result<Option<i64>, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(untagged)]
        enum StringOrInt {
            String(String),
            Int(i64),
        }

        Option::<StringOrInt>::deserialize(deserializer).map(|opt| {
            opt.and_then(|v| match v {
                StringOrInt::String(s) => s.parse().ok(),
                StringOrInt::Int(i) => Some(i),
            })
        })
    }

    /// Deserialize a string or bool as Option<bool>
    /// Handles: true, false, 0, 1, "0", "1", "true", "false"
    pub fn deserialize_optional_bool<'de, D>(deserializer: D) -> Result<Option<bool>, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(untagged)]
        enum StringOrBool {
            String(String),
            Bool(bool),
            Int(i32),
        }

        Option::<StringOrBool>::deserialize(deserializer).map(|opt| {
            opt.map(|v| match v {
                StringOrBool::String(s) => matches!(s.as_str(), "1" | "true" | "True" | "TRUE"),
                StringOrBool::Bool(b) => b,
                StringOrBool::Int(i) => i != 0,
            })
        })
    }

    /// Deserialize string "0"/"1" or integer 0/1 as Option<bool>
    pub fn deserialize_int_bool<'de, D>(deserializer: D) -> Result<Option<bool>, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserialize_optional_bool(deserializer)
    }
}

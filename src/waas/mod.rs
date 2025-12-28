//! WaaS (Wallet-as-a-Service) module
//!
//! This module provides the WaaS API client and related types.

pub mod api;
mod client;
mod config;

pub use client::{WaasClient, WaasClientBuilder};
pub use config::WaasConfig;

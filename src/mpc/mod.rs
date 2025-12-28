//! MPC (Multi-Party Computation) module
//!
//! This module provides the MPC API client and related types.

pub mod api;
mod client;
mod config;
mod sign_util;

pub use client::{MpcClient, MpcClientBuilder};
pub use config::MpcConfig;
pub use sign_util::MpcSignUtil;

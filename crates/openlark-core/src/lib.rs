//! OpenLark Core Infrastructure
//!
//! This crate provides the core infrastructure for the OpenLark SDK including
//! HTTP client configuration, error handling, authentication, and common utilities.

#![allow(missing_docs)]

// New modular structure
pub mod auth;
pub mod validation;

// Legacy modules (to be refactored)
pub mod api;
pub mod app_ticket_manager;
pub mod cache;
pub mod config;
pub mod constants;
pub mod error;
pub mod http;
pub mod improved_response_handler;
pub mod observability;
pub mod performance;
pub mod query_params;
pub mod req_option;
pub mod req_translator;
pub mod request_builder;
pub mod request_executor;
pub mod retry_middleware;
pub mod standard_response;
pub mod test_utils;
pub mod token_manager;
pub mod trait_system;
pub mod utils;

// Business modules (should be moved to separate crates)
pub mod contact;
pub mod endpoints;

// Re-export commonly used types from crate root
pub use error::{CoreError, SDKResult};

/// Prelude module for convenient imports.
pub mod prelude {
    // Re-export new API module
    pub use crate::api::prelude::*;

    // Re-export commonly used core modules directly
    pub use crate::constants::*;
    pub use crate::error::*;
    pub use crate::http::*;
    pub use crate::req_option::*;
    pub use crate::standard_response::*;

    // Re-export commonly used dependencies
    pub use anyhow::Result;

    /// Result type alias for convenience（已默认使用 CoreError）
    pub type SDKResult<T> = Result<T, LarkAPIError>;
    pub use serde::{Deserialize, Serialize};
    pub use serde_json::Value;
    pub use std::collections::HashMap;
}

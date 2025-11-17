//! OpenLark Core Infrastructure
//!
//! This crate provides the core infrastructure for the OpenLark SDK including
//! HTTP client configuration, error handling, authentication, and common utilities.

#![allow(missing_docs)]

// Core modules
pub mod api_req;
pub mod api_resp;
pub mod app_ticket_manager;
pub mod cache;
pub mod config;
pub mod constants;
pub mod error;
pub mod http;
pub mod improved_response_handler;
pub mod migration_guide;
pub mod observability;
pub mod query_params;
pub mod req_option;
pub mod req_translator;
pub mod request_executor;
pub mod request_executor_example;
pub mod retry_middleware;
pub mod standard_response;
pub mod test_utils;
pub mod token_manager;
pub mod utils;
pub mod validation;

// Directory modules
pub mod client;
pub mod contact;
pub mod endpoints;
pub mod event;
pub mod pagination;
pub mod performance;
pub mod request_builder;
pub mod trait_system;

// Re-export commonly used types from crate root
pub use error::SDKResult;

/// Prelude module for convenient imports.
pub mod prelude {
    // Re-export commonly used core modules directly
    pub use crate::api_req::*;
    pub use crate::api_resp::*;
    pub use crate::constants::*;
    pub use crate::error::*;
    pub use crate::http::*;
    pub use crate::req_option::*;
    pub use crate::standard_response::*;

    // Re-export commonly used dependencies
    pub use anyhow::Result;

    /// Result type alias for convenience
    pub type SDKResult<T> = Result<T, LarkAPIError>;
    pub use serde::{Deserialize, Serialize};
    pub use serde_json::Value;
    pub use std::collections::HashMap;
}

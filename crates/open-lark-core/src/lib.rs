//! OpenLark Core Infrastructure
//!
//! This crate provides the core infrastructure for the OpenLark SDK including
//! HTTP client configuration, error handling, authentication, and common utilities.

#![allow(missing_docs)]

pub mod client;
pub mod core;
pub mod event;

/// Prelude module for convenient imports.
pub mod prelude {
    // Re-export core functionality
    pub use crate::core::*;

    // Re-export commonly used dependencies
    pub use anyhow::Result;
    pub use serde::{Deserialize, Serialize};
    pub use serde_json::Value;
    pub use std::collections::HashMap;
}

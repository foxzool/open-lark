//! Bitable API v1 module (placeholder)
//!
//! This is a placeholder implementation for the v1 bitable API.

use openlark_core::config::Config;

/// Bitable API v1 service
pub struct V1 {
    config: Config,
}

impl V1 {
    /// Create new v1 bitable service
    pub fn new(config: Config) -> Self {
        Self { config }
    }
}
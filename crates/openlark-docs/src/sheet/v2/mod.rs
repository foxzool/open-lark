//! Sheet API v2 module (placeholder)
//!
//! This is a placeholder implementation for the v2 sheet API.

use openlark_core::config::Config;

/// Sheet API v2 service
pub struct V2 {
    config: Config,
}

impl V2 {
    /// Create new v2 sheet service
    pub fn new(config: Config) -> Self {
        Self { config }
    }
}
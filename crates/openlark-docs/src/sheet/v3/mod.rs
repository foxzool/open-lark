//! Sheet API v3 module (placeholder)
//!
//! This is a placeholder implementation for the v3 sheet API.

use openlark_core::config::Config;

/// Sheet API v3 service
pub struct V3 {
    config: Config,
}

impl V3 {
    /// Create new v3 sheet service
    pub fn new(config: Config) -> Self {
        Self { config }
    }
}

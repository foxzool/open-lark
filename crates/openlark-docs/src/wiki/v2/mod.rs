//! Wiki API v2 module (placeholder)
//!
//! This is a placeholder implementation for the v2 wiki API.

use openlark_core::config::Config;

/// Wiki API v2 service
pub struct V2 {
    config: Config,
}

impl V2 {
    /// Create new v2 wiki service
    pub fn new(config: Config) -> Self {
        Self { config }
    }
}
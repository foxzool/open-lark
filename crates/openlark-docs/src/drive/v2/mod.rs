//! Drive API v2 module (placeholder)
//!
//! This is a placeholder implementation for the v2 drive API.

use openlark_core::config::Config;

/// Drive API v2 service
pub struct V2 {
    config: Config,
}

impl V2 {
    /// Create new v2 drive service
    pub fn new(config: Config) -> Self {
        Self { config }
    }
}
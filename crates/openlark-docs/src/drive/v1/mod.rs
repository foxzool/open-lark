//! Drive API v1 module (placeholder)
//!
//! This is a placeholder implementation for the v1 drive API.

use openlark_core::config::Config;

/// Drive API v1 service
pub struct V1 {
    #[allow(dead_code)]
    config: Config,
}

impl V1 {
    /// Create new v1 drive service
    pub fn new(config: Config) -> Self {
        Self { config }
    }
}

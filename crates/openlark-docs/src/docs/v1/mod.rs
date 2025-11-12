//! Document API v1 module (placeholder)
//!
//! This is a placeholder implementation for the v1 document API.

use openlark_core::config::Config;

/// Document API v1 service
pub struct V1 {
    config: Config,
}

impl V1 {
    /// Create new v1 document service
    pub fn new(config: Config) -> Self {
        Self { config }
    }
}
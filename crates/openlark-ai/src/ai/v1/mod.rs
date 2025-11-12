//! AI API v1 module (placeholder)
//!
//! This is a placeholder implementation for the v1 AI API.

use openlark_core::config::Config;

/// AI API v1 service
pub struct V1 {
    #[allow(dead_code)]
    config: Config,
}

impl V1 {
    /// Create new v1 AI service
    pub fn new(config: Config) -> Self {
        Self { config }
    }
}

//! AI service module
//!
//! This module provides AI and machine learning functionality.

use openlark_core::{config::Config, trait_system::Service};
use std::sync::Arc;

pub mod v1;

/// AI service
pub struct AiService {
    config: Config,
    #[allow(dead_code)]
    config_arc: Arc<Config>,
    /// AI API v1 service
    pub v1: v1::V1,
}

impl AiService {
    /// Create new AI service instance
    pub fn new(config: Config) -> Self {
        Self {
            config: config.clone(),
            config_arc: Arc::new(config.clone()),
            v1: v1::V1::new(config),
        }
    }

    /// Create service with shared config (experimental)
    pub fn new_from_shared(shared: Arc<Config>) -> Self {
        Self {
            config: shared.as_ref().clone(),
            config_arc: shared.clone(),
            v1: v1::V1::new(shared.as_ref().clone()),
        }
    }
}

impl Service for AiService {
    fn config(&self) -> &Config {
        &self.config
    }

    fn service_name() -> &'static str {
        "ai"
    }

    fn service_version() -> &'static str {
        "v1"
    }
}

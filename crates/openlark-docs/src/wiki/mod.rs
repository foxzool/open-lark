//! Wiki service module
//!
//! This module provides knowledge base and wiki management functionality.

use openlark_core::{config::Config, trait_system::Service};
use std::sync::Arc;

pub mod v2;

/// Wiki service
pub struct WikiService {
    config: Config,
    #[allow(dead_code)]
    config_arc: Arc<Config>,
    pub v2: v2::V2,
}

impl WikiService {
    /// Create new wiki service instance
    pub fn new(config: Config) -> Self {
        Self {
            config: config.clone(),
            config_arc: Arc::new(config.clone()),
            v2: v2::V2::new(config),
        }
    }

    /// Create service with shared config (experimental)
    pub fn new_from_shared(shared: Arc<Config>) -> Self {
        Self {
            config: shared.as_ref().clone(),
            config_arc: shared.clone(),
            v2: v2::V2::new(shared.as_ref().clone()),
        }
    }
}

impl Service for WikiService {
    fn config(&self) -> &Config {
        &self.config
    }

    fn service_name() -> &'static str {
        "wiki"
    }

    fn service_version() -> &'static str {
        "v2"
    }
}
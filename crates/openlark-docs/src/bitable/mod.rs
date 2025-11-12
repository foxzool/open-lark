//! Bitable service module
//!
//! This module provides multi-dimensional table (bitable) functionality.

use openlark_core::{config::Config, trait_system::Service};
use std::sync::Arc;

pub mod v1;

/// Bitable service
pub struct BitableService {
    config: Config,
    #[allow(dead_code)]
    config_arc: Arc<Config>,
    pub v1: v1::V1,
}

impl BitableService {
    /// Create new bitable service instance
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

impl Service for BitableService {
    fn config(&self) -> &Config {
        &self.config
    }

    fn service_name() -> &'static str {
        "bitable"
    }

    fn service_version() -> &'static str {
        "v1"
    }
}
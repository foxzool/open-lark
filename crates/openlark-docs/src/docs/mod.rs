//! Document service module
//!
//! This module provides document processing and management functionality.

use openlark_core::{config::Config, trait_system::Service};
use std::sync::Arc;

pub mod v1;

/// Document service
pub struct DocxService {
    config: Config,
    #[allow(dead_code)]
    config_arc: Arc<Config>,
    pub v1: v1::V1,
}

impl DocxService {
    /// Create new document service instance
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

impl Service for DocxService {
    fn config(&self) -> &Config {
        &self.config
    }

    fn service_name() -> &'static str {
        "docx"
    }

    fn service_version() -> &'static str {
        "v1"
    }
}

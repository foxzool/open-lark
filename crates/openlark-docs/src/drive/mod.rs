//! Drive service module
//!
//! This module provides file and folder storage management functionality.

use openlark_core::{config::Config, trait_system::Service};
use std::sync::Arc;

/// Drive API v1 version
pub mod v1;
/// Drive API v2 version
pub mod v2;

/// Drive service
pub struct DriveService {
    config: Config,
    #[allow(dead_code)]
    config_arc: Arc<Config>,
    pub v1: v1::V1,
    pub v2: v2::V2,
}

impl DriveService {
    /// Create new drive service instance
    pub fn new(config: Config) -> Self {
        Self {
            config: config.clone(),
            config_arc: Arc::new(config.clone()),
            v1: v1::V1::new(config.clone()),
            v2: v2::V2::new(config),
        }
    }

    /// Create service with shared config (experimental)
    pub fn new_from_shared(shared: Arc<Config>) -> Self {
        Self {
            config: shared.as_ref().clone(),
            config_arc: shared.clone(),
            v1: v1::V1::new(shared.as_ref().clone()),
            v2: v2::V2::new(shared.as_ref().clone()),
        }
    }
}

impl Service for DriveService {
    fn config(&self) -> &Config {
        &self.config
    }

    fn service_name() -> &'static str {
        "drive"
    }

    fn service_version() -> &'static str {
        "v2"
    }
}

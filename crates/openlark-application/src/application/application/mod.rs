//! Application API 模块

pub mod v1;

use openlark_core::config::Config;
use std::sync::Arc;

/// Application API 入口
#[derive(Clone)]
pub struct Application {
    config: Arc<Config>,
}

impl Application {
    pub fn new(config: Arc<Config>) -> Self {
        Self { config }
    }

    pub fn v1(&self) -> v1::ApplicationV1 {
        v1::ApplicationV1::new(self.config.clone())
    }
}

//! Search API 模块

pub mod v2;

use openlark_core::config::Config;
use std::sync::Arc;

/// Search API 入口
#[derive(Clone)]
pub struct Search {
    config: Arc<Config>,
}

impl Search {
    pub fn new(config: Arc<Config>) -> Self {
        Self { config }
    }

    pub fn v2(&self) -> v2::SearchV2 {
        v2::SearchV2::new(self.config.clone())
    }
}

//! Helpdesk API 模块

pub mod v1;

use openlark_core::config::Config;
use std::sync::Arc;

/// Helpdesk API 入口
#[derive(Clone)]
pub struct Helpdesk {
    config: Arc<Config>,
}

impl Helpdesk {
    pub fn new(config: Arc<Config>) -> Self {
        Self { config }
    }

    /// 访问 v1 版本 API
    pub fn v1(&self) -> v1::HelpdeskV1 {
        v1::HelpdeskV1::new(self.config.clone())
    }
}

//! Mail API 模块

pub mod v1;

use openlark_core::config::Config;
use std::sync::Arc;

/// Mail API 入口
#[derive(Clone)]
pub struct Mail {
    config: Arc<Config>,
}

impl Mail {
    pub fn new(config: Arc<Config>) -> Self {
        Self { config }
    }

    /// 访问 v1 版本 API
    pub fn v1(&self) -> v1::MailV1 {
        v1::MailV1::new(self.config.clone())
    }
}

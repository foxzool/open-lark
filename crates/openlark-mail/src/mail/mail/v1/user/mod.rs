//! 用户邮箱模块

use openlark_core::config::Config;
use std::sync::Arc;

/// 用户邮箱服务
#[derive(Clone)]
pub struct User {
    config: Arc<Config>,
}

impl User {
    pub fn new(config: Arc<Config>) -> Self {
        Self { config }
    }
}

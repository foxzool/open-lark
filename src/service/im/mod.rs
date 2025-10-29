//! 即时消息服务
//!
//! 提供IM相关功能，包括消息发送、接收、群组管理等。

use std::sync::Arc;
use crate::{
    core::{config::Config, trait_system::Service},
};

/// IM API v1版本
pub mod v1;
/// IM API v2版本
pub mod v2;

/// 即时消息服务
#[derive(Debug)]
pub struct ImService {
    _config: Config,
}

impl ImService {
    /// 创建IM服务实例
    pub fn new(config: Config) -> Self {
        Self { _config: config }
    }

    /// 从共享配置创建服务实例
    pub fn new_from_shared(config: Arc<Config>) -> Self {
        Self::new((*config).clone())
    }
}

impl Service for ImService {
    fn config(&self) -> &Config {
        &self._config
    }

    fn service_name() -> &'static str {
        "im"
    }

    fn service_version() -> &'static str {
        "v1"
    }
}
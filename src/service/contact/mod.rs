//! 通讯录服务
//!
//! 提供企业通讯录管理功能，包括用户、部门、群组等管理。

use std::sync::Arc;
use crate::{
    core::{config::Config, trait_system::Service},
};

/// 通讯录 API v3版本
pub mod v3;

/// 通讯录服务
#[derive(Debug)]
pub struct ContactService {
    _config: Config,
}

impl ContactService {
    /// 创建通讯录服务实例
    pub fn new(config: Config) -> Self {
        Self { _config: config }
    }

    /// 从共享配置创建服务实例
    pub fn new_from_shared(config: Arc<Config>) -> Self {
        Self::new((*config).clone())
    }
}

impl Service for ContactService {
    fn config(&self) -> &Config {
        &self._config
    }

    fn service_name() -> &'static str {
        "contact"
    }

    fn service_version() -> &'static str {
        "v3"
    }
}
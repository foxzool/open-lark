//! # Base v2 版本API
//!
//! 基础服务v2版本的API实现。

use openlark_core::config::Config;

/// Base v2版本服务
#[derive(Debug, Clone)]
pub struct BaseV2Service {
    config: Config,
}

impl BaseV2Service {
    /// 创建新的v2服务实例
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 获取配置信息
    pub fn config(&self) -> &Config {
        &self.config
    }
}

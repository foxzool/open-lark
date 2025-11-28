//! # Base 基础服务
//!
//! 提供文件存储、基础操作等功能。

use openlark_core::config::Config;

/// 基础服务
#[derive(Debug, Clone)]
pub struct BaseService {
    config: Config,
}

impl BaseService {
    /// 创建新的基础服务实例
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 获取配置信息
    pub fn config(&self) -> &Config {
        &self.config
    }

    /// 获取v2版本的API服务
    pub fn v2(&self) -> v2::BaseV2Service {
        v2::BaseV2Service::new(self.config.clone())
    }
}

// 版本模块
pub mod v2;

// 重新导出常用类型
pub use v2::BaseV2Service;

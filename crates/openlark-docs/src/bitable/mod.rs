//! 多维表格服务模块
//!
//! 提供多维表格应用、数据表、视图管理等功能。

use openlark_core::config::Config;

/// 多维表格服务
#[derive(Debug, Clone)]
pub struct BitableService {
    config: Config,
}

impl BitableService {
    /// 创建新的多维表格服务实例
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 获取配置引用
    pub fn config(&self) -> &Config {
        &self.config
    }
}

// 多维表格服务模块

// 导入v1版本的完整实现
pub mod v1;

pub use v1::*;

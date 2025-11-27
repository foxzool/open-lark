//! OpenLark Admin 服务
//!
//! 提供管理和行政相关的API接口

use crate::{Config, Result};

/// 管理服务
///
/// 提供飞书平台管理和行政相关的功能接口
#[derive(Debug)]
pub struct AdminService<'a> {
    config: &'a Config,
}

impl<'a> AdminService<'a> {
    /// 创建新的管理服务实例
    pub fn new(config: &'a Config) -> Self {
        Self { config }
    }

    /// 获取服务配置
    pub fn config(&self) -> &Config {
        self.config
    }
}

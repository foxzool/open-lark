//! OpenLark Hire 服务
//!
//! 提供招聘管理相关的API接口

use crate::{Config, Result};

/// 招聘服务
///
/// 提供飞书平台招聘管理相关的功能接口
#[derive(Debug)]
pub struct HireService<'a> {
    config: &'a Config,
}

impl<'a> HireService<'a> {
    /// 创建新的招聘服务实例
    pub fn new(config: &'a Config) -> Self {
        Self { config }
    }

    /// 获取服务配置
    pub fn config(&self) -> &Config {
        self.config
    }
}
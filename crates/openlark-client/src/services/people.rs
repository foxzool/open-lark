//! OpenLark People 服务
//!
//! 提供人员和联系人管理相关的API接口

use crate::{Config, Result};

/// 人员服务
///
/// 提供飞书平台人员和联系人管理相关的功能接口
#[derive(Debug)]
pub struct PeopleService<'a> {
    config: &'a Config,
}

impl<'a> PeopleService<'a> {
    /// 创建新的人员服务实例
    pub fn new(config: &'a Config) -> Self {
        Self { config }
    }

    /// 获取服务配置
    pub fn config(&self) -> &Config {
        self.config
    }
}
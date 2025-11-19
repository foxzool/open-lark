//! OpenLark Helpdesk 服务
//!
//! 提供帮助台和搜索相关的API接口

use crate::{Config, Result};

/// 帮助台服务
///
/// 提供飞书平台帮助台和搜索相关的功能接口
#[derive(Debug)]
pub struct HelpdeskService<'a> {
    config: &'a Config,
}

impl<'a> HelpdeskService<'a> {
    /// 创建新的帮助台服务实例
    pub fn new(config: &'a Config) -> Self {
        Self { config }
    }

    /// 获取服务配置
    pub fn config(&self) -> &Config {
        self.config
    }
}
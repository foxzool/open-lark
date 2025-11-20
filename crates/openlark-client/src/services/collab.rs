//! OpenLark Collaboration 服务
//!
//! 提供协作功能相关的API接口，包括日历、会议、任务等

use crate::{Config, Result};

/// 协作服务
///
/// 提供飞书平台协作相关的功能接口
#[derive(Debug)]
pub struct CollabService<'a> {
    config: &'a Config,
}

impl<'a> CollabService<'a> {
    /// 创建新的协作服务实例
    pub fn new(config: &'a Config) -> Self {
        Self { config }
    }

    /// 获取服务配置
    pub fn config(&self) -> &Config {
        self.config
    }
}
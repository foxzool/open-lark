//! OpenLark Approval 服务
//!
//! 提供审批流程相关的API接口

use crate::{Config, Result};

/// 审批服务
///
/// 提供飞书平台审批流程相关的功能接口
#[derive(Debug)]
pub struct ApprovalService<'a> {
    config: &'a Config,
}

impl<'a> ApprovalService<'a> {
    /// 创建新的审批服务实例
    pub fn new(config: &'a Config) -> Self {
        Self { config }
    }

    /// 获取服务配置
    pub fn config(&self) -> &Config {
        self.config
    }
}

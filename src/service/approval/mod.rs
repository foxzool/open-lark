#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(non_snake_case)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::module_inception)]
//! Approval服务模块
//!
//! 提供飞书审批相关的API功能，包括：
//! - 审批实例管理
//! - 审批任务处理
//! - 审批定义配置
//! - 外部审批集成
//! - 审批事件处理

use open_lark_core::config::Config;

/// Approval服务
#[derive(Debug, Clone)]
pub struct ApprovalService {
    pub config: Config,
    pub v4: v4::ApprovalServiceV4,
}

impl ApprovalService {
    pub fn new(config: Config) -> Self {
        Self {
            config: config.clone(),
            v4: v4::ApprovalServiceV4::new(config),
        }
    }
}

pub mod v4;

// 重新导出所有模块和类型
pub use v4::*;

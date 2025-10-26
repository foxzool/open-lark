//! Bot 模块
//!
//! 此模块提供企业机器人管理功能，包括机器人配置、
//! 消息处理和自动化任务执行。

pub mod v1;

/// Bot 服务
pub struct BotService {
    /// v1版本API服务
    pub v1: v1::V1,
}

impl BotService {
    /// 创建新的机器人服务实例
    pub fn new(config: open_lark_core::core::config::Config) -> Self {
        Self {
            v1: v1::V1::new(config),
        }
    }
}

pub mod prelude {
    pub use crate::prelude::*;
}
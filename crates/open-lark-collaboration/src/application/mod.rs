//! Application 模块
//!
//! 此模块提供企业应用管理功能，包括应用信息查询、应用商店管理、
//! 应用使用统计、应用反馈等企业级应用生命周期管理能力。

pub mod v6;

/// Application 服务
pub struct ApplicationService {
    /// v6版本API服务
    pub v6: v6::V6,
}

impl ApplicationService {
    /// 创建新的应用管理服务实例
    pub fn new(config: open_lark_core::core::config::Config) -> Self {
        Self {
            v6: v6::V6::new(config),
        }
    }

pub mod prelude {
    pub use crate::prelude::*;
}
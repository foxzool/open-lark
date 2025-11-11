//! Application 模块
//!
//! 此模块目前为占位符，将来实现应用管理相关的集成功能。

/// 数据模型定义
pub mod models;

/// 应用管理服务 v6 版本
pub mod v6;

/// 应用管理服务
pub struct ApplicationService {
    /// v6版本API服务
    pub v6: v6::V6,
}

impl ApplicationService {
    pub fn new(config: openlark_core::config::Config) -> Self {
        Self {
            v6: v6::V6::new(config),
        }
    }
}

pub mod prelude {
    pub use crate::prelude::*;
}

//! Leaves - 假期管理服务
//!
//! 提供假期管理相关的所有功能，包括：
//! - 假期类型管理
//! - 假期余额查询
//! - 休假申请管理
//! - 假期授予记录管理

use crate::core::config::Config;

/// 假期管理服务
#[derive(Debug, Clone)]
pub struct LeavesService {
    config: Config,
}

impl LeavesService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }
}

/// v1版本API
pub mod v1;

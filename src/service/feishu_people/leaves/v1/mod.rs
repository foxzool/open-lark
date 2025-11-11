#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(non_snake_case)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::module_inception)]
//! Leaves v1 - 假期管理v1版本API
//!
//! 提供假期管理相关的所有功能，包括：
//! - 假期类型管理
//! - 假期余额查询
//! - 休假申请管理
//! - 假期授予记录管理

use open_lark_core::config::Config;

/// 假期管理v1版本服务
#[derive(Debug, Clone)]
pub struct LeavesV1Service {
    config: Config,
}

impl LeavesV1Service {
    pub fn new(config: Config) -> Self {
        Self { config }
    }
}

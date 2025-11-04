#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(non_snake_case)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::module_inception)]
//! 单元格保护服务
//!
//! 提供飞书电子表格单元格保护的完整功能，包括保护范围的创建、
//! 查询、更新、删除等操作，支持精细的权限控制和保护设置。

use crate::core::config::Config;

pub mod create;
pub mod delete;
pub mod get;
pub mod update;

pub use create::*;
pub use delete::*;
pub use get::*;
pub use update::*;

/// 单元格保护服务
///
/// 处理电子表格单元格保护范围的CRUD操作和权限管理。
pub struct ProtectRangeService {
    config: Config,
}

impl ProtectRangeService {
    /// 创建新的单元格保护服务实例
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 获取配置引用
    pub fn config(&self) -> &Config {
        &self.config
    }
}
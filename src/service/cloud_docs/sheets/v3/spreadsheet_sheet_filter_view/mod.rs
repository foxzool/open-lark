#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(non_snake_case)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::module_inception)]
//! 筛选视图服务
//!
//! 提供飞书电子表格筛选视图的完整功能，包括筛选视图的创建、
//! 查询、更新、删除等操作，支持自定义视图和共享设置。

use crate::config::Config;

pub mod create;
pub mod delete;
pub mod get;
pub mod patch;
pub mod query;

pub use create::*;
pub use delete::*;
pub use get::*;
pub use patch::*;
pub use query::*;

/// 筛选视图服务
///
/// 处理电子表格筛选视图的CRUD操作和视图管理。
pub struct SpreadsheetSheetFilterViewService {
    config: Config,
}

impl SpreadsheetSheetFilterViewService {
    /// 创建新的筛选视图服务实例
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 获取配置引用
    pub fn config(&self) -> &Config {
        &self.config
    }
}
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(non_snake_case)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::module_inception)]
//! 筛选视图条件服务
//!
//! 提供飞书电子表格筛选视图条件的完整功能，包括筛选条件的创建、
//! 查询、更新、删除等操作，支持复杂的条件组合和逻辑运算。

use crate::config::Config;

pub mod create;
pub mod delete;
pub mod get;
pub mod query;
pub mod update;

pub use create::*;
pub use delete::*;
pub use get::*;
pub use query::*;
pub use update::*;

/// 筛选视图条件服务
///
/// 处理电子表格筛选视图条件的CRUD操作和条件管理。
pub struct SpreadsheetSheetFilterViewConditionService {
    config: Config,
}

impl SpreadsheetSheetFilterViewConditionService {
    /// 创建新的筛选视图条件服务实例
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 获取配置引用
    pub fn config(&self) -> &Config {
        &self.config
    }
}
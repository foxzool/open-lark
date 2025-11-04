#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(non_snake_case)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::module_inception)]
//! 筛选器服务
//!
//! 提供飞书电子表格筛选器的完整功能，包括筛选器的创建、
//! 查询、更新、删除等操作，支持多种筛选条件和排序方式。

use crate::core::config::Config;

pub mod create;
pub mod delete;
pub mod get;
pub mod update;
pub mod shared;

pub use create::*;
pub use delete::*;
pub use get::*;
pub use update::*;
pub use shared::*;

/// 筛选器服务
///
/// 处理电子表格筛选器的CRUD操作和筛选管理。
pub struct SpreadsheetSheetFilterService {
    config: Config,
}

impl SpreadsheetSheetFilterService {
    /// 创建新的筛选器服务实例
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 获取配置引用
    pub fn config(&self) -> &Config {
        &self.config
    }
}
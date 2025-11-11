#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(non_snake_case)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::module_inception)]
//! 条件格式化服务
//!
//! 提供飞书电子表格条件格式化的完整功能，包括条件格式的创建、
//! 查询、更新、删除等操作，支持多种条件类型和格式样式。

use open_lark_core::config::Config;

pub mod create;
pub mod delete;
pub mod get;
pub mod update;

pub use create::*;
pub use delete::*;
pub use get::*;
pub use update::*;

/// 条件格式化服务
///
/// 处理电子表格条件格式的CRUD操作和样式管理。
pub struct ConditionFormatService {
    config: Config,
}

impl ConditionFormatService {
    /// 创建新的条件格式化服务实例
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 获取配置引用
    pub fn config(&self) -> &Config {
        &self.config
    }
}
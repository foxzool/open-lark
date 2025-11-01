//! 数据验证服务
//!
//! 提供飞书电子表格数据验证的完整功能，包括验证规则的创建、
//! 查询、更新、删除等操作，支持多种验证条件和错误提示。

use crate::core::config::Config;

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

/// 数据验证服务
///
/// 处理电子表格数据验证规则的CRUD操作和验证管理。
pub struct DataValidationService {
    config: Config,
}

impl DataValidationService {
    /// 创建新的数据验证服务实例
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 获取配置引用
    pub fn config(&self) -> &Config {
        &self.config
    }
}
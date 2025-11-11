#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(non_snake_case)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::module_inception)]
//! 浮动图片服务
//!
//! 提供飞书电子表格浮动图片的完整功能，包括图片的创建、
//! 查询、更新、删除等操作，支持图片的位置、大小和样式管理。

use config::Config;

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

/// 浮动图片服务
///
/// 处理电子表格浮动图片的CRUD操作和图片管理。
pub struct FloatImageService {
    config: Config,
}

impl FloatImageService {
    /// 创建新的浮动图片服务实例
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 获取配置引用
    pub fn config(&self) -> &Config {
        &self.config
    }
}
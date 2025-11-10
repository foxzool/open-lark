#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(non_snake_case)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::module_inception)]
//! 行列操作服务
//!
//! 提供飞书电子表格行列的完整操作功能，包括行列的插入、删除、
//! 更新、移动等操作，以及行列属性管理。

use crate::config::Config;

pub use add_rows_or_columns::*;
pub use delete_rows_or_columns::*;
pub use insert_rows_or_columns::*;
pub use move_dimension::*;
pub use update_rows_or_columns::*;

mod add_rows_or_columns;
mod delete_rows_or_columns;
mod insert_rows_or_columns;
mod move_dimension;
mod update_rows_or_columns;

/// 行列操作服务
///
/// 处理电子表格行列的CRUD操作和属性管理。
pub struct SheetRowColService {
    config: Config,
}

impl SheetRowColService {
    /// 创建新的行列操作服务实例
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 获取配置引用
    pub fn config(&self) -> &Config {
        &self.config
    }
}
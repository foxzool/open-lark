//! 工作表操作服务
//!
//! 提供飞书电子表格工作表的完整管理功能，包括工作表的创建、查询、
//! 更新、删除等基础操作，以及工作表属性管理。

use serde::{Deserialize, Serialize};

use crate::core::{
use crate::core::SDKResult;    config::Config,
    api_resp::{ApiResponseTrait, ResponseFormat},
    SDKResult,
};

pub use get::*;
pub use query::*;

mod get;
mod query;

/// 工作表服务
///
/// 处理电子表格工作表的CRUD操作和属性管理。
pub struct SpreadsheetSheetService {
    config: Config,
}

impl SpreadsheetSheetService {
    /// 创建新的工作表服务实例
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 获取配置引用
    pub fn config(&self) -> &Config {
        &self.config
    }
}

/// 工作表基础信息
#[derive(Debug, Clone, Deserialize)]
pub struct Sheet {
    /// 工作表 ID
    pub sheet_id: String,
    /// 工作表标题
    pub title: String,
    /// 工作表索引位置，索引从 0 开始计数
    pub index: u32,
    /// 工作表是否被隐藏
    ///
    /// * true：被隐藏
    /// * false：未被隐藏
    pub hidden: bool,
    /// 单元格属性，仅当 resource_type 为 sheet 即工作表类型为电子表格时返回
    pub grid_properties: Option<GridProperties>,
    /// 工作表类型
    ///
    /// * sheet：工作表
    /// * bitable：多维表格。详情参考多维表格概述
    /// * #UNSUPPORTED_TYPE：不支持的类型
    pub resource_type: String,
    /// 合并单元格的相关信息。没有合并单元格则不返回
    pub merges: Option<Vec<MergeRange>>,
}

impl ApiResponseTrait for Sheet {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 单元格属性，仅当 resource_type 为 sheet 即工作表类型为电子表格时返回
#[derive(Debug, Clone, Deserialize)]
pub struct GridProperties {
    /// 冻结的行数量
    pub frozen_row_count: i32,
    /// 冻结的列数量
    pub frozen_column_count: i32,
    /// 工作表的行数
    pub row_count: i32,
    /// 工作表的列数量
    pub column_count: i32,
}

/// 合并单元格的相关信息
#[derive(Debug, Clone, Deserialize)]
pub struct MergeRange {
    /// 起始行，从 0 开始计数
    pub start_row_index: i32,
    /// 结束行，从 0 开始计数
    pub end_row_index: i32,
    /// 起始列，从 0 开始计数
    pub start_column_index: i32,
    /// 结束列，从 0 开始计数
    pub end_column_index: i32,
}
use serde::Deserialize;

pub use get::*;
pub use query::*;

mod get;
mod query;

#[derive(Deserialize, Debug)]
pub struct Sheet {
    /// 工作表 ID
    pub sheet_id: String,
    /// 工作表标题
    pub title: String,
    /// 工作表索引位置，索引从 0 开始计数。
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

/// 单元格属性，仅当 resource_type 为 sheet 即工作表类型为电子表格时返回
#[derive(Deserialize, Debug)]
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
#[derive(Deserialize, Debug)]
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

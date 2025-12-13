/// 筛选管理模块
use serde::{Deserialize, Serialize};
pub mod create;
pub mod delete;
pub mod get;
pub mod update;

/// 筛选数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FilterData {
    /// 筛选器ID
    #[serde(rename = "filter_id")]
    pub filter_id: String,
    /// 筛选器名称
    #[serde(rename = "name")]
    pub name: Option<String>,
    /// 筛选范围
    #[serde(rename = "range")]
    pub range: Option<FilterRange>,
    /// 筛选条件
    #[serde(rename = "condition")]
    pub condition: Option<FilterCondition>,
    /// 创建时间
    #[serde(rename = "created_time")]
    pub created_time: Option<String>,
    /// 更新时间
    #[serde(rename = "updated_time")]
    pub updated_time: Option<String>,
}

/// 筛选器范围
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FilterRange {
    /// 起始行索引
    #[serde(rename = "start_row_index")]
    pub start_row_index: i32,
    /// 结束行索引
    #[serde(rename = "end_row_index")]
    pub end_row_index: i32,
    /// 起始列索引
    #[serde(rename = "start_column_index")]
    pub start_column_index: i32,
    /// 结束列索引
    #[serde(rename = "end_column_index")]
    pub end_column_index: i32,
}

/// 筛选条件
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FilterCondition {
    /// 列索引
    #[serde(rename = "column_index")]
    pub column_index: i32,
    /// 筛选操作符
    #[serde(rename = "operator")]
    pub operator: String,
    /// 筛选值
    #[serde(rename = "value")]
    pub value: Option<serde_json::Value>,
}

// 重新导出所有API函数
pub use create::*;
pub use delete::*;
pub use get::*;
pub use update::*;

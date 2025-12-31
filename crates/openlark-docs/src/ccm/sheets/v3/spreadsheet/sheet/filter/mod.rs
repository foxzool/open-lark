/// 筛选（spreadsheet.sheet.filter）
use serde::{Deserialize, Serialize};

pub mod create;
pub mod delete;
pub mod get;
pub mod update;

/// 筛选条件
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Condition {
    pub filter_type: String,
    pub compare_type: String,
    pub expected: Vec<String>,
}

/// 获取筛选返回的筛选信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SheetFilterInfo {
    pub range: String,
    pub filtered_out_rows: Vec<i32>,
    pub filter_infos: Vec<FilterInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FilterInfo {
    pub col: String,
    pub conditions: Vec<Condition>,
}

// 重新导出所有API函数
pub use create::*;
pub use delete::*;
pub use get::*;
pub use update::*;

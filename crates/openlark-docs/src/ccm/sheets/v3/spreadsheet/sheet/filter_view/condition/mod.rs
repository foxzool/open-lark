/// 筛选条件（spreadsheet.sheet.filter_view.condition）
pub mod create;
pub mod delete;
pub mod get;
pub mod query;
pub mod update;

use serde::{Deserialize, Serialize};

/// 筛选条件
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Condition {
    pub condition_id: String,
    pub filter_type: String,
    pub compare_type: String,
    pub expected: Vec<String>,
}

// 重新导出所有API函数
pub use create::*;
pub use delete::*;
pub use get::*;
pub use query::*;
pub use update::*;

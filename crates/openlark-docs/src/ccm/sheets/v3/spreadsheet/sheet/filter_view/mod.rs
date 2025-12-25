/// 筛选视图（spreadsheet.sheet.filter_view）
pub mod condition;
pub mod create;
pub mod delete;
pub mod get;
pub mod patch;
pub mod query;

use serde::{Deserialize, Serialize};

/// 筛选视图
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FilterView {
    pub filter_view_id: String,
    pub filter_view_name: String,
    pub range: String,
}

// 重新导出所有API函数
pub use condition::*;
pub use create::*;
pub use delete::*;
pub use get::*;
pub use patch::*;
pub use query::*;

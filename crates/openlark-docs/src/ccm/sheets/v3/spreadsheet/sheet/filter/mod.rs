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
// create 模块显式导出
pub use create::{
    CreateFilterRequest,
    CreateFilterResponse,
    DeleteFilterResponse,
    GetFilterResponse,
    UpdateFilterRequest,
    UpdateFilterResponse,
    create_filter,
    create_filter_with_options,
    delete_filter,
    delete_filter_with_options,
    get_filter,
    get_filter_with_options,
    update_filter,
    update_filter_with_options,
};
// delete 模块显式导出
pub use delete::{
    CreateFilterRequest,
    CreateFilterResponse,
    DeleteFilterResponse,
    GetFilterResponse,
    UpdateFilterRequest,
    UpdateFilterResponse,
    create_filter,
    create_filter_with_options,
    delete_filter,
    delete_filter_with_options,
    get_filter,
    get_filter_with_options,
    update_filter,
    update_filter_with_options,
};
// get 模块显式导出
pub use get::{
    CreateFilterRequest,
    CreateFilterResponse,
    DeleteFilterResponse,
    GetFilterResponse,
    UpdateFilterRequest,
    UpdateFilterResponse,
    create_filter,
    create_filter_with_options,
    delete_filter,
    delete_filter_with_options,
    get_filter,
    get_filter_with_options,
    update_filter,
    update_filter_with_options,
};
// update 模块显式导出
pub use update::{
    CreateFilterRequest,
    CreateFilterResponse,
    DeleteFilterResponse,
    GetFilterResponse,
    UpdateFilterRequest,
    UpdateFilterResponse,
    create_filter,
    create_filter_with_options,
    delete_filter,
    delete_filter_with_options,
    get_filter,
    get_filter_with_options,
    update_filter,
    update_filter_with_options,
};

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
// create 模块显式导出
pub use create::{
    CreateFilterConditionRequest,
    CreateFilterConditionResponse,
    DeleteFilterConditionResponse,
    GetFilterConditionResponse,
    QueryFilterConditionsResponse,
    UpdateFilterConditionRequest,
    UpdateFilterConditionResponse,
    create_filter_condition,
    create_filter_condition_with_options,
    delete_filter_condition,
    delete_filter_condition_with_options,
    get_filter_condition,
    get_filter_condition_with_options,
    query_filter_conditions,
    query_filter_conditions_with_options,
    update_filter_condition,
    update_filter_condition_with_options,
};
// delete 模块显式导出
pub use delete::{
    CreateFilterConditionRequest,
    CreateFilterConditionResponse,
    DeleteFilterConditionResponse,
    GetFilterConditionResponse,
    QueryFilterConditionsResponse,
    UpdateFilterConditionRequest,
    UpdateFilterConditionResponse,
    create_filter_condition,
    create_filter_condition_with_options,
    delete_filter_condition,
    delete_filter_condition_with_options,
    get_filter_condition,
    get_filter_condition_with_options,
    query_filter_conditions,
    query_filter_conditions_with_options,
    update_filter_condition,
    update_filter_condition_with_options,
};
// get 模块显式导出
pub use get::{
    CreateFilterConditionRequest,
    CreateFilterConditionResponse,
    DeleteFilterConditionResponse,
    GetFilterConditionResponse,
    QueryFilterConditionsResponse,
    UpdateFilterConditionRequest,
    UpdateFilterConditionResponse,
    create_filter_condition,
    create_filter_condition_with_options,
    delete_filter_condition,
    delete_filter_condition_with_options,
    get_filter_condition,
    get_filter_condition_with_options,
    query_filter_conditions,
    query_filter_conditions_with_options,
    update_filter_condition,
    update_filter_condition_with_options,
};
// query 模块显式导出
pub use query::{
    CreateFilterConditionRequest,
    CreateFilterConditionResponse,
    DeleteFilterConditionResponse,
    GetFilterConditionResponse,
    QueryFilterConditionsResponse,
    UpdateFilterConditionRequest,
    UpdateFilterConditionResponse,
    create_filter_condition,
    create_filter_condition_with_options,
    delete_filter_condition,
    delete_filter_condition_with_options,
    get_filter_condition,
    get_filter_condition_with_options,
    query_filter_conditions,
    query_filter_conditions_with_options,
    update_filter_condition,
    update_filter_condition_with_options,
};
// update 模块显式导出
pub use update::{
    CreateFilterConditionRequest,
    CreateFilterConditionResponse,
    DeleteFilterConditionResponse,
    GetFilterConditionResponse,
    QueryFilterConditionsResponse,
    UpdateFilterConditionRequest,
    UpdateFilterConditionResponse,
    create_filter_condition,
    create_filter_condition_with_options,
    delete_filter_condition,
    delete_filter_condition_with_options,
    get_filter_condition,
    get_filter_condition_with_options,
    query_filter_conditions,
    query_filter_conditions_with_options,
    update_filter_condition,
    update_filter_condition_with_options,
};

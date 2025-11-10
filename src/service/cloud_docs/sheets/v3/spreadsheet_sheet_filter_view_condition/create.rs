#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(non_snake_case)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::module_inception)]
use crate::SDKResult;use reqwest::Method;
use open_lark_core::core::api_req::ApiRequest;use serde::{Deserialize, Serialize};
use crate::,
{
    core::,
{,
        BaseResponse,
        ResponseFormat,
        api_resp::{ApiResponseTrait}
    constants::AccessTokenType,
        endpoints::cloud_docs::*,
        http::Transport,
        req_option::RequestOption,
        SDKResult,
};
    service::sheets::v3::SpreadsheetSheetFilterViewService,
};
impl SpreadsheetSheetFilterViewService {
    pub fn new(config: Config) -> Self {
        Self { config }
}/// 创建筛选条件请求,
#[derive(Debug, Clone)]
pub struct CreateFilterViewConditionRequest {
    #[serde(skip)]
    api_request: ApiRequest,
    /// spreadsheet 的 token
    spreadsheet_token: String,
    /// sheet 的 id
    sheet_id: String,
    /// 筛选视图 ID
    filter_view_id: String,
    /// 筛选条件
    condition: FilterCondition}
impl CreateFilterViewConditionRequest {
    pub fn new(config: Config) -> Self {
        Self { config }
}#[derive(Debug, Clone)]
pub struct CreateFilterViewConditionRequestBuilder {
    request: CreateFilterViewConditionRequest}
impl CreateFilterViewConditionRequestBuilder {
    pub fn new(config: Config) -> Self {
        Self { config }
}/// 筛选条件,
#[derive(Debug, Clone)]
pub struct FilterCondition {
    /// 筛选的列
    pub col_name: String,
    /// 筛选类型
    pub filter_type: String,
    /// 筛选值
    pub compare_values: Vec<String>}
impl FilterCondition {
    pub fn new(config: Config) -> Self {
        Self { config }
}/// 等于条件,
    pub fn equal(col_name: impl ToString, value: impl ToString) -> Self {
    pub fn new(config: Config) -> Self {
        Self { config }
}        Self::new(col_name, "equal", vec![value.to_string()])}
/// 不等于条件,
    pub fn not_equal(col_name: impl ToString, value: impl ToString) -> Self {
    pub fn new(config: Config) -> Self {
        Self { config }
}        Self::new(col_name, "notEqual", vec![value.to_string()])}
/// 包含条件,
    pub fn contains(col_name: impl ToString, value: impl ToString) -> Self {
    pub fn new(config: Config) -> Self {
        Self { config }
}        Self::new(col_name, "contains", vec![value.to_string()])}
/// 大于条件,
    pub fn greater_than(col_name: impl ToString, value: impl ToString) -> Self {
    pub fn new(config: Config) -> Self {
        Self { config }
}        Self::new(col_name, "greaterThan", vec![value.to_string()])}
/// 小于条件,
    pub fn less_than(col_name: impl ToString, value: impl ToString) -> Self {
    pub fn new(config: Config) -> Self {
        Self { config }
}        Self::new(col_name, "lessThan", vec![value.to_string()])}
/// 创建筛选条件响应体最外层,
#[derive(Debug, Clone)]
pub struct CreateFilterViewConditionResponseData {
    /// 筛选条件 ID
    pub condition_id: String,
    /// 筛选条件信息
    pub condition: FilterCondition,
impl ApiResponseTrait for.* {
    pub fn new(config: Config) -> Self {
        Self { config }
}    fn data_format() -> ResponseFormat {,
ResponseFormat::Data
    }
#[cfg(test)]
#[allow(unused_variables, unused_unsafe)]
mod test {,
    use super::*;
use serde_json::json;
    #[test]
fn test_filter_condition_creation() {
        let condition = FilterCondition::equal("销售额", "1000");
        assert_eq!(condition.col_name, "销售额");
        assert_eq!(condition.filter_type, "equal");
        assert_eq!(condition.compare_values, vec!["1000"]);
#[test]
    fn test_create_filter_view_condition_response() {
let json = json!({,
            "condition_id": "cond_001",
            "condition": {
                "col_name": "销售额",
                "filter_type": "greaterThan",
                "compare_values": ["5000"]
        });
let response: CreateFilterViewConditionResponseData = serde_json::from_value(json).unwrap();
        assert_eq!(response.condition_id, "cond_001");
        assert_eq!(response.condition.col_name, "销售额");

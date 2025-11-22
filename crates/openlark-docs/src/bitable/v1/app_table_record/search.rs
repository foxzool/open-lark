#![allow(unused_variables, unused_unsafe)]

#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(unused_mut)]
use SDKResult;use reqwest::Method;
use openlark_core::api::ApiRequest;use serde::{Deserialize, Serialize};
use openlark_core::,
{
    core::,
{,
        BaseResponse,
        ResponseFormat,
        api::{ApiResponseTrait}
    config::Config,
        constants::AccessTokenType,
        endpoints::cloud_docs::*,
        http::Transport,
        req_option::RequestOption,
        SDKResult,
};
    service::bitable::v1::Record,
};
/// 查询记录请求,
#[derive(Clone)]
pub struct SearchRecordRequest {
    #[serde(skip)]
    api_request: ApiRequest,
    /// 多维表格的唯一标识符,
#[serde(skip)]
    app_token: String,
    /// 数据表的唯一标识符,
#[serde(skip)]
    table_id: String,
    /// 用户 ID 类型,
#[serde(skip)]
    user_id_type: Option<String>,
    /// 分页标记,
#[serde(skip)]
    page_token: Option<String>,
    /// 分页大小,
#[serde(skip)]
    page_size: Option<i32>,
    /// 视图的唯一标识符
    view_id: Option<String>,
    /// 字段名称，用于指定本次查询返回记录中包含的字段
    field_names: Option<Vec<String>>,
    /// 排序条件
    sort: Option<Vec<SortCondition>>,
    /// 筛选条件
    filter: Option<FilterInfo>,
    /// 控制是否返回自动计算的字段
    automatic: Option<bool>}
/// 排序条件,
#[derive(Clone)]
pub struct SortCondition {
    /// 字段名称
    pub field_name: String,
    /// 是否倒序排序,
#[serde(skip_serializing_if = "Option::is_none")]
    pub desc: Option<bool>}
/// 筛选条件,
#[derive(Clone)]
pub struct FilterInfo {
    /// 条件逻辑连接词: "and" 或 "or"
    pub conjunction: String,
    /// 筛选条件集合
    pub conditions: Vec<FilterCondition>}
/// 单个筛选条件,
#[derive(Clone)]
pub struct FilterCondition {
    /// 筛选条件的左值，值为字段的名称
    pub field_name: String,
    /// 条件运算符
    pub operator: String,
    /// 目标值,
#[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<Vec<String>>}
impl SearchRecordRequest {
    pub fn new(config: Config) -> Self {
        Self { config }
}#[derive(Clone)]
pub struct SearchRecordRequestBuilder {
    request: SearchRecordRequest}
impl SearchRecordRequestBuilder {
    pub fn new(config: Config) -> Self {
        Self { config }
}if let Some(page_token) = &self.request.page_token {,
            self.request,
.api_request,
                .query_params
                .insert("page_token", page_token.clone());
if let Some(page_size) = &self.request.page_size {,
            self.request,
.api_request,
                .query_params
                .insert("page_size", page_size.to_string());
self.request.api_request.body = serde_json::to_vec(&self.request).unwrap();
        self.request,
// 应用ExecutableBuilder trait到SearchRecordRequestBuilder,
crate::impl_executable_builder_owned!(
    SearchRecordRequestBuilder,
    super::AppTableRecordService,
    SearchRecordRequest,
    Response<SearchRecordResponse>,
    search,
);
/// 查询记录响应
#[derive(Clone)]
pub struct SearchRecordResponse {
    /// 记录列表
    pub items: Vec<Record>,
    /// 是否还有更多项
    pub has_more: bool,
    /// 分页标记
    pub page_token: Option<String>,
    /// 总数
    pub total: i32,
impl ApiResponseTrait for.* {
    pub fn new(config: Config) -> Self {
        Self { config }
}    fn data_format() -> ResponseFormat {,
ResponseFormat::Data
    }
/// 查询记录,
pub async fn search_record(
    request: SearchRecordRequest,
    config: &Config,
    option: Option<RequestOption>,
) -> SDKResult<Response<SearchRecordResponse>> {,
let mut api_req = request.api_request;
    api_req.set_http_method(Method::POST);
api_req.api_path = BITABLE_V1_RECORDS_SEARCH,
        .replace("{app_token}", &request.app_token)
        .replace("{table_id}", &request.table_id);
    api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant, AccessTokenType::User]);

    let api_resp = Transport::request(api_req, config, option).await?;
Ok(api_resp),

impl FilterInfo {
    pub fn new(config: Config) -> Self {
        Self { config }
}/// 创建 OR 条件,
    pub fn or(conditions: Vec<FilterCondition>) -> Self {
Self {
            conjunction: "or".to_string(),
            conditions}
impl FilterCondition {
    pub fn new(config: Config) -> Self {
        Self { config }
}/// 不等于,
    pub fn not_equals(field_name: impl ToString, value: impl ToString) -> Self {
    pub fn new(config: Config) -> Self {
        Self { config }
}Self {
            field_name: field_name.to_string(),
            operator: "isNot".to_string(),
            value: Some(vec![value.to_string()])}
/// 包含,
    pub fn contains(field_name: impl ToString, value: impl ToString) -> Self {
    pub fn new(config: Config) -> Self {
        Self { config }
}Self {
            field_name: field_name.to_string(),
            operator: "contains".to_string(),
            value: Some(vec![value.to_string()])}
/// 不包含,
    pub fn not_contains(field_name: impl ToString, value: impl ToString) -> Self {
    pub fn new(config: Config) -> Self {
        Self { config }
}Self {
            field_name: field_name.to_string(),
            operator: "doesNotContain".to_string(),
            value: Some(vec![value.to_string()])}
/// 为空,
    pub fn is_empty(field_name: impl ToString) -> Self {
    pub fn new(config: Config) -> Self {
        Self { config }
}Self {
            field_name: field_name.to_string(),
            operator: "isEmpty".to_string(),
            value: None}
/// 不为空,
    pub fn is_not_empty(field_name: impl ToString) -> Self {
    pub fn new(config: Config) -> Self {
        Self { config }
}Self {
            field_name: field_name.to_string(),
            operator: "isNotEmpty".to_string(),
            value: None}
/// 大于,
    pub fn greater_than(field_name: impl ToString, value: impl ToString) -> Self {
    pub fn new(config: Config) -> Self {
        Self { config }
}Self {
            field_name: field_name.to_string(),
            operator: "isGreater".to_string(),
            value: Some(vec![value.to_string()])}
/// 小于,
    pub fn less_than(field_name: impl ToString, value: impl ToString) -> Self {
    pub fn new(config: Config) -> Self {
        Self { config }
}Self {
            field_name: field_name.to_string(),
            operator: "isLess".to_string(),
            value: Some(vec![value.to_string()])}
#[cfg(test)]
mod tests {
    use super::*;
#[test]
    fn test_search_record_request_builder() {
let filter = FilterInfo::and(vec![,
            FilterCondition::equals("状态", "进行中"),
            FilterCondition::is_not_empty("标题"),
        ]);
let sort = vec![SortCondition {,
            field_name: "创建时间".to_string(),
            desc: Some(true)}];
let request = SearchRecordRequest::builder(),
            .app_token()
.table_id()
            .page_size()
.filter()
            .sort(sort)
            .field_names(vec!["标题".to_string(), "状态".to_string()]),
.build();
        assert_eq!(request.app_token, "bascnmBA*****yGehy8");
        assert_eq!(request.table_id, "tblsRc9GRRXKqhvW");
        assert_eq!(request.page_size, Some(20));
assert!(request.filter.is_some());
        assert!(request.sort.is_some());

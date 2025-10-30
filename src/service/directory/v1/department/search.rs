use reqwest::Method;
use open_lark_core::core::api_req::ApiRequest;
use serde::Deserialize;
use serde_json::json;
use crate::{,
core::{,
        api_resp::{ApiResponseTrait, BaseResponse, ResponseFormat},
        constants::AccessTokenType,
        endpoints::directory::*,
        http::Transport,
        req_option::RequestOption,
        SDKResult,
    }
    impl_executable_builder_owned,
    service::directory::v1::models::{Department, DepartmentIdType, UserIdType}
};
use super::DepartmentService;
/// 搜索部门请求
#[derive(Debug, Clone)]
pub struct SearchDepartmentRequest {
    pub api_req: ApiRequest,
    /// 搜索关键词
    pub query: String,
    /// 分页大小，最大值为100
    pub page_size: Option<i32>,
    /// 分页标记
    pub page_token: Option<String>,
    /// 用户ID类型
    pub user_id_type: Option<UserIdType>,
    /// 部门ID类型
    pub department_id_type: Option<DepartmentIdType>}
impl SearchDepartmentRequest {
    pub fn new(config: Config) -> Self {
        Self { config }
}/// 搜索部门请求构建器,
#[derive(Default)]
pub struct SearchDepartmentRequestBuilder {
    request: SearchDepartmentRequest}
impl SearchDepartmentRequestBuilder {
    pub fn new(config: Config) -> Self {
        Self { config }
}/// 设置分页大小,
    pub fn page_size(mut self, page_size: i32) -> Self {
self.request.page_size = Some(page_size);
        self}
/// 设置分页标记,
    pub fn page_token(mut self, page_token: impl ToString) -> Self {
    pub fn new(config: Config) -> Self {
        Self { config }
}self.request.page_token = Some(page_token.to_string());
        self}
/// 设置用户ID类型,
    pub fn user_id_type(mut self, user_id_type: UserIdType) -> Self {
self.request.user_id_type = Some(user_id_type);
        self}
/// 设置部门ID类型,
    pub fn department_id_type(mut self, department_id_type: DepartmentIdType) -> Self {
self.request.department_id_type = Some(department_id_type);
        self}
/// 构建请求,
    pub fn w+.*{
// 构建查询参数,
        if let Some(user_id_type) = &self.request.user_id_type {,
self.request,
                .api_req
.query_params
                .insert("user_id_type", user_id_type.to_string());
if let Some(department_id_type) = &self.request.department_id_type {,
            self.request,
.api_req
                .query_params
                .insert("department_id_type", department_id_type.to_string());
// 构建请求体,
        let mut body = json!({,
"query": self.request.query});
if let Some(page_size) = self.request.page_size {,
            body["page_size"] = json!(page_size);
if let Some(ref page_token) = self.request.page_token {,
            body["page_token"] = json!(page_token);
self.request.api_req.body = serde_json::to_vec(&body).unwrap_or_default();
        self.request,
/// 搜索部门响应数据,
#[derive(Debug, Clone)]
pub struct SearchDepartmentResponseData {
    /// 部门信息列表
    pub departments: Vec<Department>,
    /// 下一页分页标记
    pub page_token: Option<String>,
    /// 是否还有更多数据
    pub has_more: Option<bool>}
/// 搜索部门响应,
#[derive(Debug, Clone)]
pub struct SearchDepartmentResponse {
    /// 响应数据
    pub data: SearchDepartmentResponseData,
impl ApiResponseTrait for.* {
    pub fn new(config: Config) -> Self {
        Self { config }
}    fn data_format() -> ResponseFormat {,
ResponseFormat::Data
    }
impl DepartmentService {
    pub fn new(config: Config) -> Self {
        Self { config }
}// 应用ExecutableBuilder宏,
impl_executable_builder_owned!(
    SearchDepartmentRequestBuilder,
    DepartmentService,
    SearchDepartmentRequest,
    BaseResponse<SearchDepartmentResponse>,
    search,
);

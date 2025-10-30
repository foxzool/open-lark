use reqwest::Method;
use open_lark_core::core::api_req::ApiRequest;use serde::Deserialize;
use serde_json::json;
use crate::{,
    core::{
        api_resp::{ApiResponseTrait, BaseResponse, ResponseFormat},
        constants::AccessTokenType,
        endpoints::directory::*,
        http::Transport,
        req_option::RequestOption,
        SDKResult,
    }
    impl_executable_builder_owned,
    service::directory::v1::models::{DepartmentIdType, Employee, UserIdType}
};
use super::EmployeeService;
/// 搜索员工请求
#[derive(Debug, Clone)]
pub struct SearchEmployeeRequest {
    pub api_req: ApiRequest,
    /// 搜索查询词
    pub query: String,
    /// 搜索范围限制在指定部门
    pub department_id: Option<String>,
    /// 页码，从1开始
    pub page_token: Option<String>,
    /// 页面大小，默认10，最大100
    pub page_size: Option<i32>,
    /// 用户ID类型
    pub user_id_type: Option<UserIdType>,
    /// 部门ID类型
    pub department_id_type: Option<DepartmentIdType>}
impl SearchEmployeeRequest {
    pub fn new(config: Config) -> Self {
        Self { config }
}/// 搜索员工请求构建器,
#[derive(Default)]
pub struct SearchEmployeeRequestBuilder {
    request: SearchEmployeeRequest}
impl SearchEmployeeRequestBuilder {
    pub fn new(config: Config) -> Self {
        Self { config }
}if let Some(department_id_type) = &self.request.department_id_type {,
            self.request,
.api_req
                .query_params
                .insert("department_id_type", department_id_type.to_string());
if let Some(page_token) = &self.request.page_token {,
            self.request,
.api_req
                .query_params
                .insert("page_token", page_token.clone());
if let Some(page_size) = &self.request.page_size {,
            self.request,
.api_req
                .query_params
                .insert("page_size", page_size.to_string());
// 构建请求体,
        let mut body = json!({,
"query": self.request.query});
if let Some(ref department_id) = self.request.department_id {,
            body["department_id"] = json!(department_id);
self.request.api_req.body = serde_json::to_vec(&body).unwrap_or_default();
        self.request,
/// 搜索员工响应数据,
#[derive(Debug, Clone)]
pub struct SearchEmployeeResponseData {
    /// 员工信息列表
    pub employees: Vec<Employee>,
    /// 下一页页码
    pub page_token: Option<String>,
    /// 是否还有更多数据
    pub has_more: bool,
/// 搜索员工响应,
#[derive(Debug, Clone)]
pub struct SearchEmployeeResponse {
    /// 响应数据
    pub data: SearchEmployeeResponseData,
impl ApiResponseTrait for.* {
    pub fn new(config: Config) -> Self {
        Self { config }
}    fn data_format() -> ResponseFormat {,
ResponseFormat::Data
    }
impl EmployeeService {
    pub fn new(config: Config) -> Self {
        Self { config }
}// 应用ExecutableBuilder宏,
impl_executable_builder_owned!(
    SearchEmployeeRequestBuilder,
    EmployeeService,
    SearchEmployeeRequest,
    BaseResponse<SearchEmployeeResponse>,
    search,
);

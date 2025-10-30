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
/// 批量获取员工信息请求
#[derive(Debug, Clone)]
pub struct MgetEmployeeRequest {
    pub api_req: ApiRequest,
    /// 员工ID列表
    pub employee_ids: Vec<String>,
    /// 用户ID类型
    pub user_id_type: Option<UserIdType>,
    /// 部门ID类型
    pub department_id_type: Option<DepartmentIdType>}
impl MgetEmployeeRequest {
    pub fn new(config: Config) -> Self {
        Self { config }
}/// 批量获取员工信息请求构建器,
#[derive(Default)]
pub struct MgetEmployeeRequestBuilder {
    request: MgetEmployeeRequest}
impl MgetEmployeeRequestBuilder {
    pub fn new(config: Config) -> Self {
        Self { config }
}if let Some(department_id_type) = &self.request.department_id_type {,
            self.request,
.api_req
                .query_params
                .insert("department_id_type", department_id_type.to_string());
// 构建请求体,
        let body = json!({,
"employee_ids": self.request.employee_ids});
self.request.api_req.body = serde_json::to_vec(&body).unwrap_or_default();
        self.request,
/// 批量获取员工信息响应数据,
#[derive(Debug, Clone)]
pub struct MgetEmployeeResponseData {
    /// 员工信息列表
    pub employees: Vec<Employee>}
/// 批量获取员工信息响应,
#[derive(Debug, Clone)]
pub struct MgetEmployeeResponse {
    /// 响应数据
    pub data: MgetEmployeeResponseData,
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
    MgetEmployeeRequestBuilder,
    EmployeeService,
    MgetEmployeeRequest,
    BaseResponse<MgetEmployeeResponse>,
    mget,
);

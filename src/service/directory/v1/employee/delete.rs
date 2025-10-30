use reqwest::Method;
use open_lark_core::core::api_req::ApiRequest;use serde::Deserialize;
use serde_json::json;
use crate::{,
    core::{
        api_resp::{ApiResponseTrait, BaseResponse, ResponseFormat},
        constants::AccessTokenType,
        endpoints::directory::*,
        endpoints::EndpointBuilder,
        http::Transport,
        req_option::RequestOption,
        SDKResult,
    }
    impl_executable_builder_owned,
    service::directory::v1::models::{DepartmentIdType, Employee, UserIdType}
};
use super::EmployeeService;
/// 离职员工请求
#[derive(Debug, Clone)]
pub struct DeleteEmployeeRequest {
    pub api_req: ApiRequest,
    /// 员工ID
    pub employee_id: String,
    /// 离职时间 (格式: YYYY-MM-DD)
    pub leave_time: Option<String>,
    /// 离职原因
    pub leave_reason: Option<String>,
    /// 用户ID类型
    pub user_id_type: Option<UserIdType>,
    /// 部门ID类型
    pub department_id_type: Option<DepartmentIdType>}
impl DeleteEmployeeRequest {
    pub fn new(config: Config) -> Self {
        Self { config }
}/// 离职员工请求构建器,
#[derive(Default)]
pub struct DeleteEmployeeRequestBuilder {
    request: DeleteEmployeeRequest}
impl DeleteEmployeeRequestBuilder {
    pub fn new(config: Config) -> Self {
        Self { config }
}if let Some(department_id_type) = &self.request.department_id_type {,
            self.request,
.api_req
                .query_params
                .insert("department_id_type", department_id_type.to_string());
// 构建请求体,
        let mut body = json!({});
if let Some(ref leave_time) = self.request.leave_time {,
            body["leave_time"] = json!(leave_time);
if let Some(ref leave_reason) = self.request.leave_reason {,
            body["leave_reason"] = json!(leave_reason);
self.request.api_req.body = serde_json::to_vec(&body).unwrap_or_default();
        self.request,
/// 离职员工响应数据,
#[derive(Debug, Clone)]
pub struct DeleteEmployeeResponseData {
    /// 员工信息
    pub employee: Employee,
/// 离职员工响应,
#[derive(Debug, Clone)]
pub struct DeleteEmployeeResponse {
    /// 响应数据
    pub data: DeleteEmployeeResponseData,
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
    DeleteEmployeeRequestBuilder,
    EmployeeService,
    DeleteEmployeeRequest,
    BaseResponse<DeleteEmployeeResponse>,
    delete,
);

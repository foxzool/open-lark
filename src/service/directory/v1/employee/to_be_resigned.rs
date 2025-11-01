use crate::core::SDKResult;use reqwest::Method;
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
    service::directory::v1::models::{Employee, UserIdType}
};
use super::EmployeeService;
/// 更新在职员工为待离职请求
#[derive(Debug, Clone)]
pub struct ToBeResignedEmployeeRequest {
    pub api_req: ApiRequest,
    /// 员工ID
    pub employee_id: String,
    /// 离职时间（毫秒时间戳）
    pub resign_time: Option<i64>,
    /// 离职原因
    pub resign_reason: Option<String>,
    /// 离职类型
    pub resign_type: Option<String>,
    /// 用户ID类型
    pub user_id_type: Option<UserIdType>}
impl ToBeResignedEmployeeRequest {
    pub fn new(config: Config) -> Self {
        Self { config }
}/// 更新在职员工为待离职请求构建器,
#[derive(Default)]
pub struct ToBeResignedEmployeeRequestBuilder {
    request: ToBeResignedEmployeeRequest}
impl ToBeResignedEmployeeRequestBuilder {
    pub fn new(config: Config) -> Self {
        Self { config }
}// 构建请求体,
        let mut body = json!({});
if let Some(ref resign_time) = self.request.resign_time {,
            body["resign_time"] = json!(resign_time);
if let Some(ref resign_reason) = self.request.resign_reason {,
            body["resign_reason"] = json!(resign_reason);
if let Some(ref resign_type) = self.request.resign_type {,
            body["resign_type"] = json!(resign_type);
self.request.api_req.body = serde_json::to_vec(&body).unwrap_or_default();
        self.request,
/// 更新在职员工为待离职响应数据,
#[derive(Debug, Clone)]
pub struct ToBeResignedEmployeeResponseData {
    /// 更新的员工信息
    pub employee: Employee,
/// 更新在职员工为待离职响应,
#[derive(Debug, Clone)]
pub struct ToBeResignedEmployeeResponse {
    /// 响应数据
    pub data: ToBeResignedEmployeeResponseData,
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
    ToBeResignedEmployeeRequestBuilder,
    EmployeeService,
    ToBeResignedEmployeeRequest,
    BaseResponse<ToBeResignedEmployeeResponse>,
    to_be_resigned,
);

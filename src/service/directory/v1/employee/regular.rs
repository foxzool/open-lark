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
/// 更新待离职成员为在职请求
#[derive(Debug, Clone)]
pub struct RegularEmployeeRequest {
    pub api_req: ApiRequest,
    /// 员工ID
    pub employee_id: String,
    /// 用户ID类型
    pub user_id_type: Option<UserIdType>}
impl RegularEmployeeRequest {
    pub fn new(config: Config) -> Self {
        Self { config }
}/// 更新待离职成员为在职请求构建器,
#[derive(Default)]
pub struct RegularEmployeeRequestBuilder {
    request: RegularEmployeeRequest}
impl RegularEmployeeRequestBuilder {
    pub fn new(config: Config) -> Self {
        Self { config }
}// 构建空的请求体,
        let body = json!({});
self.request.api_req.body = serde_json::to_vec(&body).unwrap_or_default();
        self.request,
/// 更新待离职成员为在职响应数据,
#[derive(Debug, Clone)]
pub struct RegularEmployeeResponseData {
    /// 更新的员工信息
    pub employee: Employee,
/// 更新待离职成员为在职响应,
#[derive(Debug, Clone)]
pub struct RegularEmployeeResponse {
    /// 响应数据
    pub data: RegularEmployeeResponseData,
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
    RegularEmployeeRequestBuilder,
    EmployeeService,
    RegularEmployeeRequest,
    BaseResponse<RegularEmployeeResponse>,
    regular,
);

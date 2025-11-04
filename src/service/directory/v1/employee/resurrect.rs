#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(non_snake_case)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::module_inception)]
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
    service::directory::v1::models::{DepartmentIdType, Employee, UserIdType}
};
use super::EmployeeService;
/// 恢复离职员工请求
#[derive(Debug, Clone)]
pub struct ResurrectEmployeeRequest {
    pub api_req: ApiRequest,
    /// 员工ID
    pub employee_id: String,
    /// 上级ID
    pub leader_id: Option<String>,
    /// 部门ID列表
    pub department_ids: Option<Vec<String>>,
    /// 工作地点
    pub work_location: Option<String>,
    /// 职级
    pub job_level: Option<String>,
    /// 职位
    pub job_title: Option<String>,
    /// 用户ID类型
    pub user_id_type: Option<UserIdType>,
    /// 部门ID类型
    pub department_id_type: Option<DepartmentIdType>}
impl ResurrectEmployeeRequest {
    pub fn new(config: Config) -> Self {
        Self { config }
}/// 恢复离职员工请求构建器,
#[derive(Default)]
pub struct ResurrectEmployeeRequestBuilder {
    request: ResurrectEmployeeRequest}
impl ResurrectEmployeeRequestBuilder {
    pub fn new(config: Config) -> Self {
        Self { config }
}if let Some(department_id_type) = &self.request.department_id_type {,
            self.request,
.api_req
                .query_params
                .insert("department_id_type", department_id_type.to_string());
// 构建请求体,
        let mut body = json!({});
if let Some(ref leader_id) = self.request.leader_id {,
            body["leader_id"] = json!(leader_id);
if let Some(ref department_ids) = self.request.department_ids {,
            body["department_ids"] = json!(department_ids);
if let Some(ref work_location) = self.request.work_location {,
            body["work_location"] = json!(work_location);
if let Some(ref job_level) = self.request.job_level {,
            body["job_level"] = json!(job_level);
if let Some(ref job_title) = self.request.job_title {,
            body["job_title"] = json!(job_title);
self.request.api_req.body = serde_json::to_vec(&body).unwrap_or_default();
        self.request,
/// 恢复离职员工响应数据,
#[derive(Debug, Clone)]
pub struct ResurrectEmployeeResponseData {
    /// 恢复的员工信息
    pub employee: Employee,
/// 恢复离职员工响应,
#[derive(Debug, Clone)]
pub struct ResurrectEmployeeResponse {
    /// 响应数据
    pub data: ResurrectEmployeeResponseData,
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
    ResurrectEmployeeRequestBuilder,
    EmployeeService,
    ResurrectEmployeeRequest,
    BaseResponse<ResurrectEmployeeResponse>,
    resurrect,
);

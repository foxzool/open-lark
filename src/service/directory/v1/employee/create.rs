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
/// 创建员工请求
#[derive(Debug, Clone)]
pub struct CreateEmployeeRequest {
    pub api_req: ApiRequest,
    /// 员工工号
    pub employee_no: Option<String>,
    /// 姓名
    pub name: Option<String>,
    /// 英文名
    pub en_name: Option<String>,
    /// 邮箱
    pub email: Option<String>,
    /// 手机号
    pub mobile: Option<String>,
    /// 部门ID列表
    pub department_ids: Option<Vec<String>>,
    /// 工作地点
    pub work_location: Option<String>,
    /// 职级
    pub job_level: Option<String>,
    /// 职位
    pub job_title: Option<String>,
    /// 上级ID
    pub leader_id: Option<String>,
    /// 入职时间
    pub join_time: Option<String>,
    /// 用户ID类型
    pub user_id_type: Option<UserIdType>,
    /// 部门ID类型
    pub department_id_type: Option<DepartmentIdType>}
impl CreateEmployeeRequest {
    pub fn new(config: Config) -> Self {
        Self { config }
}/// 创建员工请求构建器,
#[derive(Default)]
pub struct CreateEmployeeRequestBuilder {
    request: CreateEmployeeRequest}
impl CreateEmployeeRequestBuilder {
    pub fn new(config: Config) -> Self {
        Self { config }
}if let Some(department_id_type) = &self.request.department_id_type {,
            self.request,
.api_req
                .query_params
                .insert("department_id_type", department_id_type.to_string());
// 构建请求体,
        let mut body = json!({});
if let Some(ref employee_no) = self.request.employee_no {,
            body["employee_no"] = json!(employee_no);
if let Some(ref name) = self.request.name {,
            body["name"] = json!(name);
if let Some(ref en_name) = self.request.en_name {,
            body["en_name"] = json!(en_name);
if let Some(ref email) = self.request.email {,
            body["email"] = json!(email);
if let Some(ref mobile) = self.request.mobile {,
            body["mobile"] = json!(mobile);
if let Some(ref department_ids) = self.request.department_ids {,
            body["department_ids"] = json!(department_ids);
if let Some(ref work_location) = self.request.work_location {,
            body["work_location"] = json!(work_location);
if let Some(ref job_level) = self.request.job_level {,
            body["job_level"] = json!(job_level);
if let Some(ref job_title) = self.request.job_title {,
            body["job_title"] = json!(job_title);
if let Some(ref leader_id) = self.request.leader_id {,
            body["leader_id"] = json!(leader_id);
if let Some(ref join_time) = self.request.join_time {,
            body["join_time"] = json!(join_time);
self.request.api_req.body = serde_json::to_vec(&body).unwrap_or_default();
        self.request,
/// 创建员工响应数据,
#[derive(Debug, Clone)]
pub struct CreateEmployeeResponseData {
    /// 创建的员工信息
    pub employee: Employee,
/// 创建员工响应,
#[derive(Debug, Clone)]
pub struct CreateEmployeeResponse {
    /// 响应数据
    pub data: CreateEmployeeResponseData,
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
    CreateEmployeeRequestBuilder,
    EmployeeService,
    CreateEmployeeRequest,
    BaseResponse<CreateEmployeeResponse>,
    create,
);

#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(non_snake_case)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::module_inception)]
use open_lark_core::SDKResult;use reqwest::Method;
use open_lark_core::api_req::ApiRequest;
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
/// 创建部门请求
#[derive(Debug, Clone)]
pub struct CreateDepartmentRequest {
    pub api_req: ApiRequest,
    /// 部门名称
    pub name: Option<String>,
    /// 英文名称
    pub en_name: Option<String>,
    /// 父部门ID
    pub parent_department_id: Option<String>,
    /// 部门主管ID
    pub leader_id: Option<String>,
    /// 部门顺序
    pub order: Option<i32>,
    /// 用户ID类型
    pub user_id_type: Option<UserIdType>,
    /// 部门ID类型
    pub department_id_type: Option<DepartmentIdType>}
impl CreateDepartmentRequest {
    pub fn new(config: Config) -> Self {
        Self { config }
}/// 创建部门请求构建器,
#[derive(Default)]
pub struct CreateDepartmentRequestBuilder {
    request: CreateDepartmentRequest}
impl CreateDepartmentRequestBuilder {
    pub fn new(config: Config) -> Self {
        Self { config }
}if let Some(department_id_type) = &self.request.department_id_type {,
            self.request,
.api_req
                .query_params
                .insert("department_id_type", department_id_type.to_string());
// 构建请求体,
        let mut body = json!({});
if let Some(ref name) = self.request.name {,
            body["name"] = json!(name);
if let Some(ref en_name) = self.request.en_name {,
            body["en_name"] = json!(en_name);
if let Some(ref parent_department_id) = self.request.parent_department_id {,
            body["parent_department_id"] = json!(parent_department_id);
if let Some(ref leader_id) = self.request.leader_id {,
            body["leader_id"] = json!(leader_id);
if let Some(order) = self.request.order {,
            body["order"] = json!(order);
self.request.api_req.body = serde_json::to_vec(&body).unwrap_or_default();
        self.request,
/// 创建部门响应数据,
#[derive(Debug, Clone)]
pub struct CreateDepartmentResponseData {
    /// 创建的部门信息
    pub department: Department,
/// 创建部门响应,
#[derive(Debug, Clone)]
pub struct CreateDepartmentResponse {
    /// 响应数据
    pub data: CreateDepartmentResponseData,
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
    CreateDepartmentRequestBuilder,
    DepartmentService,
    CreateDepartmentRequest,
    BaseResponse<CreateDepartmentResponse>,
    create,
);

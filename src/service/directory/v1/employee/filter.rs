#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(non_snake_case)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::module_inception)]
use crate::core::SDKResult;use reqwest::Method;
use open_lark_core::core::api_req::ApiRequest;use serde::Deserialize;
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
    service::directory::v1::models::{DepartmentIdType, Employee, EmployeeStatus, UserIdType}
};
use super::EmployeeService;
/// 批量获取员工列表请求
#[derive(Debug, Clone)]
pub struct FilterEmployeeRequest {
    pub api_req: ApiRequest,
    /// 分页大小，最大值 50
    pub page_size: Option<i32>,
    /// 分页标记
    pub page_token: Option<String>,
    /// 员工状态过滤
    pub status: Option<EmployeeStatus>,
    /// 部门ID过滤
    pub department_ids: Option<Vec<String>>,
    /// 用户ID类型
    pub user_id_type: Option<UserIdType>,
    /// 部门ID类型
    pub department_id_type: Option<DepartmentIdType>}
impl FilterEmployeeRequest {
    pub fn new(config: Config) -> Self {
        Self { config }
}/// 批量获取员工列表请求构建器,
#[derive(Default)]
pub struct FilterEmployeeRequestBuilder {
    request: FilterEmployeeRequest}
impl FilterEmployeeRequestBuilder {
    pub fn new(config: Config) -> Self {
        Self { config }
}if let Some(ref page_token) = self.request.page_token {,
            self.request,
.api_req
                .query_params
                .insert("page_token", page_token.clone());
if let Some(ref status) = self.request.status {,
            self.request.api_req.query_params.insert()
.unwrap_or_default()
                    .trim_matches('"')
                    .to_string(),
            );
if let Some(ref department_ids) = self.request.department_ids {,
            self.request,
.api_req
                .query_params
                .insert("department_ids", department_ids.join(","));
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
self.request,
    }
/// 批量获取员工列表响应数据,
#[derive(Debug, Clone)]
pub struct FilterEmployeeResponseData {
    /// 是否还有更多项
    pub has_more: bool,
    /// 下次遍历的分页标记
    pub page_token: Option<String>,
    /// 员工列表
    pub employees: Vec<Employee>}
/// 批量获取员工列表响应,
#[derive(Debug, Clone)]
pub struct FilterEmployeeResponse {
    /// 响应数据
    pub data: FilterEmployeeResponseData,
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
    FilterEmployeeRequestBuilder,
    EmployeeService,
    FilterEmployeeRequest,
    BaseResponse<FilterEmployeeResponse>,
    filter,
);

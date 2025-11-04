#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(non_snake_case)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::module_inception)]
use crate::core::SDKResult;use reqwest::Method;
use open_lark_core::core::api_req::ApiRequest;
use serde::Deserialize;
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
    service::directory::v1::models::{Department, DepartmentIdType, UserIdType}
};
use super::DepartmentService;
/// 批量获取部门列表请求
#[derive(Debug, Clone)]
pub struct FilterDepartmentRequest {
    pub api_req: ApiRequest,
    /// 分页大小，最大值为50
    pub page_size: Option<i32>,
    /// 分页标记
    pub page_token: Option<String>,
    /// 父部门ID，获取指定父部门下的子部门列表
    pub parent_department_id: Option<String>,
    /// 部门状态过滤
    pub fetch_deleted: Option<bool>,
    /// 用户ID类型
    pub user_id_type: Option<UserIdType>,
    /// 部门ID类型
    pub department_id_type: Option<DepartmentIdType>}
impl FilterDepartmentRequest {
    pub fn new(config: Config) -> Self {
        Self { config }
}/// 批量获取部门列表请求构建器,
#[derive(Default)]
pub struct FilterDepartmentRequestBuilder {
    request: FilterDepartmentRequest}
impl FilterDepartmentRequestBuilder {
    pub fn new(config: Config) -> Self {
        Self { config }
}if let Some(ref page_token) = self.request.page_token {,
            self.request,
.api_req
                .query_params
                .insert("page_token", page_token.clone());
if let Some(ref parent_department_id) = self.request.parent_department_id {,
            self.request,
.api_req
                .query_params
                .insert("parent_department_id", parent_department_id.clone());
if let Some(fetch_deleted) = self.request.fetch_deleted {,
            self.request,
.api_req
                .query_params
                .insert("fetch_deleted", fetch_deleted.to_string());
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
/// 批量获取部门列表响应数据,
#[derive(Debug, Clone)]
pub struct FilterDepartmentResponseData {
    /// 部门信息列表
    pub departments: Vec<Department>,
    /// 下一页分页标记
    pub page_token: Option<String>,
    /// 是否还有更多数据
    pub has_more: Option<bool>}
/// 批量获取部门列表响应,
#[derive(Debug, Clone)]
pub struct FilterDepartmentResponse {
    /// 响应数据
    pub data: FilterDepartmentResponseData,
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
    FilterDepartmentRequestBuilder,
    DepartmentService,
    FilterDepartmentRequest,
    BaseResponse<FilterDepartmentResponse>,
    filter,
);

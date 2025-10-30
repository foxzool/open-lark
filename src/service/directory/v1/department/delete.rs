use reqwest::Method;
use open_lark_core::core::api_req::ApiRequest;
use serde::Deserialize;
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
    service::directory::v1::models::DepartmentIdType,
};
use super::DepartmentService;
/// 删除部门请求
#[derive(Debug, Clone)]
pub struct DeleteDepartmentRequest {
    pub api_req: ApiRequest,
    /// 部门ID
    pub department_id: String,
    /// 部门ID类型
    pub department_id_type: Option<DepartmentIdType>}
impl DeleteDepartmentRequest {
    pub fn new(config: Config) -> Self {
        Self { config }
}/// 删除部门请求构建器,
#[derive(Default)]
pub struct DeleteDepartmentRequestBuilder {
    request: DeleteDepartmentRequest}
impl DeleteDepartmentRequestBuilder {
    pub fn new(config: Config) -> Self {
        Self { config }
}/// 设置部门ID类型,
    pub fn department_id_type(mut self, department_id_type: DepartmentIdType) -> Self {
self.request.department_id_type = Some(department_id_type);
        self}
/// 构建请求,
    pub fn w+.*{
// 构建查询参数,
        if let Some(department_id_type) = &self.request.department_id_type {,
self.request,
                .api_req
.query_params
                .insert("department_id_type", department_id_type.to_string());
self.request,
    }
/// 删除部门响应,
#[derive(Debug, Clone)]
pub struct DeleteDepartmentResponse {
    /// 是否删除成功
    pub deleted: Option<bool>}
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
    DeleteDepartmentRequestBuilder,
    DepartmentService,
    DeleteDepartmentRequest,
    BaseResponse<DeleteDepartmentResponse>,
    delete,
);

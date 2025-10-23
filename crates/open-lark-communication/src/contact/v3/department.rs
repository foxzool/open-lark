use open_lark_core::core::{
use serde_json;    constants::AccessTokenType, http::Transport,
    api_req::ApiRequest, api_resp::ApiResponseTrait, config::Config,
    constants::AccessTokenType, endpoints::EndpointBuilder, http::Transport,
},
use crate::contact::models::*;
use serde::{Deserialize, Serialize};

/// 部门管理服务
///
/// 提供完整的部门管理功能，包括：
/// - 创建、修改、删除部门
/// - 获取部门信息（单个/批量）
/// - 获取子部门列表
/// - 获取父部门信息
/// - 搜索部门
pub struct DepartmentService {
    config: Config,
}
impl DepartmentService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }
    /// 获取客户端配置
    ///
    /// # 返回值
    /// 配置对象的引用
    pub fn config(&self) -> &Config {
        &self.config
    /// 创建部门
    /// 该接口用于创建新的部门。
    pub async fn create(
        &self,
        req: &CreateDepartmentRequest,
    ) -> open_lark_core::core::SDKResult<CreateDepartmentResponse> {
        let mut api_req = ApiRequest::default();
        api_req.set_http_method(reqwest::Method::POST);
        api_req.set_api_path(open_lark_core::core::endpoints::contact::CONTACT_V3_DEPARTMENTS.to_string());
        api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant]);
        api_req.body = serde_json::to_vec(req)?;
        let resp =
            Transport::<CreateDepartmentResponse>::request(api_req, &self.config, None).await?;
        Ok(resp.data.unwrap_or_default())
    /// 修改部门部分信息
    /// 该接口用于修改部门的部分信息。
    pub async fn patch(
        department_id: &str,
        req: &PatchDepartmentRequest,
    ) -> open_lark_core::core::SDKResult<PatchDepartmentResponse> {
        api_req.set_http_method(reqwest::Method::PATCH);
        api_req.set_api_path(EndpointBuilder::replace_param(
            open_lark_core::core::endpoints::contact::CONTACT_V3_DEPARTMENT_GET,
            "department_id",
            department_id,
        ));
            Transport::<PatchDepartmentResponse>::request(api_req, &self.config, None).await?;
    /// 更新部门所有信息
    /// 该接口用于更新部门的所有信息。
    pub async fn update(
        req: &UpdateDepartmentRequest,
    ) -> open_lark_core::core::SDKResult<UpdateDepartmentResponse> {
        api_req.set_http_method(reqwest::Method::PUT);
            Transport::<UpdateDepartmentResponse>::request(api_req, &self.config, None).await?;
    /// 更新部门 ID
    /// 该接口用于更新部门的部门ID。
    pub async fn update_department_id(
        req: &UpdateDepartmentIdRequest,
    ) -> open_lark_core::core::SDKResult<UpdateDepartmentIdResponse> {
            open_lark_core::core::endpoints::contact::CONTACT_V3_DEPARTMENT_UPDATE_ID,
            Transport::<UpdateDepartmentIdResponse>::request(api_req, &self.config, None).await?;
    /// 获取单个部门信息
    /// 该接口用于获取单个部门的详细信息。
    pub async fn get(
        _req: &GetDepartmentRequest,
    ) -> open_lark_core::core::SDKResult<GetDepartmentResponse> {
        api_req.set_http_method(reqwest::Method::GET);
        api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant, AccessTokenType::User]);
        api_req.body = Vec::new();
        api_req.query_params = std::collections::HashMap::new();
        let resp = Transport::<GetDepartmentResponse>::request(api_req, &self.config, None).await?;
    /// 批量获取部门信息
    pub async fn batch(
        req: &BatchGetDepartmentsRequest,
    ) -> open_lark_core::core::SDKResult<BatchGetDepartmentsResponse> {
        api_req.set_api_path(open_lark_core::core::endpoints::contact::CONTACT_V3_DEPARTMENTS_BATCH.to_string());
            Transport::<BatchGetDepartmentsResponse>::request(api_req, &self.config, None).await?;
    /// 获取子部门列表
    pub async fn children(
        _req: &GetChildrenDepartmentsRequest,
    ) -> open_lark_core::core::SDKResult<GetChildrenDepartmentsResponse> {
        api_req.set_api_path(open_lark_core::core::endpoints::contact::CONTACT_V3_DEPARTMENTS_CHILDREN.to_string());
            Transport::<GetChildrenDepartmentsResponse>::request(api_req, &self.config, None)
                .await?;
    /// 获取父部门信息
    pub async fn parent(
        _req: &GetParentDepartmentRequest,
    ) -> open_lark_core::core::SDKResult<GetParentDepartmentResponse> {
        api_req.set_api_path(open_lark_core::core::endpoints::contact::CONTACT_V3_DEPARTMENTS_PARENT.to_string());
            Transport::<GetParentDepartmentResponse>::request(api_req, &self.config, None).await?;
    /// 搜索部门
    pub async fn search(
        req: &SearchDepartmentsRequest,
    ) -> open_lark_core::core::SDKResult<SearchDepartmentsResponse> {
        api_req.set_api_path(open_lark_core::core::endpoints::contact::CONTACT_V3_DEPARTMENTS_SEARCH.to_string());
            Transport::<SearchDepartmentsResponse>::request(api_req, &self.config, None).await?;
    /// 删除部门
    pub async fn delete(
        _req: &DeleteDepartmentRequest,
    ) -> open_lark_core::core::SDKResult<DeleteDepartmentResponse> {
        api_req.set_http_method(reqwest::Method::DELETE);
            Transport::<DeleteDepartmentResponse>::request(api_req, &self.config, None).await?;
// 请求/响应结构体定义
/// 创建部门请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateDepartmentRequest {
    /// 部门信息
    pub department: Department,
    /// 用户 ID 类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id_type: Option<String>,
    /// 部门 ID 类型
    pub department_id_type: Option<String>,
    /// 客户端令牌
    pub client_token: Option<String>,
/// 创建部门响应
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreateDepartmentResponse {
impl ApiResponseTrait for CreateDepartmentResponse {
    fn data_format() -> open_lark_core::core::api_resp::ResponseFormat {
        open_lark_core::core::api_resp::ResponseFormat::Data
/// 修改部门请求
pub struct PatchDepartmentRequest {
/// 修改部门响应
pub struct PatchDepartmentResponse {
impl ApiResponseTrait for PatchDepartmentResponse {
/// 更新部门请求
pub struct UpdateDepartmentRequest {
/// 更新部门响应
pub struct UpdateDepartmentResponse {
impl ApiResponseTrait for UpdateDepartmentResponse {
/// 更新部门ID请求
pub struct UpdateDepartmentIdRequest {
    /// 新的部门ID
    pub new_department_id: String,
/// 更新部门ID响应
pub struct UpdateDepartmentIdResponse {}
impl ApiResponseTrait for UpdateDepartmentIdResponse {
/// 获取部门请求
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetDepartmentRequest {
/// 获取部门响应
pub struct GetDepartmentResponse {
impl ApiResponseTrait for GetDepartmentResponse {
/// 批量获取部门请求
pub struct BatchGetDepartmentsRequest {
    /// 部门ID列表
    pub department_ids: Vec<String>,
/// 批量获取部门响应
pub struct BatchGetDepartmentsResponse {
    /// 部门列表
    pub items: Vec<Department>,
impl ApiResponseTrait for BatchGetDepartmentsResponse {
/// 获取子部门列表请求
pub struct GetChildrenDepartmentsRequest {
    /// 父部门ID
    pub parent_department_id: Option<String>,
    /// 是否递归获取
    pub fetch_child: Option<bool>,
    /// 分页大小
    pub page_size: Option<i32>,
    /// 分页标记
    pub page_token: Option<String>,
/// 获取子部门列表响应
pub struct GetChildrenDepartmentsResponse {
    /// 是否还有更多项目
    pub has_more: Option<bool>,
impl ApiResponseTrait for GetChildrenDepartmentsResponse {
/// 获取父部门请求
pub struct GetParentDepartmentRequest {
    /// 部门ID
    pub department_id: Option<String>,
/// 获取父部门响应
pub struct GetParentDepartmentResponse {
impl ApiResponseTrait for GetParentDepartmentResponse {
/// 搜索部门请求
pub struct SearchDepartmentsRequest {
    /// 搜索关键词
    pub query: String,
/// 搜索部门响应
pub struct SearchDepartmentsResponse {
impl ApiResponseTrait for SearchDepartmentsResponse {
/// 删除部门请求
pub struct DeleteDepartmentRequest {
/// 删除部门响应
pub struct DeleteDepartmentResponse {}
impl ApiResponseTrait for DeleteDepartmentResponse {
#[cfg(test)]
mod tests {
    use super::*;
    use open_lark_core::core::config::Config;
    use crate::contact::models::Department;
    #[test]
    fn test_department_service_creation() {
        let config = Config::default();
        let service = DepartmentService::new(config.clone());
        assert_eq!(service.config.app_id, config.app_id);
        assert_eq!(service.config.app_secret, config.app_secret);
    fn test_department_service_with_custom_config() {
        let config = Config::builder()
            .app_id("dept_test_app")
            .app_secret("dept_test_secret")
            .build();
        assert_eq!(service.config.app_id, "dept_test_app");
        assert_eq!(service.config.app_secret, "dept_test_secret");
    fn test_create_department_request_construction() {
        let department = Department {
            department_id: Some("dept_123".to_string()),
            name: Some("Engineering".to_string()),
            ..Default::default()
        },

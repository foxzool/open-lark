use open_lark_core::core::{
use serde_json;    constants::AccessTokenType, http::Transport,
    api_req::ApiRequest, api_resp::ApiResponseTrait, config::Config, constants::AccessTokenType,
    endpoints::EndpointBuilder, http::Transport,
},
use serde::{Deserialize, Serialize },

/// 角色管理服务
#[derive(Debug)]
pub struct FunctionalRoleService {
    config: Config,
}
impl FunctionalRoleService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }
    /// 创建角色
    pub async fn create(
        &self,
        req: &CreateFunctionalRoleRequest,
    ) -> open_lark_core::core::SDKResult<CreateFunctionalRoleResponse> {
        let mut api_req = ApiRequest::default();
        api_req.set_http_method(reqwest::Method::POST);
        api_req.set_api_path(open_lark_core::core::endpoints::contact::CONTACT_V3_FUNCTIONAL_ROLES.to_string());
        api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant]);
        api_req.body = serde_json::to_vec(req)?;
        let resp =
            Transport::<CreateFunctionalRoleResponse>::request(api_req, &self.config, None).await?;
        Ok(resp.data.unwrap_or_default())
    /// 修改角色名称
    pub async fn update(
        role_id: &str,
        req: &UpdateFunctionalRoleRequest,
    ) -> open_lark_core::core::SDKResult<UpdateFunctionalRoleResponse> {
        api_req.set_http_method(reqwest::Method::PUT);
        api_req.set_api_path(EndpointBuilder::replace_param(
                open_lark_core::core::endpoints::contact::CONTACT_V3_FUNCTIONAL_ROLE_GET,
                "role_id",
                role_id,
            ));
            Transport::<UpdateFunctionalRoleResponse>::request(api_req, &self.config, None).await?;
    /// # API文档
    ///
    /// https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/contact/get
    /// 获取单个角色信息
    pub async fn get(&self, role_id: &str) -> open_lark_core::core::SDKResult<GetFunctionalRoleResponse> {
        api_req.set_http_method(reqwest::Method::GET);
        api_req.body = Vec::new();
            Transport::<GetFunctionalRoleResponse>::request(api_req, &self.config, None).await?;
    /// 获取角色列表
    pub async fn list(
        req: &ListFunctionalRolesRequest,
    ) -> open_lark_core::core::SDKResult<ListFunctionalRolesResponse> {
        api_req.query_params = std::collections::HashMap::new();
        // 添加查询参数
        if let Some(page_size) = req.page_size {
            api_req
                .query_params
                .insert("page_size", page_size.to_string());
        }
        if let Some(page_token) = &req.page_token {
                .insert("page_token", page_token.clone());
            Transport::<ListFunctionalRolesResponse>::request(api_req, &self.config, None).await?;
    /// 删除角色
    pub async fn delete(
    ) -> open_lark_core::core::SDKResult<DeleteFunctionalRoleResponse> {
        api_req.set_http_method(reqwest::Method::DELETE);
            Transport::<DeleteFunctionalRoleResponse>::request(api_req, &self.config, None).await?;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateFunctionalRoleRequest {
    pub role_name: String,
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreateFunctionalRoleResponse {
    pub role_id: String,
impl ApiResponseTrait for CreateFunctionalRoleResponse {
    fn data_format() -> open_lark_core::core::api_resp::ResponseFormat {
        open_lark_core::core::api_resp::ResponseFormat::Data
pub struct UpdateFunctionalRoleRequest {
pub struct UpdateFunctionalRoleResponse {}
impl ApiResponseTrait for UpdateFunctionalRoleResponse {
pub struct GetFunctionalRoleResponse {
    pub role: FunctionalRole,
impl ApiResponseTrait for GetFunctionalRoleResponse {
pub struct ListFunctionalRolesRequest {
    /// 分页大小
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 分页标记
    pub page_token: Option<String>,
pub struct ListFunctionalRolesResponse {
    pub roles: Vec<FunctionalRole>,
    pub has_more: Option<bool>,
impl ApiResponseTrait for ListFunctionalRolesResponse {
pub struct FunctionalRole {
    /// 角色ID
    pub role_id: Option<String>,
    /// 角色名称
    pub role_name: Option<String>,
pub struct DeleteFunctionalRoleResponse {}
impl ApiResponseTrait for DeleteFunctionalRoleResponse {
#[cfg(test)]
mod tests {
    use super::*;
    use open_lark_core::core::config::Config;
    #[test]
    fn test_functional_role_service_creation() {
        let config = Config::default();
        let service = FunctionalRoleService::new(config);
        assert!(!format!("{:?}", service).is_empty());
    fn test_functional_role_service_creation_with_custom_config() {
        let config = Config::builder()
            .app_id("test_app_id")
            .app_secret("test_secret")
            .build();
    fn test_create_functional_role_request_construction() {
        let request = CreateFunctionalRoleRequest {
            role_name: "测试角色".to_string(),
        },
        assert_eq!(request.role_name, "测试角色");
        assert!(format!("{:?}", request).contains("测试角色"));

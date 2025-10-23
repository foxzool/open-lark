use open_lark_core::core::{
use serde_json;    constants::AccessTokenType, http::Transport,
    api_req::ApiRequest, api_resp::ApiResponseTrait, config::Config,
    constants::AccessTokenType, http::Transport, trait_system::Service,
},
use crate::contact::models::*;
use serde::{Deserialize, Serialize};

/// 用户组管理服务
pub struct GroupService {
    config: Config,
}
impl GroupService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }
    /// 创建用户组
    pub async fn create(
        &self,
        req: &CreateGroupRequest,
    ) -> open_lark_core::core::SDKResult<CreateGroupResponse> {
        let mut api_req = ApiRequest::default();
        api_req.set_http_method(reqwest::Method::POST);
        api_req.set_api_path(open_lark_core::core::endpoints::contact::CONTACT_V3_GROUPS.to_string());
        api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant]);
        api_req.body = serde_json::to_vec(req)?;
        let resp = Transport::<CreateGroupResponse>::request(api_req, &self.config, None).await?;
        Ok(resp.data.unwrap_or_default())
    /// 更新用户组
    pub async fn patch(
        group_id: &str,
        req: &PatchGroupRequest,
    ) -> open_lark_core::core::SDKResult<PatchGroupResponse> {
        api_req.set_http_method(reqwest::Method::PATCH);
        let resp = Transport::<PatchGroupResponse>::request(api_req, &self.config, None).await?;
    /// 查询指定用户组
    pub async fn get(
        _req: &GetGroupRequest,
    ) -> open_lark_core::core::SDKResult<GetGroupResponse> {
        api_req.set_http_method(reqwest::Method::GET);
        api_req.body = Vec::new();
        api_req.query_params = std::collections::HashMap::new();
        let resp = Transport::<GetGroupResponse>::request(api_req, &self.config, None).await?;
    /// 查询用户组列表
    pub async fn simplelist(
        _req: &ListGroupsRequest,
    ) -> open_lark_core::core::SDKResult<ListGroupsResponse> {
        api_req.set_api_path(open_lark_core::core::endpoints::contact::CONTACT_V3_GROUPS_SIMPLELIST.to_string());
        let resp = Transport::<ListGroupsResponse>::request(api_req, &self.config, None).await?;
    /// 查询用户所属用户组
    pub async fn member_belong(
        _req: &GetUserGroupsRequest,
    ) -> open_lark_core::core::SDKResult<GetUserGroupsResponse> {
        api_req.set_api_path(open_lark_core::core::endpoints::contact::CONTACT_V3_GROUPS_MEMBER_BELONG.to_string());
        let resp = Transport::<GetUserGroupsResponse>::request(api_req, &self.config, None).await?;
    /// # API文档
    ///
    /// https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/contact/delete
    /// 删除用户组
    pub async fn delete(&self, group_id: &str) -> open_lark_core::core::SDKResult<DeleteGroupResponse> {
        api_req.set_http_method(reqwest::Method::DELETE);
        let resp = Transport::<DeleteGroupResponse>::request(api_req, &self.config, None).await?;
    /// 获取用户组详细信息
    pub async fn get_detail(
        req: &GetGroupDetailRequest,
    ) -> open_lark_core::core::SDKResult<GetGroupDetailResponse> {
        let mut query_params = std::collections::HashMap::new();
        if let Some(user_id_type) = &req.user_id_type {
            query_params.insert("user_id_type", user_id_type.clone());

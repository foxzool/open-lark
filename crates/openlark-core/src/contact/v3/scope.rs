use crate::{
    api::ApiRequest, api::ApiResponseTrait, config::Config,
    constants::AccessTokenType, http::Transport,
};
use serde::{Deserialize, Serialize};

/// 权限范围服务
///
/// 用于管理通讯录的访问权限范围，包括：
/// - 获取通讯录授权范围
/// - 权限范围变更事件处理
#[derive(Debug)]
pub struct ScopeService {
    config: Config,
}

impl ScopeService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 获取通讯录授权范围
    ///
    /// 获取应用在通讯录中的授权范围，包括可访问的部门、用户和用户组列表。
    /// 用于了解当前应用能够访问的通讯录资源范围。
    /// # API文档
    /// https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/contact-v3/scope/listscope/listscope/list
    pub async fn list(&self, _req: &GetScopeRequest) -> crate::SDKResult<GetScopeResponse> {
        let mut api_req = ApiRequest::default();
        api_req.set_method(reqwest::Method::GET);
        api_req.set_api_path(crate::endpoints::contact::CONTACT_V3_SCOPES.to_string());
        api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant]);

        let resp = Transport::<GetScopeResponse>::request(api_req, &self.config, None).await?;
        Ok(resp.data.unwrap_or_default())
    }

    /// 获取通讯录授权范围详情
    /// 获取通讯录授权范围的详细信息，包括权限范围的具体配置和约束条件。
    /// https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/contact-v3/scope/listscope/listscope/get
    pub async fn get_authority(
        &self,
        req: &GetScopeAuthorityRequest,
    ) -> crate::SDKResult<GetScopeAuthorityResponse> {
        let mut api_req = ApiRequest::default();
        api_req.set_method(reqwest::Method::GET);
        api_req.set_api_path(crate::endpoints::contact::CONTACT_V3_SCOPES.to_string());
        api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant]);

        let mut params = std::collections::HashMap::new();
        if let Some(user_id_type) = &req.user_id_type {
            params.insert("user_id_type".to_string(), user_id_type.clone());
        }
        api_req.set_query_params(params);

        let resp = Transport::<GetScopeAuthorityResponse>::request(api_req, &self.config, None).await?;
        Ok(resp.data.unwrap_or_default())
    }
}

impl crate::trait_system::Service for ScopeService {
    fn config(&self) -> &Config {
        &self.config
    }

    fn service_name() -> &'static str {
        "scope"
    }

    fn service_version() -> &'static str {
        "v3"
    }
}
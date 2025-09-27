use crate::core::{
    api_req::ApiRequest, api_resp::ApiResponseTrait, config::Config, constants::AccessTokenType,
    http::Transport, trait_system::Service,
};
use serde::{Deserialize, Serialize};

/// 权限范围服务
///
/// 用于管理通讯录的访问权限范围，包括：
/// - 获取通讯录授权范围
/// - 权限范围变更事件处理
pub struct ScopeService {
    config: Config,
}

impl ScopeService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 获取通讯录授权范围
    pub async fn list(&self, _req: &GetScopeRequest) -> crate::core::SDKResult<GetScopeResponse> {
        let api_req = ApiRequest {
            http_method: reqwest::Method::GET,
            api_path: crate::core::endpoints::contact::CONTACT_V3_SCOPES.to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant],
            body: Vec::new(),
            query_params: std::collections::HashMap::new(),
            ..Default::default()
        };

        let resp = Transport::<GetScopeResponse>::request(api_req, &self.config, None).await?;
        Ok(resp.data.unwrap_or_default())
    }
}

impl Service for ScopeService {
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

/// 获取权限范围请求
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetScopeRequest {
    /// 用户 ID 类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id_type: Option<String>,
    /// 部门 ID 类型  
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department_id_type: Option<String>,
    /// 分页大小
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 分页标记
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

/// 获取权限范围响应
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GetScopeResponse {
    /// 权限范围列表
    pub scopes: Vec<Scope>,
    /// 是否还有更多项目
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
    /// 分页标记
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

impl ApiResponseTrait for GetScopeResponse {
    fn data_format() -> crate::core::api_resp::ResponseFormat {
        crate::core::api_resp::ResponseFormat::Data
    }
}

/// 权限范围
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Scope {
    /// 部门列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub departments: Option<Vec<ScopeDepartment>>,
    /// 用户列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub users: Option<Vec<ScopeUser>>,
    /// 用户组列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub groups: Option<Vec<ScopeGroup>>,
}

/// 权限范围内的部门
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScopeDepartment {
    /// 部门ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department_id: Option<String>,
    /// 部门名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// 权限范围内的用户
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScopeUser {
    /// 用户ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    /// 用户名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// 权限范围内的用户组
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScopeGroup {
    /// 用户组ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
    /// 用户组名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

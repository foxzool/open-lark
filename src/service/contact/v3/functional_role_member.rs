use crate::core::{
    api_req::ApiRequest, api_resp::ApiResponseTrait, config::Config, constants::AccessTokenType,
    endpoints::EndpointBuilder, http::Transport,
};
use serde::{Deserialize, Serialize};

/// 角色成员服务
pub struct FunctionalRoleMemberService {
    config: Config,
}

impl FunctionalRoleMemberService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 添加角色成员
    pub async fn create(
        &self,
        role_id: &str,
        req: &CreateRoleMemberRequest,
    ) -> crate::core::SDKResult<CreateRoleMemberResponse> {
        let api_req = ApiRequest {
            http_method: reqwest::Method::POST,
            api_path: EndpointBuilder::replace_param(
                crate::core::endpoints::contact::CONTACT_V3_FUNCTIONAL_ROLE_MEMBERS,
                "role_id",
                role_id,
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant],
            body: serde_json::to_vec(req)?,
            ..Default::default()
        };

        let resp =
            Transport::<CreateRoleMemberResponse>::request(api_req, &self.config, None).await?;
        Ok(resp.data.unwrap_or_default())
    }

    /// 批量添加角色成员
    pub async fn batch_create(
        &self,
        role_id: &str,
        req: &BatchCreateRoleMembersRequest,
    ) -> crate::core::SDKResult<BatchCreateRoleMembersResponse> {
        let api_req = ApiRequest {
            http_method: reqwest::Method::POST,
            api_path: EndpointBuilder::replace_param(
                crate::core::endpoints::contact::CONTACT_V3_FUNCTIONAL_ROLE_MEMBERS_BATCH_CREATE,
                "role_id",
                role_id,
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant],
            body: serde_json::to_vec(req)?,
            ..Default::default()
        };

        let resp =
            Transport::<BatchCreateRoleMembersResponse>::request(api_req, &self.config, None)
                .await?;
        Ok(resp.data.unwrap_or_default())
    }

    /// 批量设置角色成员管理范围
    pub async fn scopes(
        &self,
        role_id: &str,
        req: &SetRoleMemberScopesRequest,
    ) -> crate::core::SDKResult<SetRoleMemberScopesResponse> {
        let api_req = ApiRequest {
            http_method: reqwest::Method::POST,
            api_path: EndpointBuilder::replace_param(
                crate::core::endpoints::contact::CONTACT_V3_FUNCTIONAL_ROLE_MEMBERS_SCOPES,
                "role_id",
                role_id,
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant],
            body: serde_json::to_vec(req)?,
            ..Default::default()
        };

        let resp =
            Transport::<SetRoleMemberScopesResponse>::request(api_req, &self.config, None).await?;
        Ok(resp.data.unwrap_or_default())
    }

    /// 查询角色下某个成员的管理范围
    pub async fn get(
        &self,
        role_id: &str,
        member_id: &str,
        _req: &GetRoleMemberRequest,
    ) -> crate::core::SDKResult<GetRoleMemberResponse> {
        let api_req = ApiRequest {
            http_method: reqwest::Method::GET,
            api_path: {
                let path = EndpointBuilder::replace_param(
                    crate::core::endpoints::contact::CONTACT_V3_FUNCTIONAL_ROLE_MEMBER_GET,
                    "role_id",
                    role_id,
                );
                EndpointBuilder::replace_param(&path, "member_id", member_id)
            },
            supported_access_token_types: vec![AccessTokenType::Tenant],
            body: Vec::new(),
            query_params: std::collections::HashMap::new(),
            ..Default::default()
        };

        let resp = Transport::<GetRoleMemberResponse>::request(api_req, &self.config, None).await?;
        Ok(resp.data.unwrap_or_default())
    }

    /// 查询角色下的所有成员信息
    pub async fn list(
        &self,
        role_id: &str,
        _req: &ListRoleMembersRequest,
    ) -> crate::core::SDKResult<ListRoleMembersResponse> {
        let api_req = ApiRequest {
            http_method: reqwest::Method::GET,
            api_path: EndpointBuilder::replace_param(
                crate::core::endpoints::contact::CONTACT_V3_FUNCTIONAL_ROLE_MEMBERS,
                "role_id",
                role_id,
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant],
            body: Vec::new(),
            query_params: std::collections::HashMap::new(),
            ..Default::default()
        };

        let resp =
            Transport::<ListRoleMembersResponse>::request(api_req, &self.config, None).await?;
        Ok(resp.data.unwrap_or_default())
    }

    /// 删除角色下的成员
    pub async fn batch_delete(
        &self,
        role_id: &str,
        req: &BatchDeleteRoleMembersRequest,
    ) -> crate::core::SDKResult<BatchDeleteRoleMembersResponse> {
        let api_req = ApiRequest {
            http_method: reqwest::Method::POST,
            api_path: EndpointBuilder::replace_param(
                crate::core::endpoints::contact::CONTACT_V3_FUNCTIONAL_ROLE_MEMBERS_BATCH_DELETE,
                "role_id",
                role_id,
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant],
            body: serde_json::to_vec(req)?,
            ..Default::default()
        };

        let resp =
            Transport::<BatchDeleteRoleMembersResponse>::request(api_req, &self.config, None)
                .await?;
        Ok(resp.data.unwrap_or_default())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateRoleMemberRequest {
    /// 成员信息
    pub member: RoleMemberInfo,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department_id_type: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreateRoleMemberResponse {
    pub member_id: String,
}

impl ApiResponseTrait for CreateRoleMemberResponse {
    fn data_format() -> crate::core::api_resp::ResponseFormat {
        crate::core::api_resp::ResponseFormat::Data
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchCreateRoleMembersRequest {
    pub members: Vec<RoleMemberInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department_id_type: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BatchCreateRoleMembersResponse {
    pub results: Vec<RoleMemberResult>,
}

impl ApiResponseTrait for BatchCreateRoleMembersResponse {
    fn data_format() -> crate::core::api_resp::ResponseFormat {
        crate::core::api_resp::ResponseFormat::Data
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetRoleMemberScopesRequest {
    pub members: Vec<RoleMemberScope>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department_id_type: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SetRoleMemberScopesResponse {}

impl ApiResponseTrait for SetRoleMemberScopesResponse {
    fn data_format() -> crate::core::api_resp::ResponseFormat {
        crate::core::api_resp::ResponseFormat::Data
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetRoleMemberRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department_id_type: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GetRoleMemberResponse {
    pub member: RoleMember,
}

impl ApiResponseTrait for GetRoleMemberResponse {
    fn data_format() -> crate::core::api_resp::ResponseFormat {
        crate::core::api_resp::ResponseFormat::Data
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListRoleMembersRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department_id_type: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ListRoleMembersResponse {
    pub members: Vec<RoleMember>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

impl ApiResponseTrait for ListRoleMembersResponse {
    fn data_format() -> crate::core::api_resp::ResponseFormat {
        crate::core::api_resp::ResponseFormat::Data
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchDeleteRoleMembersRequest {
    pub member_ids: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id_type: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BatchDeleteRoleMembersResponse {
    pub results: Vec<RoleMemberResult>,
}

impl ApiResponseTrait for BatchDeleteRoleMembersResponse {
    fn data_format() -> crate::core::api_resp::ResponseFormat {
        crate::core::api_resp::ResponseFormat::Data
    }
}

// 公共数据结构

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RoleMemberInfo {
    /// 成员ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_id: Option<String>,
    /// 成员类型 (user/department)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_type: Option<String>,
    /// 管理范围
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RoleMember {
    /// 成员ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_id: Option<String>,
    /// 成员类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_type: Option<String>,
    /// 成员信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_info: Option<RoleMemberDetail>,
    /// 管理范围
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scopes: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RoleMemberDetail {
    /// 成员名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 成员邮箱
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoleMemberScope {
    /// 成员ID
    pub member_id: String,
    /// 管理范围列表
    pub scopes: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoleMemberResult {
    /// 成员ID
    pub member_id: String,
    /// 操作是否成功
    pub success: bool,
    /// 错误信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

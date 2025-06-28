use crate::{
    core::{
        api_req::ApiRequest, api_resp::ApiResponseTrait, config::Config,
        constants::AccessTokenType, http::Transport,
    },
    service::contact::models::*,
};
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
    ) -> crate::core::SDKResult<CreateGroupResponse> {
        let api_req = ApiRequest {
            http_method: reqwest::Method::POST,
            api_path: "/open-apis/contact/v3/groups".to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant],
            body: serde_json::to_vec(req)?,
            ..Default::default()
        };

        let resp = Transport::<CreateGroupResponse>::request(api_req, &self.config, None).await?;
        Ok(resp.data.unwrap_or_default())
    }

    /// 更新用户组
    pub async fn patch(
        &self,
        group_id: &str,
        req: &PatchGroupRequest,
    ) -> crate::core::SDKResult<PatchGroupResponse> {
        let api_req = ApiRequest {
            http_method: reqwest::Method::PATCH,
            api_path: format!("/open-apis/contact/v3/groups/{}", group_id),
            supported_access_token_types: vec![AccessTokenType::Tenant],
            body: serde_json::to_vec(req)?,
            ..Default::default()
        };

        let resp = Transport::<PatchGroupResponse>::request(api_req, &self.config, None).await?;
        Ok(resp.data.unwrap_or_default())
    }

    /// 查询指定用户组
    pub async fn get(
        &self,
        group_id: &str,
        _req: &GetGroupRequest,
    ) -> crate::core::SDKResult<GetGroupResponse> {
        let api_req = ApiRequest {
            http_method: reqwest::Method::GET,
            api_path: format!("/open-apis/contact/v3/groups/{}", group_id),
            supported_access_token_types: vec![AccessTokenType::Tenant],
            body: Vec::new(),
            query_params: std::collections::HashMap::new(),
            ..Default::default()
        };

        let resp = Transport::<GetGroupResponse>::request(api_req, &self.config, None).await?;
        Ok(resp.data.unwrap_or_default())
    }

    /// 查询用户组列表
    pub async fn simplelist(
        &self,
        _req: &ListGroupsRequest,
    ) -> crate::core::SDKResult<ListGroupsResponse> {
        let api_req = ApiRequest {
            http_method: reqwest::Method::GET,
            api_path: "/open-apis/contact/v3/groups/simplelist".to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant],
            body: Vec::new(),
            query_params: std::collections::HashMap::new(),
            ..Default::default()
        };

        let resp = Transport::<ListGroupsResponse>::request(api_req, &self.config, None).await?;
        Ok(resp.data.unwrap_or_default())
    }

    /// 查询用户所属用户组
    pub async fn member_belong(
        &self,
        _req: &GetUserGroupsRequest,
    ) -> crate::core::SDKResult<GetUserGroupsResponse> {
        let api_req = ApiRequest {
            http_method: reqwest::Method::GET,
            api_path: "/open-apis/contact/v3/groups/member_belong".to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant],
            body: Vec::new(),
            query_params: std::collections::HashMap::new(),
            ..Default::default()
        };

        let resp = Transport::<GetUserGroupsResponse>::request(api_req, &self.config, None).await?;
        Ok(resp.data.unwrap_or_default())
    }

    /// 删除用户组
    pub async fn delete(&self, group_id: &str) -> crate::core::SDKResult<DeleteGroupResponse> {
        let api_req = ApiRequest {
            http_method: reqwest::Method::DELETE,
            api_path: format!("/open-apis/contact/v3/groups/{}", group_id),
            supported_access_token_types: vec![AccessTokenType::Tenant],
            body: Vec::new(),
            ..Default::default()
        };

        let resp = Transport::<DeleteGroupResponse>::request(api_req, &self.config, None).await?;
        Ok(resp.data.unwrap_or_default())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateGroupRequest {
    pub group: Group,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateGroupResponse {
    pub group: Group,
}

impl ApiResponseTrait for CreateGroupResponse {
    fn data_format() -> crate::core::api_resp::ResponseFormat {
        crate::core::api_resp::ResponseFormat::Data
    }
}

impl Default for CreateGroupResponse {
    fn default() -> Self {
        Self {
            group: Group::default(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatchGroupRequest {
    pub group: Group,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatchGroupResponse {
    pub group: Group,
}

impl ApiResponseTrait for PatchGroupResponse {
    fn data_format() -> crate::core::api_resp::ResponseFormat {
        crate::core::api_resp::ResponseFormat::Data
    }
}

impl Default for PatchGroupResponse {
    fn default() -> Self {
        Self {
            group: Group::default(),
        }
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetGroupRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department_id_type: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetGroupResponse {
    pub group: Group,
}

impl ApiResponseTrait for GetGroupResponse {
    fn data_format() -> crate::core::api_resp::ResponseFormat {
        crate::core::api_resp::ResponseFormat::Data
    }
}

impl Default for GetGroupResponse {
    fn default() -> Self {
        Self {
            group: Group::default(),
        }
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListGroupsRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department_id_type: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListGroupsResponse {
    pub groups: Vec<Group>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

impl ApiResponseTrait for ListGroupsResponse {
    fn data_format() -> crate::core::api_resp::ResponseFormat {
        crate::core::api_resp::ResponseFormat::Data
    }
}

impl Default for ListGroupsResponse {
    fn default() -> Self {
        Self {
            groups: Vec::new(),
            has_more: None,
            page_token: None,
        }
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetUserGroupsRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_id_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetUserGroupsResponse {
    pub group_list: Vec<Group>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

impl ApiResponseTrait for GetUserGroupsResponse {
    fn data_format() -> crate::core::api_resp::ResponseFormat {
        crate::core::api_resp::ResponseFormat::Data
    }
}

impl Default for GetUserGroupsResponse {
    fn default() -> Self {
        Self {
            group_list: Vec::new(),
            has_more: None,
            page_token: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteGroupResponse {}

impl ApiResponseTrait for DeleteGroupResponse {
    fn data_format() -> crate::core::api_resp::ResponseFormat {
        crate::core::api_resp::ResponseFormat::Data
    }
}

impl Default for DeleteGroupResponse {
    fn default() -> Self {
        Self {}
    }
}

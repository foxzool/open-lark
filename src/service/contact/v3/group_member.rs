use crate::{
    core::{
        api_req::ApiRequest, api_resp::ApiResponseTrait, config::Config,
        constants::AccessTokenType, endpoints::{EndpointBuilder, Endpoints}, http::Transport,
    },
    service::contact::models::*,
};
use serde::{Deserialize, Serialize};

/// 用户组成员服务
pub struct GroupMemberService {
    config: Config,
}

impl GroupMemberService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 添加用户组成员
    pub async fn add(
        &self,
        group_id: &str,
        req: &AddGroupMemberRequest,
    ) -> crate::core::SDKResult<AddGroupMemberResponse> {
        let api_req = ApiRequest {
            http_method: reqwest::Method::POST,
            api_path: EndpointBuilder::replace_param(Endpoints::CONTACT_V3_GROUP_MEMBERS_ADD, "group_id", group_id),
            supported_access_token_types: vec![AccessTokenType::Tenant],
            body: serde_json::to_vec(req)?,
            ..Default::default()
        };

        let resp =
            Transport::<AddGroupMemberResponse>::request(api_req, &self.config, None).await?;
        Ok(resp.data.unwrap_or_default())
    }

    /// 批量添加用户组成员
    pub async fn batch_add(
        &self,
        group_id: &str,
        req: &BatchAddGroupMembersRequest,
    ) -> crate::core::SDKResult<BatchAddGroupMembersResponse> {
        let api_req = ApiRequest {
            http_method: reqwest::Method::POST,
            api_path: EndpointBuilder::replace_param(Endpoints::CONTACT_V3_GROUP_MEMBERS_BATCH_ADD, "group_id", group_id),
            supported_access_token_types: vec![AccessTokenType::Tenant],
            body: serde_json::to_vec(req)?,
            ..Default::default()
        };

        let resp =
            Transport::<BatchAddGroupMembersResponse>::request(api_req, &self.config, None).await?;
        Ok(resp.data.unwrap_or_default())
    }

    /// 查询用户组成员列表
    pub async fn simplelist(
        &self,
        group_id: &str,
        _req: &ListGroupMembersRequest,
    ) -> crate::core::SDKResult<ListGroupMembersResponse> {
        let api_req = ApiRequest {
            http_method: reqwest::Method::GET,
            api_path: EndpointBuilder::replace_param(Endpoints::CONTACT_V3_GROUP_MEMBERS_SIMPLELIST, "group_id", group_id),
            supported_access_token_types: vec![AccessTokenType::Tenant],
            body: Vec::new(),
            query_params: std::collections::HashMap::new(),
            ..Default::default()
        };

        let resp =
            Transport::<ListGroupMembersResponse>::request(api_req, &self.config, None).await?;
        Ok(resp.data.unwrap_or_default())
    }

    /// 移除用户组成员
    pub async fn remove(
        &self,
        group_id: &str,
        req: &RemoveGroupMemberRequest,
    ) -> crate::core::SDKResult<RemoveGroupMemberResponse> {
        let api_req = ApiRequest {
            http_method: reqwest::Method::POST,
            api_path: EndpointBuilder::replace_param(Endpoints::CONTACT_V3_GROUP_MEMBERS_REMOVE, "group_id", group_id),
            supported_access_token_types: vec![AccessTokenType::Tenant],
            body: serde_json::to_vec(req)?,
            ..Default::default()
        };

        let resp =
            Transport::<RemoveGroupMemberResponse>::request(api_req, &self.config, None).await?;
        Ok(resp.data.unwrap_or_default())
    }

    /// 批量移除用户组成员
    pub async fn batch_remove(
        &self,
        group_id: &str,
        req: &BatchRemoveGroupMembersRequest,
    ) -> crate::core::SDKResult<BatchRemoveGroupMembersResponse> {
        let api_req = ApiRequest {
            http_method: reqwest::Method::POST,
            api_path: EndpointBuilder::replace_param(Endpoints::CONTACT_V3_GROUP_MEMBERS_BATCH_REMOVE, "group_id", group_id),
            supported_access_token_types: vec![AccessTokenType::Tenant],
            body: serde_json::to_vec(req)?,
            ..Default::default()
        };

        let resp =
            Transport::<BatchRemoveGroupMembersResponse>::request(api_req, &self.config, None)
                .await?;
        Ok(resp.data.unwrap_or_default())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddGroupMemberRequest {
    pub member_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_id_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_type: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AddGroupMemberResponse {}

impl ApiResponseTrait for AddGroupMemberResponse {
    fn data_format() -> crate::core::api_resp::ResponseFormat {
        crate::core::api_resp::ResponseFormat::Data
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchAddGroupMembersRequest {
    pub members: Vec<GroupMemberInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BatchAddGroupMembersResponse {
    pub results: Vec<GroupMemberResult>,
}

impl ApiResponseTrait for BatchAddGroupMembersResponse {
    fn data_format() -> crate::core::api_resp::ResponseFormat {
        crate::core::api_resp::ResponseFormat::Data
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ListGroupMembersRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_id_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ListGroupMembersResponse {
    pub memberlist: Vec<GroupMember>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

impl ApiResponseTrait for ListGroupMembersResponse {
    fn data_format() -> crate::core::api_resp::ResponseFormat {
        crate::core::api_resp::ResponseFormat::Data
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemoveGroupMemberRequest {
    pub member_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_id_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_type: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RemoveGroupMemberResponse {}

impl ApiResponseTrait for RemoveGroupMemberResponse {
    fn data_format() -> crate::core::api_resp::ResponseFormat {
        crate::core::api_resp::ResponseFormat::Data
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchRemoveGroupMembersRequest {
    pub members: Vec<GroupMemberInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BatchRemoveGroupMembersResponse {
    pub results: Vec<GroupMemberResult>,
}

impl ApiResponseTrait for BatchRemoveGroupMembersResponse {
    fn data_format() -> crate::core::api_resp::ResponseFormat {
        crate::core::api_resp::ResponseFormat::Data
    }
}

use crate::{
    core::{
        api_req::ApiRequest, api_resp::ApiResponseTrait, config::Config,
        constants::AccessTokenType, endpoints::EndpointBuilder, http::Transport,
        trait_system::Service,
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
            api_path: crate::core::endpoints::contact::CONTACT_V3_GROUPS.to_string(),
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
            api_path: EndpointBuilder::replace_param(
                crate::core::endpoints::contact::CONTACT_V3_GROUP_GET,
                "group_id",
                group_id,
            ),
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
            api_path: EndpointBuilder::replace_param(
                crate::core::endpoints::contact::CONTACT_V3_GROUP_GET,
                "group_id",
                group_id,
            ),
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
            api_path: crate::core::endpoints::contact::CONTACT_V3_GROUPS_SIMPLELIST.to_string(),
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
            api_path: crate::core::endpoints::contact::CONTACT_V3_GROUPS_MEMBER_BELONG.to_string(),
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
            api_path: EndpointBuilder::replace_param(
                crate::core::endpoints::contact::CONTACT_V3_GROUP_GET,
                "group_id",
                group_id,
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant],
            body: Vec::new(),
            ..Default::default()
        };

        let resp = Transport::<DeleteGroupResponse>::request(api_req, &self.config, None).await?;
        Ok(resp.data.unwrap_or_default())
    }

    /// 获取用户组详细信息
    pub async fn get_detail(
        &self,
        group_id: &str,
        req: &GetGroupDetailRequest,
    ) -> crate::core::SDKResult<GetGroupDetailResponse> {
        let mut query_params = std::collections::HashMap::new();

        if let Some(user_id_type) = &req.user_id_type {
            query_params.insert("user_id_type", user_id_type.clone());
        }
        if let Some(department_id_type) = &req.department_id_type {
            query_params.insert("department_id_type", department_id_type.clone());
        }
        if let Some(include_members) = req.include_members {
            query_params.insert("include_members", include_members.to_string());
        }

        let api_req = ApiRequest {
            http_method: reqwest::Method::GET,
            api_path: EndpointBuilder::replace_param(
                crate::core::endpoints::contact::CONTACT_V3_GROUP_DETAIL,
                "group_id",
                group_id,
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant],
            body: Vec::new(),
            query_params,
            ..Default::default()
        };

        let resp =
            Transport::<GetGroupDetailResponse>::request(api_req, &self.config, None).await?;
        Ok(resp.data.unwrap_or_default())
    }
}

impl Service for GroupService {
    fn config(&self) -> &Config {
        &self.config
    }

    fn service_name() -> &'static str {
        "group"
    }

    fn service_version() -> &'static str {
        "v3"
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateGroupRequest {
    pub group: Group,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreateGroupResponse {
    pub group: Group,
}

impl ApiResponseTrait for CreateGroupResponse {
    fn data_format() -> crate::core::api_resp::ResponseFormat {
        crate::core::api_resp::ResponseFormat::Data
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatchGroupRequest {
    pub group: Group,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PatchGroupResponse {
    pub group: Group,
}

impl ApiResponseTrait for PatchGroupResponse {
    fn data_format() -> crate::core::api_resp::ResponseFormat {
        crate::core::api_resp::ResponseFormat::Data
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetGroupRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department_id_type: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GetGroupResponse {
    pub group: Group,
}

impl ApiResponseTrait for GetGroupResponse {
    fn data_format() -> crate::core::api_resp::ResponseFormat {
        crate::core::api_resp::ResponseFormat::Data
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

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
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

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
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

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DeleteGroupResponse {}

impl ApiResponseTrait for DeleteGroupResponse {
    fn data_format() -> crate::core::api_resp::ResponseFormat {
        crate::core::api_resp::ResponseFormat::Data
    }
}

/// 获取用户组详细信息请求
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GetGroupDetailRequest {
    /// 用户 ID 类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id_type: Option<String>,
    /// 部门 ID 类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department_id_type: Option<String>,
    /// 是否包含成员信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_members: Option<bool>,
}

/// 获取用户组详细信息响应
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GetGroupDetailResponse {
    /// 用户组信息
    pub group: GroupDetail,
}

impl ApiResponseTrait for GetGroupDetailResponse {
    fn data_format() -> crate::core::api_resp::ResponseFormat {
        crate::core::api_resp::ResponseFormat::Data
    }
}

/// 用户组详细信息
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GroupDetail {
    /// 用户组ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
    /// 用户组名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 用户组描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 用户组类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_type: Option<i32>,
    /// 成员列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub members: Option<Vec<GroupMember>>,
    /// 创建时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    /// 更新时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_time: Option<String>,
    /// 成员数量
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_count: Option<i32>,
}

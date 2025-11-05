//! 用户组成员管理服务
//!
//! 提供用户组成员的增删改查功能：
//! - 添加/删除用户组成员
//! - 批量成员操作
//! - 查询组成员列表
//! - 成员权限管理

use serde::{Deserialize, Serialize};
use crate::core::{
    api_resp::{ApiResponseTrait, ResponseFormat},
    config::Config,
    constants::AccessTokenType,
    http::Transport,
    ApiRequest, SDKResult,
};
use crate::service::contact::models::*;

/// 用户组成员服务
#[derive(Debug, Clone)]
pub struct GroupMemberService {
    config: Config,
}

impl GroupMemberService {
    /// 创建新的用户组成员服务实例
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 添加用户组成员
    ///
    /// 向指定用户组添加单个成员
    ///
    /// # 参数
    /// * `group_id` - 用户组ID
    /// * `req` - 添加成员请求
    ///
    /// # 返回
    /// 返回添加操作的结果
    pub async fn add(
        &self,
        group_id: &str,
        req: &AddGroupMemberRequest,
    ) -> SDKResult<AddGroupMemberResponse> {
        let mut api_req = ApiRequest::default();
        api_req.set_api_path(crate::core::endpoints_original::Endpoints::CONTACT_V3_GROUP_MEMBERS_ADD.to_string().replace("{group_id}", group_id));
        api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant]);
        api_req.set_body(serde_json::to_vec(req)?);

        let resp = Transport::<AddGroupMemberResponse>::request(api_req, &self.config, None).await?;
        Ok(resp.data.unwrap_or_default())
    }

    /// 批量添加用户组成员
    ///
    /// 向指定用户组批量添加多个成员
    ///
    /// # 参数
    /// * `group_id` - 用户组ID
    /// * `req` - 批量添加成员请求
    ///
    /// # 返回
    /// 返回批量添加操作的结果
    pub async fn batch_add(
        &self,
        group_id: &str,
        req: &BatchAddGroupMembersRequest,
    ) -> SDKResult<BatchAddGroupMembersResponse> {
        let mut api_req = ApiRequest::default();
        api_req.set_api_path(crate::core::endpoints_original::Endpoints::CONTACT_V3_GROUP_MEMBERS_BATCH_ADD.to_string().replace("{group_id}", group_id));
        api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant]);
        api_req.set_body(serde_json::to_vec(req)?);

        let resp = Transport::<BatchAddGroupMembersResponse>::request(api_req, &self.config, None).await?;
        Ok(resp.data.unwrap_or_default())
    }

    /// 查询用户组成员列表
    ///
    /// 获取指定用户组的成员列表
    ///
    /// # 参数
    /// * `group_id` - 用户组ID
    /// * `req` - 查询成员列表请求
    ///
    /// # 返回
    /// 返回用户组成员列表
    pub async fn simplelist(
        &self,
        group_id: &str,
        req: &ListGroupMembersRequest,
    ) -> SDKResult<ListGroupMembersResponse> {
        let mut api_path = crate::core::endpoints_original::Endpoints::CONTACT_V3_GROUP_MEMBERS_SIMPLELIST.to_string().replace("{group_id}", group_id);

        // 添加查询参数
        let mut query_params = Vec::new();
        if let Some(member_id_type) = &req.member_id_type {
            query_params.push(format!("member_id_type={}", member_id_type));
        }
        if let Some(member_type) = &req.member_type {
            query_params.push(format!("member_type={}", member_type));
        }
        if let Some(page_size) = &req.page_size {
            query_params.push(format!("page_size={}", page_size));
        }
        if let Some(page_token) = &req.page_token {
            query_params.push(format!("page_token={}", page_token));
        }

        if !query_params.is_empty() {
            api_path.push('?');
            api_path.push_str(&query_params.join("&"));
        }

        let api_req = ApiRequest {
            http_method: reqwest::Method::GET,
            api_path,
            supported_access_token_types: vec![AccessTokenType::Tenant],
            body: Vec::new(),
            ..Default::default()
        };

        let resp = Transport::<ListGroupMembersResponse>::request(api_req, &self.config, None).await?;
        Ok(resp.data.unwrap_or_default())
    }

    /// 移除用户组成员
    ///
    /// 从指定用户组移除单个成员
    ///
    /// # 参数
    /// * `group_id` - 用户组ID
    /// * `req` - 移除成员请求
    ///
    /// # 返回
    /// 返回移除操作的结果
    pub async fn remove(
        &self,
        group_id: &str,
        req: &RemoveGroupMemberRequest,
    ) -> SDKResult<RemoveGroupMemberResponse> {
        let mut api_req = ApiRequest::default();
        api_req.set_api_path(crate::core::endpoints_original::Endpoints::CONTACT_V3_GROUP_MEMBERS_REMOVE.to_string().replace("{group_id}", group_id));
        api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant]);
        api_req.set_body(serde_json::to_vec(req)?);

        let resp = Transport::<RemoveGroupMemberResponse>::request(api_req, &self.config, None).await?;
        Ok(resp.data.unwrap_or_default())
    }

    /// 批量移除用户组成员
    ///
    /// 从指定用户组批量移除多个成员
    ///
    /// # 参数
    /// * `group_id` - 用户组ID
    /// * `req` - 批量移除成员请求
    ///
    /// # 返回
    /// 返回批量移除操作的结果
    pub async fn batch_remove(
        &self,
        group_id: &str,
        req: &BatchRemoveGroupMembersRequest,
    ) -> SDKResult<BatchRemoveGroupMembersResponse> {
        let mut api_req = ApiRequest::default();
        api_req.set_api_path(crate::core::endpoints_original::Endpoints::CONTACT_V3_GROUP_MEMBERS_BATCH_REMOVE.to_string().replace("{group_id}", group_id));
        api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant]);
        api_req.set_body(serde_json::to_vec(req)?);

        let resp = Transport::<BatchRemoveGroupMembersResponse>::request(api_req, &self.config, None).await?;
        Ok(resp.data.unwrap_or_default())
    }
}

// ==================== 数据模型 ====================

/// 添加用户组成员请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AddGroupMemberRequest {
    /// 成员ID
    pub member_id: String,
    /// 成员ID类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_id_type: Option<String>,
    /// 成员类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_type: Option<String>,
}

/// 添加用户组成员响应
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AddGroupMemberResponse {}

impl ApiResponseTrait for AddGroupMemberResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 批量添加用户组成员请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchAddGroupMembersRequest {
    /// 成员列表
    pub members: Vec<GroupMemberInfo>,
}

/// 批量添加用户组成员响应
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BatchAddGroupMembersResponse {
    /// 操作结果列表
    pub results: Vec<GroupMemberResult>,
}

impl ApiResponseTrait for BatchAddGroupMembersResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 查询用户组成员列表请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListGroupMembersRequest {
    /// 成员ID类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_id_type: Option<String>,
    /// 成员类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_type: Option<String>,
    /// 分页大小
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 分页标记
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

impl Default for ListGroupMembersRequest {
    fn default() -> Self {
        Self {
            member_id_type: None,
            member_type: None,
            page_size: None,
            page_token: None,
        }
    }
}

/// 查询用户组成员列表响应
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ListGroupMembersResponse {
    /// 成员列表
    pub memberlist: Vec<GroupMember>,
    /// 是否还有更多项目
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
    /// 分页标记
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

impl ApiResponseTrait for ListGroupMembersResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 移除用户组成员请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemoveGroupMemberRequest {
    /// 成员ID
    pub member_id: String,
    /// 成员ID类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_id_type: Option<String>,
    /// 成员类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_type: Option<String>,
}

/// 移除用户组成员响应
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RemoveGroupMemberResponse {}

impl ApiResponseTrait for RemoveGroupMemberResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 批量移除用户组成员请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchRemoveGroupMembersRequest {
    /// 成员列表
    pub members: Vec<GroupMemberInfo>,
}

/// 批量移除用户组成员响应
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BatchRemoveGroupMembersResponse {
    /// 操作结果列表
    pub results: Vec<GroupMemberResult>,
}

impl ApiResponseTrait for BatchRemoveGroupMembersResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

// ==================== 共享类型 ====================

/// 用户组成员信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupMemberInfo {
    /// 成员ID
    pub member_id: String,
    /// 成员ID类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_id_type: Option<String>,
    /// 成员类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_type: Option<String>,
}

/// 用户组成员操作结果
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GroupMemberResult {
    /// 成员ID
    pub member_id: String,
    /// 操作是否成功
    pub success: bool,
    /// 错误信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

// ==================== 单元测试 ====================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_group_member_service_creation() {
        let config = Config::default();
        let service = GroupMemberService::new(config);
        assert!(!format!("{:?}", service).is_empty());
    }

    #[test]
    fn test_add_group_member_request_construction() {
        let request = AddGroupMemberRequest {
            member_id: "user123".to_string(),
            member_id_type: Some("user_id".to_string()),
            member_type: Some("user".to_string()),
        };

        assert_eq!(request.member_id, "user123");
        assert_eq!(request.member_id_type, Some("user_id".to_string()));
        assert_eq!(request.member_type, Some("user".to_string()));
    }

    #[test]
    fn test_list_group_members_request_default() {
        let request = ListGroupMembersRequest::default();

        assert_eq!(request.member_id_type, None);
        assert_eq!(request.member_type, None);
        assert_eq!(request.page_size, None);
        assert_eq!(request.page_token, None);
    }

    #[test]
    fn test_list_group_members_request_with_pagination() {
        let request = ListGroupMembersRequest {
            member_id_type: Some("user_id".to_string()),
            member_type: Some("user".to_string()),
            page_size: Some(50),
            page_token: Some("token123".to_string()),
        };

        assert_eq!(request.member_id_type, Some("user_id".to_string()));
        assert_eq!(request.member_type, Some("user".to_string()));
        assert_eq!(request.page_size, Some(50));
        assert_eq!(request.page_token, Some("token123".to_string()));
    }

    #[test]
    fn test_group_member_info_serialization() {
        let member_info = GroupMemberInfo {
            member_id: "user456".to_string(),
            member_id_type: Some("open_id".to_string()),
            member_type: Some("user".to_string()),
        };

        let serialized = serde_json::to_string(&member_info).unwrap();
        assert!(serialized.contains("user456"));
        assert!(serialized.contains("open_id"));

        let deserialized: GroupMemberInfo = serde_json::from_str(&serialized).unwrap();
        assert_eq!(deserialized.member_id, "user456");
        assert_eq!(deserialized.member_id_type, Some("open_id".to_string()));
    }

    #[test]
    fn test_batch_requests_construction() {
        let members = vec![
            GroupMemberInfo {
                member_id: "user1".to_string(),
                member_id_type: Some("user_id".to_string()),
                member_type: Some("user".to_string()),
            },
            GroupMemberInfo {
                member_id: "user2".to_string(),
                member_id_type: Some("user_id".to_string()),
                member_type: Some("user".to_string()),
            }
        ];

        let batch_add_request = BatchAddGroupMembersRequest { members: members.clone() };
        let batch_remove_request = BatchRemoveGroupMembersRequest { members };

        assert_eq!(batch_add_request.members.len(), 2);
        assert_eq!(batch_remove_request.members.len(), 2);
        assert_eq!(batch_add_request.members[0].member_id, "user1");
    }

    #[test]
    fn test_response_default_creation() {
        let add_response = AddGroupMemberResponse::default();
        let remove_response = RemoveGroupMemberResponse::default();
        let list_response = ListGroupMembersResponse::default();

        assert!(!format!("{:?}", add_response).is_empty());
        assert!(!format!("{:?}", remove_response).is_empty());
        assert_eq!(list_response.memberlist.len(), 0);
    }

    #[test]
    fn test_api_response_trait_implementations() {
        assert_eq!(AddGroupMemberResponse::data_format(), ResponseFormat::Data);
        assert_eq!(BatchAddGroupMembersResponse::data_format(), ResponseFormat::Data);
        assert_eq!(ListGroupMembersResponse::data_format(), ResponseFormat::Data);
        assert_eq!(RemoveGroupMemberResponse::data_format(), ResponseFormat::Data);
        assert_eq!(BatchRemoveGroupMembersResponse::data_format(), ResponseFormat::Data);
    }

    #[test]
    fn test_group_member_result_creation() {
        let result = GroupMemberResult {
            member_id: "user789".to_string(),
            success: true,
            error: None,
        };

        assert_eq!(result.member_id, "user789");
        assert!(result.success);
        assert!(result.error.is_none());

        let failed_result = GroupMemberResult {
            member_id: "user790".to_string(),
            success: false,
            error: Some("User not found".to_string()),
        };

        assert!(!failed_result.success);
        assert_eq!(failed_result.error, Some("User not found".to_string()));
    }
}
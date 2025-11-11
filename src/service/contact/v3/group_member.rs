//! 用户组成员管理服务
//!
//! 提供用户组成员的增删改查功能：
//! - 添加/删除用户组成员
//! - 批量成员操作
//! - 查询组成员列表
//! - 成员权限管理

use crate::{
    api_resp::{ApiResponseTrait, ResponseFormat},
    config::Config,
    constants::AccessTokenType,
    http::Transport,
    ApiRequest, SDKResult,
};
use serde::{Deserialize, Serialize};

/// 用户组成员服务
#[derive(Debug, Clone)]
pub struct GroupMemberService {
    config: Config,
}

// ==================== 数据模型 ====================

/// 用户组成员信息
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GroupMember {
    /// 成员ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_id: Option<String>,
    /// 成员ID类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_id_type: Option<String>,
    /// 成员类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_type: Option<String>,
    /// 加入时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub joined_at: Option<String>,
    /// 邀请者ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inviter_id: Option<String>,
}

impl Default for GroupMember {
    fn default() -> Self {
        Self {
            member_id: None,
            member_id_type: None,
            member_type: None,
            joined_at: None,
            inviter_id: None,
        }
    }
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
        let api_path = crate::core::endpoints_original::Endpoints::CONTACT_V3_GROUP_MEMBERS_ADD
            .replace("{group_id}", group_id);

        let api_req = ApiRequest {
            http_method: reqwest::Method::POST,
            api_path,
            supported_access_token_types: vec![AccessTokenType::Tenant],
            body: serde_json::to_vec(req)?,
            ..Default::default()
        };

        let resp =
            Transport::<AddGroupMemberResponse>::request(api_req, &self.config, None).await?;
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
        let api_path =
            crate::core::endpoints_original::Endpoints::CONTACT_V3_GROUP_MEMBERS_BATCH_ADD
                .replace("{group_id}", group_id);

        let api_req = ApiRequest {
            http_method: reqwest::Method::POST,
            api_path,
            supported_access_token_types: vec![AccessTokenType::Tenant],
            body: serde_json::to_vec(req)?,
            ..Default::default()
        };

        let resp =
            Transport::<BatchAddGroupMembersResponse>::request(api_req, &self.config, None).await?;
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
        let mut api_path =
            crate::core::endpoints_original::Endpoints::CONTACT_V3_GROUP_MEMBERS_SIMPLELIST
                .replace("{group_id}", group_id);

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

        let resp =
            Transport::<ListGroupMembersResponse>::request(api_req, &self.config, None).await?;
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
        let api_path = crate::core::endpoints_original::Endpoints::CONTACT_V3_GROUP_MEMBERS_REMOVE
            .replace("{group_id}", group_id);

        let api_req = ApiRequest {
            http_method: reqwest::Method::DELETE,
            api_path,
            supported_access_token_types: vec![AccessTokenType::Tenant],
            body: serde_json::to_vec(req)?,
            ..Default::default()
        };

        let resp =
            Transport::<RemoveGroupMemberResponse>::request(api_req, &self.config, None).await?;
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
        let api_path =
            crate::core::endpoints_original::Endpoints::CONTACT_V3_GROUP_MEMBERS_BATCH_REMOVE
                .replace("{group_id}", group_id);

        let api_req = ApiRequest {
            http_method: reqwest::Method::DELETE,
            api_path,
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

// ==================== 数据模型 ====================

/// 添加用户组成员请求
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
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
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct AddGroupMemberResponse {}

impl ApiResponseTrait for AddGroupMemberResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 批量添加用户组成员请求
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BatchAddGroupMembersRequest {
    /// 成员列表
    pub members: Vec<GroupMemberInfo>,
}

/// 批量添加用户组成员响应
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
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
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
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
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
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
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
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
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct RemoveGroupMemberResponse {}

impl ApiResponseTrait for RemoveGroupMemberResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 批量移除用户组成员请求
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BatchRemoveGroupMembersRequest {
    /// 成员列表
    pub members: Vec<GroupMemberInfo>,
}

/// 批量移除用户组成员响应
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
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
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
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
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct GroupMemberResult {
    /// 成员ID
    pub member_id: String,
    /// 操作是否成功
    pub success: bool,
    /// 错误信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

// ==================== 构建器模式 ====================

/// 添加用户组成员构建器
#[derive(Debug, Clone)]
pub struct AddGroupMemberBuilder {
    group_id: String,
    request: AddGroupMemberRequest,
}

impl AddGroupMemberBuilder {
    /// 创建新的添加成员构建器
    pub fn new(group_id: impl Into<String>) -> Self {
        Self {
            group_id: group_id.into(),
            request: AddGroupMemberRequest {
                member_id: String::new(),
                member_id_type: None,
                member_type: None,
            },
        }
    }

    /// 设置成员ID
    pub fn member_id(mut self, member_id: impl Into<String>) -> Self {
        self.request.member_id = member_id.into();
        self
    }

    /// 设置成员ID类型
    pub fn member_id_type(mut self, member_id_type: impl Into<String>) -> Self {
        self.request.member_id_type = Some(member_id_type.into());
        self
    }

    /// 设置成员类型
    pub fn member_type(mut self, member_type: impl Into<String>) -> Self {
        self.request.member_type = Some(member_type.into());
        self
    }

    /// 执行添加操作
    pub async fn execute(self, service: &GroupMemberService) -> SDKResult<AddGroupMemberResponse> {
        service.add(&self.group_id, &self.request).await
    }
}

/// 查询用户组成员列表构建器
#[derive(Debug, Clone)]
pub struct ListGroupMembersBuilder {
    group_id: String,
    request: ListGroupMembersRequest,
}

impl ListGroupMembersBuilder {
    /// 创建新的查询构建器
    pub fn new(group_id: impl Into<String>) -> Self {
        Self {
            group_id: group_id.into(),
            request: ListGroupMembersRequest::default(),
        }
    }

    /// 设置成员ID类型
    pub fn member_id_type(mut self, member_id_type: impl Into<String>) -> Self {
        self.request.member_id_type = Some(member_id_type.into());
        self
    }

    /// 设置成员类型
    pub fn member_type(mut self, member_type: impl Into<String>) -> Self {
        self.request.member_type = Some(member_type.into());
        self
    }

    /// 设置分页大小
    pub fn page_size(mut self, page_size: i32) -> Self {
        self.request.page_size = Some(page_size);
        self
    }

    /// 设置分页标记
    pub fn page_token(mut self, page_token: impl Into<String>) -> Self {
        self.request.page_token = Some(page_token.into());
        self
    }

    /// 执行查询操作
    pub async fn execute(
        self,
        service: &GroupMemberService,
    ) -> SDKResult<ListGroupMembersResponse> {
        service.simplelist(&self.group_id, &self.request).await
    }
}

impl GroupMemberService {
    /// 创建添加成员构建器
    pub fn add_group_member_builder(&self, group_id: impl Into<String>) -> AddGroupMemberBuilder {
        AddGroupMemberBuilder::new(group_id)
    }

    /// 创建查询成员列表构建器
    pub fn list_group_members_builder(
        &self,
        group_id: impl Into<String>,
    ) -> ListGroupMembersBuilder {
        ListGroupMembersBuilder::new(group_id)
    }
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
    fn test_group_member_default_creation() {
        let member = GroupMember::default();
        assert_eq!(member.member_id, None);
        assert_eq!(member.member_id_type, None);
        assert_eq!(member.member_type, None);
        assert_eq!(member.joined_at, None);
        assert_eq!(member.inviter_id, None);
    }

    #[test]
    fn test_group_member_with_data() {
        let member = GroupMember {
            member_id: Some("user_123".to_string()),
            member_id_type: Some("open_id".to_string()),
            member_type: Some("user".to_string()),
            joined_at: Some("2023-01-01T00:00:00Z".to_string()),
            inviter_id: Some("inviter_456".to_string()),
        };

        assert_eq!(member.member_id, Some("user_123".to_string()));
        assert_eq!(member.member_id_type, Some("open_id".to_string()));
        assert_eq!(member.member_type, Some("user".to_string()));
        assert_eq!(member.joined_at, Some("2023-01-01T00:00:00Z".to_string()));
        assert_eq!(member.inviter_id, Some("inviter_456".to_string()));
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
            },
        ];

        let batch_add_request = BatchAddGroupMembersRequest {
            members: members.clone(),
        };
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
    fn test_response_with_data() {
        let mut list_response = ListGroupMembersResponse::default();
        list_response.memberlist = vec![GroupMember {
            member_id: Some("member_789".to_string()),
            member_id_type: Some("user_id".to_string()),
            member_type: Some("user".to_string()),
            joined_at: Some("2023-06-15T10:30:00Z".to_string()),
            inviter_id: Some("admin_001".to_string()),
        }];
        list_response.has_more = Some(true);
        list_response.page_token = Some("next_page_token".to_string());

        assert_eq!(list_response.memberlist.len(), 1);
        assert_eq!(
            list_response.memberlist[0].member_id,
            Some("member_789".to_string())
        );
        assert_eq!(list_response.has_more, Some(true));
        assert_eq!(
            list_response.page_token,
            Some("next_page_token".to_string())
        );
    }

    #[test]
    fn test_api_response_trait_implementations() {
        assert_eq!(AddGroupMemberResponse::data_format(), ResponseFormat::Data);
        assert_eq!(
            BatchAddGroupMembersResponse::data_format(),
            ResponseFormat::Data
        );
        assert_eq!(
            ListGroupMembersResponse::data_format(),
            ResponseFormat::Data
        );
        assert_eq!(
            RemoveGroupMemberResponse::data_format(),
            ResponseFormat::Data
        );
        assert_eq!(
            BatchRemoveGroupMembersResponse::data_format(),
            ResponseFormat::Data
        );
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

    #[test]
    fn test_add_group_member_builder() {
        let builder = AddGroupMemberBuilder::new("group_123")
            .member_id("user_456")
            .member_id_type("open_id")
            .member_type("user");

        assert_eq!(builder.group_id, "group_123");
        assert_eq!(builder.request.member_id, "user_456");
        assert_eq!(builder.request.member_id_type, Some("open_id".to_string()));
        assert_eq!(builder.request.member_type, Some("user".to_string()));
    }

    #[test]
    fn test_list_group_members_builder() {
        let builder = ListGroupMembersBuilder::new("group_789")
            .member_id_type("user_id")
            .member_type("user")
            .page_size(20)
            .page_token("page_token_123");

        assert_eq!(builder.group_id, "group_789");
        assert_eq!(builder.request.member_id_type, Some("user_id".to_string()));
        assert_eq!(builder.request.member_type, Some("user".to_string()));
        assert_eq!(builder.request.page_size, Some(20));
        assert_eq!(
            builder.request.page_token,
            Some("page_token_123".to_string())
        );
    }

    #[test]
    fn test_request_serialization() {
        let add_request = AddGroupMemberRequest {
            member_id: "user001".to_string(),
            member_id_type: Some("open_id".to_string()),
            member_type: Some("user".to_string()),
        };

        let serialized = serde_json::to_string(&add_request).unwrap();
        let deserialized: AddGroupMemberRequest = serde_json::from_str(&serialized).unwrap();

        assert_eq!(add_request.member_id, deserialized.member_id);
        assert_eq!(add_request.member_id_type, deserialized.member_id_type);
        assert_eq!(add_request.member_type, deserialized.member_type);
    }

    #[test]
    fn test_batch_result_serialization() {
        let batch_response = BatchAddGroupMembersResponse {
            results: vec![
                GroupMemberResult {
                    member_id: "user1".to_string(),
                    success: true,
                    error: None,
                },
                GroupMemberResult {
                    member_id: "user2".to_string(),
                    success: false,
                    error: Some("Already a member".to_string()),
                },
            ],
        };

        let serialized = serde_json::to_string(&batch_response).unwrap();
        let deserialized: BatchAddGroupMembersResponse = serde_json::from_str(&serialized).unwrap();

        assert_eq!(batch_response.results.len(), deserialized.results.len());
        assert_eq!(
            batch_response.results[0].member_id,
            deserialized.results[0].member_id
        );
        assert_eq!(
            batch_response.results[1].error,
            deserialized.results[1].error
        );
    }

    #[test]
    fn test_query_parameters_construction() {
        let request = ListGroupMembersRequest {
            member_id_type: Some("open_id".to_string()),
            member_type: Some("department".to_string()),
            page_size: Some(30),
            page_token: Some("test_token".to_string()),
        };

        let mut query_params = Vec::new();
        if let Some(member_id_type) = &request.member_id_type {
            query_params.push(format!("member_id_type={}", member_id_type));
        }
        if let Some(member_type) = &request.member_type {
            query_params.push(format!("member_type={}", member_type));
        }
        if let Some(page_size) = &request.page_size {
            query_params.push(format!("page_size={}", page_size));
        }
        if let Some(page_token) = &request.page_token {
            query_params.push(format!("page_token={}", page_token));
        }

        assert_eq!(query_params.len(), 4);
        assert!(query_params.contains(&"member_id_type=open_id".to_string()));
        assert!(query_params.contains(&"member_type=department".to_string()));
        assert!(query_params.contains(&"page_size=30".to_string()));
        assert!(query_params.contains(&"page_token=test_token".to_string()));
    }

    #[test]
    fn test_group_member_joined_at_format() {
        let mut member = GroupMember::default();

        // Test with ISO 8601 format
        member.joined_at = Some("2023-12-25T15:30:00Z".to_string());
        assert_eq!(member.joined_at, Some("2023-12-25T15:30:00Z".to_string()));

        // Test with different timestamp formats
        member.joined_at = Some("2023/12/25 15:30:00".to_string());
        assert_eq!(member.joined_at, Some("2023/12/25 15:30:00".to_string()));
    }

    #[test]
    fn test_complex_group_member_scenarios() {
        // Test member with full information
        let full_member = GroupMember {
            member_id: Some("user_comprehensive".to_string()),
            member_id_type: Some("union_id".to_string()),
            member_type: Some("app".to_string()),
            joined_at: Some("2023-01-01T00:00:00Z".to_string()),
            inviter_id: Some("system_admin".to_string()),
        };

        assert_eq!(
            full_member.member_id,
            Some("user_comprehensive".to_string())
        );
        assert_eq!(full_member.member_id_type, Some("union_id".to_string()));
        assert_eq!(full_member.member_type, Some("app".to_string()));
        assert_eq!(
            full_member.joined_at,
            Some("2023-01-01T00:00:00Z".to_string())
        );
        assert_eq!(full_member.inviter_id, Some("system_admin".to_string()));

        // Test minimal member
        let minimal_member = GroupMember {
            member_id: Some("minimal_user".to_string()),
            ..Default::default()
        };

        assert_eq!(minimal_member.member_id, Some("minimal_user".to_string()));
        assert_eq!(minimal_member.member_id_type, None);
        assert_eq!(minimal_member.member_type, None);
        assert_eq!(minimal_member.joined_at, None);
        assert_eq!(minimal_member.inviter_id, None);
    }

    #[test]
    fn test_member_type_variations() {
        // Test different member types
        let user_member = GroupMember {
            member_id: Some("user001".to_string()),
            member_type: Some("user".to_string()),
            ..Default::default()
        };

        let department_member = GroupMember {
            member_id: Some("dept001".to_string()),
            member_type: Some("department".to_string()),
            ..Default::default()
        };

        let app_member = GroupMember {
            member_id: Some("app001".to_string()),
            member_type: Some("app".to_string()),
            ..Default::default()
        };

        assert_eq!(user_member.member_type, Some("user".to_string()));
        assert_eq!(
            department_member.member_type,
            Some("department".to_string())
        );
        assert_eq!(app_member.member_type, Some("app".to_string()));
    }

    #[test]
    fn test_endpoint_constants() {
        // Test that the endpoint constants are properly defined
        assert_eq!(
            crate::core::endpoints_original::Endpoints::CONTACT_V3_GROUP_MEMBERS_ADD,
            "/open-apis/contact/v3/groups/{group_id}/members/add"
        );
        assert_eq!(
            crate::core::endpoints_original::Endpoints::CONTACT_V3_GROUP_MEMBERS_BATCH_ADD,
            "/open-apis/contact/v3/groups/{group_id}/members/batch_add"
        );
        assert_eq!(
            crate::core::endpoints_original::Endpoints::CONTACT_V3_GROUP_MEMBERS_SIMPLELIST,
            "/open-apis/contact/v3/groups/{group_id}/members/simplelist"
        );
        assert_eq!(
            crate::core::endpoints_original::Endpoints::CONTACT_V3_GROUP_MEMBERS_REMOVE,
            "/open-apis/contact/v3/groups/{group_id}/members/remove"
        );
        assert_eq!(
            crate::core::endpoints_original::Endpoints::CONTACT_V3_GROUP_MEMBERS_BATCH_REMOVE,
            "/open-apis/contact/v3/groups/{group_id}/members/batch_remove"
        );
    }
}

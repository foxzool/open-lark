//! Group用户组管理服务
//!
//! 提供完整的用户组管理功能：
//! - 创建、修改、删除用户组
//! - 获取用户组信息（单个/列表）
//! - 获取用户组详细信息（包含成员）
//! - 查询用户所属用户组
//! - 企业级用户组权限管理

use open_lark_core::core::{api_req::ApiRequest, LarkAPIError}; // trait_system::ExecutableBuilder temporarily disabled
use crate::core::{
    api_resp::BaseResponse,
    config::Config,
    constants::AccessTokenType,
    http::Transport,
    req_option::RequestOption,
};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

// 使用 open_lark_core 的错误类型以兼容 async trait
pub type SDKResult<T> = Result<T, LarkAPIError>;

// 导入核心类型
use super::types::*;

/// 用户组管理服务
#[derive(Debug, Clone)]
pub struct GroupService {
    pub config: Config,
}

impl GroupService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 创建用户组
    /// 创建新的用户组来管理用户权限和访问控制
    ///
    /// # API文档
    ///
    /// 创建新的用户组，支持设置组名称、描述、成员等信息。
    /// 适用于企业内部的权限管理和访问控制。
    ///
    /// # 参数
    ///
    /// * `request` - 包含用户组信息的请求参数
    ///
    /// # 返回值
    ///
    /// 返回创建成功的用户组详细信息
    ///
    /// # 示例
    ///
    /// ```rust,no_run
    /// use open_lark::prelude::*;
    /// use open_lark::service::contact::v3::group::*;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let client = LarkClient::builder()
    ///         .app_id("your_app_id")
    ///         .app_secret("your_app_secret")
    ///         .build()?;
    ///
    ///     let group = Group {
    ///         name: Some("产品管理组".to_string()),
    ///         description: Some("负责产品规划和管理的用户组".to_string()),
    ///         ..Default::default()
    ///     };
    ///
    ///     let request = CreateGroupRequest {
    ///         group,
    ///         ..Default::default()
    ///     };
    ///
    ///     let response = client.contact.v3.group
    ///         .create(&request).await?;
    ///
    ///     println!("用户组创建成功: {:?}", response.data.group.name);
    ///     Ok(())
    /// }
    /// ```
    pub async fn create(
        &self,
        request: &CreateGroupRequest,
    ) -> SDKResult<BaseResponse<CreateGroupResponse>> {
        let api_req = ApiRequest {
            http_http_http_method: reqwest::Method::POST,
            api_path: "/open-apis/contact/v3/groups".to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant],
            body: serde_json::to_vec(request)?,
            ..Default::default()
        };

        let api_resp = Transport::request(api_req, &self.config, None).await?;
        Ok(api_resp)
    }

    /// 修改用户组
    /// 部分更新用户组信息（不覆盖未提供的字段）
    pub async fn patch(
        &self,
        group_id: &str,
        request: &PatchGroupRequest,
    ) -> SDKResult<BaseResponse<PatchGroupResponse>> {
        let api_path = format!("/open-apis/contact/v3/groups/{}", group_id);

        let api_req = ApiRequest {
            http_http_http_method: reqwest::Method::PATCH,
            api_path,
            supported_access_token_types: vec![AccessTokenType::Tenant],
            body: serde_json::to_vec(request)?,
            ..Default::default()
        };

        let api_resp = Transport::request(api_req, &self.config, None).await?;
        Ok(api_resp)
    }

    /// 获取指定用户组
    /// 获取指定用户组的详细信息
    pub async fn get(
        &self,
        group_id: &str,
        request: &GetGroupRequest,
    ) -> SDKResult<BaseResponse<GetGroupResponse>> {
        let mut api_path = format!("/open-apis/contact/v3/groups/{}", group_id);

        // 添加查询参数
        let mut query_params = Vec::new();
        if let Some(user_id_type) = &request.user_id_type {
            query_params.push(format!("user_id_type={}", user_id_type));
        }
        if let Some(department_id_type) = &request.department_id_type {
            query_params.push(format!("department_id_type={}", department_id_type));
        }

        if !query_params.is_empty() {
            api_path.push('?');
            api_path.push_str(&query_params.join("&"));
        }

        let api_req = ApiRequest {
            http_http_http_method: reqwest::Method::GET,
            api_path,
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: Vec::new(),
            ..Default::default()
        };

        let api_resp = Transport::request(api_req, &self.config, None).await?;
        Ok(api_resp)
    }

    /// 查询用户组列表
    /// 获取用户组的基本信息列表
    pub async fn simple_list(
        &self,
        request: &ListGroupsRequest,
    ) -> SDKResult<BaseResponse<ListGroupsResponse>> {
        let mut api_path = "/open-apis/contact/v3/groups/simple_list".to_string();

        // 添加查询参数
        let mut query_params = Vec::new();
        if let Some(page_size) = &request.page_size {
            query_params.push(format!("page_size={}", page_size));
        }
        if let Some(page_token) = &request.page_token {
            query_params.push(format!("page_token={}", page_token));
        }
        if let Some(user_id_type) = &request.user_id_type {
            query_params.push(format!("user_id_type={}", user_id_type));
        }
        if let Some(department_id_type) = &request.department_id_type {
            query_params.push(format!("department_id_type={}", department_id_type));
        }

        if !query_params.is_empty() {
            api_path.push('?');
            api_path.push_str(&query_params.join("&"));
        }

        let api_req = ApiRequest {
            http_http_http_method: reqwest::Method::GET,
            api_path,
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: Vec::new(),
            ..Default::default()
        };

        let api_resp = Transport::request(api_req, &self.config, None).await?;
        Ok(api_resp)
    }

    /// 查询用户所属用户组
    /// 获取指定用户所属的所有用户组
    pub async fn get_user_groups(
        &self,
        request: &GetUserGroupsRequest,
    ) -> SDKResult<BaseResponse<GetUserGroupsResponse>> {
        let mut api_path = "/open-apis/contact/v3/groups/member_belong".to_string();

        // 添加查询参数
        let mut query_params = Vec::new();
        if let Some(member_id) = &request.member_id {
            query_params.push(format!("member_id={}", member_id));
        }
        if let Some(member_id_type) = &request.member_id_type {
            query_params.push(format!("member_id_type={}", member_id_type));
        }
        if let Some(page_size) = &request.page_size {
            query_params.push(format!("page_size={}", page_size));
        }
        if let Some(page_token) = &request.page_token {
            query_params.push(format!("page_token={}", page_token));
        }

        if !query_params.is_empty() {
            api_path.push('?');
            api_path.push_str(&query_params.join("&"));
        }

        let api_req = ApiRequest {
            http_http_http_method: reqwest::Method::GET,
            api_path,
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: Vec::new(),
            ..Default::default()
        };

        let api_resp = Transport::request(api_req, &self.config, None).await?;
        Ok(api_resp)
    }

    /// 删除用户组
    /// 删除指定的用户组（请谨慎操作）
    pub async fn delete(
        &self,
        group_id: &str,
    ) -> SDKResult<BaseResponse<DeleteGroupResponse>> {
        let api_path = format!("/open-apis/contact/v3/groups/{}", group_id);

        let api_req = ApiRequest {
            http_http_http_method: reqwest::Method::DELETE,
            api_path,
            supported_access_token_types: vec![AccessTokenType::Tenant],
            body: Vec::new(),
            ..Default::default()
        };

        let api_resp = Transport::request(api_req, &self.config, None).await?;
        Ok(api_resp)
    }

    /// 获取用户组详细信息
    /// 获取用户组的完整信息，包括成员列表
    pub async fn get_detail(
        &self,
        group_id: &str,
        request: &GetGroupDetailRequest,
    ) -> SDKResult<BaseResponse<GetGroupDetailResponse>> {
        let mut api_path = format!("/open-apis/contact/v3/groups/{}/detail", group_id);

        // 添加查询参数
        let mut query_params = Vec::new();
        if let Some(user_id_type) = &request.user_id_type {
            query_params.push(format!("user_id_type={}", user_id_type));
        }
        if let Some(department_id_type) = &request.department_id_type {
            query_params.push(format!("department_id_type={}", department_id_type));
        }
        if let Some(include_members) = &request.include_members {
            query_params.push(format!("include_members={}", include_members));
        }

        if !query_params.is_empty() {
            api_path.push('?');
            api_path.push_str(&query_params.join("&"));
        }

        let api_req = ApiRequest {
            http_http_http_method: reqwest::Method::GET,
            api_path,
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: Vec::new(),
            ..Default::default()
        };

        let api_resp = Transport::request(api_req, &self.config, None).await?;
        Ok(api_resp)
    }
}

// ==================== 数据模型 ====================

/// 创建用户组请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateGroupRequest {
    /// 用户组信息
    pub group: Group,
}

impl Default for CreateGroupRequest {
    fn default() -> Self {
        Self {
            group: Group::default(),
        }
    }
}

/// 创建用户组响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateGroupResponse {
    /// 用户组信息
    pub group: Group,
}

/// 修改用户组请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatchGroupRequest {
    /// 用户组信息
    pub group: Group,
}

impl Default for PatchGroupRequest {
    fn default() -> Self {
        Self {
            group: Group::default(),
        }
    }
}

/// 修改用户组响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PatchGroupResponse {
    /// 用户组信息
    pub group: Group,
}

/// 获取用户组请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetGroupRequest {
    /// 用户 ID 类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id_type: Option<String>,
    /// 部门 ID 类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department_id_type: Option<String>,
}

impl Default for GetGroupRequest {
    fn default() -> Self {
        Self {
            user_id_type: None,
            department_id_type: None,
        }
    }
}

/// 获取用户组响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetGroupResponse {
    /// 用户组信息
    pub group: Group,
}

/// 查询用户组列表请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListGroupsRequest {
    /// 分页大小
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 分页标记
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// 用户 ID 类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id_type: Option<String>,
    /// 部门 ID 类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department_id_type: Option<String>,
}

impl Default for ListGroupsRequest {
    fn default() -> Self {
        Self {
            page_size: None,
            page_token: None,
            user_id_type: None,
            department_id_type: None,
        }
    }
}

/// 查询用户组列表响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListGroupsResponse {
    /// 用户组列表
    pub groups: Vec<Group>,
    /// 是否还有更多项目
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
    /// 分页标记
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

/// 查询用户所属用户组请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetUserGroupsRequest {
    /// 成员ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_id: Option<String>,
    /// 成员ID类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_id_type: Option<String>,
    /// 分页大小
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 分页标记
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

impl Default for GetUserGroupsRequest {
    fn default() -> Self {
        Self {
            member_id: None,
            member_id_type: None,
            page_size: None,
            page_token: None,
        }
    }
}

/// 查询用户所属用户组响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetUserGroupsResponse {
    /// 用户组列表
    pub group_list: Vec<Group>,
    /// 是否还有更多项目
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
    /// 分页标记
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

/// 删除用户组响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteGroupResponse {
    /// 操作结果
    pub success: bool,
}

impl Default for DeleteGroupResponse {
    fn default() -> Self {
        Self { success: true }
    }
}

/// 获取用户组详细信息请求
#[derive(Debug, Clone, Serialize, Deserialize)]
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

impl Default for GetGroupDetailRequest {
    fn default() -> Self {
        Self {
            user_id_type: None,
            department_id_type: None,
            include_members: None,
        }
    }
}

/// 获取用户组详细信息响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetGroupDetailResponse {
    /// 用户组详细信息
    pub group: GroupDetail,
}

/// 用户组详细信息
#[derive(Debug, Clone, Serialize, Deserialize)]
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
    pub group_type: Option<String>,
    /// 创建时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    /// 更新时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_time: Option<String>,
    /// 用户组成员列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub members: Option<Vec<GroupMember>>,
}

impl Default for GroupDetail {
    fn default() -> Self {
        Self {
            group_id: None,
            name: None,
            description: None,
            group_type: None,
            create_time: None,
            update_time: None,
            members: None,
        }
    }
}

/// 用户组成员
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupMember {
    /// 成员ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_id: Option<String>,
    /// 成员类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_type: Option<String>,
    /// 加入时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub join_time: Option<String>,
}

impl Default for GroupMember {
    fn default() -> Self {
        Self {
            member_id: None,
            member_type: None,
            join_time: None,
        }
    }
}

// ==================== Builder 模式实现 ====================

/// 创建用户组请求构建器
#[derive(Debug, Clone)]
pub struct CreateGroupBuilder {
    request: CreateGroupRequest,
}

impl CreateGroupBuilder {
    /// 创建新的Builder实例
    pub fn new() -> Self {
        Self {
            request: CreateGroupRequest::default(),
        }
    }

    /// 设置用户组信息
    pub fn group(mut self, group: Group) -> Self {
        self.request.group = group;
        self
    }

    /// 构建最终的请求对象
    pub fn build(self) -> CreateGroupRequest {
        self.request
    }
}

impl Default for CreateGroupBuilder {
    fn default() -> Self {
        Self::new()
    }
}

// 应用ExecutableBuilder trait
// crate::impl_executable_builder!(
//    CreateGroupBuilder,
//    GroupService,
//    CreateGroupRequest,
//    BaseResponse<CreateGroupResponse>,
//    create
//);

/// 修改用户组请求构建器
#[derive(Debug, Clone)]
pub struct PatchGroupBuilder {
    request: PatchGroupRequest,
}

impl PatchGroupBuilder {
    /// 创建新的Builder实例
    pub fn new() -> Self {
        Self {
            request: PatchGroupRequest::default(),
        }
    }

    /// 设置用户组信息
    pub fn group(mut self, group: Group) -> Self {
        self.request.group = group;
        self
    }

    /// 构建最终的请求对象
    pub fn build(self) -> PatchGroupRequest {
        self.request
    }
}

impl Default for PatchGroupBuilder {
    fn default() -> Self {
        Self::new()
    }
}

// 应用ExecutableBuilder trait
// crate::impl_executable_builder!(
//    PatchGroupBuilder,
//    GroupService,
//    PatchGroupRequest,
//    BaseResponse<PatchGroupResponse>,
//    patch
//);

/// 查询用户组列表请求构建器
#[derive(Debug, Clone)]
pub struct ListGroupsBuilder {
    request: ListGroupsRequest,
}

impl ListGroupsBuilder {
    /// 创建新的Builder实例
    pub fn new() -> Self {
        Self {
            request: ListGroupsRequest::default(),
        }
    }

    /// 设置分页大小
    pub fn page_size(mut self, page_size: i32) -> Self {
        self.request.page_size = Some(page_size);
        self
    }

    /// 设置分页标记
    pub fn page_token(mut self, page_token: &str) -> Self {
        self.request.page_token = Some(page_token.to_string());
        self
    }

    /// 设置用户ID类型
    pub fn user_id_type(mut self, user_id_type: &str) -> Self {
        self.request.user_id_type = Some(user_id_type.to_string());
        self
    }

    /// 设置部门ID类型
    pub fn department_id_type(mut self, department_id_type: &str) -> Self {
        self.request.department_id_type = Some(department_id_type.to_string());
        self
    }

    /// 构建最终的请求对象
    pub fn build(self) -> ListGroupsRequest {
        self.request
    }
}

impl Default for ListGroupsBuilder {
    fn default() -> Self {
        Self::new()
    }
}

// 应用ExecutableBuilder trait
// crate::impl_executable_builder!(
//    ListGroupsBuilder,
//    GroupService,
//    ListGroupsRequest,
//    BaseResponse<ListGroupsResponse>,
//    simple_list
//);

/// 查询用户所属用户组请求构建器
#[derive(Debug, Clone)]
pub struct GetUserGroupsBuilder {
    request: GetUserGroupsRequest,
}

impl GetUserGroupsBuilder {
    /// 创建新的Builder实例
    pub fn new() -> Self {
        Self {
            request: GetUserGroupsRequest::default(),
        }
    }

    /// 设置成员ID
    pub fn member_id(mut self, member_id: &str) -> Self {
        self.request.member_id = Some(member_id.to_string());
        self
    }

    /// 设置成员ID类型
    pub fn member_id_type(mut self, member_id_type: &str) -> Self {
        self.request.member_id_type = Some(member_id_type.to_string());
        self
    }

    /// 设置分页大小
    pub fn page_size(mut self, page_size: i32) -> Self {
        self.request.page_size = Some(page_size);
        self
    }

    /// 设置分页标记
    pub fn page_token(mut self, page_token: &str) -> Self {
        self.request.page_token = Some(page_token.to_string());
        self
    }

    /// 构建最终的请求对象
    pub fn build(self) -> GetUserGroupsRequest {
        self.request
    }
}

impl Default for GetUserGroupsBuilder {
    fn default() -> Self {
        Self::new()
    }
}

// 应用ExecutableBuilder trait
// crate::impl_executable_builder!(
//    GetUserGroupsBuilder,
//    GroupService,
//    GetUserGroupsRequest,
//    BaseResponse<GetUserGroupsResponse>,
//    get_user_groups
//);

/// 获取用户组详细信息请求构建器
#[derive(Debug, Clone)]
pub struct GetGroupDetailBuilder {
    request: GetGroupDetailRequest,
}

impl GetGroupDetailBuilder {
    /// 创建新的Builder实例
    pub fn new() -> Self {
        Self {
            request: GetGroupDetailRequest::default(),
        }
    }

    /// 设置用户ID类型
    pub fn user_id_type(mut self, user_id_type: &str) -> Self {
        self.request.user_id_type = Some(user_id_type.to_string());
        self
    }

    /// 设置部门ID类型
    pub fn department_id_type(mut self, department_id_type: &str) -> Self {
        self.request.department_id_type = Some(department_id_type.to_string());
        self
    }

    /// 设置是否包含成员信息
    pub fn include_members(mut self, include_members: bool) -> Self {
        self.request.include_members = Some(include_members);
        self
    }

    /// 构建最终的请求对象
    pub fn build(self) -> GetGroupDetailRequest {
        self.request
    }
}

impl Default for GetGroupDetailBuilder {
    fn default() -> Self {
        Self::new()
    }
}

// 应用ExecutableBuilder trait
// crate::impl_executable_builder!(
//    GetGroupDetailBuilder,
//    GroupService,
//    GetGroupDetailRequest,
//    BaseResponse<GetGroupDetailResponse>,
//    get_detail
//);

impl GroupService {
    /// 创建用户组构建器
    pub fn create_group_builder(&self) -> CreateGroupBuilder {
        CreateGroupBuilder::new()
    }

    /// 修改用户组构建器
    pub fn patch_group_builder(&self) -> PatchGroupBuilder {
        PatchGroupBuilder::new()
    }

    /// 查询用户组列表构建器
    pub fn list_groups_builder(&self) -> ListGroupsBuilder {
        ListGroupsBuilder::new()
    }

    /// 查询用户所属用户组构建器
    pub fn get_user_groups_builder(&self) -> GetUserGroupsBuilder {
        GetUserGroupsBuilder::new()
    }

    /// 获取用户组详细信息构建器
    pub fn get_group_detail_builder(&self) -> GetGroupDetailBuilder {
        GetGroupDetailBuilder::new()
    }
}
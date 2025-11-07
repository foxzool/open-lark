#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(non_snake_case)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::module_inception)]
//! 用户组管理服务
//!
//! 提供完整的用户组管理功能：
//! - 创建用户组
//! - 修改用户组
//! - 获取单个用户组信息
//! - 获取用户组列表
//! - 获取用户组详细信息（包含成员）
//! - 查询用户所属用户组
//! - 删除用户组
//! - 支持分页查询

use crate::core::{
    api_resp::{ApiResponseTrait, ResponseFormat},
    config::Config,
    constants::AccessTokenType,
    http::Transport,
    ApiRequest, SDKResult,
};
use serde::{Deserialize, Serialize};

/// 用户组信息
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Group {
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
}

impl Default for Group {
    fn default() -> Self {
        Self {
            group_id: None,
            name: None,
            description: None,
            group_type: None,
            create_time: None,
            update_time: None,
        }
    }
}

/// 用户组成员
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GroupMember {
    /// 成员ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_id: Option<String>,
    /// 成员类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_type: Option<String>,
    /// 成员名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 加入时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub join_time: Option<String>,
}

impl Default for GroupMember {
    fn default() -> Self {
        Self {
            member_id: None,
            member_type: None,
            name: None,
            join_time: None,
        }
    }
}

/// 用户组详细信息
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GroupDetail {
    /// 用户组信息
    pub group: Group,
    /// 用户组成员列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub members: Option<Vec<GroupMember>>,
    /// 成员总数
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_count: Option<i32>,
}

impl Default for GroupDetail {
    fn default() -> Self {
        Self {
            group: Group::default(),
            members: None,
            member_count: None,
        }
    }
}

/// 用户组管理服务
#[derive(Debug, Clone)]
pub struct GroupService {
    config: Config,
}

impl GroupService {
    /// 创建新的用户组管理服务实例
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 创建用户组
    ///
    /// 创建新的用户组来管理用户权限和访问控制
    ///
    /// # 参数
    /// * `req` - 创建用户组请求
    ///
    /// # 返回值
    /// 返回创建成功的用户组信息
    pub async fn create(&self, req: &CreateGroupRequest) -> SDKResult<CreateGroupResponse> {
        let api_path = crate::core::endpoints_original::Endpoints::CONTACT_V3_GROUPS.to_string();

        let api_req = ApiRequest {
            http_method: reqwest::Method::POST,
            api_path,
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: serde_json::to_vec(req)?,
            ..Default::default()
        };

        let resp = Transport::<CreateGroupResponse>::request(api_req, &self.config, None).await?;
        Ok(resp.data.unwrap_or_default())
    }

    /// 修改用户组
    ///
    /// 部分更新用户组信息（不覆盖未提供的字段）
    ///
    /// # 参数
    /// * `group_id` - 用户组ID
    /// * `req` - 修改用户组请求
    ///
    /// # 返回值
    /// 返回修改成功的用户组信息
    pub async fn patch(
        &self,
        group_id: &str,
        req: &PatchGroupRequest,
    ) -> SDKResult<PatchGroupResponse> {
        let api_path = crate::core::endpoints_original::Endpoints::CONTACT_V3_GROUPS
            .replace("{group_id}", group_id);

        let api_req = ApiRequest {
            http_method: reqwest::Method::PATCH,
            api_path,
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: serde_json::to_vec(req)?,
            ..Default::default()
        };

        let resp = Transport::<PatchGroupResponse>::request(api_req, &self.config, None).await?;
        Ok(resp.data.unwrap_or_default())
    }

    /// 获取单个用户组信息
    ///
    /// 获取指定用户组的详细信息
    ///
    /// # 参数
    /// * `group_id` - 用户组ID
    /// * `req` - 获取用户组请求
    ///
    /// # 返回值
    /// 返回用户组的详细信息
    pub async fn get(&self, group_id: &str, req: &GetGroupRequest) -> SDKResult<GetGroupResponse> {
        let mut api_path = crate::core::endpoints_original::Endpoints::CONTACT_V3_GROUPS
            .replace("{group_id}", group_id);

        // 添加查询参数
        let mut query_params = Vec::new();
        if let Some(user_id_type) = &req.user_id_type {
            query_params.push(format!("user_id_type={}", user_id_type));
        }
        if let Some(department_id_type) = &req.department_id_type {
            query_params.push(format!("department_id_type={}", department_id_type));
        }

        if !query_params.is_empty() {
            api_path.push('?');
            api_path.push_str(&query_params.join("&"));
        }

        let api_req = ApiRequest {
            http_method: reqwest::Method::GET,
            api_path,
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: Vec::new(),
            ..Default::default()
        };

        let resp = Transport::<GetGroupResponse>::request(api_req, &self.config, None).await?;
        Ok(resp.data.unwrap_or_default())
    }

    /// 查询用户组列表
    ///
    /// 获取用户组的基本信息列表
    ///
    /// # 参数
    /// * `req` - 查询用户组列表请求
    ///
    /// # 返回值
    /// 返回用户组列表，支持分页
    pub async fn simple_list(&self, req: &ListGroupsRequest) -> SDKResult<ListGroupsResponse> {
        let mut api_path =
            crate::core::endpoints_original::Endpoints::CONTACT_V3_GROUPS_SIMPLELIST.to_string();

        // 添加查询参数
        let mut query_params = Vec::new();
        if let Some(page_size) = &req.page_size {
            query_params.push(format!("page_size={}", page_size));
        }
        if let Some(page_token) = &req.page_token {
            query_params.push(format!("page_token={}", page_token));
        }
        if let Some(user_id_type) = &req.user_id_type {
            query_params.push(format!("user_id_type={}", user_id_type));
        }
        if let Some(department_id_type) = &req.department_id_type {
            query_params.push(format!("department_id_type={}", department_id_type));
        }

        if !query_params.is_empty() {
            api_path.push('?');
            api_path.push_str(&query_params.join("&"));
        }

        let api_req = ApiRequest {
            http_method: reqwest::Method::GET,
            api_path,
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: Vec::new(),
            ..Default::default()
        };

        let resp = Transport::<ListGroupsResponse>::request(api_req, &self.config, None).await?;
        Ok(resp.data.unwrap_or_default())
    }

    /// 查询用户所属用户组
    ///
    /// 获取指定用户所属的所有用户组
    ///
    /// # 参数
    /// * `req` - 查询用户所属用户组请求
    ///
    /// # 返回值
    /// 返回用户所属的用户组列表
    pub async fn get_user_groups(
        &self,
        req: &GetUserGroupsRequest,
    ) -> SDKResult<GetUserGroupsResponse> {
        let mut api_path =
            crate::core::endpoints_original::Endpoints::CONTACT_V3_GROUPS_MEMBER_BELONG.to_string();

        // 添加查询参数
        let mut query_params = Vec::new();
        if let Some(member_id) = &req.member_id {
            query_params.push(format!("member_id={}", member_id));
        }
        if let Some(member_id_type) = &req.member_id_type {
            query_params.push(format!("member_id_type={}", member_id_type));
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
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: Vec::new(),
            ..Default::default()
        };

        let resp = Transport::<GetUserGroupsResponse>::request(api_req, &self.config, None).await?;
        Ok(resp.data.unwrap_or_default())
    }

    /// 获取用户组详细信息
    ///
    /// 获取用户组的完整信息，包括成员列表
    ///
    /// # 参数
    /// * `group_id` - 用户组ID
    /// * `req` - 获取用户组详细信息请求
    ///
    /// # 返回值
    /// 返回用户组的详细信息
    pub async fn get_detail(
        &self,
        group_id: &str,
        req: &GetGroupDetailRequest,
    ) -> SDKResult<GetGroupDetailResponse> {
        let mut api_path = crate::core::endpoints_original::Endpoints::CONTACT_V3_GROUP_DETAIL
            .replace("{group_id}", group_id);

        // 添加查询参数
        let mut query_params = Vec::new();
        if let Some(user_id_type) = &req.user_id_type {
            query_params.push(format!("user_id_type={}", user_id_type));
        }
        if let Some(department_id_type) = &req.department_id_type {
            query_params.push(format!("department_id_type={}", department_id_type));
        }
        if let Some(include_members) = &req.include_members {
            query_params.push(format!("include_members={}", include_members));
        }

        if !query_params.is_empty() {
            api_path.push('?');
            api_path.push_str(&query_params.join("&"));
        }

        let api_req = ApiRequest {
            http_method: reqwest::Method::GET,
            api_path,
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: Vec::new(),
            ..Default::default()
        };

        let resp =
            Transport::<GetGroupDetailResponse>::request(api_req, &self.config, None).await?;
        Ok(resp.data.unwrap_or_default())
    }

    /// 删除用户组
    ///
    /// 删除指定的用户组（请谨慎操作）
    ///
    /// # 参数
    /// * `group_id` - 用户组ID
    ///
    /// # 返回值
    /// 返回删除操作的结果
    pub async fn delete(&self, group_id: &str) -> SDKResult<DeleteGroupResponse> {
        let api_path = crate::core::endpoints_original::Endpoints::CONTACT_V3_GROUPS
            .replace("{group_id}", group_id);

        let api_req = ApiRequest {
            http_method: reqwest::Method::DELETE,
            api_path,
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: Vec::new(),
            ..Default::default()
        };

        let resp = Transport::<DeleteGroupResponse>::request(api_req, &self.config, None).await?;
        Ok(resp.data.unwrap_or_default())
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
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreateGroupResponse {
    /// 用户组信息
    pub group: Group,
}

impl ApiResponseTrait for CreateGroupResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
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
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PatchGroupResponse {
    /// 用户组信息
    pub group: Group,
}

impl ApiResponseTrait for PatchGroupResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
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
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GetGroupResponse {
    /// 用户组信息
    pub group: Group,
}

impl ApiResponseTrait for GetGroupResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
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
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ListGroupsResponse {
    /// 用户组列表
    pub items: Vec<Group>,
    /// 是否还有更多项目
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
    /// 分页标记
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

impl ApiResponseTrait for ListGroupsResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
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
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GetUserGroupsResponse {
    /// 用户组列表
    pub items: Vec<Group>,
    /// 是否还有更多项目
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
    /// 分页标记
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

impl ApiResponseTrait for GetUserGroupsResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
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
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GetGroupDetailResponse {
    /// 用户组详细信息
    pub group_detail: GroupDetail,
}

impl ApiResponseTrait for GetGroupDetailResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 删除用户组响应
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DeleteGroupResponse {
    /// 操作结果
    pub success: bool,
}

impl ApiResponseTrait for DeleteGroupResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

// ==================== 构建器模式 ====================

/// 创建用户组构建器
#[derive(Debug, Clone)]
pub struct CreateGroupBuilder {
    request: CreateGroupRequest,
}

impl Default for CreateGroupBuilder {
    fn default() -> Self {
        Self {
            request: CreateGroupRequest {
                group: Group::default(),
            },
        }
    }
}

impl CreateGroupBuilder {
    /// 创建新的构建器
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置用户组名称
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.request.group.name = Some(name.into());
        self
    }

    /// 设置用户组描述
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.request.group.description = Some(description.into());
        self
    }

    /// 设置用户组类型
    pub fn group_type(mut self, group_type: impl Into<String>) -> Self {
        self.request.group.group_type = Some(group_type.into());
        self
    }

    /// 执行创建
    pub async fn execute(self, service: &GroupService) -> SDKResult<CreateGroupResponse> {
        service.create(&self.request).await
    }
}

/// 修改用户组构建器
#[derive(Debug, Clone)]
pub struct PatchGroupBuilder {
    request: PatchGroupRequest,
}

impl Default for PatchGroupBuilder {
    fn default() -> Self {
        Self {
            request: PatchGroupRequest {
                group: Group::default(),
            },
        }
    }
}

impl PatchGroupBuilder {
    /// 创建新的构建器
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置用户组名称
    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.request.group.name = Some(name.into());
        self
    }

    /// 设置用户组描述
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.request.group.description = Some(description.into());
        self
    }

    /// 设置用户组类型
    pub fn group_type(mut self, group_type: impl Into<String>) -> Self {
        self.request.group.group_type = Some(group_type.into());
        self
    }

    /// 执行修改
    pub async fn execute(
        self,
        service: &GroupService,
        group_id: &str,
    ) -> SDKResult<PatchGroupResponse> {
        service.patch(group_id, &self.request).await
    }
}

/// 查询用户组列表构建器
#[derive(Debug, Clone)]
pub struct ListGroupsBuilder {
    request: ListGroupsRequest,
}

impl Default for ListGroupsBuilder {
    fn default() -> Self {
        Self {
            request: ListGroupsRequest::default(),
        }
    }
}

impl ListGroupsBuilder {
    /// 创建新的查询构建器
    pub fn new() -> Self {
        Self::default()
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

    /// 设置用户ID类型
    pub fn user_id_type(mut self, user_id_type: impl Into<String>) -> Self {
        self.request.user_id_type = Some(user_id_type.into());
        self
    }

    /// 设置部门ID类型
    pub fn department_id_type(mut self, department_id_type: impl Into<String>) -> Self {
        self.request.department_id_type = Some(department_id_type.into());
        self
    }

    /// 执行查询
    pub async fn execute(self, service: &GroupService) -> SDKResult<ListGroupsResponse> {
        service.simple_list(&self.request).await
    }
}

/// 查询用户所属用户组构建器
#[derive(Debug, Clone)]
pub struct GetUserGroupsBuilder {
    request: GetUserGroupsRequest,
}

impl Default for GetUserGroupsBuilder {
    fn default() -> Self {
        Self {
            request: GetUserGroupsRequest::default(),
        }
    }
}

impl GetUserGroupsBuilder {
    /// 创建新的查询构建器
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置成员ID
    pub fn member_id(mut self, member_id: impl Into<String>) -> Self {
        self.request.member_id = Some(member_id.into());
        self
    }

    /// 设置成员ID类型
    pub fn member_id_type(mut self, member_id_type: impl Into<String>) -> Self {
        self.request.member_id_type = Some(member_id_type.into());
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

    /// 执行查询
    pub async fn execute(self, service: &GroupService) -> SDKResult<GetUserGroupsResponse> {
        service.get_user_groups(&self.request).await
    }
}

/// 获取用户组详细信息构建器
#[derive(Debug, Clone)]
pub struct GetGroupDetailBuilder {
    request: GetGroupDetailRequest,
}

impl Default for GetGroupDetailBuilder {
    fn default() -> Self {
        Self {
            request: GetGroupDetailRequest::default(),
        }
    }
}

impl GetGroupDetailBuilder {
    /// 创建新的查询构建器
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置用户ID类型
    pub fn user_id_type(mut self, user_id_type: impl Into<String>) -> Self {
        self.request.user_id_type = Some(user_id_type.into());
        self
    }

    /// 设置部门ID类型
    pub fn department_id_type(mut self, department_id_type: impl Into<String>) -> Self {
        self.request.department_id_type = Some(department_id_type.into());
        self
    }

    /// 设置是否包含成员信息
    pub fn include_members(mut self, include_members: bool) -> Self {
        self.request.include_members = Some(include_members);
        self
    }

    /// 执行查询
    pub async fn execute(
        self,
        service: &GroupService,
        group_id: &str,
    ) -> SDKResult<GetGroupDetailResponse> {
        service.get_detail(group_id, &self.request).await
    }
}

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

// ==================== 单元测试 ====================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_group_service_creation() {
        let config = Config::default();
        let service = GroupService::new(config);
        assert!(!format!("{:?}", service).is_empty());
    }

    #[test]
    fn test_group_default_creation() {
        let group = Group::default();
        assert_eq!(group.group_id, None);
        assert_eq!(group.name, None);
        assert_eq!(group.description, None);
        assert_eq!(group.group_type, None);
        assert_eq!(group.create_time, None);
        assert_eq!(group.update_time, None);
    }

    #[test]
    fn test_group_with_data() {
        let group = Group {
            group_id: Some("group_123".to_string()),
            name: Some("产品管理组".to_string()),
            description: Some("负责产品规划和管理的用户组".to_string()),
            group_type: Some("custom".to_string()),
            create_time: Some("2023-01-01T00:00:00Z".to_string()),
            update_time: Some("2023-01-02T00:00:00Z".to_string()),
        };

        assert_eq!(group.group_id, Some("group_123".to_string()));
        assert_eq!(group.name, Some("产品管理组".to_string()));
        assert_eq!(
            group.description,
            Some("负责产品规划和管理的用户组".to_string())
        );
        assert_eq!(group.group_type, Some("custom".to_string()));
        assert_eq!(group.create_time, Some("2023-01-01T00:00:00Z".to_string()));
        assert_eq!(group.update_time, Some("2023-01-02T00:00:00Z".to_string()));
    }

    #[test]
    fn test_group_member_default_creation() {
        let member = GroupMember::default();
        assert_eq!(member.member_id, None);
        assert_eq!(member.member_type, None);
        assert_eq!(member.name, None);
        assert_eq!(member.join_time, None);
    }

    #[test]
    fn test_group_member_with_data() {
        let member = GroupMember {
            member_id: Some("user_001".to_string()),
            member_type: Some("user".to_string()),
            name: Some("张三".to_string()),
            join_time: Some("2023-01-01T00:00:00Z".to_string()),
        };

        assert_eq!(member.member_id, Some("user_001".to_string()));
        assert_eq!(member.member_type, Some("user".to_string()));
        assert_eq!(member.name, Some("张三".to_string()));
        assert_eq!(member.join_time, Some("2023-01-01T00:00:00Z".to_string()));
    }

    #[test]
    fn test_group_detail_default_creation() {
        let detail = GroupDetail::default();
        assert_eq!(detail.group.group_id, None);
        assert_eq!(detail.members, None);
        assert_eq!(detail.member_count, None);
    }

    #[test]
    fn test_group_detail_with_data() {
        let group = Group {
            group_id: Some("group_456".to_string()),
            name: Some("开发团队".to_string()),
            ..Default::default()
        };

        let member = GroupMember {
            member_id: Some("user_002".to_string()),
            member_type: Some("user".to_string()),
            name: Some("李四".to_string()),
            ..Default::default()
        };

        let detail = GroupDetail {
            group,
            members: Some(vec![member]),
            member_count: Some(1),
        };

        assert_eq!(detail.group.group_id, Some("group_456".to_string()));
        assert_eq!(detail.group.name, Some("开发团队".to_string()));
        assert_eq!(detail.members.as_ref().unwrap().len(), 1);
        assert_eq!(detail.member_count, Some(1));
    }

    #[test]
    fn test_create_group_request() {
        let group = Group {
            name: Some("新用户组".to_string()),
            description: Some("测试用户组".to_string()),
            ..Default::default()
        };

        let request = CreateGroupRequest { group };

        assert_eq!(request.group.name, Some("新用户组".to_string()));
        assert_eq!(request.group.description, Some("测试用户组".to_string()));
    }

    #[test]
    fn test_list_groups_request_default() {
        let request = ListGroupsRequest::default();
        assert_eq!(request.page_size, None);
        assert_eq!(request.page_token, None);
        assert_eq!(request.user_id_type, None);
        assert_eq!(request.department_id_type, None);
    }

    #[test]
    fn test_list_groups_request_with_all_fields() {
        let request = ListGroupsRequest {
            page_size: Some(50),
            page_token: Some("token_123".to_string()),
            user_id_type: Some("open_id".to_string()),
            department_id_type: Some("department_id".to_string()),
        };

        assert_eq!(request.page_size, Some(50));
        assert_eq!(request.page_token, Some("token_123".to_string()));
        assert_eq!(request.user_id_type, Some("open_id".to_string()));
        assert_eq!(
            request.department_id_type,
            Some("department_id".to_string())
        );
    }

    #[test]
    fn test_get_user_groups_request_default() {
        let request = GetUserGroupsRequest::default();
        assert_eq!(request.member_id, None);
        assert_eq!(request.member_id_type, None);
        assert_eq!(request.page_size, None);
        assert_eq!(request.page_token, None);
    }

    #[test]
    fn test_get_user_groups_request_with_params() {
        let request = GetUserGroupsRequest {
            member_id: Some("user_123".to_string()),
            member_id_type: Some("open_id".to_string()),
            page_size: Some(50),
            page_token: Some("next_page".to_string()),
        };

        assert_eq!(request.member_id, Some("user_123".to_string()));
        assert_eq!(request.member_id_type, Some("open_id".to_string()));
        assert_eq!(request.page_size, Some(50));
        assert_eq!(request.page_token, Some("next_page".to_string()));
    }

    #[test]
    fn test_response_default_creation() {
        let create_response = CreateGroupResponse::default();
        assert_eq!(create_response.group.group_id, None);

        let patch_response = PatchGroupResponse::default();
        assert_eq!(patch_response.group.group_id, None);

        let get_response = GetGroupResponse::default();
        assert_eq!(get_response.group.group_id, None);

        let list_response = ListGroupsResponse::default();
        assert_eq!(list_response.items.len(), 0);
        assert_eq!(list_response.has_more, None);
        assert_eq!(list_response.page_token, None);

        let get_user_groups_response = GetUserGroupsResponse::default();
        assert_eq!(get_user_groups_response.items.len(), 0);
        assert_eq!(get_user_groups_response.has_more, None);
        assert_eq!(get_user_groups_response.page_token, None);

        let get_detail_response = GetGroupDetailResponse::default();
        assert_eq!(get_detail_response.group_detail.group.group_id, None);

        let delete_response = DeleteGroupResponse::default();
        assert_eq!(delete_response.success, false);
    }

    #[test]
    fn test_api_response_trait_implementation() {
        assert_eq!(CreateGroupResponse::data_format(), ResponseFormat::Data);
        assert_eq!(PatchGroupResponse::data_format(), ResponseFormat::Data);
        assert_eq!(GetGroupResponse::data_format(), ResponseFormat::Data);
        assert_eq!(ListGroupsResponse::data_format(), ResponseFormat::Data);
        assert_eq!(GetUserGroupsResponse::data_format(), ResponseFormat::Data);
        assert_eq!(GetGroupDetailResponse::data_format(), ResponseFormat::Data);
        assert_eq!(DeleteGroupResponse::data_format(), ResponseFormat::Data);
    }

    #[test]
    fn test_request_serialization() {
        let group = Group {
            name: Some("序列化测试".to_string()),
            description: Some("测试序列化功能".to_string()),
            group_type: Some("test".to_string()),
            ..Default::default()
        };

        let create_request = CreateGroupRequest { group };

        let serialized = serde_json::to_string(&create_request).unwrap();
        let deserialized: CreateGroupRequest = serde_json::from_str(&serialized).unwrap();

        assert_eq!(create_request.group.name, deserialized.group.name);
        assert_eq!(
            create_request.group.description,
            deserialized.group.description
        );
        assert_eq!(
            create_request.group.group_type,
            deserialized.group.group_type
        );

        let list_request = ListGroupsRequest {
            page_size: Some(100),
            page_token: Some("test_token".to_string()),
            user_id_type: Some("open_id".to_string()),
            department_id_type: Some("department_id".to_string()),
        };

        let serialized = serde_json::to_string(&list_request).unwrap();
        let deserialized: ListGroupsRequest = serde_json::from_str(&serialized).unwrap();

        assert_eq!(list_request.page_size, deserialized.page_size);
        assert_eq!(list_request.page_token, deserialized.page_token);
        assert_eq!(list_request.user_id_type, deserialized.user_id_type);
        assert_eq!(
            list_request.department_id_type,
            deserialized.department_id_type
        );
    }

    #[test]
    fn test_query_parameters_construction() {
        let request = ListGroupsRequest {
            page_size: Some(20),
            page_token: Some("test_token".to_string()),
            user_id_type: Some("open_id".to_string()),
            department_id_type: Some("department_id".to_string()),
        };

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

        assert_eq!(query_params.len(), 4);
        assert!(query_params.contains(&"page_size=20".to_string()));
        assert!(query_params.contains(&"page_token=test_token".to_string()));
        assert!(query_params.contains(&"user_id_type=open_id".to_string()));
        assert!(query_params.contains(&"department_id_type=department_id".to_string()));
    }

    #[test]
    fn test_endpoint_constants() {
        // Test that the endpoint constants are properly defined
        assert_eq!(
            crate::core::endpoints_original::Endpoints::CONTACT_V3_GROUPS,
            "/open-apis/contact/v3/groups"
        );
        assert_eq!(
            crate::core::endpoints_original::Endpoints::CONTACT_V3_GROUPS_SIMPLELIST,
            "/open-apis/contact/v3/groups/simplelist"
        );
        assert_eq!(
            crate::core::endpoints_original::Endpoints::CONTACT_V3_GROUPS_MEMBER_BELONG,
            "/open-apis/contact/v3/groups/member_belong"
        );
        assert_eq!(
            crate::core::endpoints_original::Endpoints::CONTACT_V3_GROUP_DETAIL,
            "/open-apis/contact/v3/groups/{group_id}/detail"
        );
    }

    #[test]
    fn test_create_group_builder() {
        let builder = CreateGroupBuilder::new()
            .name("构建器测试组")
            .description("使用构建器创建的测试用户组")
            .group_type("test");

        assert_eq!(builder.request.group.name, Some("构建器测试组".to_string()));
        assert_eq!(
            builder.request.group.description,
            Some("使用构建器创建的测试用户组".to_string())
        );
        assert_eq!(builder.request.group.group_type, Some("test".to_string()));
    }

    #[test]
    fn test_create_group_builder_default() {
        let builder = CreateGroupBuilder::default();
        assert_eq!(builder.request.group.name, None);
        assert_eq!(builder.request.group.description, None);
        assert_eq!(builder.request.group.group_type, None);
    }

    #[test]
    fn test_patch_group_builder() {
        let builder = PatchGroupBuilder::new()
            .name("修改后的名称")
            .description("修改后的描述")
            .group_type("modified");

        assert_eq!(builder.request.group.name, Some("修改后的名称".to_string()));
        assert_eq!(
            builder.request.group.description,
            Some("修改后的描述".to_string())
        );
        assert_eq!(
            builder.request.group.group_type,
            Some("modified".to_string())
        );
    }

    #[test]
    fn test_list_groups_builder() {
        let builder = ListGroupsBuilder::new()
            .page_size(30)
            .page_token("builder_token")
            .user_id_type("open_id")
            .department_id_type("department_id");

        assert_eq!(builder.request.page_size, Some(30));
        assert_eq!(
            builder.request.page_token,
            Some("builder_token".to_string())
        );
        assert_eq!(builder.request.user_id_type, Some("open_id".to_string()));
        assert_eq!(
            builder.request.department_id_type,
            Some("department_id".to_string())
        );
    }

    #[test]
    fn test_list_groups_builder_default() {
        let builder = ListGroupsBuilder::default();
        assert_eq!(builder.request.page_size, None);
        assert_eq!(builder.request.page_token, None);
        assert_eq!(builder.request.user_id_type, None);
        assert_eq!(builder.request.department_id_type, None);
    }

    #[test]
    fn test_get_user_groups_builder() {
        let builder = GetUserGroupsBuilder::new()
            .member_id("user_001")
            .member_id_type("open_id")
            .page_size(20)
            .page_token("page_token");

        assert_eq!(builder.request.member_id, Some("user_001".to_string()));
        assert_eq!(builder.request.member_id_type, Some("open_id".to_string()));
        assert_eq!(builder.request.page_size, Some(20));
        assert_eq!(builder.request.page_token, Some("page_token".to_string()));
    }

    #[test]
    fn test_get_user_groups_builder_default() {
        let builder = GetUserGroupsBuilder::default();
        assert_eq!(builder.request.member_id, None);
        assert_eq!(builder.request.member_id_type, None);
        assert_eq!(builder.request.page_size, None);
        assert_eq!(builder.request.page_token, None);
    }

    #[test]
    fn test_get_group_detail_builder() {
        let builder = GetGroupDetailBuilder::new()
            .user_id_type("open_id")
            .department_id_type("department_id")
            .include_members(true);

        assert_eq!(builder.request.user_id_type, Some("open_id".to_string()));
        assert_eq!(
            builder.request.department_id_type,
            Some("department_id".to_string())
        );
        assert_eq!(builder.request.include_members, Some(true));
    }

    #[test]
    fn test_get_group_detail_builder_default() {
        let builder = GetGroupDetailBuilder::default();
        assert_eq!(builder.request.user_id_type, None);
        assert_eq!(builder.request.department_id_type, None);
        assert_eq!(builder.request.include_members, None);
    }

    #[test]
    fn test_group_type_variations() {
        // Test different group types
        let admin_group = Group {
            group_id: Some("admin_group".to_string()),
            name: Some("管理员组".to_string()),
            group_type: Some("admin".to_string()),
            ..Default::default()
        };

        let dev_group = Group {
            group_id: Some("dev_group".to_string()),
            name: Some("开发组".to_string()),
            group_type: Some("development".to_string()),
            ..Default::default()
        };

        let custom_group = Group {
            group_id: Some("custom_group".to_string()),
            name: Some("自定义组".to_string()),
            group_type: Some("custom".to_string()),
            ..Default::default()
        };

        assert_eq!(admin_group.group_type, Some("admin".to_string()));
        assert_eq!(dev_group.group_type, Some("development".to_string()));
        assert_eq!(custom_group.group_type, Some("custom".to_string()));
    }

    #[test]
    fn test_group_member_type_variations() {
        // Test different member types
        let user_member = GroupMember {
            member_id: Some("user_001".to_string()),
            member_type: Some("user".to_string()),
            ..Default::default()
        };

        let dept_member = GroupMember {
            member_id: Some("dept_001".to_string()),
            member_type: Some("department".to_string()),
            ..Default::default()
        };

        let group_member = GroupMember {
            member_id: Some("group_001".to_string()),
            member_type: Some("group".to_string()),
            ..Default::default()
        };

        assert_eq!(user_member.member_type, Some("user".to_string()));
        assert_eq!(dept_member.member_type, Some("department".to_string()));
        assert_eq!(group_member.member_type, Some("group".to_string()));
    }

    #[test]
    fn test_comprehensive_group_data() {
        // Test comprehensive group data with all fields
        let comprehensive_group = Group {
            group_id: Some("comprehensive_001".to_string()),
            name: Some("综合测试组".to_string()),
            description: Some(
                "这是一个用于全面测试的用户组，包含所有可能的字段和数据类型".to_string(),
            ),
            group_type: Some("comprehensive".to_string()),
            create_time: Some("2023-01-01T08:00:00Z".to_string()),
            update_time: Some("2023-12-31T16:00:00Z".to_string()),
        };

        assert_eq!(
            comprehensive_group.group_id,
            Some("comprehensive_001".to_string())
        );
        assert_eq!(comprehensive_group.name, Some("综合测试组".to_string()));
        assert_eq!(
            comprehensive_group.description,
            Some("这是一个用于全面测试的用户组，包含所有可能的字段和数据类型".to_string())
        );
        assert_eq!(
            comprehensive_group.group_type,
            Some("comprehensive".to_string())
        );
        assert_eq!(
            comprehensive_group.create_time,
            Some("2023-01-01T08:00:00Z".to_string())
        );
        assert_eq!(
            comprehensive_group.update_time,
            Some("2023-12-31T16:00:00Z".to_string())
        );
    }
}

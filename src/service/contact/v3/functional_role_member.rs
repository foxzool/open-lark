#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(non_snake_case)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::module_inception)]
//! 职能角色成员服务
//!
//! 提供完整的职能角色成员管理功能：
//! - 添加角色成员
//! - 批量添加角色成员
//! - 设置角色成员管理范围
//! - 查询角色成员信息
//! - 查询角色下所有成员
//! - 批量删除角色成员
//! - 支持分页查询

use crate::{
    api_resp::{ApiResponseTrait, ResponseFormat},
    config::Config,
    constants::AccessTokenType,
    http::Transport,
    ApiRequest, SDKResult,
};
use serde::{Deserialize, Serialize};

/// 角色成员详细信息
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RoleMemberDetail {
    /// 成员名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 成员邮箱
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// 成员手机号
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mobile: Option<String>,
    /// 头像URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar: Option<String>,
    /// 用户状态
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

impl Default for RoleMemberDetail {
    fn default() -> Self {
        Self {
            name: None,
            email: None,
            mobile: None,
            avatar: None,
            status: None,
        }
    }
}

/// 角色成员信息
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RoleMember {
    /// 成员ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_id: Option<String>,
    /// 成员类型 (user/department)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_type: Option<String>,
    /// 成员详细信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_info: Option<RoleMemberDetail>,
    /// 管理范围列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scopes: Option<Vec<String>>,
    /// 加入时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub join_time: Option<String>,
    /// 更新时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_time: Option<String>,
}

impl Default for RoleMember {
    fn default() -> Self {
        Self {
            member_id: None,
            member_type: None,
            member_info: None,
            scopes: None,
            join_time: None,
            update_time: None,
        }
    }
}

/// 角色成员信息（用于创建）
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RoleMemberInfo {
    /// 成员ID
    pub member_id: String,
    /// 成员类型 (user/department)
    pub member_type: String,
    /// 管理范围
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scopes: Option<Vec<String>>,
}

/// 角色成员管理范围
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RoleMemberScope {
    /// 成员ID
    pub member_id: String,
    /// 管理范围列表
    pub scopes: Vec<String>,
}

/// 角色成员操作结果
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RoleMemberResult {
    /// 成员ID
    pub member_id: String,
    /// 操作是否成功
    pub success: bool,
    /// 错误信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

/// 职能角色成员服务
#[derive(Debug, Clone)]
pub struct FunctionalRoleMemberService {
    config: Config,
}

impl FunctionalRoleMemberService {
    /// 创建新的职能角色成员服务实例
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 添加角色成员
    ///
    /// 向指定职能角色添加单个成员
    ///
    /// # 参数
    /// * `role_id` - 角色ID
    /// * `req` - 添加角色成员请求
    ///
    /// # 返回值
    /// 返回添加成功的角色成员信息
    pub async fn create(
        &self,
        role_id: &str,
        req: &CreateRoleMemberRequest,
    ) -> SDKResult<CreateRoleMemberResponse> {
        let api_path =
            crate::core::endpoints_original::Endpoints::CONTACT_V3_FUNCTIONAL_ROLE_MEMBERS
                .replace("{role_id}", role_id);

        let api_req = ApiRequest {
            http_method: reqwest::Method::POST,
            api_path,
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: serde_json::to_vec(req)?,
            ..Default::default()
        };

        let resp =
            Transport::<CreateRoleMemberResponse>::request(api_req, &self.config, None).await?;
        Ok(resp.data.unwrap_or_default())
    }

    /// 批量添加角色成员
    ///
    /// 向指定职能角色批量添加多个成员
    ///
    /// # 参数
    /// * `role_id` - 角色ID
    /// * `req` - 批量添加角色成员请求
    ///
    /// # 返回值
    /// 返回批量添加操作的结果
    pub async fn batch_create(
        &self,
        role_id: &str,
        req: &BatchCreateRoleMembersRequest,
    ) -> SDKResult<BatchCreateRoleMembersResponse> {
        let api_path = crate::core::endpoints_original::Endpoints::CONTACT_V3_FUNCTIONAL_ROLE_MEMBERS_BATCH_CREATE
            .replace("{role_id}", role_id);

        let api_req = ApiRequest {
            http_method: reqwest::Method::POST,
            api_path,
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: serde_json::to_vec(req)?,
            ..Default::default()
        };

        let resp =
            Transport::<BatchCreateRoleMembersResponse>::request(api_req, &self.config, None)
                .await?;
        Ok(resp.data.unwrap_or_default())
    }

    /// 批量设置角色成员管理范围
    ///
    /// 批量设置角色成员的管理权限范围
    ///
    /// # 参数
    /// * `role_id` - 角色ID
    /// * `req` - 设置角色成员管理范围请求
    ///
    /// # 返回值
    /// 返回设置操作的结果
    pub async fn set_scopes(
        &self,
        role_id: &str,
        req: &SetRoleMemberScopesRequest,
    ) -> SDKResult<SetRoleMemberScopesResponse> {
        let api_path =
            crate::core::endpoints_original::Endpoints::CONTACT_V3_FUNCTIONAL_ROLE_MEMBERS_SCOPES
                .replace("{role_id}", role_id);

        let api_req = ApiRequest {
            http_method: reqwest::Method::POST,
            api_path,
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: serde_json::to_vec(req)?,
            ..Default::default()
        };

        let resp =
            Transport::<SetRoleMemberScopesResponse>::request(api_req, &self.config, None).await?;
        Ok(resp.data.unwrap_or_default())
    }

    /// 查询角色下某个成员的信息
    ///
    /// 根据角色ID和成员ID获取指定成员的详细信息
    ///
    /// # 参数
    /// * `role_id` - 角色ID
    /// * `member_id` - 成员ID
    /// * `req` - 查询角色成员请求
    ///
    /// # 返回值
    /// 返回角色成员的详细信息
    pub async fn get(
        &self,
        role_id: &str,
        member_id: &str,
        req: &GetRoleMemberRequest,
    ) -> SDKResult<GetRoleMemberResponse> {
        let mut api_path =
            crate::core::endpoints_original::Endpoints::CONTACT_V3_FUNCTIONAL_ROLE_MEMBER_GET
                .replace("{role_id}", role_id);
        api_path = api_path.replace("{member_id}", member_id);

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

        let resp = Transport::<GetRoleMemberResponse>::request(api_req, &self.config, None).await?;
        Ok(resp.data.unwrap_or_default())
    }

    /// 查询角色下的所有成员信息
    ///
    /// 获取指定职能角色下的所有成员列表，支持分页查询
    ///
    /// # 参数
    /// * `role_id` - 角色ID
    /// * `req` - 查询角色成员列表请求
    ///
    /// # 返回值
    /// 返回角色成员列表，支持分页
    pub async fn list(
        &self,
        role_id: &str,
        req: &ListRoleMembersRequest,
    ) -> SDKResult<ListRoleMembersResponse> {
        let mut api_path =
            crate::core::endpoints_original::Endpoints::CONTACT_V3_FUNCTIONAL_ROLE_MEMBERS
                .replace("{role_id}", role_id);

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

        let resp =
            Transport::<ListRoleMembersResponse>::request(api_req, &self.config, None).await?;
        Ok(resp.data.unwrap_or_default())
    }

    /// 批量删除角色成员
    ///
    /// 从指定职能角色中批量删除多个成员
    ///
    /// # 参数
    /// * `role_id` - 角色ID
    /// * `req` - 批量删除角色成员请求
    ///
    /// # 返回值
    /// 返回批量删除操作的结果
    pub async fn batch_delete(
        &self,
        role_id: &str,
        req: &BatchDeleteRoleMembersRequest,
    ) -> SDKResult<BatchDeleteRoleMembersResponse> {
        let api_path = crate::core::endpoints_original::Endpoints::CONTACT_V3_FUNCTIONAL_ROLE_MEMBERS_BATCH_DELETE
            .replace("{role_id}", role_id);

        let api_req = ApiRequest {
            http_method: reqwest::Method::POST,
            api_path,
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: serde_json::to_vec(req)?,
            ..Default::default()
        };

        let resp =
            Transport::<BatchDeleteRoleMembersResponse>::request(api_req, &self.config, None)
                .await?;
        Ok(resp.data.unwrap_or_default())
    }
}

// ==================== 数据模型 ====================

/// 添加角色成员请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateRoleMemberRequest {
    /// 成员信息
    pub member: RoleMemberInfo,
    /// 用户ID类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id_type: Option<String>,
    /// 部门ID类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department_id_type: Option<String>,
}

/// 添加角色成员响应
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreateRoleMemberResponse {
    /// 成员ID
    pub member_id: String,
    /// 成员信息
    pub member: RoleMember,
}

impl ApiResponseTrait for CreateRoleMemberResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 批量添加角色成员请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchCreateRoleMembersRequest {
    /// 成员列表
    pub members: Vec<RoleMemberInfo>,
    /// 用户ID类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id_type: Option<String>,
    /// 部门ID类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department_id_type: Option<String>,
}

/// 批量添加角色成员响应
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BatchCreateRoleMembersResponse {
    /// 操作结果列表
    pub results: Vec<RoleMemberResult>,
    /// 成功添加的成员数量
    pub success_count: i32,
    /// 失败的成员数量
    pub failed_count: i32,
}

impl ApiResponseTrait for BatchCreateRoleMembersResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 设置角色成员管理范围请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetRoleMemberScopesRequest {
    /// 成员管理范围列表
    pub members: Vec<RoleMemberScope>,
    /// 用户ID类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id_type: Option<String>,
    /// 部门ID类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department_id_type: Option<String>,
}

/// 设置角色成员管理范围响应
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SetRoleMemberScopesResponse {
    /// 操作结果列表
    pub results: Vec<RoleMemberResult>,
    /// 成功设置的成员数量
    pub success_count: i32,
    /// 失败的成员数量
    pub failed_count: i32,
}

impl ApiResponseTrait for SetRoleMemberScopesResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 查询角色成员请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetRoleMemberRequest {
    /// 用户ID类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id_type: Option<String>,
    /// 部门ID类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department_id_type: Option<String>,
}

impl Default for GetRoleMemberRequest {
    fn default() -> Self {
        Self {
            user_id_type: None,
            department_id_type: None,
        }
    }
}

/// 查询角色成员响应
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GetRoleMemberResponse {
    /// 角色成员信息
    pub member: RoleMember,
}

impl ApiResponseTrait for GetRoleMemberResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 查询角色成员列表请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListRoleMembersRequest {
    /// 分页大小
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 分页标记
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// 用户ID类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id_type: Option<String>,
    /// 部门ID类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department_id_type: Option<String>,
}

impl Default for ListRoleMembersRequest {
    fn default() -> Self {
        Self {
            page_size: None,
            page_token: None,
            user_id_type: None,
            department_id_type: None,
        }
    }
}

/// 查询角色成员列表响应
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ListRoleMembersResponse {
    /// 角色成员列表
    pub items: Vec<RoleMember>,
    /// 是否还有更多项目
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
    /// 分页标记
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

impl ApiResponseTrait for ListRoleMembersResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 批量删除角色成员请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchDeleteRoleMembersRequest {
    /// 成员ID列表
    pub member_ids: Vec<String>,
    /// 用户ID类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id_type: Option<String>,
}

/// 批量删除角色成员响应
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BatchDeleteRoleMembersResponse {
    /// 操作结果列表
    pub results: Vec<RoleMemberResult>,
    /// 成功删除的成员数量
    pub success_count: i32,
    /// 失败的成员数量
    pub failed_count: i32,
}

impl ApiResponseTrait for BatchDeleteRoleMembersResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

// ==================== 构建器模式 ====================

/// 创建角色成员构建器
#[derive(Debug, Clone)]
pub struct CreateRoleMemberBuilder {
    request: CreateRoleMemberRequest,
}

impl Default for CreateRoleMemberBuilder {
    fn default() -> Self {
        Self {
            request: CreateRoleMemberRequest {
                member: RoleMemberInfo {
                    member_id: String::new(),
                    member_type: String::new(),
                    scopes: None,
                },
                user_id_type: None,
                department_id_type: None,
            },
        }
    }
}

impl CreateRoleMemberBuilder {
    /// 创建新的构建器
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置成员信息
    pub fn member(mut self, member: RoleMemberInfo) -> Self {
        self.request.member = member;
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

    /// 执行创建
    pub async fn execute(
        self,
        service: &FunctionalRoleMemberService,
        role_id: &str,
    ) -> SDKResult<CreateRoleMemberResponse> {
        service.create(role_id, &self.request).await
    }
}

/// 查询角色成员列表构建器
#[derive(Debug, Clone)]
pub struct ListRoleMembersBuilder {
    request: ListRoleMembersRequest,
}

impl Default for ListRoleMembersBuilder {
    fn default() -> Self {
        Self {
            request: ListRoleMembersRequest::default(),
        }
    }
}

impl ListRoleMembersBuilder {
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
    pub async fn execute(
        self,
        service: &FunctionalRoleMemberService,
        role_id: &str,
    ) -> SDKResult<ListRoleMembersResponse> {
        service.list(role_id, &self.request).await
    }
}

impl FunctionalRoleMemberService {
    /// 创建角色成员构建器
    pub fn create_role_member_builder(&self) -> CreateRoleMemberBuilder {
        CreateRoleMemberBuilder::new()
    }

    /// 创建查询构建器
    pub fn list_role_members_builder(&self) -> ListRoleMembersBuilder {
        ListRoleMembersBuilder::new()
    }
}

// ==================== 单元测试 ====================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_functional_role_member_service_creation() {
        let config = Config::default();
        let service = FunctionalRoleMemberService::new(config);
        assert!(!format!("{:?}", service).is_empty());
    }

    #[test]
    fn test_role_member_detail_default_creation() {
        let detail = RoleMemberDetail::default();
        assert_eq!(detail.name, None);
        assert_eq!(detail.email, None);
        assert_eq!(detail.mobile, None);
        assert_eq!(detail.avatar, None);
        assert_eq!(detail.status, None);
    }

    #[test]
    fn test_role_member_detail_with_data() {
        let detail = RoleMemberDetail {
            name: Some("张三".to_string()),
            email: Some("zhangsan@example.com".to_string()),
            mobile: Some("13800138000".to_string()),
            avatar: Some("https://example.com/avatar.jpg".to_string()),
            status: Some("active".to_string()),
        };

        assert_eq!(detail.name, Some("张三".to_string()));
        assert_eq!(detail.email, Some("zhangsan@example.com".to_string()));
        assert_eq!(detail.mobile, Some("13800138000".to_string()));
        assert_eq!(
            detail.avatar,
            Some("https://example.com/avatar.jpg".to_string())
        );
        assert_eq!(detail.status, Some("active".to_string()));
    }

    #[test]
    fn test_role_member_default_creation() {
        let member = RoleMember::default();
        assert_eq!(member.member_id, None);
        assert_eq!(member.member_type, None);
        assert_eq!(member.member_info, None);
        assert_eq!(member.scopes, None);
        assert_eq!(member.join_time, None);
        assert_eq!(member.update_time, None);
    }

    #[test]
    fn test_role_member_with_data() {
        let detail = RoleMemberDetail {
            name: Some("李四".to_string()),
            email: Some("lisi@example.com".to_string()),
            ..Default::default()
        };

        let member = RoleMember {
            member_id: Some("member_123".to_string()),
            member_type: Some("user".to_string()),
            member_info: Some(detail),
            scopes: Some(vec!["scope1".to_string(), "scope2".to_string()]),
            join_time: Some("2023-01-01T00:00:00Z".to_string()),
            update_time: Some("2023-01-02T00:00:00Z".to_string()),
        };

        assert_eq!(member.member_id, Some("member_123".to_string()));
        assert_eq!(member.member_type, Some("user".to_string()));
        assert!(member.member_info.is_some());
        assert_eq!(
            member.scopes,
            Some(vec!["scope1".to_string(), "scope2".to_string()])
        );
        assert_eq!(member.join_time, Some("2023-01-01T00:00:00Z".to_string()));
        assert_eq!(member.update_time, Some("2023-01-02T00:00:00Z".to_string()));
    }

    #[test]
    fn test_role_member_info() {
        let info = RoleMemberInfo {
            member_id: "dept_001".to_string(),
            member_type: "department".to_string(),
            scopes: Some(vec!["admin".to_string(), "read".to_string()]),
        };

        assert_eq!(info.member_id, "dept_001");
        assert_eq!(info.member_type, "department");
        assert_eq!(
            info.scopes,
            Some(vec!["admin".to_string(), "read".to_string()])
        );
    }

    #[test]
    fn test_role_member_scope() {
        let scope = RoleMemberScope {
            member_id: "user_001".to_string(),
            scopes: vec!["write".to_string(), "delete".to_string()],
        };

        assert_eq!(scope.member_id, "user_001");
        assert_eq!(
            scope.scopes,
            vec!["write".to_string(), "delete".to_string()]
        );
    }

    #[test]
    fn test_role_member_result() {
        let success_result = RoleMemberResult {
            member_id: "user_001".to_string(),
            success: true,
            error: None,
        };

        let error_result = RoleMemberResult {
            member_id: "user_002".to_string(),
            success: false,
            error: Some("用户不存在".to_string()),
        };

        assert_eq!(success_result.member_id, "user_001");
        assert_eq!(success_result.success, true);
        assert_eq!(success_result.error, None);

        assert_eq!(error_result.member_id, "user_002");
        assert_eq!(error_result.success, false);
        assert_eq!(error_result.error, Some("用户不存在".to_string()));
    }

    #[test]
    fn test_create_role_member_request() {
        let member_info = RoleMemberInfo {
            member_id: "user_001".to_string(),
            member_type: "user".to_string(),
            scopes: Some(vec!["read".to_string()]),
        };

        let request = CreateRoleMemberRequest {
            member: member_info,
            user_id_type: Some("open_id".to_string()),
            department_id_type: None,
        };

        assert_eq!(request.member.member_id, "user_001");
        assert_eq!(request.member.member_type, "user");
        assert_eq!(request.user_id_type, Some("open_id".to_string()));
        assert_eq!(request.department_id_type, None);
    }

    #[test]
    fn test_batch_create_role_members_request() {
        let member1 = RoleMemberInfo {
            member_id: "user_001".to_string(),
            member_type: "user".to_string(),
            scopes: None,
        };

        let member2 = RoleMemberInfo {
            member_id: "dept_001".to_string(),
            member_type: "department".to_string(),
            scopes: Some(vec!["admin".to_string()]),
        };

        let request = BatchCreateRoleMembersRequest {
            members: vec![member1, member2],
            user_id_type: Some("user_id".to_string()),
            department_id_type: Some("department_id".to_string()),
        };

        assert_eq!(request.members.len(), 2);
        assert_eq!(request.members[0].member_id, "user_001");
        assert_eq!(request.members[1].member_id, "dept_001");
        assert_eq!(request.user_id_type, Some("user_id".to_string()));
        assert_eq!(
            request.department_id_type,
            Some("department_id".to_string())
        );
    }

    #[test]
    fn test_set_role_member_scopes_request() {
        let scope1 = RoleMemberScope {
            member_id: "user_001".to_string(),
            scopes: vec!["read".to_string(), "write".to_string()],
        };

        let scope2 = RoleMemberScope {
            member_id: "user_002".to_string(),
            scopes: vec!["read".to_string()],
        };

        let request = SetRoleMemberScopesRequest {
            members: vec![scope1, scope2],
            user_id_type: Some("open_id".to_string()),
            department_id_type: None,
        };

        assert_eq!(request.members.len(), 2);
        assert_eq!(request.members[0].member_id, "user_001");
        assert_eq!(request.members[0].scopes.len(), 2);
        assert_eq!(request.members[1].member_id, "user_002");
        assert_eq!(request.members[1].scopes.len(), 1);
    }

    #[test]
    fn test_get_role_member_request_default() {
        let request = GetRoleMemberRequest::default();
        assert_eq!(request.user_id_type, None);
        assert_eq!(request.department_id_type, None);
    }

    #[test]
    fn test_get_role_member_request_with_types() {
        let request = GetRoleMemberRequest {
            user_id_type: Some("open_id".to_string()),
            department_id_type: Some("department_id".to_string()),
        };

        assert_eq!(request.user_id_type, Some("open_id".to_string()));
        assert_eq!(
            request.department_id_type,
            Some("department_id".to_string())
        );
    }

    #[test]
    fn test_list_role_members_request_default() {
        let request = ListRoleMembersRequest::default();
        assert_eq!(request.page_size, None);
        assert_eq!(request.page_token, None);
        assert_eq!(request.user_id_type, None);
        assert_eq!(request.department_id_type, None);
    }

    #[test]
    fn test_list_role_members_request_with_all_fields() {
        let request = ListRoleMembersRequest {
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
    fn test_batch_delete_role_members_request() {
        let request = BatchDeleteRoleMembersRequest {
            member_ids: vec![
                "user_001".to_string(),
                "user_002".to_string(),
                "dept_001".to_string(),
            ],
            user_id_type: Some("open_id".to_string()),
        };

        assert_eq!(request.member_ids.len(), 3);
        assert_eq!(request.member_ids[0], "user_001");
        assert_eq!(request.member_ids[1], "user_002");
        assert_eq!(request.member_ids[2], "dept_001");
        assert_eq!(request.user_id_type, Some("open_id".to_string()));
    }

    #[test]
    fn test_response_default_creation() {
        let create_response = CreateRoleMemberResponse::default();
        assert_eq!(create_response.member_id, "");
        assert_eq!(create_response.member.member_id, None);

        let batch_create_response = BatchCreateRoleMembersResponse::default();
        assert_eq!(batch_create_response.results.len(), 0);
        assert_eq!(batch_create_response.success_count, 0);
        assert_eq!(batch_create_response.failed_count, 0);

        let set_scopes_response = SetRoleMemberScopesResponse::default();
        assert_eq!(set_scopes_response.results.len(), 0);
        assert_eq!(set_scopes_response.success_count, 0);
        assert_eq!(set_scopes_response.failed_count, 0);

        let get_response = GetRoleMemberResponse::default();
        assert_eq!(get_response.member.member_id, None);

        let list_response = ListRoleMembersResponse::default();
        assert_eq!(list_response.items.len(), 0);
        assert_eq!(list_response.has_more, None);
        assert_eq!(list_response.page_token, None);

        let batch_delete_response = BatchDeleteRoleMembersResponse::default();
        assert_eq!(batch_delete_response.results.len(), 0);
        assert_eq!(batch_delete_response.success_count, 0);
        assert_eq!(batch_delete_response.failed_count, 0);
    }

    #[test]
    fn test_api_response_trait_implementation() {
        assert_eq!(
            CreateRoleMemberResponse::data_format(),
            ResponseFormat::Data
        );
        assert_eq!(
            BatchCreateRoleMembersResponse::data_format(),
            ResponseFormat::Data
        );
        assert_eq!(
            SetRoleMemberScopesResponse::data_format(),
            ResponseFormat::Data
        );
        assert_eq!(GetRoleMemberResponse::data_format(), ResponseFormat::Data);
        assert_eq!(ListRoleMembersResponse::data_format(), ResponseFormat::Data);
        assert_eq!(
            BatchDeleteRoleMembersResponse::data_format(),
            ResponseFormat::Data
        );
    }

    #[test]
    fn test_request_serialization() {
        let member_info = RoleMemberInfo {
            member_id: "user_001".to_string(),
            member_type: "user".to_string(),
            scopes: Some(vec!["read".to_string(), "write".to_string()]),
        };

        let create_request = CreateRoleMemberRequest {
            member: member_info.clone(),
            user_id_type: Some("open_id".to_string()),
            department_id_type: None,
        };

        let serialized = serde_json::to_string(&create_request).unwrap();
        let deserialized: CreateRoleMemberRequest = serde_json::from_str(&serialized).unwrap();

        assert_eq!(
            create_request.member.member_id,
            deserialized.member.member_id
        );
        assert_eq!(
            create_request.member.member_type,
            deserialized.member.member_type
        );
        assert_eq!(create_request.user_id_type, deserialized.user_id_type);

        let list_request = ListRoleMembersRequest {
            page_size: Some(100),
            page_token: Some("test_token".to_string()),
            user_id_type: Some("open_id".to_string()),
            department_id_type: Some("department_id".to_string()),
        };

        let serialized = serde_json::to_string(&list_request).unwrap();
        let deserialized: ListRoleMembersRequest = serde_json::from_str(&serialized).unwrap();

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
        let request = ListRoleMembersRequest {
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
            crate::core::endpoints_original::Endpoints::CONTACT_V3_FUNCTIONAL_ROLE_MEMBERS,
            "/open-apis/contact/v3/functional_roles/{role_id}/members"
        );
        assert_eq!(
            crate::core::endpoints_original::Endpoints::CONTACT_V3_FUNCTIONAL_ROLE_MEMBER_GET,
            "/open-apis/contact/v3/functional_roles/{role_id}/members/{member_id}"
        );
        assert_eq!(
            crate::core::endpoints_original::Endpoints::CONTACT_V3_FUNCTIONAL_ROLE_MEMBERS_BATCH_CREATE,
            "/open-apis/contact/v3/functional_roles/{role_id}/members/batch_create"
        );
        assert_eq!(
            crate::core::endpoints_original::Endpoints::CONTACT_V3_FUNCTIONAL_ROLE_MEMBERS_BATCH_DELETE,
            "/open-apis/contact/v3/functional_roles/{role_id}/members/batch_delete"
        );
        assert_eq!(
            crate::core::endpoints_original::Endpoints::CONTACT_V3_FUNCTIONAL_ROLE_MEMBERS_SCOPES,
            "/open-apis/contact/v3/functional_roles/{role_id}/members/scopes"
        );
    }

    #[test]
    fn test_create_role_member_builder() {
        let member_info = RoleMemberInfo {
            member_id: "user_001".to_string(),
            member_type: "user".to_string(),
            scopes: Some(vec!["read".to_string()]),
        };

        let builder = CreateRoleMemberBuilder::new()
            .member(member_info)
            .user_id_type("open_id")
            .department_id_type("department_id");

        assert_eq!(builder.request.member.member_id, "user_001");
        assert_eq!(builder.request.member.member_type, "user");
        assert_eq!(builder.request.user_id_type, Some("open_id".to_string()));
        assert_eq!(
            builder.request.department_id_type,
            Some("department_id".to_string())
        );
    }

    #[test]
    fn test_create_role_member_builder_default() {
        let builder = CreateRoleMemberBuilder::default();
        assert_eq!(builder.request.member.member_id, "");
        assert_eq!(builder.request.member.member_type, "");
        assert_eq!(builder.request.user_id_type, None);
        assert_eq!(builder.request.department_id_type, None);
    }

    #[test]
    fn test_list_role_members_builder() {
        let builder = ListRoleMembersBuilder::new()
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
    fn test_list_role_members_builder_default() {
        let builder = ListRoleMembersBuilder::default();
        assert_eq!(builder.request.page_size, None);
        assert_eq!(builder.request.page_token, None);
        assert_eq!(builder.request.user_id_type, None);
        assert_eq!(builder.request.department_id_type, None);
    }

    #[test]
    fn test_role_member_type_variations() {
        // Test different member types
        let user_member = RoleMember {
            member_id: Some("user_001".to_string()),
            member_type: Some("user".to_string()),
            ..Default::default()
        };

        let department_member = RoleMember {
            member_id: Some("dept_001".to_string()),
            member_type: Some("department".to_string()),
            ..Default::default()
        };

        let group_member = RoleMember {
            member_id: Some("group_001".to_string()),
            member_type: Some("group".to_string()),
            ..Default::default()
        };

        assert_eq!(user_member.member_type, Some("user".to_string()));
        assert_eq!(
            department_member.member_type,
            Some("department".to_string())
        );
        assert_eq!(group_member.member_type, Some("group".to_string()));
    }

    #[test]
    fn test_role_member_scopes_handling() {
        // Test different scopes scenarios
        let member_with_scopes = RoleMember {
            member_id: Some("member_001".to_string()),
            member_type: Some("user".to_string()),
            scopes: Some(vec![
                "read".to_string(),
                "write".to_string(),
                "delete".to_string(),
            ]),
            ..Default::default()
        };

        let member_without_scopes = RoleMember {
            member_id: Some("member_002".to_string()),
            member_type: Some("user".to_string()),
            scopes: None,
            ..Default::default()
        };

        let member_with_empty_scopes = RoleMember {
            member_id: Some("member_003".to_string()),
            member_type: Some("user".to_string()),
            scopes: Some(vec![]),
            ..Default::default()
        };

        assert_eq!(
            member_with_scopes.scopes,
            Some(
                vec!["read", "write", "delete"]
                    .iter()
                    .map(|s| s.to_string())
                    .collect()
            )
        );
        assert_eq!(member_without_scopes.scopes, None);
        assert_eq!(member_with_empty_scopes.scopes, Some(vec![]));
    }

    #[test]
    fn test_comprehensive_role_member_data() {
        // Test comprehensive role member data with all fields
        let comprehensive_detail = RoleMemberDetail {
            name: Some("王五".to_string()),
            email: Some("wangwu@example.com".to_string()),
            mobile: Some("13900139000".to_string()),
            avatar: Some("https://example.com/wangwu.jpg".to_string()),
            status: Some("active".to_string()),
        };

        let comprehensive_member = RoleMember {
            member_id: Some("comprehensive_001".to_string()),
            member_type: Some("user".to_string()),
            member_info: Some(comprehensive_detail),
            scopes: Some(vec![
                "admin".to_string(),
                "read".to_string(),
                "write".to_string(),
                "delete".to_string(),
            ]),
            join_time: Some("2023-01-01T08:00:00Z".to_string()),
            update_time: Some("2023-12-31T16:00:00Z".to_string()),
        };

        assert_eq!(
            comprehensive_member.member_id,
            Some("comprehensive_001".to_string())
        );
        assert_eq!(comprehensive_member.member_type, Some("user".to_string()));
        assert!(comprehensive_member.member_info.is_some());
        assert_eq!(comprehensive_member.scopes.unwrap().len(), 4);
        assert_eq!(
            comprehensive_member.join_time,
            Some("2023-01-01T08:00:00Z".to_string())
        );
        assert_eq!(
            comprehensive_member.update_time,
            Some("2023-12-31T16:00:00Z".to_string())
        );
    }
}

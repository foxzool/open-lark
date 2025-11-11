#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(non_snake_case)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::module_inception)]
//! 权限范围服务
//!
//! 提供完整的权限范围管理功能：
//! - 获取通讯录授权范围
//! - 获取权限范围详情
//! - 更新权限范围
//! - 支持分页查询

use crate::{
    api_resp::{ApiResponseTrait, ResponseFormat},
    config::Config,
    constants::AccessTokenType,
    http::Transport,
    ApiRequest, SDKResult,
};
use serde::{Deserialize, Serialize};

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

impl Default for Scope {
    fn default() -> Self {
        Self {
            departments: None,
            users: None,
            groups: None,
        }
    }
}

/// 权限范围内的部门
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ScopeDepartment {
    /// 部门ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department_id: Option<String>,
    /// 部门名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// 权限范围内的用户
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ScopeUser {
    /// 用户ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    /// 用户名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// 权限范围内的用户组
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ScopeGroup {
    /// 用户组ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
    /// 用户组名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// 权限范围详情
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ScopeAuthority {
    /// 应用ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_id: Option<String>,
    /// 应用名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_name: Option<String>,
    /// 权限范围类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope_type: Option<String>,
    /// 权限范围详情
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope_details: Option<ScopeDetails>,
    /// 创建时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    /// 更新时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_time: Option<String>,
}

impl Default for ScopeAuthority {
    fn default() -> Self {
        Self {
            app_id: None,
            app_name: None,
            scope_type: None,
            scope_details: None,
            create_time: None,
            update_time: None,
        }
    }
}

/// 权限范围详情
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ScopeDetails {
    /// 部门权限范围
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department_scope: Option<DepartmentScope>,
    /// 用户权限范围
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_scope: Option<UserScope>,
    /// 用户组权限范围
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_scope: Option<GroupScope>,
}

impl Default for ScopeDetails {
    fn default() -> Self {
        Self {
            department_scope: None,
            user_scope: None,
            group_scope: None,
        }
    }
}

/// 部门权限范围
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct DepartmentScope {
    /// 是否包含所有部门
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_all: Option<bool>,
    /// 可访问的部门列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department_ids: Option<Vec<String>>,
    /// 可访问的部门路径列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department_id_paths: Option<Vec<String>>,
}

/// 用户权限范围
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UserScope {
    /// 是否包含所有用户
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_all: Option<bool>,
    /// 可访问的用户列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_ids: Option<Vec<String>>,
    /// 排除的用户列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub excluded_user_ids: Option<Vec<String>>,
}

/// 用户组权限范围
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GroupScope {
    /// 是否包含所有用户组
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_all: Option<bool>,
    /// 可访问的用户组列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_ids: Option<Vec<String>>,
    /// 排除的用户组列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub excluded_group_ids: Option<Vec<String>>,
}

/// 权限范围服务
#[derive(Debug, Clone)]
pub struct ScopeService {
    config: Config,
}

impl ScopeService {
    /// 创建新的权限范围服务实例
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 获取通讯录授权范围
    ///
    /// 获取应用在通讯录中的授权范围，包括可访问的部门、用户和用户组列表。
    /// 用于了解当前应用能够访问的通讯录资源范围。
    ///
    /// # 参数
    /// * `req` - 查询权限范围请求
    ///
    /// # 返回值
    /// 返回权限范围列表，支持分页
    pub async fn list(&self, req: &GetScopeRequest) -> SDKResult<GetScopeResponse> {
        let mut api_path =
            crate::core::endpoints_original::Endpoints::CONTACT_V3_SCOPES.to_string();

        // 添加查询参数
        let mut query_params = Vec::new();
        if let Some(user_id_type) = &req.user_id_type {
            query_params.push(format!("user_id_type={}", user_id_type));
        }
        if let Some(department_id_type) = &req.department_id_type {
            query_params.push(format!("department_id_type={}", department_id_type));
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

        let resp = Transport::<GetScopeResponse>::request(api_req, &self.config, None).await?;
        Ok(resp.data.unwrap_or_default())
    }

    /// 获取通讯录授权范围详情
    ///
    /// 获取通讯录授权范围的详细信息，包括权限范围的具体配置和约束条件。
    ///
    /// # 参数
    /// * `req` - 查询权限范围详情请求
    ///
    /// # 返回值
    /// 返回权限范围的详细信息
    pub async fn get_authority(
        &self,
        req: &GetScopeAuthorityRequest,
    ) -> SDKResult<GetScopeAuthorityResponse> {
        let mut api_path =
            crate::core::endpoints_original::Endpoints::CONTACT_V3_SCOPES.to_string();

        // 添加查询参数
        let mut query_params = Vec::new();
        if let Some(user_id_type) = &req.user_id_type {
            query_params.push(format!("user_id_type={}", user_id_type));
        }
        if let Some(department_id_type) = &req.department_id_type {
            query_params.push(format!("department_id_type={}", department_id_type));
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
            Transport::<GetScopeAuthorityResponse>::request(api_req, &self.config, None).await?;
        Ok(resp.data.unwrap_or_default())
    }

    /// 更新通讯录授权范围
    ///
    /// 更新应用的通讯录授权范围，包括可访问的部门、用户和用户组。
    ///
    /// # 参数
    /// * `req` - 更新权限范围请求
    ///
    /// # 返回值
    /// 返回更新操作的结果
    pub async fn update_authority(
        &self,
        req: &UpdateScopeAuthorityRequest,
    ) -> SDKResult<UpdateScopeAuthorityResponse> {
        let api_req = ApiRequest {
            http_method: reqwest::Method::PUT,
            api_path: crate::core::endpoints_original::Endpoints::CONTACT_V3_SCOPES.to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant],
            body: serde_json::to_vec(req)?,
            ..Default::default()
        };

        let resp =
            Transport::<UpdateScopeAuthorityResponse>::request(api_req, &self.config, None).await?;
        Ok(resp.data.unwrap_or_default())
    }
}

// ==================== 数据模型 ====================

/// 获取权限范围请求
#[derive(Debug, Clone, Serialize, Deserialize)]
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

impl Default for GetScopeRequest {
    fn default() -> Self {
        Self {
            user_id_type: None,
            department_id_type: None,
            page_size: None,
            page_token: None,
        }
    }
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
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 获取权限范围详情请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetScopeAuthorityRequest {
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

impl Default for GetScopeAuthorityRequest {
    fn default() -> Self {
        Self {
            user_id_type: None,
            department_id_type: None,
            page_size: None,
            page_token: None,
        }
    }
}

/// 获取权限范围详情响应
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GetScopeAuthorityResponse {
    /// 权限范围详情
    pub scope_authority: ScopeAuthority,
}

impl ApiResponseTrait for GetScopeAuthorityResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 更新权限范围请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateScopeAuthorityRequest {
    /// 权限范围详情
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope_details: Option<ScopeDetails>,
}

impl Default for UpdateScopeAuthorityRequest {
    fn default() -> Self {
        Self {
            scope_details: None,
        }
    }
}

/// 更新权限范围响应
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UpdateScopeAuthorityResponse {
    /// 更新结果
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<String>,
}

impl ApiResponseTrait for UpdateScopeAuthorityResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

// ==================== 构建器模式 ====================

/// 获取权限范围构建器
#[derive(Debug, Clone)]
pub struct GetScopeBuilder {
    request: GetScopeRequest,
}

impl Default for GetScopeBuilder {
    fn default() -> Self {
        Self {
            request: GetScopeRequest::default(),
        }
    }
}

impl GetScopeBuilder {
    /// 创建新的查询构建器
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置用户 ID 类型
    pub fn user_id_type(mut self, user_id_type: impl Into<String>) -> Self {
        self.request.user_id_type = Some(user_id_type.into());
        self
    }

    /// 设置部门 ID 类型
    pub fn department_id_type(mut self, department_id_type: impl Into<String>) -> Self {
        self.request.department_id_type = Some(department_id_type.into());
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
    pub async fn execute(self, service: &ScopeService) -> SDKResult<GetScopeResponse> {
        service.list(&self.request).await
    }
}

/// 获取权限范围详情构建器
#[derive(Debug, Clone)]
pub struct GetScopeAuthorityBuilder {
    request: GetScopeAuthorityRequest,
}

impl Default for GetScopeAuthorityBuilder {
    fn default() -> Self {
        Self {
            request: GetScopeAuthorityRequest::default(),
        }
    }
}

impl GetScopeAuthorityBuilder {
    /// 创建新的查询构建器
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置用户 ID 类型
    pub fn user_id_type(mut self, user_id_type: impl Into<String>) -> Self {
        self.request.user_id_type = Some(user_id_type.into());
        self
    }

    /// 设置部门 ID 类型
    pub fn department_id_type(mut self, department_id_type: impl Into<String>) -> Self {
        self.request.department_id_type = Some(department_id_type.into());
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
    pub async fn execute(self, service: &ScopeService) -> SDKResult<GetScopeAuthorityResponse> {
        service.get_authority(&self.request).await
    }
}

/// 更新权限范围构建器
#[derive(Debug, Clone)]
pub struct UpdateScopeAuthorityBuilder {
    request: UpdateScopeAuthorityRequest,
}

impl Default for UpdateScopeAuthorityBuilder {
    fn default() -> Self {
        Self {
            request: UpdateScopeAuthorityRequest::default(),
        }
    }
}

impl UpdateScopeAuthorityBuilder {
    /// 创建新的更新构建器
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置权限范围详情
    pub fn scope_details(mut self, scope_details: ScopeDetails) -> Self {
        self.request.scope_details = Some(scope_details);
        self
    }

    /// 执行更新
    pub async fn execute(self, service: &ScopeService) -> SDKResult<UpdateScopeAuthorityResponse> {
        service.update_authority(&self.request).await
    }
}

impl ScopeService {
    /// 创建权限范围查询构建器
    pub fn get_scope_builder(&self) -> GetScopeBuilder {
        GetScopeBuilder::new()
    }

    /// 创建权限范围详情查询构建器
    pub fn get_scope_authority_builder(&self) -> GetScopeAuthorityBuilder {
        GetScopeAuthorityBuilder::new()
    }

    /// 创建权限范围更新构建器
    pub fn update_scope_authority_builder(&self) -> UpdateScopeAuthorityBuilder {
        UpdateScopeAuthorityBuilder::new()
    }
}

// ==================== 单元测试 ====================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scope_service_creation() {
        let config = Config::default();
        let service = ScopeService::new(config);
        assert!(!format!("{:?}", service).is_empty());
    }

    #[test]
    fn test_scope_default_creation() {
        let scope = Scope::default();
        assert_eq!(scope.departments, None);
        assert_eq!(scope.users, None);
        assert_eq!(scope.groups, None);
    }

    #[test]
    fn test_scope_with_data() {
        let scope = Scope {
            departments: Some(vec![ScopeDepartment {
                department_id: Some("dept_123".to_string()),
                name: Some("技术部".to_string()),
            }]),
            users: Some(vec![ScopeUser {
                user_id: Some("user_456".to_string()),
                name: Some("张三".to_string()),
            }]),
            groups: Some(vec![ScopeGroup {
                group_id: Some("group_789".to_string()),
                name: Some("开发组".to_string()),
            }]),
        };

        assert_eq!(scope.departments.as_ref().unwrap().len(), 1);
        assert_eq!(scope.users.as_ref().unwrap().len(), 1);
        assert_eq!(scope.groups.as_ref().unwrap().len(), 1);
        assert_eq!(
            scope.departments.as_ref().unwrap()[0].department_id,
            Some("dept_123".to_string())
        );
    }

    #[test]
    fn test_scope_authority_default_creation() {
        let scope_authority = ScopeAuthority::default();
        assert_eq!(scope_authority.app_id, None);
        assert_eq!(scope_authority.app_name, None);
        assert_eq!(scope_authority.scope_type, None);
        assert_eq!(scope_authority.scope_details, None);
        assert_eq!(scope_authority.create_time, None);
        assert_eq!(scope_authority.update_time, None);
    }

    #[test]
    fn test_scope_details_default_creation() {
        let scope_details = ScopeDetails::default();
        assert_eq!(scope_details.department_scope, None);
        assert_eq!(scope_details.user_scope, None);
        assert_eq!(scope_details.group_scope, None);
    }

    #[test]
    fn test_department_scope() {
        let dept_scope = DepartmentScope {
            include_all: Some(false),
            department_ids: Some(vec!["dept_1".to_string(), "dept_2".to_string()]),
            department_id_paths: Some(vec!["/dept_1".to_string()]),
        };

        assert_eq!(dept_scope.include_all, Some(false));
        assert_eq!(dept_scope.department_ids.as_ref().unwrap().len(), 2);
        assert_eq!(dept_scope.department_id_paths.as_ref().unwrap().len(), 1);
    }

    #[test]
    fn test_user_scope() {
        let user_scope = UserScope {
            include_all: Some(true),
            user_ids: Some(vec!["user_1".to_string()]),
            excluded_user_ids: Some(vec!["user_excluded".to_string()]),
        };

        assert_eq!(user_scope.include_all, Some(true));
        assert_eq!(user_scope.user_ids.as_ref().unwrap().len(), 1);
        assert_eq!(user_scope.excluded_user_ids.as_ref().unwrap().len(), 1);
    }

    #[test]
    fn test_group_scope() {
        let group_scope = GroupScope {
            include_all: Some(false),
            group_ids: Some(vec!["group_1".to_string(), "group_2".to_string()]),
            excluded_group_ids: Some(vec!["group_excluded".to_string()]),
        };

        assert_eq!(group_scope.include_all, Some(false));
        assert_eq!(group_scope.group_ids.as_ref().unwrap().len(), 2);
        assert_eq!(group_scope.excluded_group_ids.as_ref().unwrap().len(), 1);
    }

    #[test]
    fn test_get_scope_request_default() {
        let request = GetScopeRequest::default();
        assert_eq!(request.user_id_type, None);
        assert_eq!(request.department_id_type, None);
        assert_eq!(request.page_size, None);
        assert_eq!(request.page_token, None);
    }

    #[test]
    fn test_get_scope_request_with_data() {
        let request = GetScopeRequest {
            user_id_type: Some("open_id".to_string()),
            department_id_type: Some("department_id".to_string()),
            page_size: Some(50),
            page_token: Some("token123".to_string()),
        };

        assert_eq!(request.user_id_type, Some("open_id".to_string()));
        assert_eq!(
            request.department_id_type,
            Some("department_id".to_string())
        );
        assert_eq!(request.page_size, Some(50));
        assert_eq!(request.page_token, Some("token123".to_string()));
    }

    #[test]
    fn test_update_scope_authority_request_default() {
        let request = UpdateScopeAuthorityRequest::default();
        assert_eq!(request.scope_details, None);
    }

    #[test]
    fn test_update_scope_authority_request_with_data() {
        let scope_details = ScopeDetails::default();
        let request = UpdateScopeAuthorityRequest {
            scope_details: Some(scope_details),
        };

        assert!(request.scope_details.is_some());
    }

    #[test]
    fn test_response_default_creation() {
        let get_response = GetScopeResponse::default();
        assert_eq!(get_response.scopes.len(), 0);
        assert_eq!(get_response.has_more, None);
        assert_eq!(get_response.page_token, None);

        let authority_response = GetScopeAuthorityResponse::default();
        assert_eq!(authority_response.scope_authority.app_id, None);

        let update_response = UpdateScopeAuthorityResponse::default();
        assert_eq!(update_response.result, None);
    }

    #[test]
    fn test_response_with_data() {
        let mut get_response = GetScopeResponse::default();
        get_response.scopes = vec![Scope::default()];
        get_response.has_more = Some(true);
        get_response.page_token = Some("next_page".to_string());

        assert_eq!(get_response.scopes.len(), 1);
        assert_eq!(get_response.has_more, Some(true));
        assert_eq!(get_response.page_token, Some("next_page".to_string()));

        let mut authority_response = GetScopeAuthorityResponse::default();
        authority_response.scope_authority = ScopeAuthority {
            app_id: Some("app_123".to_string()),
            app_name: Some("测试应用".to_string()),
            ..Default::default()
        };

        assert_eq!(
            authority_response.scope_authority.app_id,
            Some("app_123".to_string())
        );
        assert_eq!(
            authority_response.scope_authority.app_name,
            Some("测试应用".to_string())
        );

        let mut update_response = UpdateScopeAuthorityResponse::default();
        update_response.result = Some("success".to_string());

        assert_eq!(update_response.result, Some("success".to_string()));
    }

    #[test]
    fn test_api_response_trait_implementation() {
        assert_eq!(GetScopeResponse::data_format(), ResponseFormat::Data);
        assert_eq!(
            GetScopeAuthorityResponse::data_format(),
            ResponseFormat::Data
        );
        assert_eq!(
            UpdateScopeAuthorityResponse::data_format(),
            ResponseFormat::Data
        );
    }

    #[test]
    fn test_request_serialization() {
        let request = GetScopeRequest {
            user_id_type: Some("open_id".to_string()),
            department_id_type: Some("department_id".to_string()),
            page_size: Some(20),
            page_token: Some("test_token".to_string()),
        };

        let serialized = serde_json::to_string(&request).unwrap();
        let deserialized: GetScopeRequest = serde_json::from_str(&serialized).unwrap();

        assert_eq!(request.user_id_type, deserialized.user_id_type);
        assert_eq!(request.department_id_type, deserialized.department_id_type);
        assert_eq!(request.page_size, deserialized.page_size);
        assert_eq!(request.page_token, deserialized.page_token);
    }

    #[test]
    fn test_query_parameters_construction() {
        let request = GetScopeRequest {
            user_id_type: Some("open_id".to_string()),
            department_id_type: Some("department_id".to_string()),
            page_size: Some(20),
            page_token: Some("test_token".to_string()),
        };

        let mut query_params = Vec::new();
        if let Some(user_id_type) = &request.user_id_type {
            query_params.push(format!("user_id_type={}", user_id_type));
        }
        if let Some(department_id_type) = &request.department_id_type {
            query_params.push(format!("department_id_type={}", department_id_type));
        }
        if let Some(page_size) = &request.page_size {
            query_params.push(format!("page_size={}", page_size));
        }
        if let Some(page_token) = &request.page_token {
            query_params.push(format!("page_token={}", page_token));
        }

        assert_eq!(query_params.len(), 4);
        assert!(query_params.contains(&"user_id_type=open_id".to_string()));
        assert!(query_params.contains(&"department_id_type=department_id".to_string()));
        assert!(query_params.contains(&"page_size=20".to_string()));
        assert!(query_params.contains(&"page_token=test_token".to_string()));
    }

    #[test]
    fn test_builder_patterns() {
        let builder = GetScopeBuilder::new()
            .user_id_type("open_id")
            .department_id_type("department_id")
            .page_size(50)
            .page_token("test_token");

        assert_eq!(builder.request.user_id_type, Some("open_id".to_string()));
        assert_eq!(
            builder.request.department_id_type,
            Some("department_id".to_string())
        );
        assert_eq!(builder.request.page_size, Some(50));
        assert_eq!(builder.request.page_token, Some("test_token".to_string()));

        let authority_builder = GetScopeAuthorityBuilder::new()
            .user_id_type("user_id")
            .page_size(20);

        assert_eq!(
            authority_builder.request.user_id_type,
            Some("user_id".to_string())
        );
        assert_eq!(authority_builder.request.page_size, Some(20));

        let update_builder =
            UpdateScopeAuthorityBuilder::new().scope_details(ScopeDetails::default());

        assert!(update_builder.request.scope_details.is_some());
    }

    #[test]
    fn test_endpoint_constants() {
        // Test that the endpoint constants are properly defined
        assert_eq!(
            crate::core::endpoints_original::Endpoints::CONTACT_V3_SCOPES,
            "/open-apis/contact/v3/scopes"
        );
    }

    #[test]
    fn test_complex_scope_structure() {
        // Test complex nested scope structure
        let scope_details = ScopeDetails {
            department_scope: Some(DepartmentScope {
                include_all: Some(false),
                department_ids: Some(vec!["dept_1".to_string(), "dept_2".to_string()]),
                department_id_paths: Some(vec!["/dept_1/sub".to_string()]),
            }),
            user_scope: Some(UserScope {
                include_all: Some(false),
                user_ids: Some(vec!["user_1".to_string()]),
                excluded_user_ids: Some(vec!["user_excluded".to_string()]),
            }),
            group_scope: Some(GroupScope {
                include_all: Some(true),
                group_ids: None,
                excluded_group_ids: None,
            }),
        };

        assert!(scope_details.department_scope.is_some());
        assert!(scope_details.user_scope.is_some());
        assert!(scope_details.group_scope.is_some());

        let dept_scope = scope_details.department_scope.unwrap();
        assert_eq!(dept_scope.include_all, Some(false));
        assert_eq!(dept_scope.department_ids.as_ref().unwrap().len(), 2);
    }
}

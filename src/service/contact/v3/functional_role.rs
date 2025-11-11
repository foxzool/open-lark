#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(non_snake_case)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::module_inception)]
//! 职能角色服务
//!
//! 提供完整的职能角色管理功能：
//! - 创建职能角色
//! - 更新职能角色
//! - 获取单个职能角色信息
//! - 获取职能角色列表
//! - 删除职能角色
//! - 支持分页查询

use crate::{
    api_resp::{ApiResponseTrait, ResponseFormat},
    config::Config,
    constants::AccessTokenType,
    http::Transport,
    ApiRequest, SDKResult,
};
use serde::{Deserialize, Serialize};

/// 职能角色信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionalRole {
    /// 角色ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_id: Option<String>,
    /// 角色名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_name: Option<String>,
    /// 角色描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 创建时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    /// 更新时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_time: Option<String>,
}

impl Default for FunctionalRole {
    fn default() -> Self {
        Self {
            role_id: None,
            role_name: None,
            description: None,
            create_time: None,
            update_time: None,
        }
    }
}

/// 职能角色服务
#[derive(Debug, Clone)]
pub struct FunctionalRoleService {
    config: Config,
}

impl FunctionalRoleService {
    /// 创建新的职能角色服务实例
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 创建职能角色
    ///
    /// 创建一个新的职能角色，用于管理系统中具有特定职责的用户群体
    ///
    /// # 参数
    /// * `req` - 创建职能角色请求
    ///
    /// # 返回值
    /// 返回创建成功的职能角色信息，包含角色ID
    pub async fn create(
        &self,
        req: &CreateFunctionalRoleRequest,
    ) -> SDKResult<CreateFunctionalRoleResponse> {
        let api_path =
            crate::core::endpoints_original::Endpoints::CONTACT_V3_FUNCTIONAL_ROLES.to_string();

        let api_req = ApiRequest {
            http_method: reqwest::Method::POST,
            api_path,
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: serde_json::to_vec(req)?,
            ..Default::default()
        };

        let resp =
            Transport::<CreateFunctionalRoleResponse>::request(api_req, &self.config, None).await?;
        Ok(resp.data.unwrap_or_default())
    }

    /// 更新职能角色
    ///
    /// 更新指定职能角色的基本信息
    ///
    /// # 参数
    /// * `role_id` - 角色ID
    /// * `req` - 更新职能角色请求
    ///
    /// # 返回值
    /// 返回更新成功的职能角色信息
    pub async fn update(
        &self,
        role_id: &str,
        req: &UpdateFunctionalRoleRequest,
    ) -> SDKResult<UpdateFunctionalRoleResponse> {
        let api_path = crate::core::endpoints_original::Endpoints::CONTACT_V3_FUNCTIONAL_ROLE_GET
            .replace("{role_id}", role_id);

        let api_req = ApiRequest {
            http_method: reqwest::Method::PUT,
            api_path,
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: serde_json::to_vec(req)?,
            ..Default::default()
        };

        let resp =
            Transport::<UpdateFunctionalRoleResponse>::request(api_req, &self.config, None).await?;
        Ok(resp.data.unwrap_or_default())
    }

    /// 获取单个职能角色信息
    ///
    /// 根据角色ID获取职能角色的详细信息
    ///
    /// # 参数
    /// * `role_id` - 角色ID
    ///
    /// # 返回值
    /// 返回职能角色的详细信息
    pub async fn get(&self, role_id: &str) -> SDKResult<GetFunctionalRoleResponse> {
        let api_path = crate::core::endpoints_original::Endpoints::CONTACT_V3_FUNCTIONAL_ROLE_GET
            .replace("{role_id}", role_id);

        let api_req = ApiRequest {
            http_method: reqwest::Method::GET,
            api_path,
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: Vec::new(),
            ..Default::default()
        };

        let resp =
            Transport::<GetFunctionalRoleResponse>::request(api_req, &self.config, None).await?;
        Ok(resp.data.unwrap_or_default())
    }

    /// 获取职能角色列表
    ///
    /// 获取企业内所有职能角色的列表，支持分页查询
    ///
    /// # 参数
    /// * `req` - 查询职能角色列表请求
    ///
    /// # 返回值
    /// 返回职能角色列表，支持分页
    pub async fn list(
        &self,
        req: &ListFunctionalRolesRequest,
    ) -> SDKResult<ListFunctionalRolesResponse> {
        let mut api_path =
            crate::core::endpoints_original::Endpoints::CONTACT_V3_FUNCTIONAL_ROLES.to_string();

        // 添加查询参数
        let mut query_params = Vec::new();
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

        let resp =
            Transport::<ListFunctionalRolesResponse>::request(api_req, &self.config, None).await?;
        Ok(resp.data.unwrap_or_default())
    }

    /// 删除职能角色
    ///
    /// 删除指定的职能角色
    ///
    /// # 参数
    /// * `role_id` - 角色ID
    ///
    /// # 返回值
    /// 返回删除操作的结果
    pub async fn delete(&self, role_id: &str) -> SDKResult<DeleteFunctionalRoleResponse> {
        let api_path = crate::core::endpoints_original::Endpoints::CONTACT_V3_FUNCTIONAL_ROLE_GET
            .replace("{role_id}", role_id);

        let api_req = ApiRequest {
            http_method: reqwest::Method::DELETE,
            api_path,
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: Vec::new(),
            ..Default::default()
        };

        let resp =
            Transport::<DeleteFunctionalRoleResponse>::request(api_req, &self.config, None).await?;
        Ok(resp.data.unwrap_or_default())
    }
}

// ==================== 数据模型 ====================

/// 创建职能角色请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateFunctionalRoleRequest {
    /// 角色名称
    pub role_name: String,
    /// 角色描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// 创建职能角色响应
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreateFunctionalRoleResponse {
    /// 角色ID
    pub role_id: String,
    /// 角色名称
    pub role_name: String,
    /// 角色描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 创建时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
}

impl ApiResponseTrait for CreateFunctionalRoleResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 更新职能角色请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateFunctionalRoleRequest {
    /// 角色名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role_name: Option<String>,
    /// 角色描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// 更新职能角色响应
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UpdateFunctionalRoleResponse {
    /// 角色信息
    pub role: FunctionalRole,
}

impl ApiResponseTrait for UpdateFunctionalRoleResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 获取单个职能角色信息响应
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GetFunctionalRoleResponse {
    /// 角色信息
    pub role: FunctionalRole,
}

impl ApiResponseTrait for GetFunctionalRoleResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 查询职能角色列表请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListFunctionalRolesRequest {
    /// 分页大小
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 分页标记
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

impl Default for ListFunctionalRolesRequest {
    fn default() -> Self {
        Self {
            page_size: None,
            page_token: None,
        }
    }
}

/// 查询职能角色列表响应
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ListFunctionalRolesResponse {
    /// 职能角色列表
    pub items: Vec<FunctionalRole>,
    /// 是否还有更多项目
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
    /// 分页标记
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

impl ApiResponseTrait for ListFunctionalRolesResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 删除职能角色响应
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DeleteFunctionalRoleResponse {
    /// 删除成功标识
    pub success: bool,
}

impl ApiResponseTrait for DeleteFunctionalRoleResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

// ==================== 构建器模式 ====================

/// 创建职能角色构建器
#[derive(Debug, Clone)]
pub struct CreateFunctionalRoleBuilder {
    request: CreateFunctionalRoleRequest,
}

impl Default for CreateFunctionalRoleBuilder {
    fn default() -> Self {
        Self {
            request: CreateFunctionalRoleRequest {
                role_name: String::new(),
                description: None,
            },
        }
    }
}

impl CreateFunctionalRoleBuilder {
    /// 创建新的构建器
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置角色名称
    pub fn role_name(mut self, role_name: impl Into<String>) -> Self {
        self.request.role_name = role_name.into();
        self
    }

    /// 设置角色描述
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.request.description = Some(description.into());
        self
    }

    /// 执行创建
    pub async fn execute(
        self,
        service: &FunctionalRoleService,
    ) -> SDKResult<CreateFunctionalRoleResponse> {
        service.create(&self.request).await
    }
}

/// 查询职能角色列表构建器
#[derive(Debug, Clone)]
pub struct ListFunctionalRolesBuilder {
    request: ListFunctionalRolesRequest,
}

impl Default for ListFunctionalRolesBuilder {
    fn default() -> Self {
        Self {
            request: ListFunctionalRolesRequest::default(),
        }
    }
}

impl ListFunctionalRolesBuilder {
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

    /// 执行查询
    pub async fn execute(
        self,
        service: &FunctionalRoleService,
    ) -> SDKResult<ListFunctionalRolesResponse> {
        service.list(&self.request).await
    }
}

impl FunctionalRoleService {
    /// 创建职能角色构建器
    pub fn create_functional_role_builder(&self) -> CreateFunctionalRoleBuilder {
        CreateFunctionalRoleBuilder::new()
    }

    /// 创建查询构建器
    pub fn list_functional_roles_builder(&self) -> ListFunctionalRolesBuilder {
        ListFunctionalRolesBuilder::new()
    }
}

// ==================== 单元测试 ====================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_functional_role_service_creation() {
        let config = Config::default();
        let service = FunctionalRoleService::new(config);
        assert!(!format!("{:?}", service).is_empty());
    }

    #[test]
    fn test_functional_role_default_creation() {
        let role = FunctionalRole::default();
        assert_eq!(role.role_id, None);
        assert_eq!(role.role_name, None);
        assert_eq!(role.description, None);
        assert_eq!(role.create_time, None);
        assert_eq!(role.update_time, None);
    }

    #[test]
    fn test_functional_role_with_data() {
        let role = FunctionalRole {
            role_id: Some("role_123".to_string()),
            role_name: Some("管理员".to_string()),
            description: Some("系统管理员角色".to_string()),
            create_time: Some("2023-01-01T00:00:00Z".to_string()),
            update_time: Some("2023-01-02T00:00:00Z".to_string()),
        };

        assert_eq!(role.role_id, Some("role_123".to_string()));
        assert_eq!(role.role_name, Some("管理员".to_string()));
        assert_eq!(role.description, Some("系统管理员角色".to_string()));
        assert_eq!(role.create_time, Some("2023-01-01T00:00:00Z".to_string()));
        assert_eq!(role.update_time, Some("2023-01-02T00:00:00Z".to_string()));
    }

    #[test]
    fn test_create_functional_role_request() {
        let request = CreateFunctionalRoleRequest {
            role_name: "新角色".to_string(),
            description: Some("角色描述".to_string()),
        };

        assert_eq!(request.role_name, "新角色");
        assert_eq!(request.description, Some("角色描述".to_string()));
    }

    #[test]
    fn test_create_functional_role_request_without_description() {
        let request = CreateFunctionalRoleRequest {
            role_name: "简单角色".to_string(),
            description: None,
        };

        assert_eq!(request.role_name, "简单角色");
        assert_eq!(request.description, None);
    }

    #[test]
    fn test_update_functional_role_request() {
        let request = UpdateFunctionalRoleRequest {
            role_name: Some("更新后角色名".to_string()),
            description: Some("更新后描述".to_string()),
        };

        assert_eq!(request.role_name, Some("更新后角色名".to_string()));
        assert_eq!(request.description, Some("更新后描述".to_string()));
    }

    #[test]
    fn test_update_functional_role_request_partial() {
        let request1 = UpdateFunctionalRoleRequest {
            role_name: Some("只更新名称".to_string()),
            description: None,
        };

        let request2 = UpdateFunctionalRoleRequest {
            role_name: None,
            description: Some("只更新描述".to_string()),
        };

        assert_eq!(request1.role_name, Some("只更新名称".to_string()));
        assert_eq!(request1.description, None);
        assert_eq!(request2.role_name, None);
        assert_eq!(request2.description, Some("只更新描述".to_string()));
    }

    #[test]
    fn test_list_functional_roles_request_default() {
        let request = ListFunctionalRolesRequest::default();
        assert_eq!(request.page_size, None);
        assert_eq!(request.page_token, None);
    }

    #[test]
    fn test_list_functional_roles_request_with_pagination() {
        let request = ListFunctionalRolesRequest {
            page_size: Some(50),
            page_token: Some("token_123".to_string()),
        };

        assert_eq!(request.page_size, Some(50));
        assert_eq!(request.page_token, Some("token_123".to_string()));
    }

    #[test]
    fn test_response_default_creation() {
        let create_response = CreateFunctionalRoleResponse::default();
        assert_eq!(create_response.role_id, "");
        assert_eq!(create_response.role_name, "");

        let update_response = UpdateFunctionalRoleResponse::default();
        assert_eq!(update_response.role.role_id, None);

        let get_response = GetFunctionalRoleResponse::default();
        assert_eq!(get_response.role.role_id, None);

        let list_response = ListFunctionalRolesResponse::default();
        assert_eq!(list_response.items.len(), 0);
        assert_eq!(list_response.has_more, None);
        assert_eq!(list_response.page_token, None);

        let delete_response = DeleteFunctionalRoleResponse::default();
        assert_eq!(delete_response.success, false);
    }

    #[test]
    fn test_response_with_data() {
        let mut create_response = CreateFunctionalRoleResponse::default();
        create_response.role_id = "role_456".to_string();
        create_response.role_name = "测试角色".to_string();

        assert_eq!(create_response.role_id, "role_456");
        assert_eq!(create_response.role_name, "测试角色");

        let role = FunctionalRole {
            role_id: Some("role_789".to_string()),
            role_name: Some("角色789".to_string()),
            ..Default::default()
        };

        let mut get_response = GetFunctionalRoleResponse::default();
        get_response.role = role.clone();
        assert_eq!(get_response.role.role_id, Some("role_789".to_string()));

        let mut list_response = ListFunctionalRolesResponse::default();
        list_response.items = vec![role.clone()];
        list_response.has_more = Some(true);
        list_response.page_token = Some("next_page".to_string());

        assert_eq!(list_response.items.len(), 1);
        assert_eq!(list_response.has_more, Some(true));
        assert_eq!(list_response.page_token, Some("next_page".to_string()));

        let mut delete_response = DeleteFunctionalRoleResponse::default();
        delete_response.success = true;
        assert_eq!(delete_response.success, true);
    }

    #[test]
    fn test_api_response_trait_implementation() {
        assert_eq!(
            CreateFunctionalRoleResponse::data_format(),
            ResponseFormat::Data
        );
        assert_eq!(
            UpdateFunctionalRoleResponse::data_format(),
            ResponseFormat::Data
        );
        assert_eq!(
            GetFunctionalRoleResponse::data_format(),
            ResponseFormat::Data
        );
        assert_eq!(
            ListFunctionalRolesResponse::data_format(),
            ResponseFormat::Data
        );
        assert_eq!(
            DeleteFunctionalRoleResponse::data_format(),
            ResponseFormat::Data
        );
    }

    #[test]
    fn test_request_serialization() {
        let create_request = CreateFunctionalRoleRequest {
            role_name: "序列化测试".to_string(),
            description: Some("测试序列化".to_string()),
        };

        let serialized = serde_json::to_string(&create_request).unwrap();
        let deserialized: CreateFunctionalRoleRequest = serde_json::from_str(&serialized).unwrap();

        assert_eq!(create_request.role_name, deserialized.role_name);
        assert_eq!(create_request.description, deserialized.description);

        let list_request = ListFunctionalRolesRequest {
            page_size: Some(100),
            page_token: Some("test_token".to_string()),
        };

        let serialized = serde_json::to_string(&list_request).unwrap();
        let deserialized: ListFunctionalRolesRequest = serde_json::from_str(&serialized).unwrap();

        assert_eq!(list_request.page_size, deserialized.page_size);
        assert_eq!(list_request.page_token, deserialized.page_token);
    }

    #[test]
    fn test_query_parameters_construction() {
        let request = ListFunctionalRolesRequest {
            page_size: Some(20),
            page_token: Some("test_token".to_string()),
        };

        let mut query_params = Vec::new();
        if let Some(page_size) = &request.page_size {
            query_params.push(format!("page_size={}", page_size));
        }
        if let Some(page_token) = &request.page_token {
            query_params.push(format!("page_token={}", page_token));
        }

        assert_eq!(query_params.len(), 2);
        assert!(query_params.contains(&"page_size=20".to_string()));
        assert!(query_params.contains(&"page_token=test_token".to_string()));
    }

    #[test]
    fn test_endpoint_constants() {
        // Test that the endpoint constants are properly defined
        assert_eq!(
            crate::core::endpoints_original::Endpoints::CONTACT_V3_FUNCTIONAL_ROLES,
            "/open-apis/contact/v3/functional_roles"
        );
        assert_eq!(
            crate::core::endpoints_original::Endpoints::CONTACT_V3_FUNCTIONAL_ROLE_GET,
            "/open-apis/contact/v3/functional_roles/{role_id}"
        );
    }

    #[test]
    fn test_create_functional_role_builder() {
        let builder = CreateFunctionalRoleBuilder::new()
            .role_name("构建器角色")
            .description("使用构建器创建的角色");

        assert_eq!(builder.request.role_name, "构建器角色");
        assert_eq!(
            builder.request.description,
            Some("使用构建器创建的角色".to_string())
        );
    }

    #[test]
    fn test_create_functional_role_builder_default() {
        let builder = CreateFunctionalRoleBuilder::default();
        assert_eq!(builder.request.role_name, "");
        assert_eq!(builder.request.description, None);
    }

    #[test]
    fn test_list_functional_roles_builder() {
        let builder = ListFunctionalRolesBuilder::new()
            .page_size(30)
            .page_token("builder_token");

        assert_eq!(builder.request.page_size, Some(30));
        assert_eq!(
            builder.request.page_token,
            Some("builder_token".to_string())
        );
    }

    #[test]
    fn test_list_functional_roles_builder_default() {
        let builder = ListFunctionalRolesBuilder::default();
        assert_eq!(builder.request.page_size, None);
        assert_eq!(builder.request.page_token, None);
    }

    #[test]
    fn test_functional_role_description_handling() {
        // Test different description scenarios
        let role_with_description = FunctionalRole {
            role_id: Some("role_desc_1".to_string()),
            role_name: Some("带描述的角色".to_string()),
            description: Some("这是一个详细的角色描述".to_string()),
            ..Default::default()
        };

        let role_without_description = FunctionalRole {
            role_id: Some("role_no_desc".to_string()),
            role_name: Some("无描述角色".to_string()),
            description: None,
            ..Default::default()
        };

        let role_with_empty_description = FunctionalRole {
            role_id: Some("role_empty_desc".to_string()),
            role_name: Some("空描述角色".to_string()),
            description: Some("".to_string()),
            ..Default::default()
        };

        assert_eq!(
            role_with_description.description,
            Some("这是一个详细的角色描述".to_string())
        );
        assert_eq!(role_without_description.description, None);
        assert_eq!(
            role_with_empty_description.description,
            Some("".to_string())
        );
    }

    #[test]
    fn test_functional_role_time_fields() {
        // Test different time formats
        let role_with_iso_time = FunctionalRole {
            role_id: Some("role_time_1".to_string()),
            role_name: Some("ISO时间角色".to_string()),
            create_time: Some("2023-12-31T23:59:59Z".to_string()),
            update_time: Some("2024-01-01T00:00:00Z".to_string()),
            ..Default::default()
        };

        let role_with_chinese_time = FunctionalRole {
            role_id: Some("role_time_2".to_string()),
            role_name: Some("中文时间角色".to_string()),
            create_time: Some("2023年12月31日 23:59:59".to_string()),
            update_time: Some("2024年01月01日 00:00:00".to_string()),
            ..Default::default()
        };

        assert_eq!(
            role_with_iso_time.create_time,
            Some("2023-12-31T23:59:59Z".to_string())
        );
        assert_eq!(
            role_with_iso_time.update_time,
            Some("2024-01-01T00:00:00Z".to_string())
        );
        assert_eq!(
            role_with_chinese_time.create_time,
            Some("2023年12月31日 23:59:59".to_string())
        );
        assert_eq!(
            role_with_chinese_time.update_time,
            Some("2024年01月01日 00:00:00".to_string())
        );
    }

    #[test]
    fn test_functional_role_name_variations() {
        // Test different role name scenarios
        let admin_role = FunctionalRole {
            role_id: Some("admin_role".to_string()),
            role_name: Some("系统管理员".to_string()),
            ..Default::default()
        };

        let user_role = FunctionalRole {
            role_id: Some("user_role".to_string()),
            role_name: Some("普通用户".to_string()),
            ..Default::default()
        };

        let guest_role = FunctionalRole {
            role_id: Some("guest_role".to_string()),
            role_name: Some("访客用户".to_string()),
            ..Default::default()
        };

        let long_name_role = FunctionalRole {
            role_id: Some("long_name_role".to_string()),
            role_name: Some(
                "这是一个非常非常长的角色名称用于测试系统对长角色名称的处理能力".to_string(),
            ),
            ..Default::default()
        };

        assert_eq!(admin_role.role_name, Some("系统管理员".to_string()));
        assert_eq!(user_role.role_name, Some("普通用户".to_string()));
        assert_eq!(guest_role.role_name, Some("访客用户".to_string()));
        assert_eq!(
            long_name_role.role_name,
            Some("这是一个非常非常长的角色名称用于测试系统对长角色名称的处理能力".to_string())
        );
    }

    #[test]
    fn test_comprehensive_functional_role_data() {
        // Test comprehensive functional role data with all fields
        let comprehensive_role = FunctionalRole {
            role_id: Some("comprehensive_001".to_string()),
            role_name: Some("综合测试角色".to_string()),
            description: Some(
                "这是一个用于全面测试的角色，包含所有可能的字段和数据类型".to_string(),
            ),
            create_time: Some("2023-01-01T08:00:00Z".to_string()),
            update_time: Some("2023-12-31T16:00:00Z".to_string()),
        };

        assert_eq!(
            comprehensive_role.role_id,
            Some("comprehensive_001".to_string())
        );
        assert_eq!(
            comprehensive_role.role_name,
            Some("综合测试角色".to_string())
        );
        assert_eq!(
            comprehensive_role.description,
            Some("这是一个用于全面测试的角色，包含所有可能的字段和数据类型".to_string())
        );
        assert_eq!(
            comprehensive_role.create_time,
            Some("2023-01-01T08:00:00Z".to_string())
        );
        assert_eq!(
            comprehensive_role.update_time,
            Some("2023-12-31T16:00:00Z".to_string())
        );
    }
}

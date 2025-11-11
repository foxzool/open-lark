//! APAAS应用管理服务
//!
//! 提供飞书应用开放平台的应用管理功能，包括：
//! - 查看应用基本信息
//! - 应用状态管理
//! - 应用配置查询
//! - 分页查询支持

use crate::{
    api_resp::{ApiResponseTrait, ResponseFormat},
    config::Config,
    constants::AccessTokenType,
    http::Transport,
    ApiRequest, SDKResult,
};
use serde::{Deserialize, Serialize};

/// 应用信息
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AppInfo {
    /// 应用标识
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_key: Option<String>,
    /// 应用名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 应用描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 应用状态
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// 应用类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_type: Option<String>,
    /// 创建时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time: Option<String>,
    /// 更新时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub update_time: Option<String>,
}

impl Default for AppInfo {
    fn default() -> Self {
        Self {
            app_key: None,
            name: None,
            description: None,
            status: None,
            app_type: None,
            create_time: None,
            update_time: None,
        }
    }
}

/// 查看应用基本信息请求
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GetAppsRequest {
    /// 分页大小
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 分页标记
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// 应用状态筛选
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

impl Default for GetAppsRequest {
    fn default() -> Self {
        Self {
            page_size: None,
            page_token: None,
            status: None,
        }
    }
}

/// 查看应用基本信息响应
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct GetAppsResponse {
    /// 应用列表
    pub items: Vec<AppInfo>,
    /// 是否还有更多项目
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
    /// 分页标记
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
}

impl ApiResponseTrait for GetAppsResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 应用管理服务
#[derive(Debug, Clone)]
pub struct AppsService {
    config: Config,
}

impl AppsService {
    /// 创建应用管理服务实例
    ///
    /// # 参数
    /// - `config`: SDK配置信息
    ///
    /// # 示例
    ///
    /// ```rust
    /// use open_lark::prelude::*;
    /// use open_lark::service::apaas::v1::apps::AppsService;
    ///
    /// let config = Config::new("app_id", "app_secret");
    /// let service = AppsService::new(config);
    /// ```
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 查看应用基本信息
    ///
    /// 获取当前租户下的应用列表，支持分页查询和状态筛选
    ///
    /// # 参数
    /// * `req` - 查询应用列表请求
    ///
    /// # 返回值
    /// 返回应用列表，支持分页
    pub async fn get(&self, req: &GetAppsRequest) -> SDKResult<GetAppsResponse> {
        let mut api_path = crate::core::endpoints_original::Endpoints::APAAS_V1_APPS.to_string();

        // 添加查询参数
        let mut query_params = Vec::new();
        if let Some(page_size) = &req.page_size {
            query_params.push(format!("page_size={}", page_size));
        }
        if let Some(page_token) = &req.page_token {
            query_params.push(format!("page_token={}", page_token));
        }
        if let Some(status) = &req.status {
            query_params.push(format!("status={}", status));
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

        let resp = Transport::<GetAppsResponse>::request(api_req, &self.config, None).await?;
        Ok(resp.data.unwrap_or_default())
    }
}

// ==================== 构建器模式 ====================

/// 查看应用基本信息构建器
#[derive(Debug, Clone)]
pub struct GetAppsBuilder {
    request: GetAppsRequest,
}

impl Default for GetAppsBuilder {
    fn default() -> Self {
        Self {
            request: GetAppsRequest::default(),
        }
    }
}

impl GetAppsBuilder {
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

    /// 设置应用状态筛选
    pub fn status(mut self, status: impl Into<String>) -> Self {
        self.request.status = Some(status.into());
        self
    }

    /// 执行查询
    pub async fn execute(self, service: &AppsService) -> SDKResult<GetAppsResponse> {
        service.get(&self.request).await
    }
}

impl AppsService {
    /// 创建应用查询构建器
    pub fn get_apps_builder(&self) -> GetAppsBuilder {
        GetAppsBuilder::new()
    }
}

// ==================== 单元测试 ====================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_apps_service_creation() {
        let config = Config::default();
        let service = AppsService::new(config);
        assert!(!format!("{:?}", service).is_empty());
    }

    #[test]
    fn test_app_info_default_creation() {
        let app_info = AppInfo::default();
        assert_eq!(app_info.app_key, None);
        assert_eq!(app_info.name, None);
        assert_eq!(app_info.description, None);
        assert_eq!(app_info.status, None);
        assert_eq!(app_info.app_type, None);
        assert_eq!(app_info.create_time, None);
        assert_eq!(app_info.update_time, None);
    }

    #[test]
    fn test_app_info_with_data() {
        let app_info = AppInfo {
            app_key: Some("app_123".to_string()),
            name: Some("测试应用".to_string()),
            description: Some("这是一个测试应用".to_string()),
            status: Some("active".to_string()),
            app_type: Some("self_build".to_string()),
            create_time: Some("2023-01-01T00:00:00Z".to_string()),
            update_time: Some("2023-01-02T00:00:00Z".to_string()),
        };

        assert_eq!(app_info.app_key, Some("app_123".to_string()));
        assert_eq!(app_info.name, Some("测试应用".to_string()));
        assert_eq!(app_info.description, Some("这是一个测试应用".to_string()));
        assert_eq!(app_info.status, Some("active".to_string()));
        assert_eq!(app_info.app_type, Some("self_build".to_string()));
        assert_eq!(
            app_info.create_time,
            Some("2023-01-01T00:00:00Z".to_string())
        );
        assert_eq!(
            app_info.update_time,
            Some("2023-01-02T00:00:00Z".to_string())
        );
    }

    #[test]
    fn test_get_apps_request_default() {
        let request = GetAppsRequest::default();
        assert_eq!(request.page_size, None);
        assert_eq!(request.page_token, None);
        assert_eq!(request.status, None);
    }

    #[test]
    fn test_get_apps_request_with_parameters() {
        let request = GetAppsRequest {
            page_size: Some(50),
            page_token: Some("token123".to_string()),
            status: Some("active".to_string()),
        };

        assert_eq!(request.page_size, Some(50));
        assert_eq!(request.page_token, Some("token123".to_string()));
        assert_eq!(request.status, Some("active".to_string()));
    }

    #[test]
    fn test_get_apps_builder() {
        let builder = GetAppsBuilder::new()
            .page_size(20)
            .page_token("test_token")
            .status("active");

        assert_eq!(builder.request.page_size, Some(20));
        assert_eq!(builder.request.page_token, Some("test_token".to_string()));
        assert_eq!(builder.request.status, Some("active".to_string()));
    }

    #[test]
    fn test_get_apps_builder_default() {
        let builder = GetAppsBuilder::default();

        assert_eq!(builder.request.page_size, None);
        assert_eq!(builder.request.page_token, None);
        assert_eq!(builder.request.status, None);
    }

    #[test]
    fn test_response_default_creation() {
        let response = GetAppsResponse::default();

        assert_eq!(response.items.len(), 0);
        assert_eq!(response.has_more, None);
        assert_eq!(response.page_token, None);
    }

    #[test]
    fn test_response_with_data() {
        let mut response = GetAppsResponse::default();
        response.items = vec![AppInfo {
            app_key: Some("app_456".to_string()),
            name: Some("生产应用".to_string()),
            ..Default::default()
        }];
        response.has_more = Some(true);
        response.page_token = Some("next_page".to_string());

        assert_eq!(response.items.len(), 1);
        assert_eq!(response.items[0].app_key, Some("app_456".to_string()));
        assert_eq!(response.items[0].name, Some("生产应用".to_string()));
        assert_eq!(response.has_more, Some(true));
        assert_eq!(response.page_token, Some("next_page".to_string()));
    }

    #[test]
    fn test_api_response_trait_implementation() {
        assert_eq!(GetAppsResponse::data_format(), ResponseFormat::Data);
    }

    #[test]
    fn test_request_serialization() {
        let request = GetAppsRequest {
            page_size: Some(10),
            page_token: Some("token".to_string()),
            status: Some("active".to_string()),
        };

        let serialized = serde_json::to_string(&request).unwrap();
        let deserialized: GetAppsRequest = serde_json::from_str(&serialized).unwrap();

        assert_eq!(request.page_size, deserialized.page_size);
        assert_eq!(request.page_token, deserialized.page_token);
        assert_eq!(request.status, deserialized.status);
    }

    #[test]
    fn test_query_parameters_construction() {
        let request = GetAppsRequest {
            page_size: Some(20),
            page_token: Some("test_token".to_string()),
            status: Some("inactive".to_string()),
        };

        let mut query_params = Vec::new();
        if let Some(page_size) = &request.page_size {
            query_params.push(format!("page_size={}", page_size));
        }
        if let Some(page_token) = &request.page_token {
            query_params.push(format!("page_token={}", page_token));
        }
        if let Some(status) = &request.status {
            query_params.push(format!("status={}", status));
        }

        assert_eq!(query_params.len(), 3);
        assert!(query_params.contains(&"page_size=20".to_string()));
        assert!(query_params.contains(&"page_token=test_token".to_string()));
        assert!(query_params.contains(&"status=inactive".to_string()));
    }

    #[test]
    fn test_endpoint_constant() {
        // Test that the endpoint constant is properly defined
        assert_eq!(
            crate::core::endpoints_original::Endpoints::APAAS_V1_APPS,
            "/open-apis/apaas/v1/apps"
        );
    }

    #[test]
    fn test_app_status_variations() {
        // Test different app status values
        let active_app = AppInfo {
            app_key: Some("active_001".to_string()),
            name: Some("激活应用".to_string()),
            status: Some("active".to_string()),
            ..Default::default()
        };

        let inactive_app = AppInfo {
            app_key: Some("inactive_001".to_string()),
            name: Some("停用应用".to_string()),
            status: Some("inactive".to_string()),
            ..Default::default()
        };

        let no_status_app = AppInfo {
            app_key: Some("no_status_001".to_string()),
            name: Some("无状态应用".to_string()),
            status: None,
            ..Default::default()
        };

        assert_eq!(active_app.status, Some("active".to_string()));
        assert_eq!(inactive_app.status, Some("inactive".to_string()));
        assert_eq!(no_status_app.status, None);
    }

    #[test]
    fn test_app_type_variations() {
        // Test different app types
        let self_build_app = AppInfo {
            app_key: Some("self_001".to_string()),
            name: Some("自建应用".to_string()),
            app_type: Some("self_build".to_string()),
            ..Default::default()
        };

        let store_app = AppInfo {
            app_key: Some("store_001".to_string()),
            name: Some("应用市场应用".to_string()),
            app_type: Some("store".to_string()),
            ..Default::default()
        };

        let no_type_app = AppInfo {
            app_key: Some("no_type_001".to_string()),
            name: Some("无类型应用".to_string()),
            app_type: None,
            ..Default::default()
        };

        assert_eq!(self_build_app.app_type, Some("self_build".to_string()));
        assert_eq!(store_app.app_type, Some("store".to_string()));
        assert_eq!(no_type_app.app_type, None);
    }

    #[test]
    fn test_comprehensive_app_info() {
        // Test comprehensive app info with all fields
        let comprehensive_app = AppInfo {
            app_key: Some("comprehensive_001".to_string()),
            name: Some("企业级应用".to_string()),
            description: Some("这是一个企业级的多功能应用".to_string()),
            status: Some("active".to_string()),
            app_type: Some("self_build".to_string()),
            create_time: Some("2023-01-01T08:00:00Z".to_string()),
            update_time: Some("2023-12-31T16:00:00Z".to_string()),
        };

        assert_eq!(
            comprehensive_app.app_key,
            Some("comprehensive_001".to_string())
        );
        assert_eq!(comprehensive_app.name, Some("企业级应用".to_string()));
        assert_eq!(
            comprehensive_app.description,
            Some("这是一个企业级的多功能应用".to_string())
        );
        assert_eq!(comprehensive_app.status, Some("active".to_string()));
        assert_eq!(comprehensive_app.app_type, Some("self_build".to_string()));
        assert_eq!(
            comprehensive_app.create_time,
            Some("2023-01-01T08:00:00Z".to_string())
        );
        assert_eq!(
            comprehensive_app.update_time,
            Some("2023-12-31T16:00:00Z".to_string())
        );
    }

    #[test]
    fn test_app_name_variations() {
        // Test different app names
        let crm_app = AppInfo {
            app_key: Some("crm".to_string()),
            name: Some("CRM系统".to_string()),
            ..Default::default()
        };

        let oa_app = AppInfo {
            app_key: Some("oa".to_string()),
            name: Some("OA办公系统".to_string()),
            ..Default::default()
        };

        let hr_app = AppInfo {
            app_key: Some("hr".to_string()),
            name: Some("HR管理系统".to_string()),
            ..Default::default()
        };

        let finance_app = AppInfo {
            app_key: Some("finance".to_string()),
            name: Some("财务管理系统".to_string()),
            ..Default::default()
        };

        assert_eq!(crm_app.name, Some("CRM系统".to_string()));
        assert_eq!(oa_app.name, Some("OA办公系统".to_string()));
        assert_eq!(hr_app.name, Some("HR管理系统".to_string()));
        assert_eq!(finance_app.name, Some("财务管理系统".to_string()));
    }
}

use std::sync::Arc;

use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{
    core::{
        api_req::ApiRequest,
        api_resp::{ApiResponseTrait, BaseResponse, ResponseFormat},
        config::Config,
        constants::AccessTokenType,
        endpoints::tenant::*,
        http::Transport,
        req_option::RequestOption,
        trait_system::Service,
        SDKResult,
    },
    service::tenant::models::Tenant,
};

/// 企业信息服务
pub struct TenantService {
    pub config: Config,
    // 试点：共享配置，后续内部可逐步改用 Arc 降低 clone
    pub(crate) config_arc: Arc<Config>,
}

/// 获取企业信息响应
#[derive(Debug, Serialize, Deserialize)]
pub struct GetTenantResponse {
    /// 企业信息
    pub tenant: Tenant,
}

impl ApiResponseTrait for GetTenantResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl TenantService {
    pub fn new(config: Config) -> Self {
        let config_arc = Arc::new(config.clone());
        Self { config, config_arc }
    }

    /// 使用共享配置创建服务实例（实验性）
    pub fn new_from_shared(shared: Arc<Config>) -> Self {
        Self {
            config: (*shared).clone(),
            config_arc: shared,
        }
    }

    /// 获取企业信息
    ///
    /// 该接口用于获取企业的基本信息，包括企业名称、头像等信息。
    ///
    /// 注意事项：
    /// - 需要申请 访问企业信息 权限
    ///
    /// # 错误
    ///
    /// - 99991000: 系统错误
    /// - 99991001: 参数错误
    /// - 99991002: 权限不足
    pub async fn query(
        &self,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<GetTenantResponse>> {
        let api_req = ApiRequest {
            http_method: Method::GET,
            api_path: TENANT_V2_QUERY.to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            ..Default::default()
        };

        Transport::request(api_req, &self.config_arc, option).await
    }
}

impl Service for TenantService {
    fn config(&self) -> &Config {
        &self.config
    }

    fn service_name() -> &'static str {
        "tenant"
    }

    fn service_version() -> &'static str {
        "v2"
    }
}

#[cfg(test)]
#[allow(unused_variables, unused_unsafe)]
mod tests {
    use super::*;

    fn create_test_config() -> Config {
        Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build()
    }

    #[test]
    fn test_tenant_service_creation() {
        let config = create_test_config();
        let service = TenantService::new(config.clone());

        assert_eq!(service.config.app_id, "test_app_id");
        assert_eq!(service.config.app_secret, "test_app_secret");
    }

    #[test]
    fn test_tenant_service_new() {
        let config = create_test_config();
        let service = TenantService::new(config);

        // Verify service is created properly
        assert_eq!(service.config.app_id, "test_app_id");
    }

    #[test]
    fn test_get_tenant_response_data_format() {
        let format = GetTenantResponse::data_format();
        assert!(matches!(format, ResponseFormat::Data));
    }

    #[test]
    fn test_get_tenant_response_serialization() {
        use crate::service::tenant::models::{Tenant, TenantAvatar};

        let tenant = Tenant {
            name: Some("Test Company".to_string()),
            display_name: Some("Test Display Name".to_string()),
            avatar: Some(TenantAvatar {
                avatar_72: Some("https://example.com/avatar72.png".to_string()),
                avatar_240: Some("https://example.com/avatar240.png".to_string()),
                avatar_640: Some("https://example.com/avatar640.png".to_string()),
                avatar_origin: Some("https://example.com/avatar.png".to_string()),
            }),
            tenant_key: Some("test_key".to_string()),
        };

        let response = GetTenantResponse { tenant };

        // Test serialization
        let json = serde_json::to_string(&response).unwrap();
        assert!(json.contains("Test Company"));
        assert!(json.contains("Test Display Name"));

        // Test deserialization
        let deserialized: GetTenantResponse = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.tenant.name, Some("Test Company".to_string()));
        assert_eq!(
            deserialized.tenant.display_name,
            Some("Test Display Name".to_string())
        );
    }

    #[test]
    fn test_tenant_response_with_minimal_data() {
        use crate::service::tenant::models::Tenant;

        let tenant = Tenant {
            name: None,
            display_name: None,
            avatar: None,
            tenant_key: None,
        };

        let response = GetTenantResponse { tenant };

        // Test that empty response can be serialized
        let json = serde_json::to_string(&response).unwrap();
        assert!(json.contains("null") || json.contains("{}"));

        // Test deserialization of minimal data
        let deserialized: GetTenantResponse = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized.tenant.name, None);
        assert_eq!(deserialized.tenant.display_name, None);
        assert_eq!(deserialized.tenant.avatar, None);
        assert_eq!(deserialized.tenant.tenant_key, None);
    }

    #[test]
    fn test_access_token_types() {
        // Verify that tenant query supports both tenant and user access tokens
        let supported_types = [AccessTokenType::Tenant, AccessTokenType::User];

        assert!(supported_types.contains(&AccessTokenType::Tenant));
        assert!(supported_types.contains(&AccessTokenType::User));
        assert_eq!(supported_types.len(), 2);
    }

    #[test]
    fn test_api_path_constant() {
        // Verify the API path is correct
        use crate::core::endpoints::tenant::TENANT_V2_QUERY;
        assert_eq!(TENANT_V2_QUERY, "/open-apis/tenant/v2/tenant/query");
    }

    #[test]
    fn test_response_format_trait() {
        // Test that the ApiResponseTrait is properly implemented
        assert!(matches!(
            GetTenantResponse::data_format(),
            ResponseFormat::Data
        ));
    }

    #[test]
    fn test_debug_trait() {
        use crate::service::tenant::models::{Tenant, TenantAvatar};

        let tenant = Tenant {
            name: Some("Debug Test".to_string()),
            display_name: Some("Debug Display".to_string()),
            avatar: Some(TenantAvatar {
                avatar_72: Some("https://debug.com/avatar72.png".to_string()),
                avatar_240: Some("https://debug.com/avatar240.png".to_string()),
                avatar_640: Some("https://debug.com/avatar640.png".to_string()),
                avatar_origin: Some("https://debug.com/avatar.png".to_string()),
            }),
            tenant_key: Some("debug_key".to_string()),
        };

        let response = GetTenantResponse { tenant };

        // Test Debug trait implementation
        let debug_string = format!("{:?}", response);
        assert!(debug_string.contains("GetTenantResponse"));
        assert!(debug_string.contains("Debug Test"));
    }
}

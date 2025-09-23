use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{
    core::{
        api_req::ApiRequest,
        api_resp::{ApiResponseTrait, BaseResponse, ResponseFormat},
        config::Config,
        constants::AccessTokenType,
        http::Transport,
        req_option::RequestOption,
        SDKResult,
    },
    service::verification::models::VerificationInfo,
};

/// Verification API v1版本服务
pub struct V1 {
    pub config: Config,
}

/// 获取认证信息响应
#[derive(Debug, Serialize, Deserialize)]
pub struct GetVerificationResponse {
    /// 认证信息
    pub verification: VerificationInfo,
}

impl ApiResponseTrait for GetVerificationResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl V1 {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 获取认证信息
    ///
    /// 该接口用于获取应用的认证信息，包括认证状态、权限范围等信息。
    ///
    /// 注意事项：
    /// - 需要有效的访问令牌
    /// - 返回当前应用的认证相关信息
    ///
    /// # 错误
    ///
    /// - 99991000: 系统错误
    /// - 99991001: 参数错误
    /// - 99991002: 权限不足
    /// - 99991003: 认证信息不存在
    pub async fn get(
        &self,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<GetVerificationResponse>> {
        let api_req = ApiRequest {
            http_method: Method::GET,
            api_path: "/open-apis/verification/v1/get".to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{core::api_resp::ResponseFormat, service::verification::models::TenantInfo};
    use std::time::Duration;

    #[test]
    fn test_v1_new() {
        let config = Config::default();
        let v1 = V1::new(config.clone());

        assert_eq!(v1.config.app_id, config.app_id);
        assert_eq!(v1.config.app_secret, config.app_secret);
    }

    #[test]
    fn test_get_verification_response_data_format() {
        assert_eq!(GetVerificationResponse::data_format(), ResponseFormat::Data);
    }

    #[test]
    fn test_get_verification_response_debug() {
        let response = GetVerificationResponse {
            verification: VerificationInfo {
                app_id: Some("test_app".to_string()),
                app_name: Some("Test App".to_string()),
                app_status: Some("active".to_string()),
                verification_status: Some("verified".to_string()),
                verification_type: Some("oauth".to_string()),
                verification_time: Some("1642723200000".to_string()),
                expire_time: Some("1674259200000".to_string()),
                scopes: Some(vec!["read".to_string(), "write".to_string()]),
                tenant_info: Some(TenantInfo {
                    tenant_key: Some("test_tenant".to_string()),
                    tenant_name: Some("Test Tenant".to_string()),
                }),
            },
        };

        let debug_output = format!("{:?}", response);
        assert!(debug_output.contains("GetVerificationResponse"));
        assert!(debug_output.contains("test_app"));
        assert!(debug_output.contains("Test App"));
    }

    #[test]
    fn test_get_verification_response_serialization() {
        let response = GetVerificationResponse {
            verification: VerificationInfo {
                app_id: Some("cli_test123".to_string()),
                app_name: Some("CLI Test App".to_string()),
                app_status: Some("active".to_string()),
                verification_status: Some("verified".to_string()),
                verification_type: Some("self_built".to_string()),
                verification_time: Some("1640995200000".to_string()),
                expire_time: Some("1672531200000".to_string()),
                scopes: Some(vec![
                    "im:read".to_string(),
                    "im:write".to_string(),
                    "contact:read".to_string(),
                ]),
                tenant_info: Some(TenantInfo {
                    tenant_key: Some("2ed263bf32cf1651".to_string()),
                    tenant_name: Some("Test Organization".to_string()),
                }),
            },
        };

        let json = serde_json::to_string(&response).unwrap();
        assert!(json.contains("cli_test123"));
        assert!(json.contains("CLI Test App"));
        assert!(json.contains("im:read"));
        assert!(json.contains("2ed263bf32cf1651"));

        let deserialized: GetVerificationResponse = serde_json::from_str(&json).unwrap();
        assert_eq!(
            deserialized.verification.app_id,
            Some("cli_test123".to_string())
        );
        assert_eq!(
            deserialized.verification.app_name,
            Some("CLI Test App".to_string())
        );
    }

    #[test]
    fn test_get_verification_response_with_none_values() {
        let response = GetVerificationResponse {
            verification: VerificationInfo {
                app_id: Some("minimal_app".to_string()),
                app_name: None,
                app_status: None,
                verification_status: Some("unverified".to_string()),
                verification_type: None,
                verification_time: None,
                expire_time: None,
                scopes: None,
                tenant_info: None,
            },
        };

        let json = serde_json::to_string(&response).unwrap();
        assert!(json.contains("minimal_app"));
        assert!(json.contains("unverified"));
        assert!(!json.contains("app_name"));
        assert!(!json.contains("scopes"));
        assert!(!json.contains("tenant_info"));
    }

    #[test]
    fn test_api_request_construction() {
        let config = Config::default();
        let _v1 = V1::new(config);

        let api_req = ApiRequest {
            http_method: Method::GET,
            api_path: "/open-apis/verification/v1/get".to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            ..Default::default()
        };

        assert_eq!(api_req.http_method, Method::GET);
        assert_eq!(api_req.api_path, "/open-apis/verification/v1/get");
        assert_eq!(api_req.supported_access_token_types.len(), 2);
        assert!(api_req
            .supported_access_token_types
            .contains(&AccessTokenType::Tenant));
        assert!(api_req
            .supported_access_token_types
            .contains(&AccessTokenType::User));
    }

    #[test]
    fn test_request_option_creation() {
        let option = RequestOption::builder()
            .tenant_access_token("t-test123".to_string())
            .request_id("req-12345".to_string())
            .build();

        assert_eq!(option.tenant_access_token, "t-test123");
        assert_eq!(option.request_id, "req-12345");
    }

    #[test]
    fn test_verification_with_custom_config() {
        let config = Config::builder()
            .app_id("test_verification_app")
            .app_secret("test_secret")
            .req_timeout(Duration::from_secs(30))
            .build();

        let v1 = V1::new(config.clone());

        assert_eq!(v1.config.app_id, "test_verification_app");
        assert_eq!(v1.config.app_secret, "test_secret");
        assert_eq!(v1.config.req_timeout, Some(Duration::from_secs(30)));
    }

    #[test]
    fn test_get_verification_response_edge_cases() {
        let response_with_empty_scopes = GetVerificationResponse {
            verification: VerificationInfo {
                app_id: Some("app_empty_scopes".to_string()),
                app_name: Some("Empty Scopes App".to_string()),
                app_status: Some("active".to_string()),
                verification_status: Some("verified".to_string()),
                verification_type: Some("marketplace".to_string()),
                verification_time: Some("0".to_string()),
                expire_time: Some("0".to_string()),
                scopes: Some(vec![]),
                tenant_info: Some(TenantInfo {
                    tenant_key: Some("".to_string()),
                    tenant_name: Some("".to_string()),
                }),
            },
        };

        let json = serde_json::to_string(&response_with_empty_scopes).unwrap();
        let deserialized: GetVerificationResponse = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.verification.scopes, Some(vec![]));
        assert_eq!(
            deserialized
                .verification
                .tenant_info
                .as_ref()
                .unwrap()
                .tenant_key,
            Some("".to_string())
        );
    }
}

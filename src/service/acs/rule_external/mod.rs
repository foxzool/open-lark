use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{
    core::{
        api_req::ApiRequest,
        api_resp::{ApiResponseTrait, BaseResponse, ResponseFormat},
        config::Config,
        constants::AccessTokenType,
        endpoints::{acs::*, EndpointBuilder},
        http::Transport,
        req_option::RequestOption,
        SDKResult,
    },
    service::acs::models::{RuleExternal, RuleStatus},
};

/// 权限组管理服务
#[derive(Debug)]
pub struct RuleExternalService {
    pub config: Config,
}

impl RuleExternalService {
    /// 创建权限组管理服务实例
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 创建或更新权限组
    ///
    /// 创建新的权限组或更新现有权限组的信息。
    ///
    /// # Arguments
    ///
    /// * `request` - 权限组创建或更新请求
    /// * `option` - 请求选项，可选
    ///
    /// # Returns
    ///
    /// 返回权限组信息
    pub async fn create_or_update_rule(
        &self,
        request: RuleCreateOrUpdateRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<RuleCreateOrUpdateResponse>> {
        let api_req = ApiRequest {
            http_method: Method::POST,
            api_path: ACS_V1_RULE_EXTERNAL.to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: serde_json::to_vec(&request)?,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }

    /// 获取权限组信息
    ///
    /// 根据权限组ID获取权限组的详细信息。
    ///
    /// # Arguments
    ///
    /// * `rule_id` - 权限组ID
    /// * `option` - 请求选项，可选
    ///
    /// # Returns
    ///
    /// 返回权限组详细信息
    pub async fn get_rule(
        &self,
        rule_id: &str,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<RuleGetResponse>> {
        let api_req = ApiRequest {
            http_method: Method::GET,
            api_path: EndpointBuilder::replace_param(
                ACS_V1_RULE_EXTERNAL_OPERATION,
                "rule_id",
                rule_id,
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: vec![],
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }

    /// 删除权限组
    ///
    /// 删除指定的权限组。
    ///
    /// # Arguments
    ///
    /// * `rule_id` - 权限组ID
    /// * `option` - 请求选项，可选
    ///
    /// # Returns
    ///
    /// 返回删除结果
    pub async fn delete_rule(
        &self,
        rule_id: &str,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<RuleDeleteResponse>> {
        let api_req = ApiRequest {
            http_method: Method::DELETE,
            api_path: EndpointBuilder::replace_param(
                ACS_V1_RULE_EXTERNAL_OPERATION,
                "rule_id",
                rule_id,
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: vec![],
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }

    /// 设备绑定权限组
    ///
    /// 将指定设备绑定到权限组，或修改设备的权限组绑定关系。
    ///
    /// # Arguments
    ///
    /// * `request` - 设备绑定请求
    /// * `option` - 请求选项，可选
    ///
    /// # Returns
    ///
    /// 返回绑定结果
    pub async fn bind_device(
        &self,
        request: DeviceBindRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<DeviceBindResponse>> {
        let api_req = ApiRequest {
            http_method: Method::POST,
            api_path: ACS_V1_RULE_EXTERNAL_DEVICE_BIND.to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: serde_json::to_vec(&request)?,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }
}

/// 权限组创建或更新请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuleCreateOrUpdateRequest {
    /// 权限组ID（更新时必须提供）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule_id: Option<String>,
    /// 权限组名称
    pub name: String,
    /// 权限组描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// 权限组状态
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<RuleStatus>,
    /// 关联的用户ID列表
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_ids: Option<Vec<String>>,
    /// 生效开始时间戳
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<i64>,
    /// 生效结束时间戳
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<i64>,
}

/// 权限组创建或更新响应
#[derive(Debug, Serialize, Deserialize)]
pub struct RuleCreateOrUpdateResponse {
    /// 权限组信息
    pub rule_external: RuleExternal,
}

impl ApiResponseTrait for RuleCreateOrUpdateResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 权限组详情查询响应
#[derive(Debug, Serialize, Deserialize)]
pub struct RuleGetResponse {
    /// 权限组详细信息
    pub rule_external: RuleExternal,
}

impl ApiResponseTrait for RuleGetResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 权限组删除响应
#[derive(Debug, Serialize, Deserialize)]
pub struct RuleDeleteResponse {
    /// 删除成功标识
    pub success: bool,
}

impl ApiResponseTrait for RuleDeleteResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 设备绑定权限组请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeviceBindRequest {
    /// 设备ID
    pub device_id: String,
    /// 权限组ID列表
    pub rule_ids: Vec<String>,
    /// 绑定操作类型 (bind: 绑定, unbind: 解绑)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation: Option<String>,
}

/// 设备绑定权限组响应
#[derive(Debug, Serialize, Deserialize)]
pub struct DeviceBindResponse {
    /// 绑定成功标识
    pub success: bool,
    /// 绑定的设备ID
    pub device_id: String,
    /// 绑定的权限组ID列表
    pub rule_ids: Vec<String>,
}

impl ApiResponseTrait for DeviceBindResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::{config::Config, constants::AppType};

    #[test]
    fn test_rule_external_service_creation() {
        let config = Config::builder()
            .app_id("test_app_id")
            .app_secret("test_secret")
            .build();
        let service = RuleExternalService::new(config);
        assert_eq!(service.config.app_id, "test_app_id");
        assert_eq!(service.config.app_secret, "test_secret");
    }

    #[test]
    fn test_rule_external_service_debug_trait() {
        let config = Config::default();
        let service = RuleExternalService::new(config);
        assert!(!format!("{:?}", service).is_empty());
    }

    #[test]
    fn test_rule_external_service_with_marketplace_config() {
        let config = Config::builder()
            .app_id("cli_rule_test")
            .app_secret("rule_secret")
            .app_type(AppType::Marketplace)
            .build();
        let service = RuleExternalService::new(config);
        assert_eq!(service.config.app_type, AppType::Marketplace);
        assert_eq!(service.config.app_id, "cli_rule_test");
    }

    #[test]
    fn test_rule_create_or_update_request_default() {
        let request = RuleCreateOrUpdateRequest {
            rule_id: None,
            name: "Test Rule".to_string(),
            description: None,
            status: None,
            user_ids: None,
            start_time: None,
            end_time: None,
        };
        assert!(request.rule_id.is_none());
        assert_eq!(request.name, "Test Rule");
        assert!(request.description.is_none());
        assert!(request.status.is_none());
        assert!(request.user_ids.is_none());
        assert!(request.start_time.is_none());
        assert!(request.end_time.is_none());
    }

    #[test]
    fn test_rule_create_or_update_request_with_all_fields() {
        let request = RuleCreateOrUpdateRequest {
            rule_id: Some("rule_123".to_string()),
            name: "Access Control Rule".to_string(),
            description: Some("A comprehensive rule for access control".to_string()),
            status: Some(RuleStatus::Active),
            user_ids: Some(vec!["user_1".to_string(), "user_2".to_string()]),
            start_time: Some(1640995200), // 2022-01-01 00:00:00 UTC
            end_time: Some(1672531200),   // 2023-01-01 00:00:00 UTC
        };

        assert_eq!(request.rule_id, Some("rule_123".to_string()));
        assert_eq!(request.name, "Access Control Rule");
        assert_eq!(
            request.description,
            Some("A comprehensive rule for access control".to_string())
        );
        assert_eq!(request.status, Some(RuleStatus::Active));
        assert_eq!(
            request.user_ids,
            Some(vec!["user_1".to_string(), "user_2".to_string()])
        );
        assert_eq!(request.start_time, Some(1640995200));
        assert_eq!(request.end_time, Some(1672531200));
    }

    #[test]
    fn test_rule_create_or_update_request_clone() {
        let original = RuleCreateOrUpdateRequest {
            rule_id: Some("rule_clone".to_string()),
            name: "Clone Test Rule".to_string(),
            description: Some("Testing clone functionality".to_string()),
            status: Some(RuleStatus::Disabled),
            user_ids: Some(vec!["user_clone".to_string()]),
            start_time: Some(1640995200),
            end_time: Some(1672531200),
        };

        let cloned = original.clone();
        assert_eq!(original.rule_id, cloned.rule_id);
        assert_eq!(original.name, cloned.name);
        assert_eq!(original.description, cloned.description);
        assert_eq!(original.status, cloned.status);
        assert_eq!(original.user_ids, cloned.user_ids);
        assert_eq!(original.start_time, cloned.start_time);
        assert_eq!(original.end_time, cloned.end_time);
    }

    #[test]
    fn test_rule_create_or_update_request_serialization() {
        let request = RuleCreateOrUpdateRequest {
            rule_id: Some("serialize_rule".to_string()),
            name: "Serialization Test".to_string(),
            description: Some("Testing serialization".to_string()),
            status: Some(RuleStatus::Active),
            user_ids: Some(vec!["user_serialize".to_string()]),
            start_time: Some(1640995200),
            end_time: Some(1672531200),
        };

        let json = serde_json::to_string(&request).expect("Should serialize");
        let deserialized: RuleCreateOrUpdateRequest =
            serde_json::from_str(&json).expect("Should deserialize");

        assert_eq!(request.rule_id, deserialized.rule_id);
        assert_eq!(request.name, deserialized.name);
        assert_eq!(request.description, deserialized.description);
        assert_eq!(request.status, deserialized.status);
        assert_eq!(request.user_ids, deserialized.user_ids);
        assert_eq!(request.start_time, deserialized.start_time);
        assert_eq!(request.end_time, deserialized.end_time);
    }

    #[test]
    fn test_rule_create_or_update_request_debug() {
        let request = RuleCreateOrUpdateRequest {
            rule_id: Some("debug_rule".to_string()),
            name: "Debug Test".to_string(),
            description: None,
            status: Some(RuleStatus::Active),
            user_ids: None,
            start_time: None,
            end_time: None,
        };
        let debug_str = format!("{:?}", request);
        assert!(debug_str.contains("RuleCreateOrUpdateRequest"));
        assert!(debug_str.contains("debug_rule"));
        assert!(debug_str.contains("Debug Test"));
        assert!(debug_str.contains("Active"));
    }

    #[test]
    fn test_device_bind_request_default() {
        let request = DeviceBindRequest {
            device_id: "device_001".to_string(),
            rule_ids: vec!["rule_1".to_string(), "rule_2".to_string()],
            operation: None,
        };
        assert_eq!(request.device_id, "device_001");
        assert_eq!(
            request.rule_ids,
            vec!["rule_1".to_string(), "rule_2".to_string()]
        );
        assert!(request.operation.is_none());
    }

    #[test]
    fn test_device_bind_request_with_operation() {
        let bind_request = DeviceBindRequest {
            device_id: "device_bind".to_string(),
            rule_ids: vec!["rule_bind_1".to_string()],
            operation: Some("bind".to_string()),
        };
        assert_eq!(bind_request.operation, Some("bind".to_string()));

        let unbind_request = DeviceBindRequest {
            device_id: "device_unbind".to_string(),
            rule_ids: vec!["rule_unbind_1".to_string()],
            operation: Some("unbind".to_string()),
        };
        assert_eq!(unbind_request.operation, Some("unbind".to_string()));
    }

    #[test]
    fn test_device_bind_request_clone() {
        let original = DeviceBindRequest {
            device_id: "device_clone".to_string(),
            rule_ids: vec!["rule_clone_1".to_string(), "rule_clone_2".to_string()],
            operation: Some("bind".to_string()),
        };

        let cloned = original.clone();
        assert_eq!(original.device_id, cloned.device_id);
        assert_eq!(original.rule_ids, cloned.rule_ids);
        assert_eq!(original.operation, cloned.operation);
    }

    #[test]
    fn test_device_bind_request_serialization() {
        let request = DeviceBindRequest {
            device_id: "device_serialize".to_string(),
            rule_ids: vec![
                "rule_serialize_1".to_string(),
                "rule_serialize_2".to_string(),
            ],
            operation: Some("bind".to_string()),
        };

        let json = serde_json::to_string(&request).expect("Should serialize");
        let deserialized: DeviceBindRequest =
            serde_json::from_str(&json).expect("Should deserialize");

        assert_eq!(request.device_id, deserialized.device_id);
        assert_eq!(request.rule_ids, deserialized.rule_ids);
        assert_eq!(request.operation, deserialized.operation);
    }

    #[test]
    fn test_device_bind_request_debug() {
        let request = DeviceBindRequest {
            device_id: "device_debug".to_string(),
            rule_ids: vec!["rule_debug".to_string()],
            operation: Some("bind".to_string()),
        };
        let debug_str = format!("{:?}", request);
        assert!(debug_str.contains("DeviceBindRequest"));
        assert!(debug_str.contains("device_debug"));
        assert!(debug_str.contains("rule_debug"));
        assert!(debug_str.contains("bind"));
    }

    #[test]
    fn test_rule_create_or_update_response_data_format() {
        assert_eq!(
            RuleCreateOrUpdateResponse::data_format(),
            ResponseFormat::Data
        );
    }

    #[test]
    fn test_rule_get_response_data_format() {
        assert_eq!(RuleGetResponse::data_format(), ResponseFormat::Data);
    }

    #[test]
    fn test_rule_delete_response_data_format() {
        assert_eq!(RuleDeleteResponse::data_format(), ResponseFormat::Data);
    }

    #[test]
    fn test_device_bind_response_data_format() {
        assert_eq!(DeviceBindResponse::data_format(), ResponseFormat::Data);
    }

    #[test]
    fn test_rule_create_or_update_response_debug() {
        let response = RuleCreateOrUpdateResponse {
            rule_external: RuleExternal {
                rule_id: "rule_debug_001".to_string(),
                name: "Debug Rule".to_string(),
                description: Some("A rule for debugging".to_string()),
                status: Some(RuleStatus::Active),
                device_ids: Some(vec!["device_001".to_string()]),
                user_ids: Some(vec!["user_001".to_string()]),
                start_time: Some(1640995200),
                end_time: Some(1672531200),
                created_at: Some(1640995200),
                updated_at: Some(1640995200),
            },
        };

        let debug_str = format!("{:?}", response);
        assert!(debug_str.contains("RuleCreateOrUpdateResponse"));
        assert!(debug_str.contains("rule_debug_001"));
        assert!(debug_str.contains("Debug Rule"));
    }

    #[test]
    fn test_rule_get_response_debug() {
        let response = RuleGetResponse {
            rule_external: RuleExternal {
                rule_id: "rule_get_001".to_string(),
                name: "Get Rule".to_string(),
                description: Some("A rule for get testing".to_string()),
                status: Some(RuleStatus::Disabled),
                device_ids: None,
                user_ids: None,
                start_time: None,
                end_time: None,
                created_at: Some(1640995200),
                updated_at: Some(1641000000),
            },
        };

        let debug_str = format!("{:?}", response);
        assert!(debug_str.contains("RuleGetResponse"));
        assert!(debug_str.contains("rule_get_001"));
        assert!(debug_str.contains("Get Rule"));
    }

    #[test]
    fn test_rule_delete_response_debug() {
        let response = RuleDeleteResponse { success: true };

        let debug_str = format!("{:?}", response);
        assert!(debug_str.contains("RuleDeleteResponse"));
        assert!(debug_str.contains("true"));
    }

    #[test]
    fn test_device_bind_response_debug() {
        let response = DeviceBindResponse {
            success: true,
            device_id: "device_bind_001".to_string(),
            rule_ids: vec!["rule_001".to_string(), "rule_002".to_string()],
        };

        let debug_str = format!("{:?}", response);
        assert!(debug_str.contains("DeviceBindResponse"));
        assert!(debug_str.contains("device_bind_001"));
        assert!(debug_str.contains("rule_001"));
        assert!(debug_str.contains("rule_002"));
        assert!(debug_str.contains("true"));
    }

    #[test]
    fn test_rule_create_or_update_response_serialization() {
        let response = RuleCreateOrUpdateResponse {
            rule_external: RuleExternal {
                rule_id: "serialize_rule_001".to_string(),
                name: "Serialize Rule".to_string(),
                description: Some("Serialization test rule".to_string()),
                status: Some(RuleStatus::Active),
                device_ids: Some(vec!["device_serialize".to_string()]),
                user_ids: Some(vec!["user_serialize".to_string()]),
                start_time: Some(1640995200),
                end_time: Some(1672531200),
                created_at: Some(1640995200),
                updated_at: Some(1640995200),
            },
        };

        let json = serde_json::to_string(&response).expect("Should serialize");
        let deserialized: RuleCreateOrUpdateResponse =
            serde_json::from_str(&json).expect("Should deserialize");

        assert_eq!(
            response.rule_external.rule_id,
            deserialized.rule_external.rule_id
        );
        assert_eq!(response.rule_external.name, deserialized.rule_external.name);
        assert_eq!(
            response.rule_external.description,
            deserialized.rule_external.description
        );
    }

    #[test]
    fn test_device_bind_response_serialization() {
        let response = DeviceBindResponse {
            success: true,
            device_id: "device_serialize_001".to_string(),
            rule_ids: vec![
                "rule_serialize_001".to_string(),
                "rule_serialize_002".to_string(),
            ],
        };

        let json = serde_json::to_string(&response).expect("Should serialize");
        let deserialized: DeviceBindResponse =
            serde_json::from_str(&json).expect("Should deserialize");

        assert_eq!(response.success, deserialized.success);
        assert_eq!(response.device_id, deserialized.device_id);
        assert_eq!(response.rule_ids, deserialized.rule_ids);
    }

    #[test]
    fn test_rule_status_enum_values() {
        let active_status = RuleStatus::Active;
        let disabled_status = RuleStatus::Disabled;

        assert_eq!(active_status, RuleStatus::Active);
        assert_eq!(disabled_status, RuleStatus::Disabled);
        assert_ne!(active_status, disabled_status);
    }

    #[test]
    fn test_rule_status_serialization() {
        let active = RuleStatus::Active;
        let disabled = RuleStatus::Disabled;

        let active_json = serde_json::to_string(&active).expect("Should serialize active");
        let disabled_json = serde_json::to_string(&disabled).expect("Should serialize disabled");

        assert_eq!(active_json, "\"active\"");
        assert_eq!(disabled_json, "\"disabled\"");

        let active_deserialized: RuleStatus =
            serde_json::from_str(&active_json).expect("Should deserialize active");
        let disabled_deserialized: RuleStatus =
            serde_json::from_str(&disabled_json).expect("Should deserialize disabled");

        assert_eq!(active, active_deserialized);
        assert_eq!(disabled, disabled_deserialized);
    }

    #[test]
    fn test_access_token_type_support() {
        // The rule external service should support tenant and user access tokens
        let supported_types = [AccessTokenType::Tenant, AccessTokenType::User];
        assert!(supported_types.contains(&AccessTokenType::Tenant));
        assert!(supported_types.contains(&AccessTokenType::User));
        assert!(!supported_types.contains(&AccessTokenType::App));
    }

    #[test]
    fn test_rule_create_or_update_request_with_empty_vectors() {
        let request = RuleCreateOrUpdateRequest {
            rule_id: Some("empty_vector_rule".to_string()),
            name: "Empty Vector Test".to_string(),
            description: Some("Testing with empty vectors".to_string()),
            status: Some(RuleStatus::Active),
            user_ids: Some(vec![]),
            start_time: Some(1640995200),
            end_time: Some(1672531200),
        };

        assert_eq!(request.user_ids, Some(vec![]));

        let json = serde_json::to_string(&request).expect("Should serialize with empty vector");
        let deserialized: RuleCreateOrUpdateRequest =
            serde_json::from_str(&json).expect("Should deserialize with empty vector");

        assert_eq!(request.user_ids, deserialized.user_ids);
    }

    #[test]
    fn test_device_bind_request_with_multiple_rules() {
        let request = DeviceBindRequest {
            device_id: "device_multi_rules".to_string(),
            rule_ids: vec![
                "rule_1".to_string(),
                "rule_2".to_string(),
                "rule_3".to_string(),
                "rule_4".to_string(),
                "rule_5".to_string(),
            ],
            operation: Some("bind".to_string()),
        };

        assert_eq!(request.rule_ids.len(), 5);
        assert!(request.rule_ids.contains(&"rule_3".to_string()));
    }

    #[test]
    fn test_device_bind_request_with_special_characters() {
        let request = DeviceBindRequest {
            device_id: "device_特殊字符_!@#$%".to_string(),
            rule_ids: vec!["rule_中文_123".to_string(), "rule_符号_&*()".to_string()],
            operation: Some("bind".to_string()),
        };

        let json = serde_json::to_string(&request).expect("Should serialize with special chars");
        let deserialized: DeviceBindRequest =
            serde_json::from_str(&json).expect("Should deserialize special chars");

        assert_eq!(request.device_id, deserialized.device_id);
        assert_eq!(request.rule_ids, deserialized.rule_ids);
    }

    #[test]
    fn test_rule_create_or_update_request_time_boundaries() {
        let request = RuleCreateOrUpdateRequest {
            rule_id: Some("time_boundary_rule".to_string()),
            name: "Time Boundary Test".to_string(),
            description: Some("Testing time boundary values".to_string()),
            status: Some(RuleStatus::Active),
            user_ids: None,
            start_time: Some(0),      // Unix epoch
            end_time: Some(i64::MAX), // Maximum timestamp
        };

        assert_eq!(request.start_time, Some(0));
        assert_eq!(request.end_time, Some(i64::MAX));

        let json = serde_json::to_string(&request).expect("Should serialize with boundary times");
        let deserialized: RuleCreateOrUpdateRequest =
            serde_json::from_str(&json).expect("Should deserialize with boundary times");

        assert_eq!(request.start_time, deserialized.start_time);
        assert_eq!(request.end_time, deserialized.end_time);
    }
}

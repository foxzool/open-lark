use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{
    core::{
        api_req::ApiRequest,
        api_resp::{ApiResponseTrait, BaseResponse, ResponseFormat},
        config::Config,
        constants::AccessTokenType,
        endpoints::acs::*,
        http::Transport,
        req_option::RequestOption,
        SDKResult,
    },
    service::acs::models::{Device, DeviceStatus, DeviceType, PageResponse},
};

/// 设备管理服务
#[derive(Debug)]
pub struct DeviceService {
    pub config: Config,
}

impl DeviceService {
    /// 创建设备管理服务实例
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 获取门禁设备列表
    ///
    /// 分页获取门禁设备列表，支持多种筛选条件。
    ///
    /// # Arguments
    ///
    /// * `request` - 设备列表查询请求
    /// * `option` - 请求选项，可选
    ///
    /// # Returns
    ///
    /// 返回设备列表
    pub async fn list_devices(
        &self,
        request: DeviceListRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<DeviceListResponse>> {
        let mut api_req = ApiRequest {
            http_method: Method::GET,
            api_path: ACS_V1_DEVICES.to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: vec![],
            ..Default::default()
        };

        // 添加查询参数
        if let Some(page_token) = request.page_token {
            api_req.query_params.insert("page_token", page_token);
        }

        if let Some(page_size) = request.page_size {
            api_req
                .query_params
                .insert("page_size", page_size.to_string());
        }

        if let Some(device_type) = request.device_type {
            api_req
                .query_params
                .insert("device_type", serde_json::to_string(&device_type)?);
        }

        if let Some(status) = request.status {
            api_req
                .query_params
                .insert("status", serde_json::to_string(&status)?);
        }

        if let Some(location) = request.location {
            api_req.query_params.insert("location", location);
        }

        Transport::request(api_req, &self.config, option).await
    }
}

/// 设备列表查询请求
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DeviceListRequest {
    /// 页码标记
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// 每页数量
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 设备类型筛选
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_type: Option<DeviceType>,
    /// 设备状态筛选
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<DeviceStatus>,
    /// 设备位置筛选
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
}

/// 设备列表查询响应
#[derive(Debug, Serialize, Deserialize)]
pub struct DeviceListResponse {
    /// 设备列表
    #[serde(flatten)]
    pub devices: PageResponse<Device>,
}

impl ApiResponseTrait for DeviceListResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::{config::Config, constants::AppType};

    #[test]
    fn test_device_service_creation() {
        let config = Config::builder()
            .app_id("test_app_id")
            .app_secret("test_secret")
            .build();
        let service = DeviceService::new(config);
        assert_eq!(service.config.app_id, "test_app_id");
        assert_eq!(service.config.app_secret, "test_secret");
    }

    #[test]
    fn test_device_service_debug_trait() {
        let config = Config::default();
        let service = DeviceService::new(config);
        assert!(!format!("{:?}", service).is_empty());
    }

    #[test]
    fn test_device_service_with_marketplace_config() {
        let config = Config::builder()
            .app_id("cli_device_test")
            .app_secret("device_secret")
            .app_type(AppType::Marketplace)
            .build();
        let service = DeviceService::new(config);
        assert_eq!(service.config.app_type, AppType::Marketplace);
        assert_eq!(service.config.app_id, "cli_device_test");
    }

    #[test]
    fn test_device_list_request_default() {
        let request = DeviceListRequest::default();
        assert!(request.page_token.is_none());
        assert!(request.page_size.is_none());
        assert!(request.device_type.is_none());
        assert!(request.status.is_none());
        assert!(request.location.is_none());
    }

    #[test]
    fn test_device_list_request_with_all_fields() {
        let request = DeviceListRequest {
            page_token: Some("device_token_123".to_string()),
            page_size: Some(25),
            device_type: Some(DeviceType::AccessControl),
            status: Some(DeviceStatus::Online),
            location: Some("Building A - Floor 1".to_string()),
        };

        assert_eq!(request.page_token, Some("device_token_123".to_string()));
        assert_eq!(request.page_size, Some(25));
        assert_eq!(request.device_type, Some(DeviceType::AccessControl));
        assert_eq!(request.status, Some(DeviceStatus::Online));
        assert_eq!(request.location, Some("Building A - Floor 1".to_string()));
    }

    #[test]
    fn test_device_list_request_partial_fields() {
        let request = DeviceListRequest {
            page_size: Some(15),
            device_type: Some(DeviceType::FaceRecognition),
            ..Default::default()
        };

        assert!(request.page_token.is_none());
        assert_eq!(request.page_size, Some(15));
        assert_eq!(request.device_type, Some(DeviceType::FaceRecognition));
        assert!(request.status.is_none());
        assert!(request.location.is_none());
    }

    #[test]
    fn test_device_list_request_clone() {
        let original = DeviceListRequest {
            page_token: Some("clone_token".to_string()),
            page_size: Some(30),
            device_type: Some(DeviceType::Turnstile),
            status: Some(DeviceStatus::Offline),
            location: Some("Main Entrance".to_string()),
        };

        let cloned = original.clone();
        assert_eq!(original.page_token, cloned.page_token);
        assert_eq!(original.page_size, cloned.page_size);
        assert_eq!(original.device_type, cloned.device_type);
        assert_eq!(original.status, cloned.status);
        assert_eq!(original.location, cloned.location);
    }

    #[test]
    fn test_device_list_request_serialization() {
        let request = DeviceListRequest {
            page_token: Some("serialize_token".to_string()),
            page_size: Some(100),
            device_type: Some(DeviceType::AccessControl),
            status: Some(DeviceStatus::Maintenance),
            location: Some("Security Office".to_string()),
        };

        let json = serde_json::to_string(&request).expect("Should serialize");
        let deserialized: DeviceListRequest =
            serde_json::from_str(&json).expect("Should deserialize");

        assert_eq!(request.page_token, deserialized.page_token);
        assert_eq!(request.page_size, deserialized.page_size);
        assert_eq!(request.device_type, deserialized.device_type);
        assert_eq!(request.status, deserialized.status);
        assert_eq!(request.location, deserialized.location);
    }

    #[test]
    fn test_device_list_request_debug() {
        let request = DeviceListRequest {
            page_token: Some("debug_token".to_string()),
            device_type: Some(DeviceType::ElevatorController),
            ..Default::default()
        };
        let debug_str = format!("{:?}", request);
        assert!(debug_str.contains("DeviceListRequest"));
        assert!(debug_str.contains("debug_token"));
        assert!(debug_str.contains("ElevatorController"));
    }

    #[test]
    fn test_device_list_response_data_format() {
        assert_eq!(DeviceListResponse::data_format(), ResponseFormat::Data);
    }

    #[test]
    fn test_device_list_response_debug() {
        use crate::service::acs::models::PageResponse;

        let response = DeviceListResponse {
            devices: PageResponse {
                has_more: Some(true),
                page_token: Some("next_device_token".to_string()),
                items: vec![],
            },
        };

        let debug_str = format!("{:?}", response);
        assert!(debug_str.contains("DeviceListResponse"));
        assert!(debug_str.contains("next_device_token"));
        assert!(debug_str.contains("true"));
    }

    #[test]
    fn test_device_list_response_serialization() {
        use crate::service::acs::models::PageResponse;

        let response = DeviceListResponse {
            devices: PageResponse {
                has_more: Some(false),
                page_token: None,
                items: vec![],
            },
        };

        let json = serde_json::to_string(&response).expect("Should serialize");
        let deserialized: DeviceListResponse =
            serde_json::from_str(&json).expect("Should deserialize");

        assert_eq!(response.devices.has_more, deserialized.devices.has_more);
        assert_eq!(response.devices.page_token, deserialized.devices.page_token);
    }

    #[test]
    fn test_device_list_request_with_empty_strings() {
        let request = DeviceListRequest {
            page_token: Some("".to_string()),
            location: Some("".to_string()),
            ..Default::default()
        };

        assert_eq!(request.page_token, Some("".to_string()));
        assert_eq!(request.location, Some("".to_string()));
    }

    #[test]
    fn test_device_list_request_with_special_characters() {
        let request = DeviceListRequest {
            page_token: Some("token_with_特殊字符_&_symbols!@#".to_string()),
            location: Some("Building 中心 - Floor 3层".to_string()),
            ..Default::default()
        };

        let json = serde_json::to_string(&request).expect("Should serialize with special chars");
        let deserialized: DeviceListRequest =
            serde_json::from_str(&json).expect("Should deserialize special chars");

        assert_eq!(request.page_token, deserialized.page_token);
        assert_eq!(request.location, deserialized.location);
    }

    #[test]
    fn test_device_list_request_boundary_values() {
        let request = DeviceListRequest {
            page_size: Some(i32::MAX),
            ..Default::default()
        };

        assert_eq!(request.page_size, Some(i32::MAX));

        let request_min = DeviceListRequest {
            page_size: Some(1),
            ..Default::default()
        };

        assert_eq!(request_min.page_size, Some(1));
    }

    #[test]
    fn test_device_types_and_statuses() {
        // Test different device types
        let access_control_request = DeviceListRequest {
            device_type: Some(DeviceType::AccessControl),
            ..Default::default()
        };
        assert_eq!(
            access_control_request.device_type,
            Some(DeviceType::AccessControl)
        );

        let face_recognition_request = DeviceListRequest {
            device_type: Some(DeviceType::FaceRecognition),
            ..Default::default()
        };
        assert_eq!(
            face_recognition_request.device_type,
            Some(DeviceType::FaceRecognition)
        );

        // Test different device statuses
        let online_request = DeviceListRequest {
            status: Some(DeviceStatus::Online),
            ..Default::default()
        };
        assert_eq!(online_request.status, Some(DeviceStatus::Online));

        let offline_request = DeviceListRequest {
            status: Some(DeviceStatus::Offline),
            ..Default::default()
        };
        assert_eq!(offline_request.status, Some(DeviceStatus::Offline));
    }

    #[test]
    fn test_access_token_type_support() {
        // The device service should support tenant and user access tokens
        let supported_types = [AccessTokenType::Tenant, AccessTokenType::User];
        assert!(supported_types.contains(&AccessTokenType::Tenant));
        assert!(supported_types.contains(&AccessTokenType::User));
        assert!(!supported_types.contains(&AccessTokenType::App));
    }
}

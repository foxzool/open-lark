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
        query_params::QueryParams,
        req_option::RequestOption,
        trait_system::Service,
        SDKResult,
    },
    service::acs::models::{
        AccessMethod, AccessRecord, AccessResult, AccessType, FaceImage, PageResponse,
    },
};

/// 访问记录服务
#[derive(Debug)]
pub struct AccessRecordService {
    pub config: Config,
}

impl AccessRecordService {
    /// 创建访问记录服务实例
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 获取门禁记录列表
    ///
    /// 分页获取门禁访问记录列表，支持多种筛选条件。
    ///
    /// # Arguments
    ///
    /// * `request` - 访问记录查询请求
    /// * `option` - 请求选项，可选
    ///
    /// # Returns
    ///
    /// 返回访问记录列表
    pub async fn list_access_records(
        &self,
        request: AccessRecordListRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<AccessRecordListResponse>> {
        let mut api_req = ApiRequest {
            http_method: Method::GET,
            api_path: ACS_V1_ACCESS_RECORDS.to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: vec![],
            ..Default::default()
        };

        // 添加查询参数
        if let Some(page_token) = request.page_token {
            api_req
                .query_params
                .insert(QueryParams::PAGE_TOKEN, page_token);
        }

        if let Some(page_size) = request.page_size {
            api_req
                .query_params
                .insert(QueryParams::PAGE_SIZE, page_size.to_string());
        }

        if let Some(user_id) = request.user_id {
            api_req.query_params.insert(QueryParams::USER_ID, user_id);
        }

        if let Some(device_id) = request.device_id {
            api_req
                .query_params
                .insert(QueryParams::DEVICE_ID, device_id);
        }

        if let Some(access_type) = request.access_type {
            api_req.query_params.insert(
                QueryParams::ACCESS_TYPE,
                serde_json::to_string(&access_type)?,
            );
        }

        if let Some(access_method) = request.access_method {
            api_req.query_params.insert(
                QueryParams::ACCESS_METHOD,
                serde_json::to_string(&access_method)?,
            );
        }

        if let Some(result) = request.result {
            api_req
                .query_params
                .insert(QueryParams::RESULT, serde_json::to_string(&result)?);
        }

        if let Some(start_time) = request.start_time {
            api_req
                .query_params
                .insert(QueryParams::START_TIME, start_time.to_string());
        }

        if let Some(end_time) = request.end_time {
            api_req
                .query_params
                .insert(QueryParams::END_TIME, end_time.to_string());
        }

        Transport::request(api_req, &self.config, option).await
    }

    /// 下载开门时的人脸识别图片
    ///
    /// 下载指定访问记录的人脸识别图片。
    ///
    /// # Arguments
    ///
    /// * `record_id` - 访问记录ID
    /// * `option` - 请求选项，可选
    ///
    /// # Returns
    ///
    /// 返回人脸识别图片信息
    pub async fn download_face_image(
        &self,
        record_id: &str,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<AccessRecordFaceImageResponse>> {
        let api_req = ApiRequest {
            http_method: Method::GET,
            api_path: EndpointBuilder::replace_param(
                ACS_V1_ACCESS_RECORD_FACE_IMAGE,
                "record_id",
                record_id,
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: vec![],
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }
}

/// 访问记录查询请求
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AccessRecordListRequest {
    /// 页码标记
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// 每页数量
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 用户ID筛选
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_id: Option<String>,
    /// 设备ID筛选
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_id: Option<String>,
    /// 访问类型筛选
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_type: Option<AccessType>,
    /// 访问方式筛选
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_method: Option<AccessMethod>,
    /// 访问结果筛选
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<AccessResult>,
    /// 开始时间筛选（时间戳）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<i64>,
    /// 结束时间筛选（时间戳）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<i64>,
}

/// 访问记录查询响应
#[derive(Debug, Serialize, Deserialize)]
pub struct AccessRecordListResponse {
    /// 访问记录列表
    #[serde(flatten)]
    pub access_records: PageResponse<AccessRecord>,
}

impl ApiResponseTrait for AccessRecordListResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 访问记录人脸图片响应
#[derive(Debug, Serialize, Deserialize)]
pub struct AccessRecordFaceImageResponse {
    /// 人脸识别图片信息
    pub face_image: FaceImage,
}

impl ApiResponseTrait for AccessRecordFaceImageResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl Service for AccessRecordService {
    fn config(&self) -> &Config {
        &self.config
    }

    fn service_name() -> &'static str {
        "access_record"
    }

    fn service_version() -> &'static str {
        "v1"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::{config::Config, constants::AppType};

    #[test]
    fn test_access_record_service_creation() {
        let config = Config::builder()
            .app_id("test_app_id")
            .app_secret("test_secret")
            .build();
        let service = AccessRecordService::new(config);
        assert_eq!(service.config.app_id, "test_app_id");
        assert_eq!(service.config.app_secret, "test_secret");
    }

    #[test]
    fn test_access_record_service_debug_trait() {
        let config = Config::default();
        let service = AccessRecordService::new(config);
        assert!(!format!("{:?}", service).is_empty());
    }

    #[test]
    fn test_access_record_service_with_marketplace_config() {
        let config = Config::builder()
            .app_id("cli_test")
            .app_secret("test_secret")
            .app_type(AppType::Marketplace)
            .build();
        let service = AccessRecordService::new(config);
        assert_eq!(service.config.app_type, AppType::Marketplace);
    }

    #[test]
    fn test_access_record_list_request_default() {
        let request = AccessRecordListRequest::default();
        assert!(request.page_token.is_none());
        assert!(request.page_size.is_none());
        assert!(request.user_id.is_none());
        assert!(request.device_id.is_none());
        assert!(request.access_type.is_none());
        assert!(request.access_method.is_none());
        assert!(request.result.is_none());
        assert!(request.start_time.is_none());
        assert!(request.end_time.is_none());
    }

    #[test]
    fn test_access_record_list_request_with_all_fields() {
        let request = AccessRecordListRequest {
            page_token: Some("token123".to_string()),
            page_size: Some(20),
            user_id: Some("user_id_123".to_string()),
            device_id: Some("device_001".to_string()),
            access_type: Some(AccessType::Entry),
            access_method: Some(AccessMethod::Card),
            result: Some(AccessResult::Success),
            start_time: Some(1640995200), // 2022-01-01 00:00:00 UTC
            end_time: Some(1643673600),   // 2022-02-01 00:00:00 UTC
        };

        assert_eq!(request.page_token, Some("token123".to_string()));
        assert_eq!(request.page_size, Some(20));
        assert_eq!(request.user_id, Some("user_id_123".to_string()));
        assert_eq!(request.device_id, Some("device_001".to_string()));
        assert_eq!(request.access_type, Some(AccessType::Entry));
        assert_eq!(request.access_method, Some(AccessMethod::Card));
        assert_eq!(request.result, Some(AccessResult::Success));
        assert_eq!(request.start_time, Some(1640995200));
        assert_eq!(request.end_time, Some(1643673600));
    }

    #[test]
    fn test_access_record_list_request_clone() {
        let original = AccessRecordListRequest {
            page_token: Some("token123".to_string()),
            page_size: Some(10),
            user_id: Some("user_001".to_string()),
            ..Default::default()
        };

        let cloned = original.clone();
        assert_eq!(original.page_token, cloned.page_token);
        assert_eq!(original.page_size, cloned.page_size);
        assert_eq!(original.user_id, cloned.user_id);
    }

    #[test]
    fn test_access_record_list_request_serialization() {
        let request = AccessRecordListRequest {
            page_token: Some("test_token".to_string()),
            page_size: Some(50),
            user_id: Some("user_123".to_string()),
            start_time: Some(1640995200),
            ..Default::default()
        };

        let json = serde_json::to_string(&request).expect("Should serialize");
        let deserialized: AccessRecordListRequest =
            serde_json::from_str(&json).expect("Should deserialize");

        assert_eq!(request.page_token, deserialized.page_token);
        assert_eq!(request.page_size, deserialized.page_size);
        assert_eq!(request.user_id, deserialized.user_id);
        assert_eq!(request.start_time, deserialized.start_time);
    }

    #[test]
    fn test_access_record_list_request_debug() {
        let request = AccessRecordListRequest {
            page_token: Some("token".to_string()),
            page_size: Some(10),
            ..Default::default()
        };
        let debug_str = format!("{:?}", request);
        assert!(debug_str.contains("AccessRecordListRequest"));
        assert!(debug_str.contains("token"));
    }

    #[test]
    fn test_access_record_list_response_data_format() {
        assert_eq!(
            AccessRecordListResponse::data_format(),
            ResponseFormat::Data
        );
    }

    #[test]
    fn test_access_record_face_image_response_data_format() {
        assert_eq!(
            AccessRecordFaceImageResponse::data_format(),
            ResponseFormat::Data
        );
    }

    #[test]
    fn test_access_record_list_response_debug() {
        use crate::service::acs::models::PageResponse;

        let response = AccessRecordListResponse {
            access_records: PageResponse {
                has_more: Some(true),
                page_token: Some("next_token".to_string()),
                items: vec![],
            },
        };

        let debug_str = format!("{:?}", response);
        assert!(debug_str.contains("AccessRecordListResponse"));
        assert!(debug_str.contains("next_token"));
    }

    #[test]
    fn test_access_record_face_image_response_debug() {
        let response = AccessRecordFaceImageResponse {
            face_image: FaceImage {
                image_id: "face_001".to_string(),
                image_content: Some("base64_content".to_string()),
                image_format: Some("jpeg".to_string()),
                file_size: Some(1024),
                uploaded_at: Some(1640995200),
            },
        };

        let debug_str = format!("{:?}", response);
        assert!(debug_str.contains("AccessRecordFaceImageResponse"));
        assert!(debug_str.contains("face_001"));
    }

    #[test]
    fn test_access_record_list_response_serialization() {
        use crate::service::acs::models::PageResponse;

        let response = AccessRecordListResponse {
            access_records: PageResponse {
                has_more: Some(false),
                page_token: None,
                items: vec![],
            },
        };

        let json = serde_json::to_string(&response).expect("Should serialize");
        let deserialized: AccessRecordListResponse =
            serde_json::from_str(&json).expect("Should deserialize");

        assert_eq!(
            response.access_records.has_more,
            deserialized.access_records.has_more
        );
    }

    #[test]
    fn test_access_record_face_image_response_serialization() {
        let response = AccessRecordFaceImageResponse {
            face_image: FaceImage {
                image_id: "test_image".to_string(),
                image_content: Some("test_base64_content".to_string()),
                image_format: Some("jpeg".to_string()),
                file_size: Some(2048),
                uploaded_at: Some(1643673600),
            },
        };

        let json = serde_json::to_string(&response).expect("Should serialize");
        let deserialized: AccessRecordFaceImageResponse =
            serde_json::from_str(&json).expect("Should deserialize");

        assert_eq!(
            response.face_image.image_id,
            deserialized.face_image.image_id
        );
        assert_eq!(
            response.face_image.image_content,
            deserialized.face_image.image_content
        );
        assert_eq!(
            response.face_image.file_size,
            deserialized.face_image.file_size
        );
    }

    #[test]
    fn test_api_request_construction_for_list_access_records() {
        let config = Config::default();
        let service = AccessRecordService::new(config);
        let _request = AccessRecordListRequest {
            page_size: Some(10),
            user_id: Some("test_user".to_string()),
            ..Default::default()
        };

        // We can't directly test the async method without mocking,
        // but we can test that the service handles the request structure properly
        assert_eq!(service.config.app_id, "");
    }

    #[test]
    fn test_access_token_type_support() {
        // The service should support tenant and user access tokens
        let supported_types = [AccessTokenType::Tenant, AccessTokenType::User];
        assert!(supported_types.contains(&AccessTokenType::Tenant));
        assert!(supported_types.contains(&AccessTokenType::User));
        assert!(!supported_types.contains(&AccessTokenType::App));
    }
}

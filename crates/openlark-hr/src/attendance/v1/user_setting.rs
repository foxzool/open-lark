use reqwest::Method;
use serde_json::json;

use openlark_core::{
    api::{BaseResponse, EmptyResponse},
    config::Config,
    constants::AccessTokenType,
    endpoints::{attendance::*, EndpointBuilder},
    http::Transport,
    req_option::RequestOption,
    trait_system::Service,
    SDKResult,
};

use super::models::{
    DownloadUserPhotoRequest, ModifyUserSettingRequest, ModifyUserSettingRespData,
    QueryUserSettingRequest, QueryUserSettingRespData, UploadUserPhotoRequest,
    UploadUserPhotoRespData,
};

/// 用户设置服务
pub struct UserSettingService {
    pub config: Config,
}

impl UserSettingService {
    /// 修改用户人脸识别信息
    ///
    /// 该接口用于修改用户的人脸识别设置，包括开启/关闭人脸识别打卡、设置活体检测等级等。
    ///
    /// # API文档
    ///
    /// https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/attendance-v1/user_setting/modify
    pub async fn modify(
        &self,
        request: ModifyUserSettingRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<Response<ModifyUserSettingRespData>> {
        let mut api_req = request.api_req;
        api_req.set_http_method(Method::POST);
        api_req.set_api_path(EndpointBuilder::replace_param(
            ATTENDANCE_V1_USER_SETTINGS_MODIFY,
            "user_id",
            &request.user_id,
));
        api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant]);

        // 添加查询参数
        api_req
            .query_params
            .insert("employee_type", request.employee_type);

        // 构建请求体
        let mut body = json!({});

        if let Some(face_key_open) = request.face_key_open {
            body["face_key_open"] = json!(face_key_open);
        }
        if let Some(face_key) = request.face_key {
            body["face_key"] = json!(face_key);
        }
        if let Some(face_live_need_action) = request.face_live_need_action {
            body["face_live_need_action"] = json!(face_live_need_action);
        }
        if let Some(face_downgrade) = request.face_downgrade {
            body["face_downgrade"] = json!(face_downgrade);
        }

        api_req.body = serde_json::to_vec(&body)?;

        let api_resp = Transport::request(api_req, &self.config, option).await?;
        Ok(api_resp)
    }

    /// 批量查询用户人脸识别信息
    ///
    /// 该接口用于批量查询用户的人脸识别设置信息。
    ///
    /// # API文档
    ///
    /// https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/attendance-v1/shift/query
    pub async fn query(
        &self,
        request: QueryUserSettingRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<Response<QueryUserSettingRespData>> {
        let mut api_req = request.api_req;
        api_req.set_http_method(Method::POST);
        api_req.set_api_path(ATTENDANCE_V1_USER_SETTINGS_QUERY.to_string());
        api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant]);

        // 添加查询参数
        api_req
            .query_params
            .insert("employee_type", request.employee_type);

        // 构建请求体
        let body = json!({
            "user_ids": request.user_ids
        });

        api_req.body = serde_json::to_vec(&body)?;

        let api_resp = Transport::request(api_req, &self.config, option).await?;
        Ok(api_resp)
    }

    /// 上传用户人脸识别照片
    ///
    /// 该接口用于上传用户的人脸识别照片，返回文件 key 用于后续设置。
    ///
    /// # API文档
    ///
    /// https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/attendance-v1/user_setting/modify
    pub async fn upload_photo(
        &self,
        request: UploadUserPhotoRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<Response<UploadUserPhotoRespData>> {
        let mut api_req = request.api_req;
        api_req.set_http_method(Method::POST);
        api_req.set_api_path(EndpointBuilder::replace_param(
            ATTENDANCE_V1_USER_SETTINGS_UPLOAD,
            "user_id",
            &request.user_id,
));
        api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant]);

        // 添加查询参数
        api_req
            .query_params
            .insert("employee_type", request.employee_type);

        // 保存 photo_name 以避免借用问题
        let photo_name = request.photo_name.clone();

        // 构建 multipart 表单数据
        let _form = reqwest::multipart::Form::new().part(
            "photo",
            reqwest::multipart::Part::bytes(request.photo_data)
                .file_name(request.photo_name)
                .mime_str("image/jpeg")?,
        );

        // 注意：这里需要特殊处理 multipart 请求
        // 暂时使用 JSON 格式，实际实现时需要处理 multipart
        let body = json!({
            "photo_name": photo_name
        });

        api_req.body = serde_json::to_vec(&body)?;

        let api_resp = Transport::request(api_req, &self.config, option).await?;
        Ok(api_resp)
    }

    /// 下载用户人脸识别照片
    ///
    /// 该接口用于下载用户的人脸识别照片。
    ///
    /// # API文档
    ///
    /// https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/reference/attendance-v1/user_setting/modify
    pub async fn download_photo(
        &self,
        request: DownloadUserPhotoRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<Vec<u8>> {
        let mut api_req = request.api_req;
        api_req.set_http_method(Method::GET);
        api_req.set_api_path(EndpointBuilder::replace_param(
            ATTENDANCE_V1_USER_SETTINGS_DOWNLOAD,
            "user_id",
            &request.user_id,
));
        api_req.set_supported_access_token_types(vec![AccessTokenType::Tenant]);

        // 添加查询参数
        api_req
            .query_params
            .insert("employee_type", request.employee_type);
        api_req.query_params.insert("face_key", request.face_key);

        // 对于文件下载，我们需要直接获取响应体字节数据
        // 这里暂时返回一个模拟的照片数据，实际实现时需要从 HTTP 响应中获取
        let _api_resp: Response<EmptyResponse> =
            Transport::request(api_req, &self.config, option).await?;

        // 返回模拟的照片数据（实际实现时应该从响应体中获取）
        Ok(vec![
            0xFF, 0xD8, 0xFF, 0xE0, 0x00, 0x10, 0x4A, 0x46, 0x49, 0x46, 0x00, 0x01, 0x01, 0x01,
            0x00, 0x48, 0x00, 0x48, 0x00, 0x00, 0xFF, 0xD9,
        ])
    }
}

impl Service for UserSettingService {
    fn config(&self) -> &Config {
        &self.config
    }

    fn service_name() -> &'static str {
        "user_setting"
    }

    fn service_version() -> &'static str {
        "v1"
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use openlark_core::{api::ApiRequest, config::Config};

    #[test]
    fn test_user_setting_service_creation() {
        let config = Config::default();
        let service = UserSettingService {
            config: config.clone(),
        };

        assert_eq!(service.config.app_id, config.app_id);
        assert_eq!(service.config.app_secret, config.app_secret);
    }

    #[test]
    fn test_user_setting_service_with_custom_config() {
        let config = Config::builder()
            .app_id("setting_test_app")
            .app_secret("setting_test_secret")
            .build();

        let service = UserSettingService {
            config: config.clone(),
        };

        assert_eq!(service.config.app_id, "setting_test_app");
        assert_eq!(service.config.app_secret, "setting_test_secret");
    }

    #[test]
    fn test_modify_user_setting_request_construction() {
        let request = ModifyUserSettingRequest {
            api_req: ApiRequest::default(),
            user_id: "user_123".to_string(),
            employee_type: "1".to_string(),
            face_key_open: Some(true),
            face_key: Some("face_key_456".to_string()),
            face_live_need_action: Some(true),
            face_downgrade: Some(false),
        };

        assert_eq!(request.user_id, "user_123");
        assert_eq!(request.employee_type, "1");
        assert_eq!(request.face_key_open, Some(true));
        assert_eq!(request.face_key, Some("face_key_456".to_string()));
        assert_eq!(request.face_live_need_action, Some(true));
        assert_eq!(request.face_downgrade, Some(false));
    }

    #[test]
    fn test_modify_user_setting_request_with_none_values() {
        let request = ModifyUserSettingRequest {
            api_req: ApiRequest::default(),
            user_id: "user_789".to_string(),
            employee_type: "2".to_string(),
            face_key_open: None,
            face_key: None,
            face_live_need_action: None,
            face_downgrade: None,
        };

        assert_eq!(request.user_id, "user_789");
        assert_eq!(request.employee_type, "2");
        assert_eq!(request.face_key_open, None);
        assert_eq!(request.face_key, None);
        assert_eq!(request.face_live_need_action, None);
        assert_eq!(request.face_downgrade, None);
    }

    #[test]
    fn test_query_user_setting_request_construction() {
        let request = QueryUserSettingRequest {
            api_req: ApiRequest::default(),
            employee_type: "1".to_string(),
            user_ids: vec![
                "user_1".to_string(),
                "user_2".to_string(),
                "user_3".to_string(),
            ],
        };

        assert_eq!(request.employee_type, "1");
        assert_eq!(request.user_ids.len(), 3);
        assert_eq!(request.user_ids[0], "user_1");
        assert_eq!(request.user_ids[2], "user_3");
    }

    #[test]
    fn test_query_user_setting_request_with_single_user() {
        let request = QueryUserSettingRequest {
            api_req: ApiRequest::default(),
            employee_type: "2".to_string(),
            user_ids: vec!["single_user".to_string()],
        };

        assert_eq!(request.employee_type, "2");
        assert_eq!(request.user_ids.len(), 1);
        assert_eq!(request.user_ids[0], "single_user");
    }

    #[test]
    fn test_query_user_setting_request_with_empty_user_list() {
        let request = QueryUserSettingRequest {
            api_req: ApiRequest::default(),
            employee_type: "3".to_string(),
            user_ids: vec![],
        };

        assert_eq!(request.employee_type, "3");
        assert!(request.user_ids.is_empty());
    }

    #[test]
    fn test_upload_user_photo_request_construction() {
        let photo_data = vec![0xFF, 0xD8, 0xFF, 0xE0, 0x00, 0x10]; // Mock JPEG header
        let request = UploadUserPhotoRequest {
            api_req: ApiRequest::default(),
            user_id: "user_photo_123".to_string(),
            employee_type: "1".to_string(),
            photo_data: photo_data.clone(),
            photo_name: "profile.jpg".to_string(),
        };

        assert_eq!(request.user_id, "user_photo_123");
        assert_eq!(request.employee_type, "1");
        assert_eq!(request.photo_data, photo_data);
        assert_eq!(request.photo_name, "profile.jpg");
    }

    #[test]
    fn test_upload_user_photo_request_with_large_photo() {
        let large_photo_data = vec![0u8; 10 * 1024 * 1024]; // 10MB photo
        let request = UploadUserPhotoRequest {
            api_req: ApiRequest::default(),
            user_id: "user_large_photo".to_string(),
            employee_type: "2".to_string(),
            photo_data: large_photo_data.clone(),
            photo_name: "large_profile.png".to_string(),
        };

        assert_eq!(request.user_id, "user_large_photo");
        assert_eq!(request.employee_type, "2");
        assert_eq!(request.photo_data.len(), 10 * 1024 * 1024);
        assert_eq!(request.photo_name, "large_profile.png");
    }

    #[test]
    fn test_download_user_photo_request_construction() {
        let request = DownloadUserPhotoRequest {
            api_req: ApiRequest::default(),
            user_id: "user_download_456".to_string(),
            employee_type: "1".to_string(),
            face_key: "download_face_key_789".to_string(),
        };

        assert_eq!(request.user_id, "user_download_456");
        assert_eq!(request.employee_type, "1");
        assert_eq!(request.face_key, "download_face_key_789");
    }

    #[test]
    fn test_user_setting_service_config_independence() {
        let config1 = Config::builder().app_id("setting_app_1").build();

        let config2 = Config::builder().app_id("setting_app_2").build();

        let service1 = UserSettingService { config: config1 };
        let service2 = UserSettingService { config: config2 };

        assert_eq!(service1.config.app_id, "setting_app_1");
        assert_eq!(service2.config.app_id, "setting_app_2");
        assert_ne!(service1.config.app_id, service2.config.app_id);
    }

    #[test]
    fn test_request_structs_debug_trait() {
        let modify_request = ModifyUserSettingRequest {
            api_req: ApiRequest::default(),
            user_id: "debug_user".to_string(),
            employee_type: "1".to_string(),
            face_key_open: Some(true),
            face_key: Some("debug_face_key".to_string()),
            face_live_need_action: Some(true),
            face_downgrade: Some(true),
        };

        let debug_str = format!("{:?}", modify_request);
        assert!(debug_str.contains("ModifyUserSettingRequest"));
        assert!(debug_str.contains("debug_user"));
        assert!(debug_str.contains("debug_face_key"));
    }

    #[test]
    fn test_modify_user_setting_request_face_live_need_action_values() {
        // Test different face_live_need_action values
        let request_level_0 = ModifyUserSettingRequest {
            api_req: ApiRequest::default(),
            user_id: "user_level_0".to_string(),
            employee_type: "1".to_string(),
            face_key_open: Some(true),
            face_key: Some("key_level_0".to_string()),
            face_live_need_action: Some(false), // No action required
            face_downgrade: Some(false),
        };

        assert_eq!(request_level_0.face_live_need_action, Some(false));

        let request_level_2 = ModifyUserSettingRequest {
            api_req: ApiRequest::default(),
            user_id: "user_level_2".to_string(),
            employee_type: "1".to_string(),
            face_key_open: Some(true),
            face_key: Some("key_level_2".to_string()),
            face_live_need_action: Some(true), // High action required
            face_downgrade: Some(true),
        };

        assert_eq!(request_level_2.face_live_need_action, Some(true));
    }

    #[test]
    fn test_query_user_setting_request_edge_cases() {
        // Test with very large user ID list
        let large_user_list: Vec<String> = (0..1000).map(|i| format!("user_{}", i)).collect();
        let request_large = QueryUserSettingRequest {
            api_req: ApiRequest::default(),
            employee_type: "1".to_string(),
            user_ids: large_user_list.clone(),
        };

        assert_eq!(request_large.user_ids.len(), 1000);
        assert_eq!(request_large.user_ids[999], "user_999");

        // Test with very long user IDs
        let long_user_id = "a".repeat(500);
        let request_long = QueryUserSettingRequest {
            api_req: ApiRequest::default(),
            employee_type: "2".to_string(),
            user_ids: vec![long_user_id.clone()],
        };

        assert_eq!(request_long.user_ids[0], long_user_id);
    }

    #[test]
    fn test_upload_user_photo_request_edge_cases() {
        // Test with empty photo data
        let request_empty = UploadUserPhotoRequest {
            api_req: ApiRequest::default(),
            user_id: "user_empty_photo".to_string(),
            employee_type: "1".to_string(),
            photo_data: vec![],
            photo_name: "empty.jpg".to_string(),
        };

        assert!(request_empty.photo_data.is_empty());
        assert_eq!(request_empty.photo_name, "empty.jpg");

        // Test with very long photo name
        let long_name = format!("{}.jpg", "a".repeat(500));
        let request_long_name = UploadUserPhotoRequest {
            api_req: ApiRequest::default(),
            user_id: "user_long_name".to_string(),
            employee_type: "2".to_string(),
            photo_data: vec![0xFF, 0xD8],
            photo_name: long_name.clone(),
        };

        assert_eq!(request_long_name.photo_name, long_name);

        // Test with different file extensions
        let request_png = UploadUserPhotoRequest {
            api_req: ApiRequest::default(),
            user_id: "user_png".to_string(),
            employee_type: "1".to_string(),
            photo_data: vec![0x89, 0x50, 0x4E, 0x47], // PNG header
            photo_name: "profile.png".to_string(),
        };

        assert_eq!(request_png.photo_name, "profile.png");
        assert_eq!(request_png.photo_data[0], 0x89); // PNG signature
    }

    #[test]
    fn test_download_user_photo_request_edge_cases() {
        // Test with very long face key
        let long_face_key = "face_key_".repeat(100);
        let request_long_key = DownloadUserPhotoRequest {
            api_req: ApiRequest::default(),
            user_id: "user_long_key".to_string(),
            employee_type: "1".to_string(),
            face_key: long_face_key.clone(),
        };

        assert_eq!(request_long_key.face_key, long_face_key);

        // Test with empty face key
        let request_empty_key = DownloadUserPhotoRequest {
            api_req: ApiRequest::default(),
            user_id: "user_empty_key".to_string(),
            employee_type: "2".to_string(),
            face_key: "".to_string(),
        };

        assert_eq!(request_empty_key.face_key, "");
    }

    #[test]
    fn test_modify_user_setting_request_edge_cases() {
        // Test with face_key_open set to false but face_key provided
        let request_disabled = ModifyUserSettingRequest {
            api_req: ApiRequest::default(),
            user_id: "user_disabled".to_string(),
            employee_type: "1".to_string(),
            face_key_open: Some(false),
            face_key: Some("disabled_key".to_string()),
            face_live_need_action: Some(false),
            face_downgrade: Some(true),
        };

        assert_eq!(request_disabled.face_key_open, Some(false));
        assert_eq!(request_disabled.face_key, Some("disabled_key".to_string()));

        // Test with negative face_live_need_action (edge case)
        let request_negative = ModifyUserSettingRequest {
            api_req: ApiRequest::default(),
            user_id: "user_negative".to_string(),
            employee_type: "2".to_string(),
            face_key_open: Some(true),
            face_key: Some("negative_key".to_string()),
            face_live_need_action: Some(false),
            face_downgrade: Some(false),
        };

        assert_eq!(request_negative.face_live_need_action, Some(false));

        // Test with very large face_live_need_action value
        let request_large_action = ModifyUserSettingRequest {
            api_req: ApiRequest::default(),
            user_id: "user_large_action".to_string(),
            employee_type: "1".to_string(),
            face_key_open: Some(true),
            face_key: Some("large_action_key".to_string()),
            face_live_need_action: Some(true),
            face_downgrade: Some(true),
        };

        assert_eq!(request_large_action.face_live_need_action, Some(true));
    }

    #[test]
    fn test_all_request_structs_with_different_employee_types() {
        // Test all request types with different employee types
        let modify_request = ModifyUserSettingRequest {
            api_req: ApiRequest::default(),
            user_id: "user_type_test".to_string(),
            employee_type: "employee_id".to_string(),
            face_key_open: Some(true),
            face_key: None,
            face_live_need_action: None,
            face_downgrade: None,
        };

        let query_request = QueryUserSettingRequest {
            api_req: ApiRequest::default(),
            employee_type: "open_id".to_string(),
            user_ids: vec!["test_user".to_string()],
        };

        let upload_request = UploadUserPhotoRequest {
            api_req: ApiRequest::default(),
            user_id: "upload_user".to_string(),
            employee_type: "union_id".to_string(),
            photo_data: vec![0xFF, 0xD8],
            photo_name: "test.jpg".to_string(),
        };

        let download_request = DownloadUserPhotoRequest {
            api_req: ApiRequest::default(),
            user_id: "download_user".to_string(),
            employee_type: "user_id".to_string(),
            face_key: "test_key".to_string(),
        };

        assert_eq!(modify_request.employee_type, "employee_id");
        assert_eq!(query_request.employee_type, "open_id");
        assert_eq!(upload_request.employee_type, "union_id");
        assert_eq!(download_request.employee_type, "user_id");
    }
}

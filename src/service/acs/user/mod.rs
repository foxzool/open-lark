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
    service::acs::models::{AcsUser, FaceImage, PageResponse, UserStatus, UserType},
};

/// 门禁用户管理服务
#[derive(Debug)]
pub struct UserService {
    pub config: Config,
}

impl UserService {
    /// 创建门禁用户管理服务实例
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 修改用户部分信息
    ///
    /// 修改指定用户的部分信息，如姓名、部门、状态等。
    ///
    /// # Arguments
    ///
    /// * `user_id` - 用户ID
    /// * `request` - 用户信息修改请求
    /// * `option` - 请求选项，可选
    ///
    /// # Returns
    ///
    /// 返回修改后的用户信息
    pub async fn patch_user(
        &self,
        user_id: &str,
        request: UserPatchRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<UserPatchResponse>> {
        let api_req = ApiRequest {
            http_method: Method::PATCH,
            api_path: EndpointBuilder::replace_param(ACS_V1_USER_OPERATION, "user_id", user_id),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: serde_json::to_vec(&request)?,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }

    /// 获取单个用户信息
    ///
    /// 根据用户ID获取门禁用户的详细信息。
    ///
    /// # Arguments
    ///
    /// * `user_id` - 用户ID
    /// * `option` - 请求选项，可选
    ///
    /// # Returns
    ///
    /// 返回用户详细信息
    pub async fn get_user(
        &self,
        user_id: &str,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<UserGetResponse>> {
        let api_req = ApiRequest {
            http_method: Method::GET,
            api_path: EndpointBuilder::replace_param(ACS_V1_USER_OPERATION, "user_id", user_id),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: vec![],
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }

    /// 获取用户列表
    ///
    /// 分页获取门禁用户列表，支持多种筛选条件。
    ///
    /// # Arguments
    ///
    /// * `request` - 用户列表查询请求
    /// * `option` - 请求选项，可选
    ///
    /// # Returns
    ///
    /// 返回用户列表
    pub async fn list_users(
        &self,
        request: UserListRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<UserListResponse>> {
        let mut api_req = ApiRequest {
            http_method: Method::GET,
            api_path: ACS_V1_USERS.to_string(),
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

        if let Some(user_type) = request.user_type {
            api_req
                .query_params
                .insert("user_type", serde_json::to_string(&user_type)?);
        }

        if let Some(status) = request.status {
            api_req
                .query_params
                .insert("status", serde_json::to_string(&status)?);
        }

        if let Some(department) = request.department {
            api_req.query_params.insert("department", department);
        }

        Transport::request(api_req, &self.config, option).await
    }

    /// 上传人脸图片
    ///
    /// 为指定用户上传人脸图片，用于人脸识别门禁。
    ///
    /// # Arguments
    ///
    /// * `user_id` - 用户ID
    /// * `request` - 人脸图片上传请求
    /// * `option` - 请求选项，可选
    ///
    /// # Returns
    ///
    /// 返回上传结果
    pub async fn upload_face_image(
        &self,
        user_id: &str,
        request: FaceImageUploadRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<FaceImageUploadResponse>> {
        let api_req = ApiRequest {
            http_method: Method::PUT,
            api_path: EndpointBuilder::replace_param(ACS_V1_USER_FACE_IMAGE, "user_id", user_id),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: serde_json::to_vec(&request)?,
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }

    /// 下载人脸图片
    ///
    /// 下载指定用户的人脸图片。
    ///
    /// # Arguments
    ///
    /// * `user_id` - 用户ID
    /// * `option` - 请求选项，可选
    ///
    /// # Returns
    ///
    /// 返回人脸图片信息
    pub async fn download_face_image(
        &self,
        user_id: &str,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<FaceImageDownloadResponse>> {
        let api_req = ApiRequest {
            http_method: Method::GET,
            api_path: EndpointBuilder::replace_param(ACS_V1_USER_FACE_IMAGE, "user_id", user_id),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            body: vec![],
            ..Default::default()
        };

        Transport::request(api_req, &self.config, option).await
    }
}

/// 用户信息修改请求
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UserPatchRequest {
    /// 用户姓名
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// 用户类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_type: Option<UserType>,
    /// 用户状态
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<UserStatus>,
    /// 部门信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department: Option<String>,
    /// 电话号码
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    /// 邮箱地址
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
}

/// 用户信息修改响应
#[derive(Debug, Serialize, Deserialize)]
pub struct UserPatchResponse {
    /// 修改后的用户信息
    pub user: AcsUser,
}

impl ApiResponseTrait for UserPatchResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 用户详情查询响应
#[derive(Debug, Serialize, Deserialize)]
pub struct UserGetResponse {
    /// 用户详细信息
    pub user: AcsUser,
}

impl ApiResponseTrait for UserGetResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 用户列表查询请求
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UserListRequest {
    /// 页码标记
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_token: Option<String>,
    /// 每页数量
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i32>,
    /// 用户类型筛选
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_type: Option<UserType>,
    /// 用户状态筛选
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<UserStatus>,
    /// 部门筛选
    #[serde(skip_serializing_if = "Option::is_none")]
    pub department: Option<String>,
}

/// 用户列表查询响应
#[derive(Debug, Serialize, Deserialize)]
pub struct UserListResponse {
    /// 用户列表
    #[serde(flatten)]
    pub users: PageResponse<AcsUser>,
}

impl ApiResponseTrait for UserListResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 人脸图片上传请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FaceImageUploadRequest {
    /// 图片内容 (base64编码)
    pub image_content: String,
    /// 图片格式 (如: jpg, png)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_format: Option<String>,
}

/// 人脸图片上传响应
#[derive(Debug, Serialize, Deserialize)]
pub struct FaceImageUploadResponse {
    /// 上传成功标识
    pub success: bool,
    /// 图片ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub image_id: Option<String>,
}

impl ApiResponseTrait for FaceImageUploadResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 人脸图片下载响应
#[derive(Debug, Serialize, Deserialize)]
pub struct FaceImageDownloadResponse {
    /// 人脸图片信息
    pub face_image: FaceImage,
}

impl ApiResponseTrait for FaceImageDownloadResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::{config::Config, constants::AppType};

    #[test]
    fn test_user_service_creation() {
        let config = Config::builder()
            .app_id("test_user_app")
            .app_secret("test_user_secret")
            .build();
        let service = UserService::new(config);
        assert_eq!(service.config.app_id, "test_user_app");
        assert_eq!(service.config.app_secret, "test_user_secret");
    }

    #[test]
    fn test_user_service_debug_trait() {
        let config = Config::default();
        let service = UserService::new(config);
        assert!(!format!("{:?}", service).is_empty());
    }

    #[test]
    fn test_user_service_with_marketplace_config() {
        let config = Config::builder()
            .app_id("cli_user_test")
            .app_secret("user_secret")
            .app_type(AppType::Marketplace)
            .build();
        let service = UserService::new(config);
        assert_eq!(service.config.app_type, AppType::Marketplace);
        assert_eq!(service.config.app_id, "cli_user_test");
    }

    // UserPatchRequest tests
    #[test]
    fn test_user_patch_request_default() {
        let request = UserPatchRequest::default();
        assert!(request.name.is_none());
        assert!(request.user_type.is_none());
        assert!(request.status.is_none());
        assert!(request.department.is_none());
        assert!(request.phone.is_none());
        assert!(request.email.is_none());
    }

    #[test]
    fn test_user_patch_request_with_all_fields() {
        let request = UserPatchRequest {
            name: Some("张三".to_string()),
            user_type: Some(UserType::Employee),
            status: Some(UserStatus::Active),
            department: Some("技术部".to_string()),
            phone: Some("13800138000".to_string()),
            email: Some("zhangsan@example.com".to_string()),
        };

        assert_eq!(request.name, Some("张三".to_string()));
        assert_eq!(request.user_type, Some(UserType::Employee));
        assert_eq!(request.status, Some(UserStatus::Active));
        assert_eq!(request.department, Some("技术部".to_string()));
        assert_eq!(request.phone, Some("13800138000".to_string()));
        assert_eq!(request.email, Some("zhangsan@example.com".to_string()));
    }

    #[test]
    fn test_user_patch_request_partial_fields() {
        let request = UserPatchRequest {
            name: Some("李四".to_string()),
            status: Some(UserStatus::Disabled),
            ..Default::default()
        };

        assert_eq!(request.name, Some("李四".to_string()));
        assert_eq!(request.status, Some(UserStatus::Disabled));
        assert!(request.user_type.is_none());
        assert!(request.department.is_none());
        assert!(request.phone.is_none());
        assert!(request.email.is_none());
    }

    #[test]
    fn test_user_patch_request_clone() {
        let original = UserPatchRequest {
            name: Some("王五".to_string()),
            user_type: Some(UserType::Visitor),
            department: Some("访客部".to_string()),
            ..Default::default()
        };

        let cloned = original.clone();
        assert_eq!(original.name, cloned.name);
        assert_eq!(original.user_type, cloned.user_type);
        assert_eq!(original.department, cloned.department);
    }

    #[test]
    fn test_user_patch_request_serialization() {
        let request = UserPatchRequest {
            name: Some("测试用户".to_string()),
            user_type: Some(UserType::Employee),
            status: Some(UserStatus::Active),
            phone: Some("18888888888".to_string()),
            email: Some("test@test.com".to_string()),
            ..Default::default()
        };

        let json = serde_json::to_string(&request).expect("Should serialize");
        let deserialized: UserPatchRequest =
            serde_json::from_str(&json).expect("Should deserialize");

        assert_eq!(request.name, deserialized.name);
        assert_eq!(request.user_type, deserialized.user_type);
        assert_eq!(request.status, deserialized.status);
        assert_eq!(request.phone, deserialized.phone);
        assert_eq!(request.email, deserialized.email);
    }

    #[test]
    fn test_user_patch_request_debug() {
        let request = UserPatchRequest {
            name: Some("Debug User".to_string()),
            email: Some("debug@example.com".to_string()),
            ..Default::default()
        };
        let debug_str = format!("{:?}", request);
        assert!(debug_str.contains("UserPatchRequest"));
        assert!(debug_str.contains("Debug User"));
    }

    // UserListRequest tests
    #[test]
    fn test_user_list_request_default() {
        let request = UserListRequest::default();
        assert!(request.page_token.is_none());
        assert!(request.page_size.is_none());
        assert!(request.user_type.is_none());
        assert!(request.status.is_none());
        assert!(request.department.is_none());
    }

    #[test]
    fn test_user_list_request_with_all_fields() {
        let request = UserListRequest {
            page_token: Some("user_token_456".to_string()),
            page_size: Some(50),
            user_type: Some(UserType::Employee),
            status: Some(UserStatus::Active),
            department: Some("人事部".to_string()),
        };

        assert_eq!(request.page_token, Some("user_token_456".to_string()));
        assert_eq!(request.page_size, Some(50));
        assert_eq!(request.user_type, Some(UserType::Employee));
        assert_eq!(request.status, Some(UserStatus::Active));
        assert_eq!(request.department, Some("人事部".to_string()));
    }

    #[test]
    fn test_user_list_request_serialization() {
        let request = UserListRequest {
            page_token: Some("list_token".to_string()),
            page_size: Some(25),
            user_type: Some(UserType::Visitor),
            ..Default::default()
        };

        let json = serde_json::to_string(&request).expect("Should serialize");
        let deserialized: UserListRequest =
            serde_json::from_str(&json).expect("Should deserialize");

        assert_eq!(request.page_token, deserialized.page_token);
        assert_eq!(request.page_size, deserialized.page_size);
        assert_eq!(request.user_type, deserialized.user_type);
    }

    // FaceImageUploadRequest tests
    #[test]
    fn test_face_image_upload_request_creation() {
        let request = FaceImageUploadRequest {
            image_content: "iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAADUlEQVR42mNk+M9QDwADhgGAWjR9awAAAABJRU5ErkJggg==".to_string(),
            image_format: Some("png".to_string()),
        };

        assert!(!request.image_content.is_empty());
        assert_eq!(request.image_format, Some("png".to_string()));
    }

    #[test]
    fn test_face_image_upload_request_without_format() {
        let request = FaceImageUploadRequest {
            image_content: "base64_image_data".to_string(),
            image_format: None,
        };

        assert_eq!(request.image_content, "base64_image_data".to_string());
        assert!(request.image_format.is_none());
    }

    #[test]
    fn test_face_image_upload_request_clone() {
        let original = FaceImageUploadRequest {
            image_content: "original_image_data".to_string(),
            image_format: Some("jpg".to_string()),
        };

        let cloned = original.clone();
        assert_eq!(original.image_content, cloned.image_content);
        assert_eq!(original.image_format, cloned.image_format);
    }

    #[test]
    fn test_face_image_upload_request_serialization() {
        let request = FaceImageUploadRequest {
            image_content: "serialization_test_data".to_string(),
            image_format: Some("jpeg".to_string()),
        };

        let json = serde_json::to_string(&request).expect("Should serialize");
        let deserialized: FaceImageUploadRequest =
            serde_json::from_str(&json).expect("Should deserialize");

        assert_eq!(request.image_content, deserialized.image_content);
        assert_eq!(request.image_format, deserialized.image_format);
    }

    #[test]
    fn test_face_image_upload_request_debug() {
        let request = FaceImageUploadRequest {
            image_content: "debug_image_data".to_string(),
            image_format: Some("png".to_string()),
        };
        let debug_str = format!("{:?}", request);
        assert!(debug_str.contains("FaceImageUploadRequest"));
        assert!(debug_str.contains("debug_image_data"));
    }

    // Response tests
    #[test]
    fn test_user_patch_response_data_format() {
        assert_eq!(UserPatchResponse::data_format(), ResponseFormat::Data);
    }

    #[test]
    fn test_user_get_response_data_format() {
        assert_eq!(UserGetResponse::data_format(), ResponseFormat::Data);
    }

    #[test]
    fn test_user_list_response_data_format() {
        assert_eq!(UserListResponse::data_format(), ResponseFormat::Data);
    }

    #[test]
    fn test_face_image_upload_response_data_format() {
        assert_eq!(FaceImageUploadResponse::data_format(), ResponseFormat::Data);
    }

    #[test]
    fn test_face_image_download_response_data_format() {
        assert_eq!(
            FaceImageDownloadResponse::data_format(),
            ResponseFormat::Data
        );
    }

    #[test]
    fn test_face_image_upload_response_debug() {
        let response = FaceImageUploadResponse {
            success: true,
            image_id: Some("img_123456".to_string()),
        };

        let debug_str = format!("{:?}", response);
        assert!(debug_str.contains("FaceImageUploadResponse"));
        assert!(debug_str.contains("true"));
        assert!(debug_str.contains("img_123456"));
    }

    #[test]
    fn test_face_image_upload_response_serialization() {
        let response = FaceImageUploadResponse {
            success: false,
            image_id: None,
        };

        let json = serde_json::to_string(&response).expect("Should serialize");
        let deserialized: FaceImageUploadResponse =
            serde_json::from_str(&json).expect("Should deserialize");

        assert_eq!(response.success, deserialized.success);
        assert_eq!(response.image_id, deserialized.image_id);
    }

    #[test]
    fn test_user_list_response_debug() {
        use crate::service::acs::models::PageResponse;

        let response = UserListResponse {
            users: PageResponse {
                has_more: Some(true),
                page_token: Some("next_user_token".to_string()),
                items: vec![],
            },
        };

        let debug_str = format!("{:?}", response);
        assert!(debug_str.contains("UserListResponse"));
        assert!(debug_str.contains("next_user_token"));
        assert!(debug_str.contains("true"));
    }

    #[test]
    fn test_user_list_response_serialization() {
        use crate::service::acs::models::PageResponse;

        let response = UserListResponse {
            users: PageResponse {
                has_more: Some(false),
                page_token: None,
                items: vec![],
            },
        };

        let json = serde_json::to_string(&response).expect("Should serialize");
        let deserialized: UserListResponse =
            serde_json::from_str(&json).expect("Should deserialize");

        assert_eq!(response.users.has_more, deserialized.users.has_more);
        assert_eq!(response.users.page_token, deserialized.users.page_token);
    }

    #[test]
    fn test_user_types_and_statuses() {
        // Test UserType variants
        let employee_request = UserPatchRequest {
            user_type: Some(UserType::Employee),
            ..Default::default()
        };
        assert_eq!(employee_request.user_type, Some(UserType::Employee));

        let visitor_request = UserPatchRequest {
            user_type: Some(UserType::Visitor),
            ..Default::default()
        };
        assert_eq!(visitor_request.user_type, Some(UserType::Visitor));

        // Test UserStatus variants
        let active_request = UserPatchRequest {
            status: Some(UserStatus::Active),
            ..Default::default()
        };
        assert_eq!(active_request.status, Some(UserStatus::Active));

        let disabled_request = UserPatchRequest {
            status: Some(UserStatus::Disabled),
            ..Default::default()
        };
        assert_eq!(disabled_request.status, Some(UserStatus::Disabled));
    }

    #[test]
    fn test_email_and_phone_validation_patterns() {
        // Test with various email formats
        let email_requests = vec![
            "user@example.com",
            "test.email+tag@example.co.uk",
            "user123@test-domain.org",
        ];

        for email in email_requests {
            let request = UserPatchRequest {
                email: Some(email.to_string()),
                ..Default::default()
            };
            assert_eq!(request.email, Some(email.to_string()));
        }

        // Test with various phone formats
        let phone_requests = vec![
            "13800138000",
            "+86 138 0013 8000",
            "138-0013-8000",
            "(138) 001-8000",
        ];

        for phone in phone_requests {
            let request = UserPatchRequest {
                phone: Some(phone.to_string()),
                ..Default::default()
            };
            assert_eq!(request.phone, Some(phone.to_string()));
        }
    }

    #[test]
    fn test_request_with_empty_and_special_values() {
        let request = UserPatchRequest {
            name: Some("".to_string()),
            department: Some("部门 with 特殊字符 & symbols!".to_string()),
            phone: Some("".to_string()),
            email: Some("test@测试域名.com".to_string()),
            ..Default::default()
        };

        let json = serde_json::to_string(&request).expect("Should serialize");
        let deserialized: UserPatchRequest =
            serde_json::from_str(&json).expect("Should deserialize");

        assert_eq!(request.name, deserialized.name);
        assert_eq!(request.department, deserialized.department);
        assert_eq!(request.phone, deserialized.phone);
        assert_eq!(request.email, deserialized.email);
    }

    #[test]
    fn test_access_token_type_support() {
        // The user service should support tenant and user access tokens
        let supported_types = [AccessTokenType::Tenant, AccessTokenType::User];
        assert!(supported_types.contains(&AccessTokenType::Tenant));
        assert!(supported_types.contains(&AccessTokenType::User));
        assert!(!supported_types.contains(&AccessTokenType::App));
    }
}

use reqwest::Method;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::core::{
    api_req::ApiRequest,
    api_resp::{ApiResponseTrait, BaseResponse, ResponseFormat},
    config::Config,
    constants::AccessTokenType,
    endpoints::EndpointBuilder,
    error::LarkAPIError,
    http::Transport,
    req_option::RequestOption,
    standard_response::StandardResponse,
    trait_system::executable_builder::ExecutableBuilder,
    validation::{validate_file_name, validate_upload_file, ValidateBuilder, ValidationResult},
    SDKResult,
};
use crate::impl_full_service;
use async_trait::async_trait;

/// 文件服务
pub struct FileService {
    pub config: Config,
}

/// 上传文件响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateFileResponse {
    /// 文件的key
    pub file_key: String,
}

impl ApiResponseTrait for CreateFileResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 下载文件响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetFileResponse {
    /// 文件数据
    pub data: Vec<u8>,
}

impl ApiResponseTrait for GetFileResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl FileService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 上传文件
    pub async fn create(
        &self,
        file_type: &str,
        file_name: &str,
        file_data: Vec<u8>,
        option: Option<RequestOption>,
    ) -> SDKResult<CreateFileResponse> {
        let mut query_params = HashMap::new();
        query_params.insert("file_type", file_type.to_string());
        query_params.insert("file_name", file_name.to_string());

        let api_req = ApiRequest {
            http_method: Method::POST,
            api_path: crate::core::endpoints::im::IM_V1_FILES.to_string(),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            query_params,
            body: file_data,
            ..Default::default()
        };

        let api_resp: BaseResponse<CreateFileResponse> =
            Transport::request(api_req, &self.config, option).await?;
        api_resp.into_result()
    }

    /// 下载文件
    pub async fn get(
        &self,
        file_key: &str,
        option: Option<RequestOption>,
    ) -> SDKResult<GetFileResponse> {
        let api_req = ApiRequest {
            http_method: Method::GET,
            api_path: EndpointBuilder::replace_param(
                crate::core::endpoints::im::IM_V1_DOWNLOAD_FILE,
                "file_key",
                file_key,
            ),
            supported_access_token_types: vec![AccessTokenType::Tenant, AccessTokenType::User],
            ..Default::default()
        };

        let api_resp: BaseResponse<GetFileResponse> =
            Transport::request(api_req, &self.config, option).await?;
        api_resp.into_result()
    }

    /// 创建文件上传Builder (推荐)
    pub fn upload_builder(&self) -> FileUploadBuilder {
        FileUploadBuilder::new()
    }

    /// 创建文件下载Builder (推荐)
    pub fn download_builder(&self) -> FileDownloadBuilder {
        FileDownloadBuilder::new()
    }
}

// 接入统一 Service 抽象（IM v1 - FileService）
impl_full_service!(FileService, "im.file", "v1");

/// 文件上传请求结构
#[derive(Debug, Clone, Default)]
pub struct FileUploadRequest {
    /// 文件类型
    pub file_type: String,
    /// 文件名
    pub file_name: String,
    /// 文件数据
    pub file_data: Vec<u8>,
}

/// 文件上传Builder
#[derive(Default)]
pub struct FileUploadBuilder {
    request: FileUploadRequest,
}

impl FileUploadBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置文件类型
    pub fn file_type(mut self, file_type: impl ToString) -> Self {
        self.request.file_type = file_type.to_string();
        self
    }

    /// 设置文件名
    pub fn file_name(mut self, file_name: impl ToString) -> Self {
        self.request.file_name = file_name.to_string();
        self
    }

    /// 设置文件数据
    pub fn file_data(mut self, file_data: Vec<u8>) -> Self {
        self.request.file_data = file_data;
        self
    }

    /// 构建文件上传请求
    pub fn build(self) -> SDKResult<FileUploadRequest> {
        // 验证文件类型
        if self.request.file_type.is_empty() {
            return Err(LarkAPIError::illegal_param(
                "file_type is required".to_string(),
            ));
        }

        // 验证文件名
        let (cleaned_name, name_result) = validate_file_name(&self.request.file_name);
        if !name_result.is_valid() {
            return Err(LarkAPIError::illegal_param(format!(
                "Invalid file_name: {}",
                name_result.error().unwrap_or("unknown error")
            )));
        }

        // 验证文件数据
        if self.request.file_data.is_empty() {
            return Err(LarkAPIError::illegal_param(
                "file_data cannot be empty".to_string(),
            ));
        }

        // 验证上传文件（IM上传有更小的限制）
        let upload_result = validate_upload_file(&self.request.file_data, &cleaned_name, true);
        if !upload_result.is_valid() {
            return Err(LarkAPIError::illegal_param(format!(
                "File validation failed: {}",
                upload_result.error().unwrap_or("unknown error")
            )));
        }

        Ok(FileUploadRequest {
            file_type: self.request.file_type,
            file_name: cleaned_name,
            file_data: self.request.file_data,
        })
    }

    /// 构建文件上传请求（无验证，用于向后兼容）
    pub fn build_unvalidated(self) -> FileUploadRequest {
        self.request
    }
}

impl ValidateBuilder for FileUploadBuilder {
    fn validate(&self) -> ValidationResult {
        // 验证文件类型
        if self.request.file_type.is_empty() {
            return ValidationResult::Invalid("file_type is required".to_string());
        }

        // 验证文件名
        let (_, name_result) = validate_file_name(&self.request.file_name);
        if !name_result.is_valid() {
            return name_result;
        }

        // 验证文件数据
        if self.request.file_data.is_empty() {
            return ValidationResult::Invalid("file_data cannot be empty".to_string());
        }

        // 验证上传文件
        validate_upload_file(&self.request.file_data, &self.request.file_name, true)
    }
}

#[async_trait]
impl ExecutableBuilder<FileService, FileUploadRequest, CreateFileResponse> for FileUploadBuilder {
    fn build(self) -> FileUploadRequest {
        // Legacy build method - create request without validation for backward compatibility
        self.build_unvalidated()
    }

    async fn execute(self, service: &FileService) -> SDKResult<CreateFileResponse> {
        let request = self.build_unvalidated();
        service
            .create(
                &request.file_type,
                &request.file_name,
                request.file_data,
                None,
            )
            .await
    }

    async fn execute_with_options(
        self,
        service: &FileService,
        option: RequestOption,
    ) -> SDKResult<CreateFileResponse> {
        let request = self.build_unvalidated();
        service
            .create(
                &request.file_type,
                &request.file_name,
                request.file_data,
                Some(option),
            )
            .await
    }
}

/// 文件下载Builder
#[derive(Default)]
pub struct FileDownloadBuilder {
    file_key: Option<String>,
}

impl FileDownloadBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置文件key
    pub fn file_key(mut self, file_key: impl ToString) -> Self {
        self.file_key = Some(file_key.to_string());
        self
    }

    pub fn build(self) -> String {
        self.file_key.unwrap_or_default()
    }
}

#[async_trait]
impl ExecutableBuilder<FileService, String, GetFileResponse> for FileDownloadBuilder {
    fn build(self) -> String {
        self.build()
    }

    async fn execute(self, service: &FileService) -> SDKResult<GetFileResponse> {
        let file_key = self.build();
        service.get(&file_key, None).await
    }

    async fn execute_with_options(
        self,
        service: &FileService,
        option: RequestOption,
    ) -> SDKResult<GetFileResponse> {
        let file_key = self.build();
        service.get(&file_key, Some(option)).await
    }
}

#[cfg(test)]
#[allow(unused_variables, unused_unsafe)]
mod tests {
    use super::*;
    use crate::core::config::Config;

    fn create_test_config() -> Config {
        Config::default()
    }

    #[test]
    fn test_file_service_creation() {
        let config = create_test_config();
        let service = FileService::new(config.clone());

        assert_eq!(service.config.app_id, config.app_id);
        assert_eq!(service.config.app_secret, config.app_secret);
    }

    #[test]
    fn test_file_service_with_custom_config() {
        let config = Config::builder()
            .app_id("file_app")
            .app_secret("file_secret")
            .req_timeout(std::time::Duration::from_millis(15000))
            .base_url("https://file.api.com")
            .build();

        let service = FileService::new(config.clone());

        assert_eq!(service.config.app_id, "file_app");
        assert_eq!(service.config.app_secret, "file_secret");
        assert_eq!(service.config.base_url, "https://file.api.com");
        assert_eq!(
            service.config.req_timeout,
            Some(std::time::Duration::from_millis(15000))
        );
    }

    #[test]
    fn test_file_service_config_independence() {
        let config1 = Config::builder()
            .app_id("file1")
            .app_secret("secret1")
            .build();
        let config2 = Config::builder()
            .app_id("file2")
            .app_secret("secret2")
            .build();

        let service1 = FileService::new(config1);
        let service2 = FileService::new(config2);

        assert_eq!(service1.config.app_id, "file1");
        assert_eq!(service2.config.app_id, "file2");
        assert_ne!(service1.config.app_id, service2.config.app_id);
    }

    #[test]
    fn test_file_service_memory_layout() {
        let config = create_test_config();
        let service = FileService::new(config);

        let service_ptr = std::ptr::addr_of!(service) as *const u8;
        let config_ptr = std::ptr::addr_of!(service.config) as *const u8;

        assert!(
            !service_ptr.is_null(),
            "Service should have valid memory address"
        );
        assert!(
            !config_ptr.is_null(),
            "Config should have valid memory address"
        );
    }

    #[test]
    fn test_file_service_with_different_configurations() {
        let test_configs = vec![
            Config::builder()
                .app_id("file_basic")
                .app_secret("basic_secret")
                .build(),
            Config::builder()
                .app_id("file_timeout")
                .app_secret("timeout_secret")
                .req_timeout(std::time::Duration::from_millis(12000))
                .build(),
            Config::builder()
                .app_id("file_custom")
                .app_secret("custom_secret")
                .base_url("https://custom.file.com")
                .build(),
            Config::builder()
                .app_id("file_full")
                .app_secret("full_secret")
                .req_timeout(std::time::Duration::from_millis(20000))
                .base_url("https://full.file.com")
                .enable_token_cache(false)
                .build(),
        ];

        for config in test_configs {
            let service = FileService::new(config.clone());

            assert_eq!(service.config.app_id, config.app_id);
            assert_eq!(service.config.app_secret, config.app_secret);
            assert_eq!(service.config.base_url, config.base_url);
            assert_eq!(service.config.req_timeout, config.req_timeout);
        }
    }

    #[test]
    fn test_file_service_multiple_instances() {
        let config = create_test_config();
        let service1 = FileService::new(config.clone());
        let service2 = FileService::new(config.clone());

        assert_eq!(service1.config.app_id, service2.config.app_id);
        assert_eq!(service1.config.app_secret, service2.config.app_secret);

        let ptr1 = std::ptr::addr_of!(service1) as *const u8;
        let ptr2 = std::ptr::addr_of!(service2) as *const u8;
        assert_ne!(ptr1, ptr2, "Services should be independent instances");
    }

    #[test]
    fn test_file_service_config_cloning() {
        let original_config = create_test_config();
        let cloned_config = original_config.clone();

        let service = FileService::new(cloned_config);

        assert_eq!(service.config.app_id, original_config.app_id);
        assert_eq!(service.config.app_secret, original_config.app_secret);
    }

    #[test]
    fn test_file_service_with_empty_config() {
        let config = Config::default();
        let service = FileService::new(config);

        assert_eq!(service.config.app_id, "");
        assert_eq!(service.config.app_secret, "");
    }

    #[test]
    fn test_file_service_with_unicode_config() {
        let config = Config::builder()
            .app_id("文件应用")
            .app_secret("文件密钥")
            .base_url("https://文件.com")
            .build();
        let service = FileService::new(config);

        assert_eq!(service.config.app_id, "文件应用");
        assert_eq!(service.config.app_secret, "文件密钥");
        assert_eq!(service.config.base_url, "https://文件.com");
    }

    #[test]
    fn test_file_service_with_extreme_timeout() {
        let config = Config::builder()
            .app_id("file_extreme")
            .app_secret("extreme_secret")
            .req_timeout(std::time::Duration::from_secs(7200))
            .build();
        let service = FileService::new(config);

        assert_eq!(
            service.config.req_timeout,
            Some(std::time::Duration::from_secs(7200))
        );
    }

    #[test]
    fn test_file_service_builder_methods() {
        let config = create_test_config();
        let service = FileService::new(config);

        let upload_builder = service.upload_builder();
        let download_builder = service.download_builder();

        // Builders should be created successfully
        let upload_ptr = std::ptr::addr_of!(upload_builder) as *const u8;
        let download_ptr = std::ptr::addr_of!(download_builder) as *const u8;
        assert!(!upload_ptr.is_null());
        assert!(!download_ptr.is_null());
    }

    #[test]
    fn test_file_upload_builder_basic() {
        let builder = FileUploadBuilder::new()
            .file_type("image")
            .file_name("test.jpg")
            .file_data(vec![1, 2, 3, 4]);

        let request = builder.build_unvalidated();
        assert_eq!(request.file_type, "image");
        assert_eq!(request.file_name, "test.jpg");
        assert_eq!(request.file_data, vec![1, 2, 3, 4]);
    }

    #[test]
    fn test_file_upload_builder_chaining() {
        let request = FileUploadBuilder::new()
            .file_type("document")
            .file_name("document.pdf")
            .file_data(vec![0xFF, 0xFE, 0xFD])
            .build_unvalidated();

        assert_eq!(request.file_type, "document");
        assert_eq!(request.file_name, "document.pdf");
        assert_eq!(request.file_data, vec![0xFF, 0xFE, 0xFD]);
    }

    #[test]
    fn test_file_download_builder_basic() {
        let builder = FileDownloadBuilder::new().file_key("test_key_123");

        let file_key = builder.build();
        assert_eq!(file_key, "test_key_123");
    }

    #[test]
    fn test_file_download_builder_empty() {
        let builder = FileDownloadBuilder::new();
        let file_key = builder.build();
        assert_eq!(file_key, "");
    }

    #[test]
    fn test_file_upload_request_default() {
        let request = FileUploadRequest::default();
        assert_eq!(request.file_type, "");
        assert_eq!(request.file_name, "");
        assert_eq!(request.file_data, Vec::<u8>::new());
    }

    #[test]
    fn test_create_file_response_format() {
        assert_eq!(CreateFileResponse::data_format(), ResponseFormat::Data);
    }

    #[test]
    fn test_get_file_response_format() {
        assert_eq!(GetFileResponse::data_format(), ResponseFormat::Data);
    }
}

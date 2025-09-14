use reqwest::Method;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::core::{
    api_req::ApiRequest,
    api_resp::{ApiResponseTrait, BaseResponse, ResponseFormat},
    config::Config,
    constants::AccessTokenType,
    endpoints::{EndpointBuilder, Endpoints},
    error::LarkAPIError,
    http::Transport,
    req_option::RequestOption,
    standard_response::StandardResponse,
    trait_system::executable_builder::ExecutableBuilder,
    validation::{validate_file_name, validate_upload_file, ValidateBuilder, ValidationResult},
    SDKResult,
};
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
            api_path: Endpoints::IM_V1_FILES.to_string(),
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
            api_path: EndpointBuilder::replace_param(Endpoints::IM_V1_DOWNLOAD_FILE, "file_key", file_key),
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

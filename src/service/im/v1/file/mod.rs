use reqwest::Method;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::core::{
    api_req::ApiRequest,
    api_resp::{ApiResponseTrait, BaseResponse, ResponseFormat},
    config::Config,
    constants::AccessTokenType,
    http::Transport,
    req_option::RequestOption,
    standard_response::StandardResponse,
    trait_system::executable_builder::ExecutableBuilder,
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
        query_params.insert("file_type".to_string(), file_type.to_string());
        query_params.insert("file_name".to_string(), file_name.to_string());

        let api_req = ApiRequest {
            http_method: Method::POST,
            api_path: "/open-apis/im/v1/files".to_string(),
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
            api_path: format!("/open-apis/im/v1/files/{}", file_key),
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

/// 文件上传Builder
#[derive(Default)]
pub struct FileUploadBuilder {
    file_type: Option<String>,
    file_name: Option<String>,
    file_data: Option<Vec<u8>>,
}

impl FileUploadBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    /// 设置文件类型
    pub fn file_type(mut self, file_type: impl ToString) -> Self {
        self.file_type = Some(file_type.to_string());
        self
    }

    /// 设置文件名
    pub fn file_name(mut self, file_name: impl ToString) -> Self {
        self.file_name = Some(file_name.to_string());
        self
    }

    /// 设置文件数据
    pub fn file_data(mut self, file_data: Vec<u8>) -> Self {
        self.file_data = Some(file_data);
        self
    }

    pub fn build(self) -> (String, String, Vec<u8>) {
        (
            self.file_type.unwrap_or_default(),
            self.file_name.unwrap_or_default(),
            self.file_data.unwrap_or_default(),
        )
    }
}

#[async_trait]
impl ExecutableBuilder<FileService, (String, String, Vec<u8>), CreateFileResponse>
    for FileUploadBuilder
{
    fn build(self) -> (String, String, Vec<u8>) {
        self.build()
    }

    async fn execute(self, service: &FileService) -> SDKResult<CreateFileResponse> {
        let (file_type, file_name, file_data) = self.build();
        service
            .create(&file_type, &file_name, file_data, None)
            .await
    }

    async fn execute_with_options(
        self,
        service: &FileService,
        option: RequestOption,
    ) -> SDKResult<CreateFileResponse> {
        let (file_type, file_name, file_data) = self.build();
        service
            .create(&file_type, &file_name, file_data, Some(option))
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

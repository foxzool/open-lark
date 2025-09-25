use log;
use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::{
    core::{
        api_req::ApiRequest,
        api_resp::{ApiResponseTrait, BaseResponse, BinaryResponse, ResponseFormat},
        config::Config,
        constants::AccessTokenType,
        endpoints::cloud_docs::*,
        http::Transport,
        req_option::RequestOption,
        standard_response::StandardResponse,
        validation::{validate_file_name, validate_upload_file, ValidateBuilder, ValidationResult},
        SDKResult,
    },
    impl_executable_builder_owned,
};

pub struct FilesService {
    config: Config,
}

/// 上传文件 请求体
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct UploadAllRequest {
    /// 请求体
    #[serde(skip)]
    api_req: ApiRequest,
    /// 文件名。
    ///
    /// 示例值："demo.pdf"
    file_name: String,
    /// 上传点类型。
    ///
    /// 示例值："explorer"
    parent_type: String,
    /// 文件夹token，获取方式见 概述
    ///
    /// 示例值："fldbcO1UuPz8VwnpPx5a92abcef"
    parent_node: String,
    /// 文件大小（以字节为单位）。
    ///
    /// 示例值：1024
    size: i32,
    /// 文件adler32校验和(可选)。
    checksum: Option<String>,
}

impl UploadAllRequest {
    pub fn builder() -> UploadAllRequestBuilder {
        UploadAllRequestBuilder::default()
    }
}

/// 上传文件 请求体构建器
#[derive(Default)]
pub struct UploadAllRequestBuilder {
    request: UploadAllRequest,
}

impl UploadAllRequestBuilder {
    /// 文件名
    pub fn file_name(mut self, file_name: impl ToString) -> Self {
        self.request.file_name = file_name.to_string();
        self
    }

    /// 上传点类型。
    pub fn parent_type(mut self, parent_type: impl ToString) -> Self {
        self.request.parent_type = parent_type.to_string();
        self
    }

    /// 文件夹token
    pub fn parent_node(mut self, parent_node: impl ToString) -> Self {
        self.request.parent_node = parent_node.to_string();
        self
    }

    /// 文件大小（以字节为单位）
    pub fn size(mut self, size: i32) -> Self {
        self.request.size = size;
        self
    }

    /// 文件adler32校验和(可选)
    pub fn checksum(mut self, checksum: impl ToString) -> Self {
        self.request.checksum = Some(checksum.to_string());
        self
    }

    /// 文件二进制内容。
    pub fn file(mut self, file: Vec<u8>) -> Self {
        self.request.api_req.file = file;
        self
    }

    pub fn build(mut self) -> UploadAllRequest {
        // 验证必填字段
        if self.request.file_name.is_empty() {
            log::error!("file_name is required for upload");
            return UploadAllRequest {
                api_req: ApiRequest {
                    body: Vec::new(),
                    ..Default::default()
                },
                ..self.request
            };
        }

        if self.request.parent_type.is_empty() {
            log::error!("parent_type is required for upload");
            return UploadAllRequest {
                api_req: ApiRequest {
                    body: Vec::new(),
                    ..Default::default()
                },
                ..self.request
            };
        }

        if self.request.parent_node.is_empty() {
            log::error!("parent_node is required for upload");
            return UploadAllRequest {
                api_req: ApiRequest {
                    body: Vec::new(),
                    ..Default::default()
                },
                ..self.request
            };
        }

        // 验证文件大小
        if self.request.size <= 0 {
            log::error!("file size must be greater than 0");
            return UploadAllRequest {
                api_req: ApiRequest {
                    body: Vec::new(),
                    ..Default::default()
                },
                ..self.request
            };
        }

        // 验证文件名
        let (_, name_result) = validate_file_name(&self.request.file_name);
        if !name_result.is_valid() {
            log::error!(
                "Invalid file_name: {}",
                name_result.error().unwrap_or("unknown error")
            );
            return UploadAllRequest {
                api_req: ApiRequest {
                    body: Vec::new(),
                    ..Default::default()
                },
                ..self.request
            };
        }

        // 验证文件数据（如果有）
        if !self.request.api_req.file.is_empty() {
            let upload_result =
                validate_upload_file(&self.request.api_req.file, &self.request.file_name, false);
            if !upload_result.is_valid() {
                log::error!(
                    "File validation failed: {}",
                    upload_result.error().unwrap_or("unknown error")
                );
                return UploadAllRequest {
                    api_req: ApiRequest {
                        body: Vec::new(),
                        ..Default::default()
                    },
                    ..self.request
                };
            }
        }

        self.request.api_req.body = match serde_json::to_vec(&self.request) {
            Ok(body) => body,
            Err(e) => {
                log::error!("Failed to serialize upload request: {}", e);
                return UploadAllRequest {
                    api_req: ApiRequest {
                        body: Vec::new(),
                        ..Default::default()
                    },
                    ..self.request
                };
            }
        };
        self.request
    }
}

impl ValidateBuilder for UploadAllRequestBuilder {
    fn validate(&self) -> ValidationResult {
        // 验证必填字段
        if self.request.file_name.is_empty() {
            return ValidationResult::Invalid("file_name is required".to_string());
        }

        if self.request.parent_type.is_empty() {
            return ValidationResult::Invalid("parent_type is required".to_string());
        }

        if self.request.parent_node.is_empty() {
            return ValidationResult::Invalid("parent_node is required".to_string());
        }

        // 验证文件大小
        if self.request.size <= 0 {
            return ValidationResult::Invalid("file size must be greater than 0".to_string());
        }

        // 验证文件名
        let (_, name_result) = validate_file_name(&self.request.file_name);
        if !name_result.is_valid() {
            return name_result;
        }

        // 验证文件数据（如果有）
        if !self.request.api_req.file.is_empty() {
            validate_upload_file(&self.request.api_req.file, &self.request.file_name, false)
        } else {
            ValidationResult::Valid
        }
    }
}

impl FilesService {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 创建上传文件Builder
    pub fn upload_all_builder(&self) -> UploadAllRequestBuilder {
        UploadAllRequestBuilder::default()
    }

    /// 创建下载文件Builder  
    pub fn download_builder(&self) -> DownloadRequestBuilder {
        DownloadRequestBuilder::default()
    }

    /// 使用Builder上传文件（带验证）
    pub async fn upload_all_with_builder(
        &self,
        builder_result: SDKResult<UploadAllRequest>,
        option: Option<RequestOption>,
    ) -> SDKResult<UploadAllResponse> {
        let request = builder_result?;
        self.upload_all(request, option).await
    }

    /// 上传文件
    pub async fn upload_all(
        &self,
        upload_all_request: UploadAllRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<UploadAllResponse> {
        let mut api_req = upload_all_request.api_req;
        api_req.http_method = Method::POST;
        api_req.api_path = DRIVE_V1_FILES_UPLOAD_ALL.to_string();
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];

        let api_resp: BaseResponse<UploadAllResponse> =
            Transport::request(api_req, &self.config, option).await?;

        api_resp.into_result()
    }

    /// 下载文件
    pub async fn download(
        &self,
        request: DownloadRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BinaryResponse> {
        let mut api_req = request.api_req;
        api_req.http_method = Method::GET;
        api_req.api_path = DRIVE_V1_FILE_DOWNLOAD.replace("{}", &request.file_token);
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::User];

        let api_resp: BaseResponse<BinaryResponse> =
            Transport::request(api_req, &self.config, option).await?;

        api_resp.into_result()
    }
}

/// 上传文件响应体
#[derive(Debug, Serialize, Deserialize)]
pub struct UploadAllResponse {
    /// 新创建文件的 token
    pub file_token: String,
}

impl ApiResponseTrait for UploadAllResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 下载文件 请求体
#[derive(Default, Serialize, Deserialize)]
pub struct DownloadRequest {
    /// 请求体
    #[serde(skip)]
    api_req: ApiRequest,
    /// 文件的 token
    file_token: String,
}

/// 下载文件 请求体构建器
#[derive(Default)]
pub struct DownloadRequestBuilder {
    req: DownloadRequest,
}

impl DownloadRequestBuilder {
    pub fn file_token(mut self, file_token: impl ToString) -> Self {
        self.req.file_token = file_token.to_string();
        self
    }

    pub fn build(self) -> DownloadRequest {
        self.req
    }
}

impl DownloadRequest {
    pub fn builder() -> DownloadRequestBuilder {
        DownloadRequestBuilder::default()
    }
}

// === 宏实现 ===

impl_executable_builder_owned!(
    UploadAllRequestBuilder,
    FilesService,
    UploadAllRequest,
    UploadAllResponse,
    upload_all
);

impl_executable_builder_owned!(
    DownloadRequestBuilder,
    FilesService,
    DownloadRequest,
    BinaryResponse,
    download
);

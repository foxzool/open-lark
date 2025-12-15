/// 上传图片
///
/// API文档: https://open.feishu.cn/document/lingo-v1/file/upload

use serde::{Deserialize, Serialize};
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, Response, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
use crate::baike::models::*;

/// 上传图片响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UploadFileResponse {
    /// 文件token
    pub file_token: String,
    /// 文件URL
    pub url: String,
    /// 文件名
    pub file_name: String,
    /// 文件大小
    pub file_size: i64,
    /// 文件类型
    pub content_type: String,
}

impl ApiResponseTrait for UploadFileResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 上传图片构建器
pub struct UploadFileBuilder<'a> {
    config: &'a Config,
    request: FileUploadRequest,
    option: Option<openlark_core::req_option::RequestOption>,
}

impl<'a> UploadFileBuilder<'a> {
    /// 创建新的上传图片构建器
    pub fn new(config: &'a Config, file_path: String) -> Self {
        Self {
            config,
            request: FileUploadRequest {
                file_name: String::new(),
                file_content: String::new(),
                file_size: 0,
                content_type: String::new(),
            },
            option: None,
        }
    }

    /// 设置文件名
    pub fn file_name(mut self, file_name: String) -> Self {
        self.request.file_name = file_name;
        self
    }

    /// 设置文件内容（base64编码）
    pub fn file_content(mut self, file_content: String) -> Self {
        self.request.file_content = file_content;
        self
    }

    /// 设置文件大小
    pub fn file_size(mut self, file_size: i64) -> Self {
        self.request.file_size = file_size;
        self
    }

    /// 设置文件类型
    pub fn content_type(mut self, content_type: String) -> Self {
        self.request.content_type = content_type;
        self
    }

    /// 设置请求选项
    pub fn request_option(mut self, option: openlark_core::req_option::RequestOption) -> Self {
        self.option = Some(option);
        self
    }

    /// 执行上传图片请求
    pub async fn execute(self) -> SDKResult<Response<UploadFileResponse>> {
        // 构建API端点
        let url = "/open-apis/lingo/v1/files/upload";

        // 创建API请求
        let mut api_request: ApiRequest<UploadFileResponse> =
            ApiRequest::post(url)
                .body(serde_json::to_value(self.request)?);

        // 如果有请求选项，应用它们
        if let Some(opt) = self.option {
            api_request = api_request.request_option(opt);
        }

        // 发送请求
        Transport::request(api_request, self.config, None).await
    }
}

/// 上传图片
/// API文档: https://open.feishu.cn/document/lingo-v1/file/upload
pub async fn upload_file(
    request: FileUploadRequest,
    config: &Config,
    option: Option<openlark_core::req_option::RequestOption>,
) -> SDKResult<Response<UploadFileResponse>> {
    // 构建API端点
    let url = "/open-apis/lingo/v1/files/upload";

    // 创建API请求
    let mut api_request: ApiRequest<UploadFileResponse> =
        ApiRequest::post(url)
            .body(serde_json::to_value(request)?);

    // 如果有请求选项，应用它们
    if let Some(opt) = option {
        api_request = api_request.request_option(opt);
    }

    // 发送请求
    Transport::request(api_request, config, None).await
}
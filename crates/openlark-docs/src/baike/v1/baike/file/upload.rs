/// 上传图片
///
/// API文档: https://open.feishu.cn/document/server-docs/baike-v1/file/upload
///
/// 词条图片资源上传。

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
    _file_path: String,
    request: FileUploadRequest,
    option: Option<openlark_core::req_option::RequestOption>,
}

impl<'a> UploadFileBuilder<'a> {
    /// 创建新的上传图片构建器
    pub fn new(config: &'a Config, file_path: String) -> Self {
        Self {
            config,
            _file_path: file_path,
            request: FileUploadRequest::default(),
            option: None,
        }
    }

    /// 设置请求选项
    pub fn request_option(mut self, option: openlark_core::req_option::RequestOption) -> Self {
        self.option = Some(option);
        self
    }

    /// 执行上传图片请求
    pub async fn execute(self) -> SDKResult<Response<UploadFileResponse>> {
        // 构建API端点
        let url = "/open-apis/baike/v1/files/upload";

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
/// API文档: https://open.feishu.cn/document/server-docs/baike-v1/file/upload
pub async fn upload_file(
    request: FileUploadRequest,
    config: &Config,
    option: Option<openlark_core::req_option::RequestOption>,
) -> SDKResult<Response<UploadFileResponse>> {
    // 构建API端点
    let url = "/open-apis/baike/v1/files/upload";

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
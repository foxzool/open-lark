use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, Response, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
/// 下载文件
///
/// 使用该接口可以下载在云空间目录下的文件（不含飞书文档/电子表格/多维表格等在线文档）。
/// docPath: https://open.feishu.cn/document/server-docs/docs/drive-v1/file/download
use serde::{Deserialize, Serialize};

use crate::common::api_endpoints::DriveApi;

/// 下载文件请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DownloadFileRequest {
    /// 文件token
    pub file_token: String,
}

impl DownloadFileRequest {
    /// 创建下载文件请求
    pub fn new(file_token: impl Into<String>) -> Self {
        Self {
            file_token: file_token.into(),
        }
    }
}

/// 下载文件响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DownloadFileResponse {
    /// 文件内容（二进制数据由核心层处理）
    pub data: Option<serde_json::Value>,
}

impl ApiResponseTrait for DownloadFileResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 下载文件
///
/// 使用该接口可以下载在云空间目录下的文件（不含飞书文档/电子表格/多维表格等在线文档）。
/// docPath: https://open.feishu.cn/document/server-docs/docs/drive-v1/file/download
pub async fn download_file(
    request: DownloadFileRequest,
    config: &Config,
    option: Option<openlark_core::req_option::RequestOption>,
) -> SDKResult<openlark_core::api::Response<DownloadFileResponse>> {
    // 使用DriveApi枚举生成API端点
    let api_endpoint = DriveApi::DownloadFile(request.file_token.clone());

    // 创建API请求
    let mut api_request: ApiRequest<DownloadFileResponse> =
        ApiRequest::get(&api_endpoint.to_url());

    // 如果有请求选项，应用它们
    if let Some(opt) = option {
        api_request = api_request.request_option(opt);
    }

    // 发送请求
    Transport::request(api_request, config, None).await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_download_file_request_builder() {
        let request = DownloadFileRequest::new("file_token");
        assert_eq!(request.file_token, "file_token");
    }

    #[test]
    fn test_response_trait() {
        assert_eq!(
            DownloadFileResponse::data_format(),
            ResponseFormat::Data
        );
    }
}
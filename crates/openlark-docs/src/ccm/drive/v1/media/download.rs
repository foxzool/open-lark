use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, Response, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
/// 下载素材
///
/// 下载媒体素材。
/// docPath: https://open.feishu.cn/document/server-docs/docs/drive-v1/media/download
use serde::{Deserialize, Serialize};

use crate::common::api_endpoints::DriveApi;

/// 下载素材请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DownloadMediaRequest {
    /// 媒体token
    pub file_token: String,
}

impl DownloadMediaRequest {
    pub fn new(file_token: impl Into<String>) -> Self {
        Self {
            file_token: file_token.into(),
        }
    }
}

/// 下载素材响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DownloadMediaResponse {
    /// 二进制内容由核心层处理
    pub data: Option<serde_json::Value>,
}

impl ApiResponseTrait for DownloadMediaResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 下载素材
pub async fn download_media(
    request: DownloadMediaRequest,
    config: &Config,
    option: Option<openlark_core::req_option::RequestOption>,
) -> SDKResult<Response<DownloadMediaResponse>> {
    let api_endpoint = DriveApi::DownloadMedia(request.file_token.clone());

    let mut api_request: ApiRequest<DownloadMediaResponse> =
        ApiRequest::get(&api_endpoint.to_url());

    if let Some(opt) = option {
        api_request = api_request.request_option(opt);
    }

    Transport::request(api_request, config, None).await
}

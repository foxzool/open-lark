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
    #[serde(skip)]
    config: Config,
    /// 媒体token
    pub file_token: String,
}

impl DownloadMediaRequest {
    pub fn new(config: Config, file_token: impl Into<String>) -> Self {
        Self {
            config,
            file_token: file_token.into(),
        }
    }

    pub async fn execute(self) -> SDKResult<Response<DownloadMediaResponse>> {
        let api_endpoint = DriveApi::DownloadMedia(self.file_token.clone());
        let request = ApiRequest::<DownloadMediaResponse>::get(&api_endpoint.to_url());

        Transport::request(request, &self.config, None).await
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_download_media_request() {
        let config = Config::default();
        let request = DownloadMediaRequest::new(config, "media_token");
        assert_eq!(request.file_token, "media_token");
    }

    #[test]
    fn test_response_trait() {
        assert_eq!(DownloadMediaResponse::data_format(), ResponseFormat::Data);
    }
}

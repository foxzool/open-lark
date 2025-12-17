use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, Response, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
/// 下载导出文件
///
/// 下载导出的文件内容。
/// docPath: https://open.feishu.cn/document/server-docs/docs/drive-v1/export_task/download
use serde::{Deserialize, Serialize};

use crate::common::api_endpoints::DriveApi;

/// 下载导出文件请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DownloadExportRequest {
    #[serde(skip)]
    config: Config,
    /// 文件token
    pub file_token: String,
}

impl DownloadExportRequest {
    pub fn new(config: Config, file_token: impl Into<String>) -> Self {
        Self {
            config,
            file_token: file_token.into(),
        }
    }

    pub async fn execute(self) -> SDKResult<Response<DownloadExportResponse>> {
        let api_endpoint = DriveApi::DownloadExportFile(self.file_token.clone());

        let api_request = ApiRequest::<DownloadExportResponse>::get(&api_endpoint.to_url());

        Transport::request(api_request, &self.config, None).await
    }
}

/// 下载导出文件响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DownloadExportResponse {
    /// 二进制内容由核心层处理
    pub data: Option<serde_json::Value>,
}

impl ApiResponseTrait for DownloadExportResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_download_export_request_builder() {
        let config = Config::default();
        let request = DownloadExportRequest::new(config, "file_token");
        assert_eq!(request.file_token, "file_token");
    }

    #[test]
    fn test_response_trait() {
        assert_eq!(DownloadExportResponse::data_format(), ResponseFormat::Data);
    }
}

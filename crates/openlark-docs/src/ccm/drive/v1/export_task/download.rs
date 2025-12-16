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
    /// 文件token
    pub file_token: String,
}

impl DownloadExportRequest {
    pub fn new(file_token: impl Into<String>) -> Self {
        Self {
            file_token: file_token.into(),
        }
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

/// 下载导出文件
pub async fn download_export(
    request: DownloadExportRequest,
    config: &Config,
    option: Option<openlark_core::req_option::RequestOption>,
) -> SDKResult<Response<DownloadExportResponse>> {
    let api_endpoint = DriveApi::DownloadExportFile(request.file_token.clone());

    let mut api_request: ApiRequest<DownloadExportResponse> =
        ApiRequest::get(&api_endpoint.to_url());

    if let Some(opt) = option {
        api_request = api_request.request_option(opt);
    }

    Transport::request(api_request, config, None).await
}

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, Response, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
/// 下载文件
///
/// 下载云空间下的文件，不含飞书文档、电子表格以及多维表格等在线文档，支持指定文件 Range 进行下载。
/// docPath: https://open.feishu.cn/document/server-docs/docs/drive-v1/download/download
use serde::{Deserialize, Serialize};

use crate::common::{api_endpoints::DriveApi, api_utils::*};

/// 下载文件请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DownloadFileRequest {
    /// 文件token
    pub file_token: String,
    /// 下载范围，可选，用于分片下载
    pub range: Option<String>,
}

impl DownloadFileRequest {
    /// 创建下载文件请求
    ///
    /// # 参数
    /// * `file_token` - 文件token
    pub fn new(file_token: impl Into<String>) -> Self {
        Self {
            file_token: file_token.into(),
            range: None,
        }
    }

    /// 设置下载范围
    pub fn range(mut self, range: impl Into<String>) -> Self {
        self.range = Some(range.into());
        self
    }
}

/// 下载文件响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DownloadFileResponse {
    /// 文件下载信息
    pub data: Option<DownloadData>,
}

/// 下载数据
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DownloadData {
    /// 下载URL
    pub download_url: String,
    /// 文件名
    pub file_name: String,
    /// 文件大小
    pub file_size: i64,
    /// 过期时间
    pub expire_time: i64,
}

impl ApiResponseTrait for DownloadFileResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 下载文件
///
/// 下载云空间下的文件，不含飞书文档、电子表格以及多维表格等在线文档，支持指定文件 Range 进行下载。
/// docPath: https://open.feishu.cn/document/server-docs/docs/drive-v1/download/download
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

    // 如果有范围参数，添加到查询参数
    if let Some(range) = &request.range {
        api_request = api_request.query("range", range);
    }

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
        assert!(request.range.is_none());
    }

    #[test]
    fn test_download_file_request_with_range() {
        let request = DownloadFileRequest::new("file_token")
            .range("bytes=0-1023");

        assert_eq!(request.file_token, "file_token");
        assert_eq!(request.range.unwrap(), "bytes=0-1023");
    }

    #[test]
    fn test_download_data_structure() {
        let download_data = DownloadData {
            download_url: "https://example.com/download".to_string(),
            file_name: "test_file.txt".to_string(),
            file_size: 1024,
            expire_time: 1640995200,
        };

        assert_eq!(download_data.file_name, "test_file.txt");
        assert_eq!(download_data.file_size, 1024);
    }

    #[test]
    fn test_response_trait() {
        assert_eq!(
            DownloadFileResponse::data_format(),
            ResponseFormat::Data
        );
    }
}
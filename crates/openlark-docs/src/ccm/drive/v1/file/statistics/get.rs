/// 获取文件统计信息
///
/// 获取文件的访问、预览、下载等统计信息。
/// docPath: https://open.feishu.cn/open-apis/drive/v1/files/:file_token/statistics
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::api_endpoints::DriveApi;

/// 获取文件统计信息请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetFileStatisticsRequest {
    /// 文件token
    pub file_token: String,
}

impl GetFileStatisticsRequest {
    /// 创建获取文件统计信息请求
    ///
    /// # 参数
    /// * `file_token` - 文件token
    pub fn new(file_token: impl Into<String>) -> Self {
        Self {
            file_token: file_token.into(),
        }
    }
}

/// 获取文件统计信息响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetFileStatisticsResponse {
    /// 文件统计信息
    pub data: Option<FileStatistics>,
}

impl ApiResponseTrait for GetFileStatisticsResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 文件统计信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileStatistics {
    /// 文件token
    pub file_token: String,
    /// 总访问次数
    pub total_visits: u32,
    /// 今日访问次数
    pub today_visits: u32,
    /// 总预览次数
    pub total_previews: u32,
    /// 今日预览次数
    pub today_previews: u32,
    /// 总下载次数
    pub total_downloads: u32,
    /// 今日下载次数
    pub today_downloads: u32,
    /// 总评论数
    pub total_comments: u32,
    /// 总点赞数
    pub total_likes: u32,
    /// 最后访问时间
    pub last_visit_time: Option<String>,
    /// 最后预览时间
    pub last_preview_time: Option<String>,
    /// 最后下载时间
    pub last_download_time: Option<String>,
}

/// 获取文件统计信息
///
/// 获取文件的访问、预览、下载等统计信息。
/// docPath: https://open.feishu.cn/open-apis/drive/v1/files/:file_token/statistics
pub async fn get_file_statistics(
    request: GetFileStatisticsRequest,
    config: &Config,
    option: Option<openlark_core::req_option::RequestOption>,
) -> SDKResult<openlark_core::api::Response<GetFileStatisticsResponse>> {
    // 创建API请求
    let url = DriveApi::GetFileStatistics(request.file_token.clone()).to_url();
    let mut api_request: ApiRequest<GetFileStatisticsResponse> =
        ApiRequest::get(&url);

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
    fn test_get_file_statistics_request_builder() {
        let request = GetFileStatisticsRequest::new("file_token");
        assert_eq!(request.file_token, "file_token");
    }

    #[test]
    fn test_file_statistics_structure() {
        let stats = FileStatistics {
            file_token: "file_123".to_string(),
            total_visits: 100,
            today_visits: 5,
            total_previews: 80,
            today_previews: 3,
            total_downloads: 20,
            today_downloads: 2,
            total_comments: 10,
            total_likes: 15,
            last_visit_time: Some("2023-01-01T00:00:00Z".to_string()),
            last_preview_time: Some("2023-01-01T01:00:00Z".to_string()),
            last_download_time: Some("2023-01-01T02:00:00Z".to_string()),
        };

        assert_eq!(stats.file_token, "file_123");
        assert_eq!(stats.total_visits, 100);
        assert_eq!(stats.today_visits, 5);
        assert_eq!(stats.last_visit_time, Some("2023-01-01T00:00:00Z".to_string()));
    }

    #[test]
    fn test_response_trait() {
        assert_eq!(GetFileStatisticsResponse::data_format(), ResponseFormat::Data);
    }
}
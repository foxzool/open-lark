use serde::{Deserialize, Serialize};
use openlark_core::{
    api:: ApiResponseTrait,
    models::{OpenLarkConfig, OpenLarkRequest},
    OpenLarkClient, SDKResult,
};

/// 获取文件统计信息请求
#[derive(Debug, Serialize, Default)]
pub struct GetFileStatisticsRequest {
    /// 文件token
    pub file_token: String,
}

/// 获取文件统计信息响应
#[derive(Debug, Deserialize, Default)]
pub struct GetFileStatisticsResponse {
    /// 文件统计信息
    pub statistics: FileStatistics,
    /// 操作结果
    pub result: String,
}

/// 文件统计信息
#[derive(Debug, Deserialize, Default)]
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
/// docPath: https://open.feishu.cn/open-apis/drive/v1/files/:file_token/statistics
pub async fn get_file_statistics(
    request: GetFileStatisticsRequest,
    config: &OpenLarkConfig,
    option: Option<openlark_core::req_option::RequestOption>,
) -> SDKResult<openlark_core::api::Response<GetFileStatisticsResponse>> {
    let url = format!(
        "{}/open-apis/drive/v1/files/{}/statistics",
        config.base_url, request.file_token
    );

    let req = OpenLarkRequest {
        url,
        method: http::Method::GET,
        headers: vec![],
        query_params: vec![],
        body: None,
    };

    OpenLarkClient::request(req, config, option).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio;

    #[tokio::test]
    async fn test_get_file_statistics() {
        let config = OpenLarkConfig {
            app_id: "test_app_id".to_string(),
            app_secret: "test_app_secret".to_string(),
            base_url: "https://open.feishu.cn".to_string(),
            ..Default::default()
        };

        let request = GetFileStatisticsRequest {
            file_token: "test_file_token".to_string(),
        };

        let result = get_file_statistics(request, &config, None).await;
        assert!(result.is_ok());
    }
}
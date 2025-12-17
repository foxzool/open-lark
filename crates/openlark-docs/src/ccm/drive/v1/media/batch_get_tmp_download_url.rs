use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, Response, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
/// 获取素材临时下载链接
///
/// 通过`file_tokens`获取素材临时下载链接，链接时效性是 24 小时，过期失效。
/// docPath: https://open.feishu.cn/document/server-docs/docs/drive-v1/media/batch_get_tmp_download_url
use serde::{Deserialize, Serialize};

use crate::common::api_endpoints::DriveApi;

/// 获取素材临时下载链接请求
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchGetTmpDownloadUrlRequest {
    #[serde(skip)]
    config: Config,
    /// 文件token列表
    pub file_tokens: Vec<String>,
}

impl BatchGetTmpDownloadUrlRequest {
    /// 创建获取素材临时下载链接请求
    ///
    /// # 参数
    /// * `config` - 配置
    /// * `file_tokens` - 文件token列表
    pub fn new(config: Config, file_tokens: Vec<String>) -> Self {
        Self { 
            config,
            file_tokens 
        }
    }

    /// 添加文件token
    pub fn add_file_token(mut self, file_token: impl Into<String>) -> Self {
        self.file_tokens.push(file_token.into());
        self
    }

    pub async fn execute(self) -> SDKResult<Response<BatchGetTmpDownloadUrlResponse>> {
        let api_endpoint = DriveApi::GetMediaTempDownloadUrls;
        let request = ApiRequest::<BatchGetTmpDownloadUrlResponse>::get(&api_endpoint.to_url())
            .query("file_tokens", &self.file_tokens.join(","));

        Transport::request(request, &self.config, None).await
    }
}

/// 临时下载链接信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TmpDownloadUrlInfo {
    /// 文件token
    pub file_token: String,
    /// 临时下载链接
    pub tmp_download_url: String,
    /// 过期时间
    pub expire_time: i64,
}

/// 获取素材临时下载链接响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchGetTmpDownloadUrlResponse {
    /// 临时下载链接信息列表
    pub data: Option<Vec<TmpDownloadUrlInfo>>,
}

impl ApiResponseTrait for BatchGetTmpDownloadUrlResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_batch_get_tmp_download_url_request_builder() {
        let config = Config::default();
        let request = BatchGetTmpDownloadUrlRequest::new(config, vec![
            "file_token_1".to_string(),
            "file_token_2".to_string(),
        ]);

        assert_eq!(request.file_tokens.len(), 2);
        assert_eq!(request.file_tokens[0], "file_token_1");
    }

    #[test]
    fn test_batch_get_tmp_download_url_request_add_token() {
        let config = Config::default();
        let request = BatchGetTmpDownloadUrlRequest::new(config, vec!["file_token_1".to_string()])
            .add_file_token("file_token_2");

        assert_eq!(request.file_tokens.len(), 2);
        assert_eq!(request.file_tokens[1], "file_token_2");
    }

    #[test]
    fn test_tmp_download_url_info_structure() {
        let url_info = TmpDownloadUrlInfo {
            file_token: "file_token".to_string(),
            tmp_download_url: "https://example.com/download".to_string(),
            expire_time: 1640995200,
        };

        assert_eq!(url_info.file_token, "file_token");
        assert_eq!(url_info.tmp_download_url, "https://example.com/download");
        assert_eq!(url_info.expire_time, 1640995200);
    }

    #[test]
    fn test_response_trait() {
        assert_eq!(
            BatchGetTmpDownloadUrlResponse::data_format(),
            ResponseFormat::Data
        );
    }
}
/// 下载图片
///
/// API文档: https://open.feishu.cn/document/lingo-v1/file/download
///
/// 通过 file_token 下载原图片。

use openlark_core::{
    error::SDKResult,
    config::Config,
    request_builder::UnifiedRequestBuilder,
    constants::AccessTokenType,
    api::{ApiRequest, Response},
    req_option::RequestOption,
};
use serde::{Deserialize, Serialize};

/// 下载图片响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DownloadFileResponse {
    /// 文件内容（二进制数据的base64编码）
    pub content: String,
    /// 文件名
    pub file_name: String,
    /// 文件大小
    pub file_size: i64,
    /// 文件类型
    pub content_type: String,
}

/// 下载图片构建器
pub struct DownloadFileBuilder<'a> {
    config: &'a Config,
    file_token: String,
}

impl<'a> DownloadFileBuilder<'a> {
    /// 创建新的下载图片构建器
    pub fn new(config: &'a Config, file_token: String) -> Self {
        Self { config, file_token }
    }

    /// 执行下载图片操作
    pub async fn execute(self) -> SDKResult<DownloadFileResponse> {
        let path = format!("/open-apis/lingo/v1/files/{}/download", self.file_token);
        let mut api_request = ApiRequest::get(&path);

        let http_request = UnifiedRequestBuilder::build(
            &mut api_request,
            AccessTokenType::App,
            self.config,
            &RequestOption::default(),
        ).await?;

        let response = self.config.http_client().execute(http_request).await?;

        // 对于文件下载，需要特殊处理响应
        if response.status().is_success() {
            let bytes = response.bytes().await.map_err(|e| {
                openlark_core::error::CoreError::NetworkError(e.to_string())
            })?;

            // 将二进制数据转为base64编码
            let content = base64::encode(&bytes);

            // 尝试从响应头获取文件信息
            let content_type = response.headers()
                .get("content-type")
                .and_then(|v| v.to_str().ok())
                .unwrap_or("application/octet-stream")
                .to_string();

            let file_size = bytes.len() as i64;
            let file_name = format!("file_{}", self.file_token);

            Ok(DownloadFileResponse {
                content,
                file_name,
                file_size,
                content_type,
            })
        } else {
            // 处理错误响应
            let raw_response = Response::from_reqwest_response(response).await?;
            raw_response.into_result()
        }
    }
}
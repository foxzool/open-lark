//! 下载素材
//!
//! 下载各种类型文档中的素材（如电子表格图片、附件等），支持通过 Range 分片下载。
//!
//! docPath: https://open.feishu.cn/document/server-docs/docs/drive-v1/media/download

use crate::common::api_endpoints::DriveApi;
use openlark_core::{
    api::{ApiRequest, Response},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    SDKResult,
};

/// 下载素材请求
#[derive(Debug)]
pub struct DownloadMediaRequest {
    config: Config,
    /// 素材文件 token
    pub file_token: String,
    /// 拓展参数（如多维表格高级权限下载鉴权）
    pub extra: Option<String>,
    /// Range HTTP header（可选），示例：bytes=0-1024
    pub range: Option<String>,
}

impl DownloadMediaRequest {
    pub fn new(config: Config, file_token: impl Into<String>) -> Self {
        Self {
            config,
            file_token: file_token.into(),
            extra: None,
            range: None,
        }
    }

    pub fn extra(mut self, extra: impl Into<String>) -> Self {
        self.extra = Some(extra.into());
        self
    }

    pub fn range(mut self, range: impl Into<String>) -> Self {
        self.range = Some(range.into());
        self
    }

    /// 执行下载请求，返回二进制内容
    pub async fn execute(self) -> SDKResult<Response<Vec<u8>>> {
        if self.file_token.is_empty() {
            return Err(openlark_core::error::validation_error(
                "file_token",
                "file_token 不能为空",
            ));
        }
        if let Some(range) = &self.range {
            if !range.starts_with("bytes=") || !range.contains('-') {
                return Err(openlark_core::error::validation_error(
                    "range",
                    "range 格式必须为 bytes=start-end（例如 bytes=0-1024）",
                ));
            }
        }

        let api_endpoint = DriveApi::DownloadMedia(self.file_token.clone());
        let mut request =
            ApiRequest::<Vec<u8>>::get(&api_endpoint.to_url()).query_opt("extra", self.extra);

        if let Some(r) = self.range {
            request = request.header("Range", &r);
        }

        Transport::request(request, &self.config, None).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_download_media_request() {
        let config = Config::default();
        let request = DownloadMediaRequest::new(config, "media_token")
            .extra("extra")
            .range("bytes=0-100");

        assert_eq!(request.file_token, "media_token");
        assert_eq!(request.extra, Some("extra".to_string()));
        assert_eq!(request.range, Some("bytes=0-100".to_string()));
    }
}

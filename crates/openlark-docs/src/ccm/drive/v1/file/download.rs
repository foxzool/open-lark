use openlark_core::{
    api::{ApiRequest, Response},
    config::Config,
    http::Transport,
    SDKResult,
};
/// 下载文件
///
/// 使用该接口可以下载在云空间目录下的文件（不含飞书文档/电子表格/多维表格等在线文档）。
/// docPath: /document/uAjLw4CM/ukTMukTMukTM/reference/drive-v1/file/download

use crate::common::api_endpoints::DriveApi;

/// 下载文件请求
#[derive(Debug)]
pub struct DownloadFileRequest {
    config: Config,
    /// 文件token
    pub file_token: String,
    /// Range HTTP header, optional. e.g. "bytes=0-100"
    pub range: Option<String>,
}

impl DownloadFileRequest {
    /// 创建下载文件请求
    pub fn new(config: Config, file_token: impl Into<String>) -> Self {
        Self {
            config,
            file_token: file_token.into(),
            range: None,
        }
    }

    /// 设置Range头部
    pub fn range(mut self, range: impl Into<String>) -> Self {
        self.range = Some(range.into());
        self
    }

    /// 执行下载请求
    ///
    /// 成功时返回文件二进制内容（`Response<Vec<u8>>`）。
    pub async fn execute(self) -> SDKResult<Response<Vec<u8>>> {
        if self.file_token.is_empty() {
            return Err(openlark_core::error::validation_error(
                "file_token",
                "file_token 不能为空",
            ));
        }

        let api_endpoint = DriveApi::DownloadFile(self.file_token.clone());
        let mut request = ApiRequest::<Vec<u8>>::get(&api_endpoint.to_url());

        if let Some(r) = self.range {
            // 文档约束：Range 形如 bytes=start-end（start/end 均为非负整数，end 可省略）
            let r = r.trim();
            if !r.starts_with("bytes=") {
                return Err(openlark_core::error::validation_error(
                    "range",
                    "Range 必须以 bytes= 开头，例如 bytes=0-1023",
                ));
            }
            let range_spec = &r["bytes=".len()..];
            let (start, end) = range_spec.split_once('-').ok_or_else(|| {
                openlark_core::error::validation_error(
                    "range",
                    "Range 格式错误，应为 bytes=start-end，例如 bytes=0-1023",
                )
            })?;
            if start.trim().is_empty() {
                return Err(openlark_core::error::validation_error(
                    "range",
                    "Range start 不能为空，例如 bytes=0-1023",
                ));
            }
            if start.trim().parse::<u64>().is_err() {
                return Err(openlark_core::error::validation_error(
                    "range",
                    "Range start 必须为非负整数",
                ));
            }
            if !end.trim().is_empty() && end.trim().parse::<u64>().is_err() {
                return Err(openlark_core::error::validation_error(
                    "range",
                    "Range end 必须为非负整数或为空（例如 bytes=0-）",
                ));
            }
            request = request.header("Range", r);
        }

        Transport::request(request, &self.config, None).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_download_file_request_builder() {
        let config = Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build();
        let file_token = "test_file_token_123";
        let range_header = "bytes=0-100";

        let request = DownloadFileRequest::new(config.clone(), file_token).range(range_header);

        assert_eq!(request.file_token, file_token);
        assert_eq!(request.range, Some(range_header.to_string()));
    }
}

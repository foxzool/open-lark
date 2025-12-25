//! 下载图片
//!
//! docPath: /document/uAjLw4CM/ukTMukTMukTM/reference/baike-v1/file/download

use openlark_core::{api::ApiRequest, config::Config, http::Transport, validate_required, SDKResult};

use crate::common::api_endpoints::BaikeApiV1;

/// 下载图片请求
pub struct DownloadFileRequest {
    config: Config,
    file_token: String,
}

impl DownloadFileRequest {
    pub fn new(config: Config, file_token: impl Into<String>) -> Self {
        Self {
            config,
            file_token: file_token.into(),
        }
    }

    /// 下载原图，返回二进制内容
    pub async fn send(self) -> SDKResult<Vec<u8>> {
        validate_required!(self.file_token, "file_token 不能为空");

        let api_request: ApiRequest<Vec<u8>> =
            ApiRequest::get(&BaikeApiV1::FileDownload(self.file_token).to_url());
        let response = Transport::request(api_request, &self.config, None).await?;
        response
            .data
            .ok_or_else(|| openlark_core::error::validation_error("response", "响应数据为空"))
    }
}

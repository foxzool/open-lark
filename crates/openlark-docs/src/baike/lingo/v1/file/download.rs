//! 下载图片
//!
//! docPath: /document/uAjLw4CM/ukTMukTMukTM/lingo-v1/file/download
//! doc: https://open.feishu.cn/document/uAjLw4CM/ukTMukTMukTM/lingo-v1/file/download

use openlark_core::{api::ApiRequest, config::Config, http::Transport, validate_required, SDKResult};

use crate::common::api_endpoints::LingoApiV1;
use crate::common::api_utils::*;

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

    /// 下载图片二进制内容
    pub async fn send(self) -> SDKResult<Vec<u8>> {
        validate_required!(self.file_token, "file_token 不能为空");

        let api_request: ApiRequest<Vec<u8>> =
            ApiRequest::get(&LingoApiV1::FileDownload(self.file_token).to_url());

        let response = Transport::request(api_request, &self.config, None).await?;
        extract_response_data(response, "下载图片")
    }
}

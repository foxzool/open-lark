//! 下载图片
//!
//! docPath: https://open.feishu.cn/document/lingo-v1/file/download

use openlark_core::{
    api::{ApiRequest, Response}, config::Config, http::Transport, req_option::RequestOption,
    validate_required, SDKResult,
};

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
    pub async fn execute(self) -> SDKResult<Response<Vec<u8>>> {
        self.execute_with_options(RequestOption::default()).await
    }

    /// 下载图片二进制内容（支持自定义选项）
    pub async fn execute_with_options(self, option: RequestOption) -> SDKResult<Response<Vec<u8>>> {
        validate_required!(self.file_token, "file_token 不能为空");

        let api_request: ApiRequest<Vec<u8>> =
            ApiRequest::get(&LingoApiV1::FileDownload(self.file_token).to_url());

        let response = Transport::request(api_request, &self.config, Some(option)).await?;
        Ok(response)
    }
}

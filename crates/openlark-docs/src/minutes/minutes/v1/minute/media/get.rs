/// 下载妙记音视频文件
///
/// 下载妙记的音视频文件。
/// docPath: /document/uAjLw4CM/ukTMukTMukTM/minutes-v1/minute-media/get
/// doc: https://open.feishu.cn/document/minutes-v1/minute-media/get
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::api_utils::*;

/// 下载妙记音视频文件请求
pub struct GetMinuteMediaRequest {
    minute_token: String,
    config: Config,
}

/// 下载妙记音视频文件响应
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetMinuteMediaResponse {
    /// 妙记音视频文件下载链接（有效期 1 天）
    pub download_url: String,
}

impl ApiResponseTrait for GetMinuteMediaResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl GetMinuteMediaRequest {
    /// 创建下载妙记音视频文件请求
    pub fn new(config: Config) -> Self {
        Self {
            minute_token: String::new(),
            config,
        }
    }

    /// 设置妙记Token
    pub fn minute_token(mut self, minute_token: impl Into<String>) -> Self {
        self.minute_token = minute_token.into();
        self
    }

    /// 发送请求
    pub async fn send(self) -> SDKResult<GetMinuteMediaResponse> {
        self.execute().await
    }

    /// 执行请求
    ///
    /// doc: https://open.feishu.cn/document/minutes-v1/minute-media/get
    pub async fn execute(self) -> SDKResult<GetMinuteMediaResponse> {
        validate_required!(self.minute_token, "妙记Token不能为空");

        use crate::common::api_endpoints::MinutesApiV1;
        let api_endpoint = MinutesApiV1::MediaGet(self.minute_token.clone());

        let api_request: ApiRequest<GetMinuteMediaResponse> =
            ApiRequest::get(&api_endpoint.to_url());

        let response = Transport::request(api_request, &self.config, None).await?;
        extract_response_data(response, "下载妙记音视频文件")
    }
}

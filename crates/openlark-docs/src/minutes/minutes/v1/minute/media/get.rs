//! 下载妙记音视频文件
//!
//! docPath: https://open.feishu.cn/document/minutes-v1/minute-media/get

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::{api_endpoints::MinutesApiV1, api_utils::*};

#[derive(Debug, Clone)]
pub struct GetMinuteMediaRequest {
    config: Config,
    minute_token: Option<String>,
}

impl GetMinuteMediaRequest {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            minute_token: None,
        }
    }

    pub fn minute_token(mut self, minute_token: impl Into<String>) -> Self {
        self.minute_token = Some(minute_token.into());
        self
    }

    pub async fn execute(self) -> SDKResult<GetMinuteMediaResponse> {
        let minute_token = self.minute_token.ok_or_else(|| {
            openlark_core::error::validation_error("minute_token", "minute_token 不能为空")
        })?;
        if minute_token.chars().count() != 24 {
            return Err(openlark_core::error::validation_error(
                "minute_token",
                "minute_token 长度必须为 24 字符",
            ));
        }

        let api_endpoint = MinutesApiV1::MediaGet(minute_token);
        let api_request: ApiRequest<GetMinuteMediaResponse> =
            ApiRequest::get(&api_endpoint.to_url());

        let response = Transport::request(api_request, &self.config, None).await?;
        extract_response_data(response, "下载妙记音视频文件")
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetMinuteMediaResponse {
    pub download_url: String,
}

impl ApiResponseTrait for GetMinuteMediaResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

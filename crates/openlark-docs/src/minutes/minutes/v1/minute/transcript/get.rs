//! 导出妙记文字记录
//!
//! docPath: https://open.feishu.cn/document/minutes-v1/minute-transcript/get

use openlark_core::{
    api::{ApiRequest, Response},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    SDKResult,
};

use crate::common::api_endpoints::MinutesApiV1;

#[derive(Debug, Clone)]
pub struct GetMinuteTranscriptRequest {
    config: Config,
    minute_token: Option<String>,
    need_speaker: Option<bool>,
    need_timestamp: Option<bool>,
    file_format: Option<String>,
}

impl GetMinuteTranscriptRequest {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            minute_token: None,
            need_speaker: None,
            need_timestamp: None,
            file_format: None,
        }
    }

    pub fn minute_token(mut self, minute_token: impl Into<String>) -> Self {
        self.minute_token = Some(minute_token.into());
        self
    }

    pub fn need_speaker(mut self, need_speaker: bool) -> Self {
        self.need_speaker = Some(need_speaker);
        self
    }

    pub fn need_timestamp(mut self, need_timestamp: bool) -> Self {
        self.need_timestamp = Some(need_timestamp);
        self
    }

    pub fn file_format(mut self, file_format: impl Into<String>) -> Self {
        self.file_format = Some(file_format.into());
        self
    }

    /// 成功时返回文件二进制内容（`Response<Vec<u8>>`）。
    pub async fn execute(self) -> SDKResult<Response<Vec<u8>>> {
        let minute_token = self.minute_token.ok_or_else(|| {
            openlark_core::error::validation_error("minute_token", "minute_token 不能为空")
        })?;
        if minute_token.chars().count() != 24 {
            return Err(openlark_core::error::validation_error(
                "minute_token",
                "minute_token 长度必须为 24 字符",
            ));
        }
        if let Some(fmt) = &self.file_format {
            match fmt.as_str() {
                "txt" | "srt" => {}
                _ => {
                    return Err(openlark_core::error::validation_error(
                        "file_format",
                        "file_format 仅支持 txt/srt",
                    ));
                }
            }
        }

        let api_endpoint = MinutesApiV1::TranscriptGet(minute_token);
        let mut api_request: ApiRequest<Vec<u8>> = ApiRequest::get(&api_endpoint.to_url());

        if let Some(v) = self.need_speaker {
            api_request = api_request.query("need_speaker", v.to_string());
        }
        if let Some(v) = self.need_timestamp {
            api_request = api_request.query("need_timestamp", v.to_string());
        }
        if let Some(fmt) = &self.file_format {
            api_request = api_request.query("file_format", fmt);
        }

        Transport::request(api_request, &self.config, Some(option)).await
    }
}

/// 导出妙记文字记录
///
/// 导出妙记的文字记录。
/// docPath: /document/uAjLw4CM/ukTMukTMukTM/minutes-v1/minute-transcript/get
/// doc: https://open.feishu.cn/document/minutes-v1/minute-transcript/get
use openlark_core::{
    api::ApiRequest,
    config::Config,
    http::Transport,
    validate_required, SDKResult,
};

use crate::common::api_utils::*;

/// 导出妙记文字记录请求
pub struct GetMinuteTranscriptRequest {
    minute_token: String,
    need_speaker: Option<bool>,
    need_timestamp: Option<bool>,
    file_format: Option<String>,
    config: Config,
}

impl GetMinuteTranscriptRequest {
    /// 创建导出妙记文字记录请求
    pub fn new(config: Config) -> Self {
        Self {
            minute_token: String::new(),
            need_speaker: None,
            need_timestamp: None,
            file_format: None,
            config,
        }
    }

    /// 设置妙记Token
    pub fn minute_token(mut self, minute_token: impl Into<String>) -> Self {
        self.minute_token = minute_token.into();
        self
    }

    /// 是否包含说话人（query: need_speaker）
    pub fn need_speaker(mut self, need_speaker: bool) -> Self {
        self.need_speaker = Some(need_speaker);
        self
    }

    /// 是否包含时间戳（query: need_timestamp）
    pub fn need_timestamp(mut self, need_timestamp: bool) -> Self {
        self.need_timestamp = Some(need_timestamp);
        self
    }

    /// 导出文件格式（query: file_format）
    ///
    /// 文档示例值包含 `txt` / `srt` 等，可按文档要求传入。
    pub fn file_format(mut self, file_format: impl Into<String>) -> Self {
        self.file_format = Some(file_format.into());
        self
    }

    /// 发送请求
    pub async fn send(self) -> SDKResult<Vec<u8>> {
        self.execute().await
    }

    /// 执行请求
    ///
    /// doc: https://open.feishu.cn/document/minutes-v1/minute-transcript/get
    pub async fn execute(self) -> SDKResult<Vec<u8>> {
        validate_required!(self.minute_token, "妙记Token不能为空");

        use crate::common::api_endpoints::MinutesApiV1;
        let api_endpoint = MinutesApiV1::TranscriptGet(self.minute_token.clone());

        let mut api_request: ApiRequest<Vec<u8>> = ApiRequest::get(&api_endpoint.to_url());
        if let Some(need_speaker) = self.need_speaker {
            api_request = api_request.query("need_speaker", need_speaker.to_string());
        }
        if let Some(need_timestamp) = self.need_timestamp {
            api_request = api_request.query("need_timestamp", need_timestamp.to_string());
        }
        if let Some(file_format) = &self.file_format {
            api_request = api_request.query("file_format", file_format);
        }

        let response = Transport::request(api_request, &self.config, None).await?;
        extract_response_data(response, "导出妙记文字记录")
    }
}

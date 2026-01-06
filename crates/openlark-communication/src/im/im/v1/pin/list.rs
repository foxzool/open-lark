//! 获取群内 Pin 消息
//!
//! docPath: https://open.feishu.cn/document/server-docs/im-v1/pin/list

use openlark_core::{
    api::ApiRequest, config::Config, http::Transport, validate_required, SDKResult,
};

use crate::{
    common::api_utils::extract_response_data, endpoints::IM_V1_PINS,
    im::im::v1::pin::models::ListPinsResponse,
};

/// 获取群内 Pin 消息请求
pub struct ListPinsRequest {
    config: Config,
    chat_id: String,
    start_time: Option<String>,
    end_time: Option<String>,
    page_size: Option<i32>,
    page_token: Option<String>,
}

impl ListPinsRequest {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            chat_id: String::new(),
            start_time: None,
            end_time: None,
            page_size: None,
            page_token: None,
        }
    }

    /// 群 ID（查询参数）
    pub fn chat_id(mut self, chat_id: impl Into<String>) -> Self {
        self.chat_id = chat_id.into();
        self
    }

    /// 起始时间（查询参数，可选，毫秒级时间戳）
    pub fn start_time(mut self, start_time: impl Into<String>) -> Self {
        self.start_time = Some(start_time.into());
        self
    }

    /// 结束时间（查询参数，可选，毫秒级时间戳）
    pub fn end_time(mut self, end_time: impl Into<String>) -> Self {
        self.end_time = Some(end_time.into());
        self
    }

    /// 分页大小（查询参数，可选，默认 20，范围 1~50）
    pub fn page_size(mut self, page_size: i32) -> Self {
        self.page_size = Some(page_size);
        self
    }

    /// 分页标记（查询参数，可选）
    pub fn page_token(mut self, page_token: impl Into<String>) -> Self {
        self.page_token = Some(page_token.into());
        self
    }

    /// 执行请求
    ///
    /// docPath: https://open.feishu.cn/document/server-docs/im-v1/pin/list
    pub async fn execute(self) -> SDKResult<ListPinsResponse> {
        validate_required!(self.chat_id, "chat_id 不能为空");

        // url: GET:/open-apis/im/v1/pins
        let mut req: ApiRequest<ListPinsResponse> =
            ApiRequest::get(IM_V1_PINS).query("chat_id", self.chat_id);

        if let Some(start_time) = self.start_time {
            req = req.query("start_time", start_time);
        }
        if let Some(end_time) = self.end_time {
            req = req.query("end_time", end_time);
        }
        if let Some(page_size) = self.page_size {
            req = req.query("page_size", page_size.to_string());
        }
        if let Some(page_token) = self.page_token {
            req = req.query("page_token", page_token);
        }

        let resp = Transport::request(req, &self.config, None).await?;
        extract_response_data(resp, "获取群内 Pin 消息")
    }
}

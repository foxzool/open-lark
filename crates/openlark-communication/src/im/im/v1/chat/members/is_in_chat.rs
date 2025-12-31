//! 判断用户或机器人是否在群里
//!
//! docPath: https://open.feishu.cn/document/server-docs/group/chat-member/is_in_chat

use openlark_core::{api::ApiRequest, config::Config, http::Transport, validate_required, SDKResult};

use crate::{
    common::api_utils::extract_response_data,
    endpoints::IM_V1_CHATS,
    im::im::v1::chat::members::models::IsInChatResponse,
};

/// 判断用户或机器人是否在群里请求
pub struct IsInChatRequest {
    config: Config,
    chat_id: String,
}

impl IsInChatRequest {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            chat_id: String::new(),
        }
    }

    /// 群 ID（路径参数）
    pub fn chat_id(mut self, chat_id: impl Into<String>) -> Self {
        self.chat_id = chat_id.into();
        self
    }

    /// 执行请求
    ///
    /// docPath: https://open.feishu.cn/document/server-docs/group/chat-member/is_in_chat
    pub async fn execute(self) -> SDKResult<IsInChatResponse> {
        validate_required!(self.chat_id, "chat_id 不能为空");

        // url: GET:/open-apis/im/v1/chats/:chat_id/members/is_in_chat
        let req: ApiRequest<IsInChatResponse> = ApiRequest::get(format!(
            "{}/{}/members/is_in_chat",
            IM_V1_CHATS, self.chat_id
        ));

        let resp = Transport::request(req, &self.config, None).await?;
        extract_response_data(resp, "判断用户或机器人是否在群里")
    }
}


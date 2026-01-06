//! 添加会话标签页
//!
//! docPath: https://open.feishu.cn/document/server-docs/group/chat-tab/create

use openlark_core::{
    api::ApiRequest, config::Config, http::Transport, validate_required, SDKResult,
};

use crate::{
    common::api_utils::{extract_response_data, serialize_params},
    endpoints::IM_V1_CHATS,
};

/// 添加会话标签页请求
pub struct CreateChatTabRequest {
    config: Config,
    chat_id: String,
}

impl CreateChatTabRequest {
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
    /// 说明：该接口请求体字段较多，建议直接按文档构造 JSON 传入。
    ///
    /// docPath: https://open.feishu.cn/document/server-docs/group/chat-tab/create
    pub async fn execute(self, body: serde_json::Value) -> SDKResult<serde_json::Value> {
        validate_required!(self.chat_id, "chat_id 不能为空");

        // url: POST:/open-apis/im/v1/chats/:chat_id/chat_tabs
        let req: ApiRequest<serde_json::Value> =
            ApiRequest::post(format!("{}/{}/chat_tabs", IM_V1_CHATS, self.chat_id))
                .body(serialize_params(&body, "添加会话标签页")?);

        let resp = Transport::request(req, &self.config, None).await?;
        extract_response_data(resp, "添加会话标签页")
    }
}

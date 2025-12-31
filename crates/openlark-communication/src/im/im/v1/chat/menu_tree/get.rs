//! 获取群菜单
//!
//! docPath: https://open.feishu.cn/document/server-docs/group/chat-menu_tree/get

use openlark_core::{api::ApiRequest, config::Config, http::Transport, validate_required, SDKResult};

use crate::{common::api_utils::extract_response_data, endpoints::IM_V1_CHATS};

/// 获取群菜单请求
pub struct GetChatMenuTreeRequest {
    config: Config,
    chat_id: String,
}

impl GetChatMenuTreeRequest {
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
    /// docPath: https://open.feishu.cn/document/server-docs/group/chat-menu_tree/get
    pub async fn execute(self) -> SDKResult<serde_json::Value> {
        validate_required!(self.chat_id, "chat_id 不能为空");

        // url: GET:/open-apis/im/v1/chats/:chat_id/menu_tree
        let req: ApiRequest<serde_json::Value> =
            ApiRequest::get(format!("{}/{}/menu_tree", IM_V1_CHATS, self.chat_id));

        let resp = Transport::request(req, &self.config, None).await?;
        extract_response_data(resp, "获取群菜单")
    }
}


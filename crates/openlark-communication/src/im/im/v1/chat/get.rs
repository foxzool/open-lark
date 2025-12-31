//! 获取群信息
//!
//! docPath: https://open.feishu.cn/document/server-docs/group/chat/get-2

use openlark_core::{api::ApiRequest, config::Config, http::Transport, validate_required, SDKResult};

use crate::{
    common::api_utils::extract_response_data,
    endpoints::IM_V1_CHATS,
    im::im::v1::message::models::UserIdType,
};

/// 获取群信息请求
pub struct GetChatRequest {
    config: Config,
    chat_id: String,
    user_id_type: Option<UserIdType>,
}

impl GetChatRequest {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            chat_id: String::new(),
            user_id_type: None,
        }
    }

    /// 群 ID（路径参数）
    pub fn chat_id(mut self, chat_id: impl Into<String>) -> Self {
        self.chat_id = chat_id.into();
        self
    }

    /// 用户 ID 类型（查询参数，可选，默认 open_id）
    pub fn user_id_type(mut self, user_id_type: UserIdType) -> Self {
        self.user_id_type = Some(user_id_type);
        self
    }

    /// 执行请求
    ///
    /// docPath: https://open.feishu.cn/document/server-docs/group/chat/get-2
    pub async fn execute(self) -> SDKResult<serde_json::Value> {
        validate_required!(self.chat_id, "chat_id 不能为空");

        // url: GET:/open-apis/im/v1/chats/:chat_id
        let mut req: ApiRequest<serde_json::Value> =
            ApiRequest::get(format!("{}/{}", IM_V1_CHATS, self.chat_id));

        if let Some(user_id_type) = self.user_id_type {
            req = req.query("user_id_type", user_id_type.as_str());
        }

        let resp = Transport::request(req, &self.config, None).await?;
        extract_response_data(resp, "获取群信息")
    }
}


//! 创建群
//!
//! docPath: https://open.feishu.cn/document/server-docs/group/chat/create

use openlark_core::{api::ApiRequest, config::Config, http::Transport, SDKResult};

use crate::{
    common::api_utils::{extract_response_data, serialize_params},
    endpoints::IM_V1_CHATS,
    im::im::v1::message::models::UserIdType,
};

/// 创建群请求
pub struct CreateChatRequest {
    config: Config,
    user_id_type: Option<UserIdType>,
    set_bot_manager: Option<bool>,
    uuid: Option<String>,
}

impl CreateChatRequest {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            user_id_type: None,
            set_bot_manager: None,
            uuid: None,
        }
    }

    /// 用户 ID 类型（查询参数，可选，默认 open_id）
    pub fn user_id_type(mut self, user_id_type: UserIdType) -> Self {
        self.user_id_type = Some(user_id_type);
        self
    }

    /// 是否同时设置创建群的机器人为管理员（查询参数，可选，默认 false）
    pub fn set_bot_manager(mut self, set_bot_manager: bool) -> Self {
        self.set_bot_manager = Some(set_bot_manager);
        self
    }

    /// 幂等 uuid（查询参数，可选）
    pub fn uuid(mut self, uuid: impl Into<String>) -> Self {
        self.uuid = Some(uuid.into());
        self
    }

    /// 执行请求
    ///
    /// 说明：创建群请求体字段较多，建议直接按文档构造 JSON 传入。
    ///
    /// docPath: https://open.feishu.cn/document/server-docs/group/chat/create
    pub async fn execute(self, body: serde_json::Value) -> SDKResult<serde_json::Value> {
        // url: POST:/open-apis/im/v1/chats
        let mut req: ApiRequest<serde_json::Value> =
            ApiRequest::post(IM_V1_CHATS).body(serialize_params(&body, "创建群")?);

        if let Some(user_id_type) = self.user_id_type {
            req = req.query("user_id_type", user_id_type.as_str());
        }
        if let Some(set_bot_manager) = self.set_bot_manager {
            req = req.query("set_bot_manager", set_bot_manager.to_string());
        }
        if let Some(uuid) = self.uuid {
            req = req.query("uuid", uuid);
        }

        let resp = Transport::request(req, &self.config, None).await?;
        extract_response_data(resp, "创建群")
    }
}

//! 添加消息表情回复
//!
//! docPath: https://open.feishu.cn/document/server-docs/im-v1/message-reaction/create

use openlark_core::{
    api::ApiRequest, config::Config, http::Transport, validate_required, SDKResult,
};

use crate::{
    common::api_utils::{extract_response_data, serialize_params},
    endpoints::IM_V1_MESSAGES,
    im::im::v1::message::reaction::models::{CreateMessageReactionBody, MessageReaction},
};

/// 添加消息表情回复请求
pub struct CreateMessageReactionRequest {
    config: Config,
    message_id: String,
}

impl CreateMessageReactionRequest {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            message_id: String::new(),
        }
    }

    /// 消息 ID（路径参数）
    pub fn message_id(mut self, message_id: impl Into<String>) -> Self {
        self.message_id = message_id.into();
        self
    }

    /// 执行请求
    ///
    /// docPath: https://open.feishu.cn/document/server-docs/im-v1/message-reaction/create
    pub async fn execute(self, body: CreateMessageReactionBody) -> SDKResult<MessageReaction> {
        self.execute_with_options(body, openlark_core::req_option::RequestOption::default())
            .await
    }

    pub async fn execute_with_options(
        self,
        body: CreateMessageReactionBody,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<MessageReaction> {
        validate_required!(self.message_id, "message_id 不能为空");
        validate_required!(
            body.reaction_type.emoji_type,
            "reaction_type.emoji_type 不能为空"
        );

        // url: POST:/open-apis/im/v1/messages/:message_id/reactions
        let req: ApiRequest<MessageReaction> =
            ApiRequest::post(format!("{}/{}/reactions", IM_V1_MESSAGES, self.message_id))
                .body(serialize_params(&body, "添加消息表情回复")?);

        let resp = Transport::request(req, &self.config, Some(option)).await?;

        extract_response_data(resp, "添加消息表情回复")
    }
}

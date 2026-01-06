//! 更新已发送的消息卡片
//!
//! docPath: https://open.feishu.cn/document/server-docs/im-v1/message-card/patch

use openlark_core::{
    api::ApiRequest, config::Config, http::Transport, validate_required, SDKResult,
};

use crate::{
    common::api_utils::{extract_response_data, serialize_params},
    common::models::EmptyData,
    endpoints::IM_V1_MESSAGES,
};

/// 更新已发送的消息卡片请求
pub struct PatchMessageCardRequest {
    config: Config,
    message_id: String,
}

impl PatchMessageCardRequest {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            message_id: String::new(),
        }
    }

    /// 待更新的消息 ID（路径参数）
    pub fn message_id(mut self, message_id: impl Into<String>) -> Self {
        self.message_id = message_id.into();
        self
    }

    /// 执行请求
    ///
    /// 说明：该接口支持两种请求体形态：
    /// 1) `{\"content\": \"...\"}` 传入卡片 JSON（需为 JSON 序列化后的字符串）
    /// 2) `{\"type\":\"template\",\"data\":{...}}` 传入卡片模板（搭建工具）
    ///
    /// docPath: https://open.feishu.cn/document/server-docs/im-v1/message-card/patch
    pub async fn execute(self, body: serde_json::Value) -> SDKResult<EmptyData> {
        validate_required!(self.message_id, "message_id 不能为空");

        // url: PATCH:/open-apis/im/v1/messages/:message_id
        let req: ApiRequest<EmptyData> =
            ApiRequest::patch(format!("{}/{}", IM_V1_MESSAGES, self.message_id))
                .body(serialize_params(&body, "更新已发送的消息卡片")?);

        let resp = Transport::request(req, &self.config, None).await?;
        extract_response_data(resp, "更新已发送的消息卡片")
    }
}

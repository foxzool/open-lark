//! 添加跟随气泡
//!
//! docPath: https://open.feishu.cn/document/im-v1/message/push_follow_up

use openlark_core::{api::ApiRequest, config::Config, error, http::Transport, validate_required, SDKResult};
use serde::{Deserialize, Serialize};

use crate::{
    common::{
        api_utils::{extract_response_data, serialize_params},
        models::EmptyData,
    },
    endpoints::IM_V1_MESSAGES,
};

/// 跟随气泡多语言内容
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FollowUpI18nContent {
    pub content: String,
    pub language: String,
}

/// 跟随气泡
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FollowUp {
    pub content: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub i18n_contents: Option<Vec<FollowUpI18nContent>>,
}

/// 添加跟随气泡请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PushFollowUpBody {
    pub follow_ups: Vec<FollowUp>,
}

/// 添加跟随气泡请求
pub struct PushFollowUpRequest {
    config: Config,
    message_id: String,
}

impl PushFollowUpRequest {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            message_id: String::new(),
        }
    }

    /// 机器人发送的消息 ID（路径参数）
    pub fn message_id(mut self, message_id: impl Into<String>) -> Self {
        self.message_id = message_id.into();
        self
    }

    /// 执行请求
    ///
    /// docPath: https://open.feishu.cn/document/im-v1/message/push_follow_up
    pub async fn execute(self, body: PushFollowUpBody) -> SDKResult<EmptyData> {
        validate_required!(self.message_id, "message_id 不能为空");
        if body.follow_ups.is_empty() {
            return Err(error::validation_error(
                "follow_ups 不能为空".to_string(),
                "跟随气泡列表至少包含 1 项".to_string(),
            ));
        }

        // url: POST:/open-apis/im/v1/messages/:message_id/push_follow_up
        let req: ApiRequest<EmptyData> = ApiRequest::post(format!(
            "{}/{}/push_follow_up",
            IM_V1_MESSAGES, self.message_id
        ))
        .body(serialize_params(&body, "添加跟随气泡")?);

        let resp = Transport::request(req, &self.config, None).await?;
        extract_response_data(resp, "添加跟随气泡")
    }
}


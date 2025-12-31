//! 更新群置顶
//!
//! docPath: https://open.feishu.cn/document/server-docs/group/chat/put_top_notice

use openlark_core::{api::ApiRequest, config::Config, http::Transport, validate_required, SDKResult};

use crate::{
    common::{
        api_utils::{extract_response_data, serialize_params},
        models::EmptyData,
    },
    endpoints::IM_V1_CHATS,
};

/// 更新群置顶请求
pub struct PutTopNoticeRequest {
    config: Config,
    chat_id: String,
}

impl PutTopNoticeRequest {
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
    /// docPath: https://open.feishu.cn/document/server-docs/group/chat/put_top_notice
    pub async fn execute(self, body: serde_json::Value) -> SDKResult<EmptyData> {
        validate_required!(self.chat_id, "chat_id 不能为空");

        // url: POST:/open-apis/im/v1/chats/:chat_id/top_notice/put_top_notice
        let req: ApiRequest<EmptyData> = ApiRequest::post(format!(
            "{}/{}/top_notice/put_top_notice",
            IM_V1_CHATS, self.chat_id
        ))
        .body(serialize_params(&body, "更新群置顶")?);

        let resp = Transport::request(req, &self.config, None).await?;
        extract_response_data(resp, "更新群置顶")
    }
}


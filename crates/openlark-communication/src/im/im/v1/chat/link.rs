//! 获取群分享链接
//!
//! docPath: https://open.feishu.cn/document/server-docs/group/chat/link

use openlark_core::{
    api::ApiRequest, config::Config, http::Transport, validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::{
    common::api_utils::{extract_response_data, serialize_params},
    endpoints::IM_V1_CHATS,
    im::im::v1::chat::models::{ChatLinkValidityPeriod, GetChatLinkResponse},
};

/// 获取群分享链接请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetChatLinkBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub validity_period: Option<String>,
}

/// 获取群分享链接请求
pub struct GetChatLinkRequest {
    config: Config,
    chat_id: String,
    validity_period: Option<ChatLinkValidityPeriod>,
}

impl GetChatLinkRequest {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            chat_id: String::new(),
            validity_period: None,
        }
    }

    /// 群 ID（路径参数）
    pub fn chat_id(mut self, chat_id: impl Into<String>) -> Self {
        self.chat_id = chat_id.into();
        self
    }

    /// 群分享链接有效期（请求体，可选，默认 week）
    pub fn validity_period(mut self, validity_period: ChatLinkValidityPeriod) -> Self {
        self.validity_period = Some(validity_period);
        self
    }

    /// 执行请求
    ///
    /// docPath: https://open.feishu.cn/document/server-docs/group/chat/link
    pub async fn execute(self) -> SDKResult<GetChatLinkResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<GetChatLinkResponse> {
        validate_required!(self.chat_id, "chat_id 不能为空");

        let body = GetChatLinkBody {
            validity_period: self.validity_period.map(|v| v.as_str().to_string()),
        };

        // url: POST:/open-apis/im/v1/chats/:chat_id/link
        let req: ApiRequest<GetChatLinkResponse> =
            ApiRequest::post(format!("{}/{}/link", IM_V1_CHATS, self.chat_id))
                .body(serialize_params(&body, "获取群分享链接")?);

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        extract_response_data(resp, "获取群分享链接")
    }
}

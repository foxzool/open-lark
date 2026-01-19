//! 撤销群置顶
//!
//! docPath: https://open.feishu.cn/document/server-docs/group/chat/delete_top_notice

use openlark_core::{
    api::ApiRequest, config::Config, http::Transport, validate_required, SDKResult,
};

use crate::{
    common::{api_utils::extract_response_data, models::EmptyData},
    endpoints::IM_V1_CHATS,
};

/// 撤销群置顶请求
pub struct DeleteTopNoticeRequest {
    config: Config,
    chat_id: String,
}

impl DeleteTopNoticeRequest {
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
    /// docPath: https://open.feishu.cn/document/server-docs/group/chat/delete_top_notice
    pub async fn execute(self) -> SDKResult<EmptyData> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<EmptyData> {
        validate_required!(self.chat_id, "chat_id 不能为空");

        // url: POST:/open-apis/im/v1/chats/:chat_id/top_notice/delete_top_notice
        let req: ApiRequest<EmptyData> = ApiRequest::post(format!(
            "{}/{}/top_notice/delete_top_notice",
            IM_V1_CHATS, self.chat_id
        ));

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        extract_response_data(resp, "撤销群置顶")
    }
}

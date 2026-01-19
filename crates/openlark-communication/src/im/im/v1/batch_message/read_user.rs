//! 查询批量消息推送和阅读人数
//!
//! docPath: https://open.feishu.cn/document/server-docs/im-v1/batch_message/read_user

use openlark_core::{
    api::ApiRequest, config::Config, http::Transport, validate_required, SDKResult,
};

use crate::{
    common::api_utils::extract_response_data, endpoints::IM_V1_BATCH_MESSAGES,
    im::im::v1::batch_message::models::BatchMessageReadUserResponse,
};

/// 查询批量消息推送和阅读人数请求
pub struct GetBatchMessageReadUserRequest {
    config: Config,
    batch_message_id: String,
}

impl GetBatchMessageReadUserRequest {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            batch_message_id: String::new(),
        }
    }

    /// 批量消息任务 ID（路径参数）
    pub fn batch_message_id(mut self, batch_message_id: impl Into<String>) -> Self {
        self.batch_message_id = batch_message_id.into();
        self
    }

    /// 执行请求
    ///
    /// docPath: https://open.feishu.cn/document/server-docs/im-v1/batch_message/read_user
    pub async fn execute(self) -> SDKResult<BatchMessageReadUserResponse> {
        self.execute_with_options(openlark_core::req_option::RequestOption::default())
            .await
    }

    pub async fn execute_with_options(
        self,
        option: openlark_core::req_option::RequestOption,
    ) -> SDKResult<BatchMessageReadUserResponse> {
        validate_required!(self.batch_message_id, "batch_message_id 不能为空");

        // url: GET:/open-apis/im/v1/batch_messages/:batch_message_id/read_user
        let req: ApiRequest<BatchMessageReadUserResponse> = ApiRequest::get(format!(
            "{}/{}/read_user",
            IM_V1_BATCH_MESSAGES, self.batch_message_id
        ));

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        extract_response_data(resp, "查询批量消息推送和阅读人数")
    }
}

//! 查询批量消息整体进度
//!
//! docPath: https://open.feishu.cn/document/server-docs/im-v1/batch_message/get_progress

use openlark_core::{api::ApiRequest, config::Config, http::Transport, validate_required, SDKResult};

use crate::{
    common::api_utils::extract_response_data,
    endpoints::IM_V1_BATCH_MESSAGES,
};

/// 查询批量消息整体进度请求
pub struct GetBatchMessageProgressRequest {
    config: Config,
    batch_message_id: String,
}

impl GetBatchMessageProgressRequest {
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
    /// docPath: https://open.feishu.cn/document/server-docs/im-v1/batch_message/get_progress
    pub async fn execute(self) -> SDKResult<serde_json::Value> {
        validate_required!(self.batch_message_id, "batch_message_id 不能为空");

        // url: GET:/open-apis/im/v1/batch_messages/:batch_message_id/get_progress
        let req: ApiRequest<serde_json::Value> = ApiRequest::get(format!(
            "{}/{}/get_progress",
            IM_V1_BATCH_MESSAGES, self.batch_message_id
        ));

        let resp = Transport::request(req, &self.config, None).await?;
        extract_response_data(resp, "查询批量消息整体进度")
    }
}


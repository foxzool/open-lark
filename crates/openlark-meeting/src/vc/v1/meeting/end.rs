//! 结束会议
//!
//! docPath: https://open.feishu.cn/document/server-docs/vc-v1/meeting/end

use openlark_core::{
    api::ApiRequest, config::Config, http::Transport, validate_required, SDKResult,
};

use crate::common::api_utils::{extract_response_data, serialize_params};
use crate::endpoints::VC_V1_MEETINGS;

/// 结束会议请求
pub struct EndMeetingRequest {
    config: Config,
    meeting_id: String,
}

impl EndMeetingRequest {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            meeting_id: String::new(),
        }
    }

    /// 会议 ID（路径参数）
    pub fn meeting_id(mut self, meeting_id: impl Into<String>) -> Self {
        self.meeting_id = meeting_id.into();
        self
    }

    /// 执行请求
    ///
    /// 说明：如文档不要求请求体，可传入空对象 `{}`。
    ///
    /// docPath: https://open.feishu.cn/document/server-docs/vc-v1/meeting/end
    pub async fn execute(self, body: serde_json::Value) -> SDKResult<serde_json::Value> {
        validate_required!(self.meeting_id, "meeting_id 不能为空");

        // url: PATCH:/open-apis/vc/v1/meetings/:meeting_id/end
        let req: ApiRequest<serde_json::Value> =
            ApiRequest::patch(format!("{}/{}/end", VC_V1_MEETINGS, self.meeting_id))
                .body(serialize_params(&body, "结束会议")?);

        let resp = Transport::request(req, &self.config, None).await?;
        extract_response_data(resp, "结束会议")
    }
}

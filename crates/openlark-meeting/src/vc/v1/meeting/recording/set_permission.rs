//! 授权录制文件
//!
//! docPath: https://open.feishu.cn/document/server-docs/vc-v1/meeting-recording/set_permission

use openlark_core::{
    api::ApiRequest, config::Config, http::Transport, validate_required, SDKResult,
};

use crate::common::api_utils::{extract_response_data, serialize_params};
use crate::endpoints::VC_V1_MEETINGS;

/// 授权录制文件请求
pub struct SetRecordingPermissionRequest {
    config: Config,
    meeting_id: String,
}

impl SetRecordingPermissionRequest {
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
    /// 说明：该接口请求体字段较多，建议直接按文档构造 JSON 传入。
    ///
    /// docPath: https://open.feishu.cn/document/server-docs/vc-v1/meeting-recording/set_permission
    pub async fn execute(self, body: serde_json::Value) -> SDKResult<serde_json::Value> {
        validate_required!(self.meeting_id, "meeting_id 不能为空");

        // url: PATCH:/open-apis/vc/v1/meetings/:meeting_id/recording/set_permission
        let req: ApiRequest<serde_json::Value> = ApiRequest::patch(format!(
            "{}/{}/recording/set_permission",
            VC_V1_MEETINGS, self.meeting_id
        ))
        .body(serialize_params(&body, "授权录制文件")?);

        let resp = Transport::request(req, &self.config, None).await?;
        extract_response_data(resp, "授权录制文件")
    }
}

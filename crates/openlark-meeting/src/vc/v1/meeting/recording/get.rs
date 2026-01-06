//! 获取录制文件
//!
//! docPath: https://open.feishu.cn/document/server-docs/vc-v1/meeting-recording/get

use openlark_core::{
    api::ApiRequest, config::Config, http::Transport, validate_required, SDKResult,
};

use crate::common::api_utils::extract_response_data;
use crate::endpoints::VC_V1_MEETINGS;

/// 获取录制文件请求
pub struct GetRecordingRequest {
    config: Config,
    meeting_id: String,
    query_params: Vec<(String, String)>,
}

impl GetRecordingRequest {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            meeting_id: String::new(),
            query_params: Vec::new(),
        }
    }

    /// 会议 ID（路径参数）
    pub fn meeting_id(mut self, meeting_id: impl Into<String>) -> Self {
        self.meeting_id = meeting_id.into();
        self
    }

    /// 追加查询参数
    pub fn query_param(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.query_params.push((key.into(), value.into()));
        self
    }

    /// 执行请求
    ///
    /// docPath: https://open.feishu.cn/document/server-docs/vc-v1/meeting-recording/get
    pub async fn execute(self) -> SDKResult<serde_json::Value> {
        validate_required!(self.meeting_id, "meeting_id 不能为空");

        // url: GET:/open-apis/vc/v1/meetings/:meeting_id/recording
        let mut req: ApiRequest<serde_json::Value> =
            ApiRequest::get(format!("{}/{}/recording", VC_V1_MEETINGS, self.meeting_id));
        for (k, v) in self.query_params {
            req = req.query(k, v);
        }

        let resp = Transport::request(req, &self.config, None).await?;
        extract_response_data(resp, "获取录制文件")
    }
}

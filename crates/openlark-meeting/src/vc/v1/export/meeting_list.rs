//! 导出会议明细
//!
//! docPath: https://open.feishu.cn/document/server-docs/vc-v1/export/meeting_list

use openlark_core::{api::ApiRequest, config::Config, http::Transport, SDKResult};

use crate::common::api_utils::{extract_response_data, serialize_params};
use crate::endpoints::VC_V1_EXPORTS;

/// 导出会议明细请求
pub struct ExportMeetingListRequest {
    config: Config,
}

impl ExportMeetingListRequest {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 执行请求
    ///
    /// 说明：该接口请求体字段较多，建议直接按文档构造 JSON 传入。
    ///
    /// docPath: https://open.feishu.cn/document/server-docs/vc-v1/export/meeting_list
    pub async fn execute(self, body: serde_json::Value) -> SDKResult<serde_json::Value> {
        // url: POST:/open-apis/vc/v1/exports/meeting_list
        let req: ApiRequest<serde_json::Value> =
            ApiRequest::post(format!("{}/meeting_list", VC_V1_EXPORTS))
                .body(serialize_params(&body, "导出会议明细")?);

        let resp = Transport::request(req, &self.config, None).await?;
        extract_response_data(resp, "导出会议明细")
    }
}

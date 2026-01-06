//! 导出参会人明细
//!
//! docPath: https://open.feishu.cn/document/server-docs/vc-v1/export/participant_list

use openlark_core::{api::ApiRequest, config::Config, http::Transport, SDKResult};

use crate::common::api_utils::{extract_response_data, serialize_params};
use crate::endpoints::VC_V1_EXPORTS;

/// 导出参会人明细请求
pub struct ExportParticipantListRequest {
    config: Config,
}

impl ExportParticipantListRequest {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 执行请求
    ///
    /// 说明：该接口请求体字段较多，建议直接按文档构造 JSON 传入。
    ///
    /// docPath: https://open.feishu.cn/document/server-docs/vc-v1/export/participant_list
    pub async fn execute(self, body: serde_json::Value) -> SDKResult<serde_json::Value> {
        // url: POST:/open-apis/vc/v1/exports/participant_list
        let req: ApiRequest<serde_json::Value> =
            ApiRequest::post(format!("{}/participant_list", VC_V1_EXPORTS))
                .body(serialize_params(&body, "导出参会人明细")?);

        let resp = Transport::request(req, &self.config, None).await?;
        extract_response_data(resp, "导出参会人明细")
    }
}

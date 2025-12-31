//! 获取与会议号关联的会议列表
//!
//! docPath: https://open.feishu.cn/document/server-docs/vc-v1/meeting/list_by_no

use openlark_core::{api::ApiRequest, config::Config, http::Transport, SDKResult};

use crate::{common::api_utils::extract_response_data, endpoints::VC_V1_MEETINGS};

/// 获取与会议号关联的会议列表请求
pub struct ListByNoMeetingRequest {
    config: Config,
    query_params: Vec<(String, String)>,
}

impl ListByNoMeetingRequest {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            query_params: Vec::new(),
        }
    }

    /// 追加查询参数
    pub fn query_param(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.query_params.push((key.into(), value.into()));
        self
    }

    /// 执行请求
    ///
    /// docPath: https://open.feishu.cn/document/server-docs/vc-v1/meeting/list_by_no
    pub async fn execute(self) -> SDKResult<serde_json::Value> {
        // url: GET:/open-apis/vc/v1/meetings/list_by_no
        let mut req: ApiRequest<serde_json::Value> = ApiRequest::get(format!("{}/list_by_no", VC_V1_MEETINGS));
        for (k, v) in self.query_params {
            req = req.query(k, v);
        }

        let resp = Transport::request(req, &self.config, None).await?;
        extract_response_data(resp, "获取与会议号关联的会议列表")
    }
}


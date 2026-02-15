//! 移除规则看板

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    SDKResult,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Debug, Clone)]
pub struct RemoveReportRuleViewRequest {
    config: Arc<Config>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemoveReportRuleViewResponse {
    pub data: Option<serde_json::Value>,
}

impl ApiResponseTrait for RemoveReportRuleViewResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl RemoveReportRuleViewRequest {
    pub fn new(config: Arc<Config>) -> Self {
        Self { config }
    }

    pub async fn execute(self) -> SDKResult<RemoveReportRuleViewResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<RemoveReportRuleViewResponse> {
        let path = "/open-apis/report/v1/rules/{}/views/remove".to_string();
        let req: ApiRequest<RemoveReportRuleViewResponse> = ApiRequest::post(&path);

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        resp.data.ok_or_else(|| {
            openlark_core::error::validation_error("移除规则看板", "响应数据为空")
        })
    }
}

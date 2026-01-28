//! 拒绝人工任务 API

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};

pub struct RejectTaskBuilder {
    approval_task_id: String,
    reason: Option<String>,
    config: Config,
}

impl RejectTaskBuilder {
    pub fn new(config: Config) -> Self {
        Self {
            approval_task_id: String::new(),
            reason: None,
            config,
        }
    }

    pub fn approval_task_id(mut self, approval_task_id: impl Into<String>) -> Self {
        self.approval_task_id = approval_task_id.into();
        self
    }

    pub fn reason(mut self, reason: impl Into<String>) -> Self {
        self.reason = Some(reason.into());
        self
    }

    pub async fn execute(self) -> SDKResult<RejectTaskResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    pub async fn execute_with_options(self, option: RequestOption) -> SDKResult<RejectTaskResponse> {
        validate_required!(self.approval_task_id, "任务ID不能为空");

        let url = format!("/open-apis/apaas/v1/approval_tasks/{}/reject", self.approval_task_id);
        let request_body = RejectTaskRequest { reason: self.reason };
        let api_request: ApiRequest<RejectTaskResponse> =
            ApiRequest::post(url).body(serde_json::to_value(&request_body)?);

        let response = Transport::request(api_request, &self.config, Some(option)).await?;
        response.data.ok_or_else(|| {
            openlark_core::error::validation_error("拒绝人工任务", "响应数据为空")
        })
    }
}

#[derive(Debug, Serialize)]
struct RejectTaskRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    reason: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RejectTaskResponse {
    pub result: String,
}

impl ApiResponseTrait for RejectTaskResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

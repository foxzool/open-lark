//! 同意人工任务 API

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};

pub struct AgreeTaskBuilder {
    approval_task_id: String,
    config: Config,
}

impl AgreeTaskBuilder {
    pub fn new(config: Config) -> Self {
        Self {
            approval_task_id: String::new(),
            config,
        }
    }

    pub fn approval_task_id(mut self, approval_task_id: impl Into<String>) -> Self {
        self.approval_task_id = approval_task_id.into();
        self
    }

    pub async fn execute(self) -> SDKResult<AgreeTaskResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    pub async fn execute_with_options(self, option: RequestOption) -> SDKResult<AgreeTaskResponse> {
        validate_required!(self.approval_task_id, "任务ID不能为空");

        let url = format!(
            "/open-apis/apaas/v1/approval_tasks/{}/agree",
            self.approval_task_id
        );
        let api_request: ApiRequest<AgreeTaskResponse> = ApiRequest::post(url);

        let response = Transport::request(api_request, &self.config, Some(option)).await?;
        response
            .data
            .ok_or_else(|| openlark_core::error::validation_error("同意人工任务", "响应数据为空"))
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AgreeTaskResponse {
    pub result: String,
}

impl ApiResponseTrait for AgreeTaskResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

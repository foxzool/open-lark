//! 撤销人工任务 API

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};

pub struct CancelTaskBuilder {
    approval_task_id: String,
    config: Config,
}

impl CancelTaskBuilder {
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

    pub async fn execute(self) -> SDKResult<CancelTaskResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<CancelTaskResponse> {
        validate_required!(self.approval_task_id, "任务ID不能为空");

        let url = format!(
            "/open-apis/apaas/v1/approval_tasks/{}/cancel",
            self.approval_task_id
        );
        let api_request: ApiRequest<CancelTaskResponse> = ApiRequest::post(url);

        let response = Transport::request(api_request, &self.config, Some(option)).await?;
        response
            .data
            .ok_or_else(|| openlark_core::error::validation_error("撤销人工任务", "响应数据为空"))
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CancelTaskResponse {
    pub result: String,
}

impl ApiResponseTrait for CancelTaskResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

//! 撤销人工任务 API

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};

pub struct CancelInstanceBuilder {
    approval_instance_id: String,
    config: Config,
}

impl CancelInstanceBuilder {
    pub fn new(config: Config) -> Self {
        Self {
            approval_instance_id: String::new(),
            config,
        }
    }

    pub fn approval_instance_id(mut self, approval_instance_id: impl Into<String>) -> Self {
        self.approval_instance_id = approval_instance_id.into();
        self
    }

    pub async fn execute(self) -> SDKResult<CancelInstanceResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<CancelInstanceResponse> {
        validate_required!(self.approval_instance_id, "实例ID不能为空");

        let url = format!(
            "/open-apis/apaas/v1/approval_instances/{}/cancel",
            self.approval_instance_id
        );
        let api_request: ApiRequest<CancelInstanceResponse> = ApiRequest::post(url);

        let response = Transport::request(api_request, &self.config, Some(option)).await?;
        response
            .data
            .ok_or_else(|| openlark_core::error::validation_error("撤销人工任务", "响应数据为空"))
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CancelInstanceResponse {
    pub result: String,
}

impl ApiResponseTrait for CancelInstanceResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

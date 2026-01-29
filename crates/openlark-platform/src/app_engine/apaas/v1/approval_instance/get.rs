//! 获取人工任务详情 API

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};

pub struct GetInstanceBuilder {
    approval_instance_id: String,
    config: Config,
}

impl GetInstanceBuilder {
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

    pub async fn execute(self) -> SDKResult<GetInstanceResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<GetInstanceResponse> {
        validate_required!(self.approval_instance_id, "实例ID不能为空");

        let url = format!(
            "/open-apis/apaas/v1/approval_instances/{}",
            self.approval_instance_id
        );
        let api_request: ApiRequest<GetInstanceResponse> = ApiRequest::get(url);

        let response = Transport::request(api_request, &self.config, Some(option)).await?;
        response.data.ok_or_else(|| {
            openlark_core::error::validation_error("获取人工任务详情", "响应数据为空")
        })
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct GetInstanceResponse {
    pub instance_id: String,
    pub status: String,
    pub initiator_id: String,
    pub create_time: String,
}

impl ApiResponseTrait for GetInstanceResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

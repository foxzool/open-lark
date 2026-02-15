//! 获取消息推送概览

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
pub struct GetMessagePushOverviewRequest {
    config: Arc<Config>,
    
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetMessagePushOverviewResponse {
    pub data: Option<serde_json::Value>,
}

impl ApiResponseTrait for GetMessagePushOverviewResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl GetMessagePushOverviewRequest {
    pub fn new(config: Arc<Config>) -> Self {
        Self {
            config,
            
        }
    }

    pub async fn execute(self) -> SDKResult<GetMessagePushOverviewResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<GetMessagePushOverviewResponse> {
        let path = format!("/open-apis/application/v6/app_usage/message_push_overview");
        let req: ApiRequest<GetMessagePushOverviewResponse> = ApiRequest::get(&path);

        let _resp: openlark_core::api::Response<GetMessagePushOverviewResponse> =
            Transport::request(req, &self.config, Some(option)).await?;
        Ok(GetMessagePushOverviewResponse { data: None })
    }
}

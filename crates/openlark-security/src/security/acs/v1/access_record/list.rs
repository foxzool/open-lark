//! 获取门禁记录列表

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
pub struct ListAccessRecordsRequest {
    config: Arc<Config>,
    
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListAccessRecordsResponse {
    pub data: Option<serde_json::Value>,
}

impl ApiResponseTrait for ListAccessRecordsResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl ListAccessRecordsRequest {
    pub fn new(config: Arc<Config>) -> Self {
        Self {
            config,
            
        }
    }

    pub async fn execute(self) -> SDKResult<ListAccessRecordsResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<ListAccessRecordsResponse> {
        let path = format!("/open-apis/acs/v1/access_records");
        let req: ApiRequest<ListAccessRecordsResponse> = ApiRequest::get(&path);

        let _resp: openlark_core::api::Response<ListAccessRecordsResponse> =
            Transport::request(req, &self.config, Some(option)).await?;
        Ok(ListAccessRecordsResponse { data: None })
    }
}

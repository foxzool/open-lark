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
pub struct ListAccessRecordRequest {
    config: Arc<Config>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListAccessRecordResponse {
    pub data: Option<AccessRecordListData>,
}

impl ApiResponseTrait for ListAccessRecordResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessRecordListData {
    pub records: Vec<AccessRecord>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccessRecord {
    pub record_id: String,
    pub user_id: String,
    pub device_id: String,
    pub access_time: String,
}

impl ListAccessRecordRequest {
    pub fn new(config: Arc<Config>) -> Self {
        Self { config }
    }

    pub async fn execute(self) -> SDKResult<ListAccessRecordResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<ListAccessRecordResponse> {
        let path = "/open-apis/acs/v1/access_records";
        let req: ApiRequest<ListAccessRecordResponse> = ApiRequest::get(path);

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        resp.data.ok_or_else(|| {
            openlark_core::error::validation_error("获取门禁记录列表", "响应数据为空")
        })
    }
}

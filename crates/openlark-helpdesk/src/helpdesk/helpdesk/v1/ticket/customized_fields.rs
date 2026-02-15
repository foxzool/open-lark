//! 获取服务台自定义字段

use crate::common::{api_endpoints::HelpdeskApiV1, api_utils::*};
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
pub struct GetCustomizedFieldsRequest {
    config: Arc<Config>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetCustomizedFieldsResponse {
    pub data: Option<GetCustomizedFieldsData>,
}

impl ApiResponseTrait for GetCustomizedFieldsResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetCustomizedFieldsData {
    pub fields: Vec<CustomizedField>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomizedField {
    pub field_id: String,
    pub field_name: String,
    pub field_type: String,
}

impl GetCustomizedFieldsRequest {
    pub fn new(config: Arc<Config>) -> Self {
        Self { config }
    }

    pub async fn execute(self) -> SDKResult<GetCustomizedFieldsResponse> {
        self.execute_with_options(RequestOption::default()).await
    }

    pub async fn execute_with_options(
        self,
        option: RequestOption,
    ) -> SDKResult<GetCustomizedFieldsResponse> {
        let path = HelpdeskApiV1::TicketCustomizedFields.to_url();
        let req: ApiRequest<GetCustomizedFieldsResponse> = ApiRequest::get(&path);

        let resp = Transport::request(req, &self.config, Some(option)).await?;
        resp.data.ok_or_else(|| {
            openlark_core::error::validation_error("获取服务台自定义字段", "响应数据为空")
        })
    }
}

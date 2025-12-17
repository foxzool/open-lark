//! 读取多个范围
//!
//! docPath: https://open.feishu.cn/document/server-docs/docs/sheets-v3/data-operation/reading-multiple-ranges

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, Response, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::api_endpoints::CcmSheetApiOld;

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct BatchGetValuesRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ranges: Option<String>, // Query param: ranges=Range1,Range2
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valueRenderOption: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dateTimeRenderOption: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct BatchGetValuesResponse {
    pub revision: i32,
    pub spreadsheetToken: String,
    pub valueRanges: Vec<ValueRange>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct ValueRange {
    pub range: String,
    pub values: Vec<Vec<serde_json::Value>>,
}

impl ApiResponseTrait for BatchGetValuesResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 读取多个范围
pub async fn values_batch_get(
    spreadsheet_token: String,
    request: BatchGetValuesRequest,
    config: &Config,
    option: Option<openlark_core::req_option::RequestOption>,
) -> SDKResult<Response<BatchGetValuesResponse>> {
    let api_endpoint = CcmSheetApiOld::ValuesBatchGet(spreadsheet_token);
    let mut api_request: ApiRequest<BatchGetValuesResponse> =
        ApiRequest::get(&api_endpoint.to_url())
            .query_opt("ranges", request.ranges)
            .query_opt("valueRenderOption", request.valueRenderOption)
            .query_opt("dateTimeRenderOption", request.dateTimeRenderOption);

    if let Some(opt) = option {
        api_request = api_request.request_option(opt);
    }

    Transport::request(api_request, config, None).await
}

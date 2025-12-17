//! 读取单个范围
//!
//! docPath: https://open.feishu.cn/document/server-docs/docs/sheets-v3/data-operation/reading-a-single-range

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, Response, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::api_endpoints::CcmSheetApiOld;

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct GetValuesRangeRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valueRenderOption: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dateTimeRenderOption: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct GetValuesRangeResponse {
    pub revision: i32,
    pub spreadsheetToken: String,
    pub valueRange: ValueRange,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct ValueRange {
    pub range: String,
    pub values: Vec<Vec<serde_json::Value>>,
}

impl ApiResponseTrait for GetValuesRangeResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 读取单个范围
pub async fn values_range(
    spreadsheet_token: String,
    range: String,
    request: GetValuesRangeRequest,
    config: &Config,
    option: Option<openlark_core::req_option::RequestOption>,
) -> SDKResult<Response<GetValuesRangeResponse>> {
    let api_endpoint = CcmSheetApiOld::ValuesRange(spreadsheet_token, range);
    let mut api_request: ApiRequest<GetValuesRangeResponse> =
        ApiRequest::get(&api_endpoint.to_url())
            .query_opt("valueRenderOption", request.valueRenderOption)
            .query_opt("dateTimeRenderOption", request.dateTimeRenderOption);

    if let Some(opt) = option {
        api_request = api_request.request_option(opt);
    }

    Transport::request(api_request, config, None).await
}

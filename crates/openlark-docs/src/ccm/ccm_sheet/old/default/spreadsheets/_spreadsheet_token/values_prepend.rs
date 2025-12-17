//! 插入数据
//!
//! docPath: https://open.feishu.cn/document/server-docs/docs/sheets-v3/data-operation/prepend-data

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, Response, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::api_endpoints::CcmSheetApiOld;

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct ValuesPrependRequest {
    pub valueRange: ValueRange,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct ValueRange {
    pub range: String,
    pub values: Vec<Vec<serde_json::Value>>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct ValuesPrependResponse {
    pub spreadsheetToken: String,
    pub tableRange: String,
    pub revision: i32,
    pub updates: UpdateResult,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct UpdateResult {
    pub spreadsheetToken: String,
    pub updatedRange: String,
    pub updatedRows: i32,
    pub updatedColumns: i32,
    pub updatedCells: i32,
    pub revision: i32,
}

impl ApiResponseTrait for ValuesPrependResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 插入数据
pub async fn values_prepend(
    spreadsheet_token: String,
    request: ValuesPrependRequest,
    config: &Config,
    option: Option<openlark_core::req_option::RequestOption>,
) -> SDKResult<Response<ValuesPrependResponse>> {
    let api_endpoint = CcmSheetApiOld::ValuesPrepend(spreadsheet_token);
    let mut api_request: ApiRequest<ValuesPrependResponse> =
        ApiRequest::post(&api_endpoint.to_url()).body(serde_json::to_value(request)?);

    if let Some(opt) = option {
        api_request = api_request.request_option(opt);
    }

    Transport::request(api_request, config, None).await
}

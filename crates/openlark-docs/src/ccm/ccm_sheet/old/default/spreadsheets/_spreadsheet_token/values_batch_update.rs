//! 批量更新数据
//!
//! docPath: /document/ukTMukTMukTM/uEjMzUjLxIzM14SMyMTN
//! doc: https://open.feishu.cn/document/server-docs/docs/sheets-v3/data-operation/writing-multiple-ranges

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::api_utils::*;
use crate::common::api_endpoints::CcmSheetApiOld;

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct BatchUpdateValuesRequest {
    pub valueRanges: Vec<ValueRange>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct ValueRange {
    pub range: String,
    pub values: Vec<Vec<serde_json::Value>>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct BatchUpdateValuesResponse {
    pub spreadsheetToken: String,
    pub responses: Vec<UpdateResponse>,
    pub revision: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct UpdateResponse {
    pub spreadsheetToken: String,
    pub updatedRange: String,
    pub updatedRows: i32,
    pub updatedColumns: i32,
    pub updatedCells: i32,
}

impl ApiResponseTrait for BatchUpdateValuesResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 批量更新数据
pub async fn values_batch_update(
    spreadsheet_token: String,
    request: BatchUpdateValuesRequest,
    config: &Config,
    option: Option<openlark_core::req_option::RequestOption>,
) -> SDKResult<BatchUpdateValuesResponse> {
    let api_endpoint = CcmSheetApiOld::ValuesBatchUpdate(spreadsheet_token);
    let mut api_request: ApiRequest<BatchUpdateValuesResponse> =
        ApiRequest::post(&api_endpoint.to_url())
            .body(serialize_params(&request, "向多个范围写入数据")?);

    if let Some(opt) = option {
        api_request = api_request.request_option(opt);
    }

    let response = Transport::request(api_request, config, None).await?;
    extract_response_data(response, "向多个范围写入数据")
}

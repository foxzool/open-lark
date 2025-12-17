//! 插入行列
//!
//! docPath: https://open.feishu.cn/document/server-docs/docs/sheets-v3/sheet-rowcol/insert-rows-or-columns

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, Response, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::api_endpoints::CcmSheetApiOld;

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct InsertDimensionRangeRequest {
    pub dimension: DimensionInsert,
    pub inheritStyle: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct DimensionInsert {
    pub sheetId: String,
    pub majorDimension: String,
    pub startIndex: i32,
    pub endIndex: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct InsertDimensionRangeResponse {}

impl ApiResponseTrait for InsertDimensionRangeResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 插入行列
pub async fn insert_dimension_range(
    spreadsheet_token: String,
    request: InsertDimensionRangeRequest,
    config: &Config,
    option: Option<openlark_core::req_option::RequestOption>,
) -> SDKResult<Response<InsertDimensionRangeResponse>> {
    let api_endpoint = CcmSheetApiOld::InsertDimensionRange(spreadsheet_token);
    let mut api_request: ApiRequest<InsertDimensionRangeResponse> =
        ApiRequest::post(&api_endpoint.to_url()).body(serde_json::to_value(request)?);

    if let Some(opt) = option {
        api_request = api_request.request_option(opt);
    }

    Transport::request(api_request, config, None).await
}

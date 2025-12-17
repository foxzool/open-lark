//! 批量设置单元格样式
//!
//! docPath: https://open.feishu.cn/document/server-docs/docs/sheets-v3/data-operation/batch-set-cell-style

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, Response, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::api_endpoints::CcmSheetApiOld;

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct BatchSetStyleRequest {
    pub data: Vec<BatchStyleData>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct BatchStyleData {
    pub ranges: Vec<String>,
    pub style: serde_json::Value,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct BatchSetStyleResponse {
    pub spreadsheetToken: String,
    pub totalUpdatedRows: i32,
    pub totalUpdatedColumns: i32,
    pub totalUpdatedCells: i32,
}

impl ApiResponseTrait for BatchSetStyleResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 批量设置单元格样式
pub async fn styles_batch_update(
    spreadsheet_token: String,
    request: BatchSetStyleRequest,
    config: &Config,
    option: Option<openlark_core::req_option::RequestOption>,
) -> SDKResult<Response<BatchSetStyleResponse>> {
    let api_endpoint = CcmSheetApiOld::StylesBatchUpdate(spreadsheet_token);
    let mut api_request: ApiRequest<BatchSetStyleResponse> =
        ApiRequest::put(&api_endpoint.to_url()).body(serde_json::to_value(request)?);

    if let Some(opt) = option {
        api_request = api_request.request_option(opt);
    }

    Transport::request(api_request, config, None).await
}

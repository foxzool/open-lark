//! 合并单元格
//!
//! docPath: https://open.feishu.cn/document/server-docs/docs/sheets-v3/data-operation/merge-cells

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, Response, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::api_endpoints::CcmSheetApiOld;

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct MergeCellsRequest {
    pub ranges: Vec<String>,
    pub mergeType: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct MergeCellsResponse {
    pub spreadsheetToken: String,
}

impl ApiResponseTrait for MergeCellsResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 合并单元格
pub async fn merge_cells(
    spreadsheet_token: String,
    request: MergeCellsRequest,
    config: &Config,
    option: Option<openlark_core::req_option::RequestOption>,
) -> SDKResult<Response<MergeCellsResponse>> {
    let api_endpoint = CcmSheetApiOld::MergeCells(spreadsheet_token);
    let mut api_request: ApiRequest<MergeCellsResponse> =
        ApiRequest::post(&api_endpoint.to_url()).body(serde_json::to_value(request)?);

    if let Some(opt) = option {
        api_request = api_request.request_option(opt);
    }

    Transport::request(api_request, config, None).await
}

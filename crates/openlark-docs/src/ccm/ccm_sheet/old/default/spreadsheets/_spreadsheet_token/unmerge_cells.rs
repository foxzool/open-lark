//! 拆分单元格
//!
//! docPath: /document/ukTMukTMukTM/uATNzUjLwUzM14CM1MTN
//! doc: https://open.feishu.cn/document/server-docs/docs/sheets-v3/data-operation/split-cells

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
pub struct UnmergeCellsRequest {
    pub ranges: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct UnmergeCellsResponse {
    pub spreadsheetToken: String,
}

impl ApiResponseTrait for UnmergeCellsResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 拆分单元格
pub async fn unmerge_cells(
    spreadsheet_token: String,
    request: UnmergeCellsRequest,
    config: &Config,
    option: Option<openlark_core::req_option::RequestOption>,
) -> SDKResult<UnmergeCellsResponse> {
    let api_endpoint = CcmSheetApiOld::UnmergeCells(spreadsheet_token);
    let mut api_request: ApiRequest<UnmergeCellsResponse> =
        ApiRequest::post(&api_endpoint.to_url()).body(serialize_params(&request, "拆分单元格")?);

    if let Some(opt) = option {
        api_request = api_request.request_option(opt);
    }

    let response = Transport::request(api_request, config, None).await?;
    extract_response_data(response, "拆分单元格")
}

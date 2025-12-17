//! 获取保护范围
//!
//! docPath: https://open.feishu.cn/document/server-docs/docs/sheets-v3/protect-range/obtain-protection-scopes

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, Response, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::api_endpoints::CcmSheetApiOld;

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct BatchGetProtectedRangeRequest {
    pub protectIds: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct BatchGetProtectedRangeResponse {
    pub protectedRanges: Vec<ProtectedRange>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct ProtectedRange {
    pub protectId: String,
    pub dimension: Dimension,
    pub editors: Vec<i64>,
    pub lockInfo: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Dimension {
    pub sheetId: String,
    pub majorDimension: String,
    pub startIndex: i32,
    pub endIndex: i32,
}

impl ApiResponseTrait for BatchGetProtectedRangeResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 获取保护范围
pub async fn protected_range_batch_get(
    spreadsheet_token: String,
    request: BatchGetProtectedRangeRequest,
    config: &Config,
    option: Option<openlark_core::req_option::RequestOption>,
) -> SDKResult<Response<BatchGetProtectedRangeResponse>> {
    let api_endpoint = CcmSheetApiOld::ProtectedRangeBatchGet(spreadsheet_token);
    let mut api_request: ApiRequest<BatchGetProtectedRangeResponse> =
        ApiRequest::get(&api_endpoint.to_url()).query("protectIds", request.protectIds.join(","));

    if let Some(opt) = option {
        api_request = api_request.request_option(opt);
    }

    Transport::request(api_request, config, None).await
}

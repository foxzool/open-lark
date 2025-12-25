//! 修改保护范围
//!
//! docPath: /document/ukTMukTMukTM/uUTM5YjL1ETO24SNxkjN
//! doc: https://open.feishu.cn/document/server-docs/docs/sheets-v3/protect-range/modify-protection-scopes

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
pub struct BatchUpdateProtectedRangeRequest {
    pub requests: Vec<UpdateProtectedRangeRequest>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct UpdateProtectedRangeRequest {
    pub protectId: String,
    pub dimension: Option<Dimension>,
    pub editors: Option<Vec<i64>>,
    pub lockInfo: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Dimension {
    pub sheetId: String,
    pub majorDimension: String,
    pub startIndex: i32,
    pub endIndex: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct BatchUpdateProtectedRangeResponse {
    pub replies: Vec<ProtectedRangeReply>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct ProtectedRangeReply {
    pub protectId: String,
    pub dimension: Option<Dimension>,
    pub editors: Option<Vec<i64>>,
    pub lockInfo: Option<String>,
}

impl ApiResponseTrait for BatchUpdateProtectedRangeResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 修改保护范围
pub async fn protected_range_batch_update(
    spreadsheet_token: String,
    request: BatchUpdateProtectedRangeRequest,
    config: &Config,
    option: Option<openlark_core::req_option::RequestOption>,
) -> SDKResult<BatchUpdateProtectedRangeResponse> {
    let api_endpoint = CcmSheetApiOld::ProtectedRangeBatchUpdate(spreadsheet_token);
    let mut api_request: ApiRequest<BatchUpdateProtectedRangeResponse> =
        ApiRequest::post(&api_endpoint.to_url())
            .body(serialize_params(&request, "修改保护范围")?);

    if let Some(opt) = option {
        api_request = api_request.request_option(opt);
    }

    let response = Transport::request(api_request, config, None).await?;
    extract_response_data(response, "修改保护范围")
}

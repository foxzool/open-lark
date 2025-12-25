//! 增加保护范围
//!
//! docPath: /document/ukTMukTMukTM/ugDNzUjL4QzM14CO0MTN
//! doc: https://open.feishu.cn/document/server-docs/docs/sheets-v3/protect-range/add-locked-cells

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
pub struct AddProtectedDimensionRequest {
    pub addProtectedDimension: Vec<ProtectedDimension>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct ProtectedDimension {
    pub dimension: Dimension,
    pub editors: Option<Vec<i64>>, // User IDs
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
pub struct AddProtectedDimensionResponse {
    pub addProtectedDimension: Vec<ProtectedDimensionResult>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct ProtectedDimensionResult {
    pub dimension: Dimension,
    pub editors: Vec<i64>,
    pub protectId: String,
    pub lockInfo: String,
}

impl ApiResponseTrait for AddProtectedDimensionResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 增加保护范围
pub async fn protected_dimension(
    spreadsheet_token: String,
    request: AddProtectedDimensionRequest,
    config: &Config,
    option: Option<openlark_core::req_option::RequestOption>,
) -> SDKResult<AddProtectedDimensionResponse> {
    let api_endpoint = CcmSheetApiOld::ProtectedDimension(spreadsheet_token);
    let mut api_request: ApiRequest<AddProtectedDimensionResponse> =
        ApiRequest::post(&api_endpoint.to_url())
            .body(serialize_params(&request, "增加保护范围")?);

    if let Some(opt) = option {
        api_request = api_request.request_option(opt);
    }

    let response = Transport::request(api_request, config, None).await?;
    extract_response_data(response, "增加保护范围")
}

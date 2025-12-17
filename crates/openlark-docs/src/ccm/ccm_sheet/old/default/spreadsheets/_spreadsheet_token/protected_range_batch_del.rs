//! 删除保护范围
//!
//! docPath: https://open.feishu.cn/document/server-docs/docs/sheets-v3/protect-range/delete-protection-scopes

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, Response, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::api_endpoints::CcmSheetApiOld;

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct BatchDelProtectedRangeRequest {
    pub protectIds: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct BatchDelProtectedRangeResponse {
    pub delProtectIds: Vec<String>,
}

impl ApiResponseTrait for BatchDelProtectedRangeResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 删除保护范围
pub async fn protected_range_batch_del(
    spreadsheet_token: String,
    request: BatchDelProtectedRangeRequest,
    config: &Config,
    option: Option<openlark_core::req_option::RequestOption>,
) -> SDKResult<Response<BatchDelProtectedRangeResponse>> {
    let api_endpoint = CcmSheetApiOld::ProtectedRangeBatchDel(spreadsheet_token);
    let mut api_request: ApiRequest<BatchDelProtectedRangeResponse> =
        ApiRequest::delete(&api_endpoint.to_url()).body(serde_json::to_value(request)?);

    if let Some(opt) = option {
        api_request = api_request.request_option(opt);
    }

    Transport::request(api_request, config, None).await
}

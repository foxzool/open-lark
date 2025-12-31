//! 获取保护范围
//!
//! docPath: /document/ukTMukTMukTM/uQTM5YjL0ETO24CNxkjN

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::api_utils::*;

use crate::common::api_endpoints::CcmSheetApiOld;

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct BatchGetProtectedRangeRequest {
    pub protectIds: Vec<String>,
    /// 可选，默认 userId（建议 openId）（query 参数）
    #[serde(skip)]
    pub memberType: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct BatchGetProtectedRangeResponse {
    #[serde(default)]
    pub protectedRanges: Vec<ProtectedRange>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct ProtectedRange {
    pub protectId: String,
    pub sheetId: Option<String>,
    pub dimension: Option<Dimension>,
    pub editors: Option<Editors>,
    pub lockInfo: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Editors {
    #[serde(default)]
    pub users: Vec<Member>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Member {
    pub memberType: String,
    pub memberId: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Dimension {
    pub sheetId: Option<String>,
    pub majorDimension: Option<String>,
    pub startIndex: Option<i32>,
    pub endIndex: Option<i32>,
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
) -> SDKResult<BatchGetProtectedRangeResponse> {
    validate_required!(spreadsheet_token, "spreadsheet_token 不能为空");
    if request.protectIds.is_empty() {
        return Err(openlark_core::error::validation_error(
            "protectIds",
            "protectIds 不能为空",
        ));
    }
    if request.protectIds.len() > 5 {
        return Err(openlark_core::error::validation_error(
            "protectIds",
            "protectIds 单次最多 5 个",
        ));
    }
    if let Some(member_type) = request.memberType.as_deref() {
        if member_type != "userId" && member_type != "openId" && member_type != "unionId" {
            return Err(openlark_core::error::validation_error(
                "memberType",
                "memberType 必须为 userId、openId 或 unionId",
            ));
        }
    }

    let api_endpoint = CcmSheetApiOld::ProtectedRangeBatchGet(spreadsheet_token);
    let mut api_request: ApiRequest<BatchGetProtectedRangeResponse> =
        ApiRequest::get(&api_endpoint.to_url()).query("protectIds", request.protectIds.join(","));

    if let Some(member_type) = request.memberType.as_deref() {
        api_request = api_request.query("memberType", member_type);
    }

    if let Some(opt) = option {
        api_request = api_request.request_option(opt);
    }

    let response = Transport::request(api_request, config, None).await?;
    extract_response_data(response, "获取保护范围")
}

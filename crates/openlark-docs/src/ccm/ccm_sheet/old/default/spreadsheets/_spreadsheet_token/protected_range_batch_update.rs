//! 修改保护范围
//!
//! docPath: /document/ukTMukTMukTM/uUTM5YjL1ETO24SNxkjN

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required,
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
    pub editors: Option<EditorsDelta>,
    pub lockInfo: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Dimension {
    pub sheetId: String,
    pub majorDimension: Option<String>,
    pub startIndex: i32,
    pub endIndex: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct EditorsDelta {
    #[serde(default)]
    pub addEditors: Vec<Member>,
    #[serde(default)]
    pub delEditors: Vec<Member>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Member {
    pub memberType: Option<String>,
    pub memberId: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct BatchUpdateProtectedRangeResponse {
    #[serde(default)]
    pub replies: Vec<ProtectedRangeReply>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct ProtectedRangeReply {
    pub protectId: String,
    pub sheetId: Option<String>,
    pub dimension: Option<Dimension>,
    pub editors: Option<EditorsDelta>,
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
    validate_required!(spreadsheet_token, "spreadsheet_token 不能为空");
    if request.requests.is_empty() {
        return Err(openlark_core::error::validation_error(
            "requests",
            "requests 不能为空",
        ));
    }
    if request.requests.len() > 10 {
        return Err(openlark_core::error::validation_error(
            "requests",
            "requests 单次最多 10 个",
        ));
    }
    for (idx, req) in request.requests.iter().enumerate() {
        if req.protectId.trim().is_empty() {
            return Err(openlark_core::error::validation_error(
                "protectId",
                "protectId 不能为空",
            ));
        }
        if let Some(dimension) = req.dimension.as_ref() {
            if dimension.sheetId.trim().is_empty() {
                return Err(openlark_core::error::validation_error("sheetId", "sheetId 不能为空"));
            }
            if let Some(major_dimension) = dimension.majorDimension.as_deref() {
                if major_dimension != "ROWS" && major_dimension != "COLUMNS" {
                    return Err(openlark_core::error::validation_error(
                        "majorDimension",
                        "majorDimension 必须为 ROWS 或 COLUMNS",
                    ));
                }
            }
            // 注意：保护范围文档中 startIndex/endIndex 为 1-based 且包含端点
            if dimension.startIndex < 1 {
                return Err(openlark_core::error::validation_error(
                    "startIndex",
                    "startIndex 必须 >= 1",
                ));
            }
            if dimension.endIndex < dimension.startIndex {
                return Err(openlark_core::error::validation_error(
                    "endIndex",
                    "endIndex 必须 >= startIndex",
                ));
            }
            let len = dimension.endIndex - dimension.startIndex + 1;
            if len > 5000 {
                return Err(openlark_core::error::validation_error(
                    "endIndex",
                    "单次操作不超过 5000 行或列",
                ));
            }
        }

        if let Some(editors) = req.editors.as_ref() {
            for member in editors
                .addEditors
                .iter()
                .chain(editors.delEditors.iter())
            {
                if member.memberId.trim().is_empty() {
                    return Err(openlark_core::error::validation_error(
                        &format!("requests[{}].editors.memberId", idx),
                        "memberId 不能为空",
                    ));
                }
                if let Some(member_type) = member.memberType.as_deref() {
                    if member_type != "userId"
                        && member_type != "openId"
                        && member_type != "unionId"
                    {
                        return Err(openlark_core::error::validation_error(
                            &format!("requests[{}].editors.memberType", idx),
                            "memberType 必须为 userId、openId 或 unionId",
                        ));
                    }
                }
            }
        }
    }

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

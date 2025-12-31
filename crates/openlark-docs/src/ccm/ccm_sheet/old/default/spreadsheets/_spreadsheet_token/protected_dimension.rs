//! 增加保护范围
//!
//! docPath: /document/ukTMukTMukTM/ugDNzUjL4QzM14CO0MTN

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
pub struct AddProtectedDimensionRequest {
    pub addProtectedDimension: Vec<ProtectedDimension>,
    /// 当 `users` 字段传值时必填（query 参数）
    #[serde(skip)]
    pub user_id_type: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct ProtectedDimension {
    pub dimension: Dimension,
    /// 不推荐继续使用（历史字段），建议使用 `users`
    pub editors: Option<Vec<i64>>, // User IDs
    /// 用户标识列表，配合 query 参数 `user_id_type` 使用
    pub users: Option<Vec<String>>,
    pub lockInfo: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Dimension {
    pub sheetId: String,
    /// 可选：ROWS | COLUMNS，不传时默认 ROWS
    pub majorDimension: Option<String>,
    pub startIndex: i32,
    pub endIndex: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct AddProtectedDimensionResponse {
    #[serde(default)]
    pub addProtectedDimension: Vec<ProtectedDimensionResult>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct ProtectedDimensionResult {
    pub dimension: Dimension,
    /// 不推荐继续使用（历史字段），可能为空
    #[serde(default)]
    pub editors: Vec<i64>,
    /// 新字段：用户标识列表
    #[serde(default)]
    pub users: Vec<String>,
    pub protectId: String,
    pub lockInfo: Option<String>,
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
    validate_required!(spreadsheet_token, "spreadsheet_token 不能为空");
    if request.addProtectedDimension.is_empty() {
        return Err(openlark_core::error::validation_error(
            "addProtectedDimension",
            "addProtectedDimension 不能为空",
        ));
    }
    if request.addProtectedDimension.len() > 50 {
        return Err(openlark_core::error::validation_error(
            "addProtectedDimension",
            "addProtectedDimension 单次最多 50 个",
        ));
    }

    let mut need_user_id_type = false;
    for (idx, item) in request.addProtectedDimension.iter().enumerate() {
        if item.dimension.sheetId.trim().is_empty() {
            return Err(openlark_core::error::validation_error(
                "sheetId",
                "sheetId 不能为空",
            ));
        }

        if let Some(major_dimension) = item.dimension.majorDimension.as_deref() {
            if major_dimension != "ROWS" && major_dimension != "COLUMNS" {
                return Err(openlark_core::error::validation_error(
                    "majorDimension",
                    "majorDimension 必须为 ROWS 或 COLUMNS",
                ));
            }
        }

        // 注意：保护范围文档中 startIndex/endIndex 为 1-based 且包含端点
        if item.dimension.startIndex < 1 {
            return Err(openlark_core::error::validation_error(
                "startIndex",
                "startIndex 必须 >= 1",
            ));
        }
        if item.dimension.endIndex < item.dimension.startIndex {
            return Err(openlark_core::error::validation_error(
                "endIndex",
                "endIndex 必须 >= startIndex",
            ));
        }
        let len = item.dimension.endIndex - item.dimension.startIndex + 1;
        if len > 5000 {
            return Err(openlark_core::error::validation_error(
                "endIndex",
                "单次操作不超过 5000 行或列",
            ));
        }
        if let Some(users) = item.users.as_ref() {
            if !users.is_empty() {
                need_user_id_type = true;
                if users.iter().any(|u| u.trim().is_empty()) {
                    return Err(openlark_core::error::validation_error(
                        &format!("users[{}]", idx),
                        "users 不能包含空字符串",
                    ));
                }
            }
        }
    }
    if need_user_id_type {
        validate_required!(
            request.user_id_type.clone().unwrap_or_default(),
            "当传 users 时，user_id_type 不能为空"
        );
    }

    let api_endpoint = CcmSheetApiOld::ProtectedDimension(spreadsheet_token);
    let mut api_request: ApiRequest<AddProtectedDimensionResponse> =
        ApiRequest::post(&api_endpoint.to_url()).body(serialize_params(&request, "增加保护范围")?);

    if let Some(user_id_type) = request.user_id_type.as_deref() {
        api_request = api_request.query("user_id_type", user_id_type);
    }

    if let Some(opt) = option {
        api_request = api_request.request_option(opt);
    }

    let response = Transport::request(api_request, config, None).await?;
    extract_response_data(response, "增加保护范围")
}

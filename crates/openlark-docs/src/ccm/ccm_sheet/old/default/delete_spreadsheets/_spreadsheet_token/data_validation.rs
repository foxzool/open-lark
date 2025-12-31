//! 删除下拉列表设置
//!
//! docPath: https://open.feishu.cn/document/server-docs/docs/sheets-v3/datavalidation/delete-datavalidation

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
pub struct DeleteDataValidationRequest {
    pub dataValidationRanges: Vec<DataValidationRange>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct DataValidationRange {
    pub range: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct DeleteDataValidationResponse {
    #[serde(default)]
    pub rangeResults: Vec<RangeResult>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct RangeResult {
    pub range: String,
    pub msg: Option<String>,
    pub success: bool,
    pub updatedCells: i32,
}

impl ApiResponseTrait for DeleteDataValidationResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 删除下拉列表设置
pub async fn data_validation(
    spreadsheet_token: String,
    request: DeleteDataValidationRequest,
    config: &Config,
    option: Option<openlark_core::req_option::RequestOption>,
) -> SDKResult<DeleteDataValidationResponse> {
    validate_required!(spreadsheet_token, "spreadsheet_token 不能为空");
    if request.dataValidationRanges.is_empty() {
        return Err(openlark_core::error::validation_error(
            "dataValidationRanges",
            "dataValidationRanges 不能为空",
        ));
    }
    if request.dataValidationRanges.len() > 100 {
        return Err(openlark_core::error::validation_error(
            "dataValidationRanges",
            "dataValidationRanges 单次最多 100 个",
        ));
    }
    for (idx, r) in request.dataValidationRanges.iter().enumerate() {
        if r.range.trim().is_empty() {
            return Err(openlark_core::error::validation_error(
                &format!("dataValidationRanges[{}].range", idx),
                "range 不能为空",
            ));
        }
        if !r.range.contains('!') {
            return Err(openlark_core::error::validation_error(
                &format!("dataValidationRanges[{}].range", idx),
                "range 格式必须包含 sheetId（形如 <sheetId>!A1:A10）",
            ));
        }
    }

    let api_endpoint = CcmSheetApiOld::DataValidationDelete(spreadsheet_token);
    let mut api_request: ApiRequest<DeleteDataValidationResponse> =
        ApiRequest::delete(&api_endpoint.to_url())
            .body(serialize_params(&request, "删除下拉列表设置")?);

    if let Some(opt) = option {
        api_request = api_request.request_option(opt);
    }

    let response = Transport::request(api_request, config, None).await?;
    extract_response_data(response, "删除下拉列表设置")
}

//! 批量删除条件格式
//!
//! docPath: https://open.feishu.cn/document/server-docs/docs/sheets-v3/conditionformat/condition-format-delete

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required, SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::api_endpoints::CcmSheetApiOld;
use crate::common::api_utils::*;

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct BatchDeleteConditionFormatRequest {
    /// 要删除的条件格式 ID 列表，最多 10 个
    pub sheet_cf_ids: Vec<SheetConditionFormatDelete>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct SheetConditionFormatDelete {
    pub sheet_id: String,
    pub cf_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct BatchDeleteConditionFormatResponse {
    #[serde(default)]
    pub responses: Vec<ConditionFormatResponse>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct ConditionFormatResponse {
    pub sheet_id: String,
    pub cf_id: String,
    pub res_code: i32,
    pub res_msg: Option<String>,
}

impl ApiResponseTrait for BatchDeleteConditionFormatResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 批量删除条件格式
pub async fn batch_delete(
    spreadsheet_token: String,
    request: BatchDeleteConditionFormatRequest,
    config: &Config,
    option: Option<openlark_core::req_option::RequestOption>,
) -> SDKResult<BatchDeleteConditionFormatResponse> {
    validate_required!(spreadsheet_token, "spreadsheet_token 不能为空");
    if request.sheet_cf_ids.is_empty() {
        return Err(openlark_core::error::validation_error(
            "sheet_cf_ids",
            "sheet_cf_ids 不能为空",
        ));
    }
    if request.sheet_cf_ids.len() > 10 {
        return Err(openlark_core::error::validation_error(
            "sheet_cf_ids",
            "sheet_cf_ids 单次最多 10 个",
        ));
    }
    for (idx, item) in request.sheet_cf_ids.iter().enumerate() {
        if item.sheet_id.trim().is_empty() {
            return Err(openlark_core::error::validation_error(
                "sheet_id",
                "sheet_id 不能为空",
            ));
        }
        if item.cf_id.trim().is_empty() {
            return Err(openlark_core::error::validation_error(
                &format!("sheet_cf_ids[{}].cf_id", idx),
                "cf_id 不能为空",
            ));
        }
    }

    let api_endpoint = CcmSheetApiOld::ConditionFormatsBatchDelete(spreadsheet_token);
    let mut api_request: ApiRequest<BatchDeleteConditionFormatResponse> =
        ApiRequest::delete(&api_endpoint.to_url())
            .body(serialize_params(&request, "批量删除条件格式")?);

    if let Some(opt) = option {
        api_request = api_request.request_option(opt);
    }

    let response = Transport::request(api_request, config, None).await?;
    extract_response_data(response, "批量删除条件格式")
}

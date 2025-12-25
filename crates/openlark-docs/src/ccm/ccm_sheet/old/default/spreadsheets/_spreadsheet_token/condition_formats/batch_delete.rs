//! 批量删除条件格式
//!
//! docPath: /document/ukTMukTMukTM/uATMzUjLwEzM14CMxMTN/conditionformat/condition-format-delete
//! doc: https://open.feishu.cn/document/server-docs/docs/sheets-v3/conditionformat/condition-format-delete

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
pub struct BatchDeleteConditionFormatRequest {
    pub sheet_condition_formats: Vec<SheetConditionFormatDelete>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct SheetConditionFormatDelete {
    pub sheet_id: String,
    pub cf_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct BatchDeleteConditionFormatResponse {
    pub responses: Vec<ConditionFormatResponse>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct ConditionFormatResponse {
    pub sheet_id: String,
    pub cf_id: String,
    pub res_code: i32,
    pub res_msg: String,
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

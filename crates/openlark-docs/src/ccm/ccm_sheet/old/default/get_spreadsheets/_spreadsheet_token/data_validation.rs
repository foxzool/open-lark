//! 查询下拉列表设置
//!
//! docPath: https://open.feishu.cn/document/server-docs/docs/sheets-v3/datavalidation/query-datavalidation

use std::collections::HashMap;

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
pub struct QueryDataValidationRequest {
    pub range: String,
    pub dataValidationType: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct QueryDataValidationResponse {
    pub spreadsheetToken: String,
    pub sheetId: String,
    pub revision: i32,
    #[serde(default)]
    pub dataValidations: Vec<DataValidation>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct DataValidation {
    #[serde(default)]
    pub ranges: Vec<String>,
    pub dataValidationType: String,
    #[serde(default)]
    pub conditionValues: Vec<String>,
    pub options: Option<DataValidationOptions>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct DataValidationOptions {
    pub multipleValues: Option<bool>,
    pub highlightValidData: Option<bool>,
    pub colorValueMap: Option<HashMap<String, String>>,
}

impl ApiResponseTrait for QueryDataValidationResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 查询下拉列表设置
pub async fn data_validation(
    spreadsheet_token: String,
    request: QueryDataValidationRequest,
    config: &Config,
    option: Option<openlark_core::req_option::RequestOption>,
) -> SDKResult<QueryDataValidationResponse> {
    validate_required!(spreadsheet_token, "spreadsheet_token 不能为空");
    validate_required!(request.range, "range 不能为空");
    if !request.range.contains('!') {
        return Err(openlark_core::error::validation_error(
            "range",
            "range 格式必须包含 sheetId（形如 <sheetId>!A1:A10）",
        ));
    }
    if request.dataValidationType != "list" {
        return Err(openlark_core::error::validation_error(
            "dataValidationType",
            "dataValidationType 必须为 list",
        ));
    }

    let api_endpoint = CcmSheetApiOld::DataValidation(spreadsheet_token);
    let mut api_request: ApiRequest<QueryDataValidationResponse> =
        ApiRequest::get(&api_endpoint.to_url())
            .query("range", request.range)
            .query("dataValidationType", request.dataValidationType);

    if let Some(opt) = option {
        api_request = api_request.request_option(opt);
    }

    let response = Transport::request(api_request, config, None).await?;
    extract_response_data(response, "查询下拉列表设置")
}

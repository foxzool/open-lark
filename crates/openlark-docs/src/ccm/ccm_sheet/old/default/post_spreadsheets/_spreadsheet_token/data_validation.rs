//! 设置下拉列表
//!
//! docPath: https://open.feishu.cn/document/server-docs/docs/sheets-v3/datavalidation/set-dropdown

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
pub struct SetDataValidationRequest {
    pub range: String,
    pub dataValidationType: String,
    pub dataValidation: DataValidationSetting,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct DataValidationSetting {
    pub conditionValues: Vec<String>,
    pub options: Option<DataValidationOptions>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct DataValidationOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multipleValues: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub highlightValidData: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub colors: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct SetDataValidationResponse {}

impl ApiResponseTrait for SetDataValidationResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 设置下拉列表
pub async fn data_validation(
    spreadsheet_token: String,
    request: SetDataValidationRequest,
    config: &Config,
    option: Option<openlark_core::req_option::RequestOption>,
) -> SDKResult<SetDataValidationResponse> {
    validate_required!(spreadsheet_token, "spreadsheet_token 不能为空");
    if request.range.trim().is_empty() {
        return Err(openlark_core::error::validation_error(
            "range",
            "range 不能为空",
        ));
    }
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
    if request.dataValidation.conditionValues.is_empty() {
        return Err(openlark_core::error::validation_error(
            "conditionValues",
            "conditionValues 不能为空",
        ));
    }
    if request.dataValidation.conditionValues.len() > 500 {
        return Err(openlark_core::error::validation_error(
            "conditionValues",
            "conditionValues 最多 500 个",
        ));
    }
    for (idx, v) in request.dataValidation.conditionValues.iter().enumerate() {
        if v.contains(',') {
            return Err(openlark_core::error::validation_error(
                &format!("conditionValues[{}]", idx),
                "conditionValues 单个值不能包含逗号(,)",
            ));
        }
        if v.chars().count() > 100 {
            return Err(openlark_core::error::validation_error(
                &format!("conditionValues[{}]", idx),
                "conditionValues 单个值长度不可超过 100 字符",
            ));
        }
    }
    if let Some(opts) = request.dataValidation.options.as_ref() {
        if opts.highlightValidData == Some(true) {
            let colors = opts.colors.as_ref().ok_or_else(|| {
                openlark_core::error::validation_error(
                    "colors",
                    "highlightValidData 为 true 时 colors 不能为空",
                )
            })?;
            if colors.len() != request.dataValidation.conditionValues.len() {
                return Err(openlark_core::error::validation_error(
                    "colors",
                    "colors 数量必须与 conditionValues 数量一致",
                ));
            }
        }
    }

    let api_endpoint = CcmSheetApiOld::DataValidationCreate(spreadsheet_token);
    let mut api_request: ApiRequest<SetDataValidationResponse> =
        ApiRequest::post(&api_endpoint.to_url()).body(serialize_params(&request, "设置下拉列表")?);

    if let Some(opt) = option {
        api_request = api_request.request_option(opt);
    }

    let response = Transport::request(api_request, config, None).await?;
    extract_response_data(response, "设置下拉列表")
}

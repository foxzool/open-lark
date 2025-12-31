//! 更新下拉列表设置
//!
//! docPath: /document/ukTMukTMukTM/uATMzUjLwEzM14CMxMTN/datavalidation/update-datavalidation

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
pub struct UpdateDataValidationRequest {
    /// 更新的范围。格式通常为 `<sheetId>!A1:A2`；如果仅传 `A1:A2`，会自动补全为 `<sheetId>!A1:A2`
    pub ranges: Vec<String>,
    /// 数据验证类型：下拉列表请填写 `"list"`
    pub dataValidationType: String,
    /// 下拉列表规则属性
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
pub struct UpdateDataValidationResponse {}

impl ApiResponseTrait for UpdateDataValidationResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 更新下拉列表设置
pub async fn update(
    spreadsheet_token: String,
    sheet_id: String,
    mut request: UpdateDataValidationRequest,
    config: &Config,
    option: Option<openlark_core::req_option::RequestOption>,
) -> SDKResult<UpdateDataValidationResponse> {
    validate_required!(spreadsheet_token, "spreadsheet_token 不能为空");
    validate_required!(sheet_id, "sheet_id 不能为空");

    if request.ranges.is_empty() {
        return Err(openlark_core::error::validation_error(
            "ranges",
            "ranges 不能为空",
        ));
    }

    // 兼容：ranges 既可传 `<sheetId>!A1:A2`，也可仅传 `A1:A2`（此时补全前缀）。
    for (idx, range) in request.ranges.iter_mut().enumerate() {
        let trimmed = range.trim();
        if trimmed.is_empty() {
            return Err(openlark_core::error::validation_error(
                &format!("ranges[{}]", idx),
                "range 不能为空",
            ));
        }
        if !trimmed.contains('!') {
            *range = format!("{}!{}", sheet_id, trimmed);
            continue;
        }
        if let Some((range_sheet_id, _)) = trimmed.split_once('!') {
            if range_sheet_id != sheet_id {
                return Err(openlark_core::error::validation_error(
                    &format!("ranges[{}]", idx),
                    "ranges 中的 sheetId 必须与路径参数 sheetId 一致",
                ));
            }
        }
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

    let api_endpoint = CcmSheetApiOld::DataValidationUpdate(spreadsheet_token, sheet_id);
    let mut api_request: ApiRequest<UpdateDataValidationResponse> =
        ApiRequest::put(&api_endpoint.to_url())
            .body(serialize_params(&request, "更新下拉列表设置")?);

    if let Some(opt) = option {
        api_request = api_request.request_option(opt);
    }

    let response = Transport::request(api_request, config, None).await?;
    extract_response_data(response, "更新下拉列表设置")
}

//! 批量更新条件格式
//!
//! docPath: /document/ukTMukTMukTM/uATMzUjLwEzM14CMxMTN/conditionformat/condition-format-update

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
pub struct BatchUpdateConditionFormatRequest {
    pub sheet_condition_formats: Vec<SheetConditionFormatUpdate>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct SheetConditionFormatUpdate {
    pub sheet_id: String,
    pub cf_id: String,
    pub condition_format: ConditionFormat,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct ConditionFormat {
    pub ranges: Option<Vec<String>>,
    pub rule_type: Option<String>,
    pub attrs: Option<Vec<ConditionFormatAttr>>,
    pub style: Option<ConditionFormatStyle>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct ConditionFormatAttr {
    pub operator: Option<String>,
    pub formula: Option<Vec<String>>,
    pub text: Option<String>,
    pub time_period: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct ConditionFormatStyle {
    pub font: Option<Font>,
    pub text_decoration: Option<i32>,
    pub fore_color: Option<String>,
    pub back_color: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Font {
    pub bold: Option<bool>,
    pub italic: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct BatchUpdateConditionFormatResponse {
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

impl ApiResponseTrait for BatchUpdateConditionFormatResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 批量更新条件格式
pub async fn batch_update(
    spreadsheet_token: String,
    request: BatchUpdateConditionFormatRequest,
    config: &Config,
    option: Option<openlark_core::req_option::RequestOption>,
) -> SDKResult<BatchUpdateConditionFormatResponse> {
    validate_required!(spreadsheet_token, "spreadsheet_token 不能为空");
    if request.sheet_condition_formats.is_empty() {
        return Err(openlark_core::error::validation_error(
            "sheet_condition_formats",
            "sheet_condition_formats 不能为空",
        ));
    }
    if request.sheet_condition_formats.len() > 10 {
        return Err(openlark_core::error::validation_error(
            "sheet_condition_formats",
            "sheet_condition_formats 单次最多 10 个",
        ));
    }
    let allowed = [
        "containsBlanks",
        "notContainsBlanks",
        "duplicateValues",
        "uniqueValues",
        "cellIs",
        "containsText",
        "timePeriod",
    ];
    for (idx, item) in request.sheet_condition_formats.iter().enumerate() {
        if item.sheet_id.trim().is_empty() {
            return Err(openlark_core::error::validation_error(
                "sheet_id",
                "sheet_id 不能为空",
            ));
        }
        if item.cf_id.trim().is_empty() {
            return Err(openlark_core::error::validation_error(
                "cf_id",
                "cf_id 不能为空",
            ));
        }
        if let Some(ranges) = item.condition_format.ranges.as_ref() {
            if ranges.is_empty() {
                return Err(openlark_core::error::validation_error(
                    "ranges",
                    "ranges 不能为空",
                ));
            }
            if ranges.iter().any(|r| r.trim().is_empty()) {
                return Err(openlark_core::error::validation_error(
                    &format!("sheet_condition_formats[{}].ranges", idx),
                    "ranges 不能包含空字符串",
                ));
            }
            if ranges
                .iter()
                .any(|r| !(r == &item.sheet_id || r.starts_with(&format!("{}!", item.sheet_id))))
            {
                return Err(openlark_core::error::validation_error(
                    &format!("sheet_condition_formats[{}].ranges", idx),
                    "ranges 中的 sheetId 必须与 sheet_id 一致",
                ));
            }
        }
        if let Some(rule_type) = item.condition_format.rule_type.as_deref() {
            if !allowed.contains(&rule_type) {
                return Err(openlark_core::error::validation_error(
                    "rule_type",
                    "rule_type 不合法",
                ));
            }
        }
        if let Some(attrs) = item.condition_format.attrs.as_ref() {
            if attrs.is_empty() {
                return Err(openlark_core::error::validation_error(
                    "attrs",
                    "attrs 不能为空",
                ));
            }
            for attr in attrs.iter() {
                if let Some(op) = attr.operator.as_deref() {
                    if op.trim().is_empty() {
                        return Err(openlark_core::error::validation_error(
                            "operator",
                            "operator 不能为空",
                        ));
                    }
                }
            }
        }
    }

    let api_endpoint = CcmSheetApiOld::ConditionFormatsBatchUpdate(spreadsheet_token);
    let mut api_request: ApiRequest<BatchUpdateConditionFormatResponse> =
        ApiRequest::post(&api_endpoint.to_url())
            .body(serialize_params(&request, "批量更新条件格式")?);

    if let Some(opt) = option {
        api_request = api_request.request_option(opt);
    }

    let response = Transport::request(api_request, config, None).await?;
    extract_response_data(response, "批量更新条件格式")
}

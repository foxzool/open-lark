//! 批量创建条件格式
//!
//! docPath: /document/ukTMukTMukTM/uATMzUjLwEzM14CMxMTN/conditionformat/condition-format-set

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
pub struct BatchCreateConditionFormatRequest {
    pub sheet_condition_formats: Vec<SheetConditionFormat>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct SheetConditionFormat {
    pub sheet_id: String,
    pub condition_format: ConditionFormat,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct ConditionFormat {
    pub ranges: Vec<String>,
    pub rule_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
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
pub struct BatchCreateConditionFormatResponse {
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

impl ApiResponseTrait for BatchCreateConditionFormatResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 批量创建条件格式
pub async fn batch_create(
    spreadsheet_token: String,
    request: BatchCreateConditionFormatRequest,
    config: &Config,
    option: Option<openlark_core::req_option::RequestOption>,
) -> SDKResult<BatchCreateConditionFormatResponse> {
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
    for (idx, item) in request.sheet_condition_formats.iter().enumerate() {
        if item.sheet_id.trim().is_empty() {
            return Err(openlark_core::error::validation_error(
                "sheet_id",
                "sheet_id 不能为空",
            ));
        }
        if item.condition_format.ranges.is_empty() {
            return Err(openlark_core::error::validation_error(
                "ranges",
                "ranges 不能为空",
            ));
        }
        if item
            .condition_format
            .ranges
            .iter()
            .any(|r| r.trim().is_empty())
        {
            return Err(openlark_core::error::validation_error(
                &format!("sheet_condition_formats[{}].ranges", idx),
                "ranges 不能包含空字符串",
            ));
        }
        // 文档要求：ranges 中每个范围的 sheetId 必须与 sheet_id 一致
        if item
            .condition_format
            .ranges
            .iter()
            .any(|r| !(r == &item.sheet_id || r.starts_with(&format!("{}!", item.sheet_id))))
        {
            return Err(openlark_core::error::validation_error(
                &format!("sheet_condition_formats[{}].ranges", idx),
                "ranges 中的 sheetId 必须与 sheet_id 一致",
            ));
        }

        let rule_type = item.condition_format.rule_type.as_str();
        let allowed = [
            "containsBlanks",
            "notContainsBlanks",
            "duplicateValues",
            "uniqueValues",
            "cellIs",
            "containsText",
            "timePeriod",
        ];
        if !allowed.contains(&rule_type) {
            return Err(openlark_core::error::validation_error(
                "rule_type",
                "rule_type 不合法",
            ));
        }
        match rule_type {
            "containsBlanks" | "notContainsBlanks" | "duplicateValues" | "uniqueValues" => {}
            _ => {
                let attrs = item.condition_format.attrs.as_ref().ok_or_else(|| {
                    openlark_core::error::validation_error("attrs", "attrs 不能为空")
                })?;
                if attrs.is_empty() {
                    return Err(openlark_core::error::validation_error(
                        "attrs",
                        "attrs 不能为空",
                    ));
                }
            }
        }
        if let Some(attrs) = item.condition_format.attrs.as_ref() {
            for attr in attrs.iter() {
                if let Some(op) = attr.operator.as_deref() {
                    if op.trim().is_empty() {
                        return Err(openlark_core::error::validation_error(
                            "operator",
                            "operator 不能为空",
                        ));
                    }
                }
                if rule_type == "timePeriod" {
                    if attr.time_period.as_deref().unwrap_or("").trim().is_empty() {
                        return Err(openlark_core::error::validation_error(
                            "time_period",
                            "rule_type 为 timePeriod 时 time_period 不能为空",
                        ));
                    }
                    if attr.operator.as_deref() != Some("is") {
                        return Err(openlark_core::error::validation_error(
                            "operator",
                            "rule_type 为 timePeriod 时 operator 仅支持 is",
                        ));
                    }
                }
                if rule_type == "cellIs"
                    && attr.formula.as_ref().map(|v| v.is_empty()).unwrap_or(true)
                {
                    return Err(openlark_core::error::validation_error(
                        "formula",
                        "rule_type 为 cellIs 时 formula 不能为空",
                    ));
                }
                if rule_type == "containsText"
                    && attr.text.as_deref().unwrap_or("").trim().is_empty()
                {
                    return Err(openlark_core::error::validation_error(
                        "text",
                        "rule_type 为 containsText 时 text 不能为空",
                    ));
                }
            }
        }
    }

    let api_endpoint = CcmSheetApiOld::ConditionFormatsBatchCreate(spreadsheet_token);
    let mut api_request: ApiRequest<BatchCreateConditionFormatResponse> =
        ApiRequest::post(&api_endpoint.to_url())
            .body(serialize_params(&request, "批量创建条件格式")?);

    if let Some(opt) = option {
        api_request = api_request.request_option(opt);
    }

    let response = Transport::request(api_request, config, None).await?;
    extract_response_data(response, "批量创建条件格式")
}

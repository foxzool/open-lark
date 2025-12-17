//! 批量更新条件格式
//!
//! docPath: https://open.feishu.cn/document/server-docs/docs/sheets-v3/conditionformat/condition-format-update

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, Response, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::api_endpoints::CcmSheetApiOld;

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
    pub operator: String,
    pub formula: Option<Vec<String>>,
    pub text: Option<String>,
    pub time_period: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct ConditionFormatStyle {
    pub font: Option<Font>,
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
    pub responses: Vec<ConditionFormatResponse>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct ConditionFormatResponse {
    pub sheet_id: String,
    pub cf_id: String,
    pub res_code: i32,
    pub res_msg: String,
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
) -> SDKResult<Response<BatchUpdateConditionFormatResponse>> {
    let api_endpoint = CcmSheetApiOld::ConditionFormatsBatchUpdate(spreadsheet_token);
    let mut api_request: ApiRequest<BatchUpdateConditionFormatResponse> =
        ApiRequest::post(&api_endpoint.to_url()).body(serde_json::to_value(request)?);

    if let Some(opt) = option {
        api_request = api_request.request_option(opt);
    }

    Transport::request(api_request, config, None).await
}

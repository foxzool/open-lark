//! 批量获取条件格式
//!
//! docPath: /document/ukTMukTMukTM/uATMzUjLwEzM14CMxMTN/conditionformat/condition-format-get

pub mod batch_create;
pub mod batch_delete;
pub mod batch_update;
pub use batch_create::*;
pub use batch_delete::*;
pub use batch_update::*;

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    validate_required,
    SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::api_utils::*;
use crate::common::api_endpoints::CcmSheetApiOld;

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct GetConditionFormatRequest {
    /// 工作表 ID 列表，多个 ID 使用逗号分隔，最多 10 个
    pub sheet_ids: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct GetConditionFormatResponse {
    #[serde(default)]
    pub sheet_condition_formats: Vec<SheetConditionFormat>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct SheetConditionFormat {
    pub sheet_id: String,
    #[serde(default)]
    pub condition_format: ConditionFormat,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct ConditionFormat {
    pub cf_id: String,
    pub ranges: Vec<String>,
    pub rule_type: String,
    #[serde(default)]
    pub attrs: Vec<ConditionFormatAttr>,
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

impl ApiResponseTrait for GetConditionFormatResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 批量获取条件格式
pub async fn condition_formats(
    spreadsheet_token: String,
    request: GetConditionFormatRequest,
    config: &Config,
    option: Option<openlark_core::req_option::RequestOption>,
) -> SDKResult<GetConditionFormatResponse> {
    validate_required!(spreadsheet_token, "spreadsheet_token 不能为空");
    if request.sheet_ids.trim().is_empty() {
        return Err(openlark_core::error::validation_error(
            "sheet_ids",
            "sheet_ids 不能为空",
        ));
    }
    let sheet_ids: Vec<&str> = request
        .sheet_ids
        .split(',')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .collect();
    if sheet_ids.is_empty() {
        return Err(openlark_core::error::validation_error(
            "sheet_ids",
            "sheet_ids 不能为空",
        ));
    }
    if sheet_ids.len() > 10 {
        return Err(openlark_core::error::validation_error(
            "sheet_ids",
            "sheet_ids 单次最多 10 个",
        ));
    }

    let api_endpoint = CcmSheetApiOld::ConditionFormats(spreadsheet_token);
    let mut api_request: ApiRequest<GetConditionFormatResponse> =
        ApiRequest::get(&api_endpoint.to_url()).query("sheet_ids", request.sheet_ids);

    if let Some(opt) = option {
        api_request = api_request.request_option(opt);
    }

    let response = Transport::request(api_request, config, None).await?;
    extract_response_data(response, "批量获取条件格式")
}

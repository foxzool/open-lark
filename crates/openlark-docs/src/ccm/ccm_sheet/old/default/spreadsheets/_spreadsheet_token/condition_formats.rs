//! 批量获取条件格式
//!
//! docPath: https://open.feishu.cn/document/server-docs/docs/sheets-v3/conditionformat/condition-format-get

pub mod batch_create;
pub mod batch_delete;
pub mod batch_update;
pub use batch_create::*;
pub use batch_delete::*;
pub use batch_update::*;

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, Response, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::api_endpoints::CcmSheetApiOld;

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct GetConditionFormatRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sheet_ids: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct GetConditionFormatResponse {
    pub sheet_condition_formats: Vec<SheetConditionFormat>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct SheetConditionFormat {
    pub sheet_id: String,
    pub condition_format: ConditionFormat,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct ConditionFormat {
    pub cf_id: String,
    pub ranges: Vec<String>,
    pub rule_type: String,
    pub attrs: Vec<ConditionFormatAttr>,
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
) -> SDKResult<Response<GetConditionFormatResponse>> {
    let api_endpoint = CcmSheetApiOld::ConditionFormats(spreadsheet_token);
    let mut api_request: ApiRequest<GetConditionFormatResponse> =
        ApiRequest::get(&api_endpoint.to_url()).query_opt("sheet_ids", request.sheet_ids);

    if let Some(opt) = option {
        api_request = api_request.request_option(opt);
    }

    Transport::request(api_request, config, None).await
}

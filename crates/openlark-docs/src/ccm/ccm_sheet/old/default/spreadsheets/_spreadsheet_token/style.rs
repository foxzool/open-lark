//! 设置单元格样式
//!
//! docPath: /document/ukTMukTMukTM/ukjMzUjL5IzM14SOyMTN
//! doc: https://open.feishu.cn/document/server-docs/docs/sheets-v3/data-operation/set-cell-style

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
pub struct SetStyleRequest {
    pub appendStyle: Option<serde_json::Value>,
    pub range: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct SetStyleResponse {
    pub spreadsheetToken: String,
    pub updatedRange: String,
}

impl ApiResponseTrait for SetStyleResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 设置单元格样式
pub async fn style(
    spreadsheet_token: String,
    request: SetStyleRequest,
    config: &Config,
    option: Option<openlark_core::req_option::RequestOption>,
) -> SDKResult<SetStyleResponse> {
    let api_endpoint = CcmSheetApiOld::Style(spreadsheet_token);
    let mut api_request: ApiRequest<SetStyleResponse> =
        ApiRequest::put(&api_endpoint.to_url()).body(serialize_params(&request, "设置单元格样式")?);

    if let Some(opt) = option {
        api_request = api_request.request_option(opt);
    }

    let response = Transport::request(api_request, config, None).await?;
    extract_response_data(response, "设置单元格样式")
}

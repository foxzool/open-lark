//! 设置单元格样式
//!
//! docPath: https://open.feishu.cn/document/server-docs/docs/sheets-v3/data-operation/set-cell-style

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, Response, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};

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
) -> SDKResult<Response<SetStyleResponse>> {
    let api_endpoint = CcmSheetApiOld::Style(spreadsheet_token);
    let mut api_request: ApiRequest<SetStyleResponse> =
        ApiRequest::put(&api_endpoint.to_url()).body(serde_json::to_value(request)?);

    if let Some(opt) = option {
        api_request = api_request.request_option(opt);
    }

    Transport::request(api_request, config, None).await
}

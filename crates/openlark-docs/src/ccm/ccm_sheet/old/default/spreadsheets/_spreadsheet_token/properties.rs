//! 更新表格属性
//!
//! docPath: https://open.feishu.cn/document/server-docs/historic-version/docs/sheets/update-spreadsheet-properties

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, Response, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::api_endpoints::CcmSheetApiOld;

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct UpdateSpreadsheetPropertiesRequest {
    pub properties: SpreadsheetProperties,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct SpreadsheetProperties {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct UpdateSpreadsheetPropertiesResponse {
    pub spreadsheet_token: String,
    pub title: String,
}

impl ApiResponseTrait for UpdateSpreadsheetPropertiesResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 更新表格属性
pub async fn properties(
    spreadsheet_token: String,
    request: UpdateSpreadsheetPropertiesRequest,
    config: &Config,
    option: Option<openlark_core::req_option::RequestOption>,
) -> SDKResult<Response<UpdateSpreadsheetPropertiesResponse>> {
    let api_endpoint = CcmSheetApiOld::Properties(spreadsheet_token);
    let mut api_request: ApiRequest<UpdateSpreadsheetPropertiesResponse> =
        ApiRequest::put(&api_endpoint.to_url()).body(serde_json::to_value(request)?);

    if let Some(opt) = option {
        api_request = api_request.request_option(opt);
    }

    Transport::request(api_request, config, None).await
}

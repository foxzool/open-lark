//! 更新表格属性
//!
//! docPath: /document/ukTMukTMukTM/ucTMzUjL3EzM14yNxMTN

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
pub struct UpdateSpreadsheetPropertiesRequest {
    pub properties: SpreadsheetProperties,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct SpreadsheetProperties {
    pub title: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct UpdateSpreadsheetPropertiesResponse {
    pub spreadsheetToken: String,
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
) -> SDKResult<UpdateSpreadsheetPropertiesResponse> {
    validate_required!(spreadsheet_token, "spreadsheet_token 不能为空");
    if request.properties.title.trim().is_empty() {
        return Err(openlark_core::error::validation_error(
            "title",
            "title 不能为空",
        ));
    }
    if request.properties.title.chars().count() > 100 {
        return Err(openlark_core::error::validation_error(
            "title",
            "title 最大长度 100 个字符",
        ));
    }

    let api_endpoint = CcmSheetApiOld::Properties(spreadsheet_token);
    let mut api_request: ApiRequest<UpdateSpreadsheetPropertiesResponse> =
        ApiRequest::put(&api_endpoint.to_url()).body(serialize_params(&request, "更新表格属性")?);

    if let Some(opt) = option {
        api_request = api_request.request_option(opt);
    }

    let response = Transport::request(api_request, config, None).await?;
    extract_response_data(response, "更新表格属性")
}

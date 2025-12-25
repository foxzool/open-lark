//! 设置下拉列表
//!
//! docPath: /document/ukTMukTMukTM/uATMzUjLwEzM14CMxMTN/datavalidation/set-dropdown
//! doc: https://open.feishu.cn/document/server-docs/docs/sheets-v3/datavalidation/set-dropdown

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
pub struct SetDataValidationRequest {
    pub range: String,
    pub dataValidationType: String,
    pub dataValidation: Option<DataValidationSetting>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct DataValidationSetting {
    pub conditionValues: Vec<String>,
    pub options: Option<DataValidationOptions>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct DataValidationOptions {
    pub multipleValues: bool,
    pub highlightValidData: bool,
    pub colors: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct SetDataValidationResponse {}

impl ApiResponseTrait for SetDataValidationResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 设置下拉列表
pub async fn data_validation(
    spreadsheet_token: String,
    request: SetDataValidationRequest,
    config: &Config,
    option: Option<openlark_core::req_option::RequestOption>,
) -> SDKResult<SetDataValidationResponse> {
    let api_endpoint = CcmSheetApiOld::DataValidationCreate(spreadsheet_token);
    let mut api_request: ApiRequest<SetDataValidationResponse> =
        ApiRequest::post(&api_endpoint.to_url()).body(serialize_params(&request, "设置下拉列表")?);

    if let Some(opt) = option {
        api_request = api_request.request_option(opt);
    }

    let response = Transport::request(api_request, config, None).await?;
    extract_response_data(response, "设置下拉列表")
}

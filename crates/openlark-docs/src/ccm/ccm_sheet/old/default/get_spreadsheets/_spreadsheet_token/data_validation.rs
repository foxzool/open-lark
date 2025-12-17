//! 查询下拉列表设置
//!
//! docPath: https://open.feishu.cn/document/server-docs/docs/sheets-v3/datavalidation/query-datavalidation

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, Response, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::api_endpoints::CcmSheetApiOld;

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct QueryDataValidationRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub range: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dataValidationType: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct QueryDataValidationResponse {
    pub spreadsheetToken: String,
    pub dataValidations: Vec<DataValidation>,
    pub revision: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct DataValidation {
    pub dataValidationId: i32,
    pub dataValidationType: String,
    pub conditionValues: Vec<String>,
    pub options: Option<DataValidationOptions>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct DataValidationOptions {
    pub multipleValues: bool,
    pub highlightValidData: bool,
    pub colors: Vec<String>,
}

impl ApiResponseTrait for QueryDataValidationResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 查询下拉列表设置
pub async fn data_validation(
    spreadsheet_token: String,
    request: QueryDataValidationRequest,
    config: &Config,
    option: Option<openlark_core::req_option::RequestOption>,
) -> SDKResult<Response<QueryDataValidationResponse>> {
    let api_endpoint = CcmSheetApiOld::DataValidation(spreadsheet_token);
    let mut api_request: ApiRequest<QueryDataValidationResponse> =
        ApiRequest::get(&api_endpoint.to_url())
            .query_opt("range", request.range)
            .query_opt("dataValidationType", request.dataValidationType);

    if let Some(opt) = option {
        api_request = api_request.request_option(opt);
    }

    Transport::request(api_request, config, None).await
}

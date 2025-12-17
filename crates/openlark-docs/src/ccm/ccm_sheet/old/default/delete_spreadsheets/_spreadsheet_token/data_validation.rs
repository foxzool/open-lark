//! 删除下拉列表设置
//!
//! docPath: https://open.feishu.cn/document/server-docs/docs/sheets-v3/datavalidation/delete-datavalidation

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, Response, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::api_endpoints::CcmSheetApiOld;

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct DeleteDataValidationRequest {
    pub range: String,
    pub dataValidationIds: Option<Vec<i32>>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct DeleteDataValidationResponse {
    pub range: String,
}

impl ApiResponseTrait for DeleteDataValidationResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 删除下拉列表设置
pub async fn data_validation(
    spreadsheet_token: String,
    request: DeleteDataValidationRequest,
    config: &Config,
    option: Option<openlark_core::req_option::RequestOption>,
) -> SDKResult<Response<DeleteDataValidationResponse>> {
    let api_endpoint = CcmSheetApiOld::DataValidationDelete(spreadsheet_token);
    let mut api_request: ApiRequest<DeleteDataValidationResponse> =
        ApiRequest::delete(&api_endpoint.to_url()).body(serde_json::to_value(request)?);

    if let Some(opt) = option {
        api_request = api_request.request_option(opt);
    }

    Transport::request(api_request, config, None).await
}

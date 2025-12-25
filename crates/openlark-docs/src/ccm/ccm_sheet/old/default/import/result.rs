//! 查询导入结果
//!
//! docPath: /document/ukTMukTMukTM/uETO2YjLxkjN24SM5YjN
//! doc: https://open.feishu.cn/document/server-docs/historic-version/docs/sheets/sheet-operation/query-import-results

use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::api_utils::*;

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct ImportResultRequest {
    pub ticket: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct ImportResultResponse {
    pub result: ImportResult,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct ImportResult {
    pub ticket: String,
    pub url: String,
    pub warning_code: i32,
    pub warning_msg: String,
}

impl ApiResponseTrait for ImportResultResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 查询导入结果
pub async fn result(
    request: ImportResultRequest,
    config: &Config,
    option: Option<openlark_core::req_option::RequestOption>,
) -> SDKResult<ImportResultResponse> {
    use crate::common::api_endpoints::CcmSheetApiOld;

    let mut api_request: ApiRequest<ImportResultResponse> =
        ApiRequest::get(&CcmSheetApiOld::ImportResult.to_url()).query("ticket", request.ticket);

    if let Some(opt) = option {
        api_request = api_request.request_option(opt);
    }

    let response = Transport::request(api_request, config, None).await?;
    extract_response_data(response, "查询导入结果")
}

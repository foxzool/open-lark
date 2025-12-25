//! 写入图片
//!
//! docPath: /document/ukTMukTMukTM/uUDNxYjL1QTM24SN0EjN
//! doc: https://open.feishu.cn/document/server-docs/docs/sheets-v3/data-operation/write-images

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
pub struct ValuesImageRequest {
    pub range: String,
    pub image: Vec<u8>,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct ValuesImageResponse {
    pub spreadsheetToken: String,
    pub range: String,
}

impl ApiResponseTrait for ValuesImageResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 写入图片
pub async fn values_image(
    spreadsheet_token: String,
    request: ValuesImageRequest,
    config: &Config,
    option: Option<openlark_core::req_option::RequestOption>,
) -> SDKResult<ValuesImageResponse> {
    let api_endpoint = CcmSheetApiOld::ValuesImage(spreadsheet_token);
    let mut api_request: ApiRequest<ValuesImageResponse> =
        ApiRequest::post(&api_endpoint.to_url()).body(serialize_params(&request, "写入图片")?);

    if let Some(opt) = option {
        api_request = api_request.request_option(opt);
    }

    let response = Transport::request(api_request, config, None).await?;
    extract_response_data(response, "写入图片")
}

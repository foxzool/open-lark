/// 查询浮动图片
///
/// 返回子表内所有的浮动图片信息。
/// docPath: /document/ukTMukTMukTM/uUDN04SN0QjL1QDN/sheets-v3/spreadsheet-sheet-float_image/query
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};

use super::FloatImage;
use crate::common::{api_endpoints::SheetsApiV3, api_utils::*};

/// 查询浮动图片响应体 data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryFloatImagesResponse {
    pub items: Vec<FloatImage>,
}

impl ApiResponseTrait for QueryFloatImagesResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 查询浮动图片
pub async fn query_float_images(
    config: &Config,
    spreadsheet_token: &str,
    sheet_id: &str,
) -> SDKResult<QueryFloatImagesResponse> {
    let api_endpoint =
        SheetsApiV3::QueryFloatImages(spreadsheet_token.to_string(), sheet_id.to_string());
    let api_request: ApiRequest<QueryFloatImagesResponse> = ApiRequest::get(&api_endpoint.to_url());

    let response = Transport::request(api_request, config, None).await?;
    extract_response_data(response, "查询浮动图片")
}

/// 删除浮动图片
///
/// 删除 float_image_id 对应的浮动图片。
/// docPath: /document/ukTMukTMukTM/uUDN04SN0QjL1QDN/sheets-v3/spreadsheet-sheet-float_image/delete
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};

use crate::common::{api_endpoints::SheetsApiV3, api_utils::*};

/// 删除浮动图片响应体 data（data 为 `{}`）
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DeleteFloatImageResponse {}

impl ApiResponseTrait for DeleteFloatImageResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 删除浮动图片
pub async fn delete_float_image(
    config: &Config,
    spreadsheet_token: &str,
    sheet_id: &str,
    float_image_id: &str,
) -> SDKResult<DeleteFloatImageResponse> {
    let api_endpoint = SheetsApiV3::DeleteFloatImage(
        spreadsheet_token.to_string(),
        sheet_id.to_string(),
        float_image_id.to_string(),
    );
    let api_request: ApiRequest<DeleteFloatImageResponse> =
        ApiRequest::delete(&api_endpoint.to_url());

    let response = Transport::request(api_request, config, None).await?;
    extract_response_data(response, "删除浮动图片")
}

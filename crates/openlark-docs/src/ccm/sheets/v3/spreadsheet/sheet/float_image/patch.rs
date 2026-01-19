/// 更新浮动图片
///
/// 更新已有的浮动图片位置和宽高。
/// docPath: /document/ukTMukTMukTM/uUDN04SN0QjL1QDN/sheets-v3/spreadsheet-sheet-float_image/patch
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
    SDKResult,
};
use serde::{Deserialize, Serialize};

use super::FloatImage;
use crate::common::{api_endpoints::SheetsApiV3, api_utils::*};

/// 更新浮动图片请求体（字段均为可选）
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UpdateFloatImageRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub range: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset_x: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset_y: Option<f64>,
}

/// 更新浮动图片响应体 data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateFloatImageResponse {
    pub float_image: FloatImage,
}

impl ApiResponseTrait for UpdateFloatImageResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 更新浮动图片
pub async fn update_float_image(
    config: &Config,
    spreadsheet_token: &str,
    sheet_id: &str,
    float_image_id: &str,
    params: UpdateFloatImageRequest,
) -> SDKResult<UpdateFloatImageResponse> {
    let api_endpoint = SheetsApiV3::PatchFloatImage(
        spreadsheet_token.to_string(),
        sheet_id.to_string(),
        float_image_id.to_string(),
    );
    let api_request: ApiRequest<UpdateFloatImageResponse> =
        ApiRequest::patch(&api_endpoint.to_url()).body(serialize_params(&params, "更新浮动图片")?);

    let response = Transport::request(api_request, config, None).await?;
    extract_response_data(response, "更新浮动图片")
}

/// 创建浮动图片
///
/// 根据传入的参数创建一张浮动图片。
/// docPath: /document/ukTMukTMukTM/uUDN04SN0QjL1QDN/sheets-v3/spreadsheet-sheet-float_image/create
/// doc: https://open.feishu.cn/document/server-docs/docs/sheets-v3/spreadsheet-sheet-float_image/create
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};

use super::FloatImage;
use crate::common::{api_endpoints::SheetsApiV3, api_utils::*};

/// 创建浮动图片请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateFloatImageRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub float_image_id: Option<String>,
    /// 浮动图片 token（素材 file_token）
    pub float_image_token: String,
    /// 浮动图片左上角所在单元格位置（只允许单个单元格）
    pub range: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset_x: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset_y: Option<f64>,
}

/// 创建浮动图片响应体 data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateFloatImageResponse {
    pub float_image: FloatImage,
}

impl ApiResponseTrait for CreateFloatImageResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 创建浮动图片
pub async fn create_float_image(
    config: &Config,
    spreadsheet_token: &str,
    sheet_id: &str,
    params: CreateFloatImageRequest,
) -> SDKResult<CreateFloatImageResponse> {
    let api_endpoint = SheetsApiV3::CreateFloatImage(spreadsheet_token.to_string(), sheet_id.to_string());
    let api_request: ApiRequest<CreateFloatImageResponse> =
        ApiRequest::post(&api_endpoint.to_url()).body(serialize_params(&params, "创建浮动图片")?);

    let response = Transport::request(api_request, config, None).await?;
    extract_response_data(response, "创建浮动图片")
}


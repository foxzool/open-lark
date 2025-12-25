/// 获取浮动图片
///
/// 根据 float_image_id 获取对应浮动图片的信息。
/// docPath: /document/ukTMukTMukTM/uUDN04SN0QjL1QDN/sheets-v3/spreadsheet-sheet-float_image/get
/// doc: https://open.feishu.cn/document/server-docs/docs/sheets-v3/spreadsheet-sheet-float_image/get
use openlark_core::{
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    SDKResult,
};
use serde::{Deserialize, Serialize};

use super::FloatImage;
use crate::common::{api_endpoints::SheetsApiV3, api_utils::*};

/// 获取浮动图片响应体 data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetFloatImageResponse {
    pub float_image: FloatImage,
}

impl ApiResponseTrait for GetFloatImageResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 获取浮动图片
pub async fn get_float_image(
    config: &Config,
    spreadsheet_token: &str,
    sheet_id: &str,
    float_image_id: &str,
) -> SDKResult<GetFloatImageResponse> {
    let api_endpoint = SheetsApiV3::GetFloatImage(
        spreadsheet_token.to_string(),
        sheet_id.to_string(),
        float_image_id.to_string(),
    );
    let api_request: ApiRequest<GetFloatImageResponse> = ApiRequest::get(&api_endpoint.to_url());

    let response = Transport::request(api_request, config, None).await?;
    extract_response_data(response, "获取浮动图片")
}


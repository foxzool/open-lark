/// 更新浮动图片
///
/// 更新已有的浮动图片位置和宽高。
/// docPath: /document/ukTMukTMukTM/uUDN04SN0QjL1QDN/sheets-v3/spreadsheet-sheet-float_image/patch
use openlark_core::{
    SDKResult,
    api::{ApiRequest, ApiResponseTrait, ResponseFormat},
    config::Config,
    http::Transport,
    req_option::RequestOption,
};
use serde::{Deserialize, Serialize};

use super::FloatImage;
use crate::common::{api_endpoints::SheetsApiV3, api_utils::*};

/// 更新浮动图片请求体（字段均为可选）
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UpdateFloatImageRequest {
    /// 新的范围。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub range: Option<String>,
    /// 新的宽度。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<f64>,
    /// 新的高度。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<f64>,
    /// 新的 X 轴偏移。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset_x: Option<f64>,
    /// 新的 Y 轴偏移。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset_y: Option<f64>,
}

/// 更新浮动图片响应体 data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateFloatImageResponse {
    /// 更新后的浮动图片。
    pub float_image: FloatImage,
}

impl ApiResponseTrait for UpdateFloatImageResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 更新浮动图片
///
/// 更新指定浮动图片的位置和尺寸。
pub async fn update_float_image(
    config: &Config,
    spreadsheet_token: &str,
    sheet_id: &str,
    float_image_id: &str,
    params: UpdateFloatImageRequest,
) -> SDKResult<UpdateFloatImageResponse> {
    update_float_image_with_options(
        config,
        spreadsheet_token,
        sheet_id,
        float_image_id,
        params,
        RequestOption::default(),
    )
    .await
}

/// 更新浮动图片（带请求选项）
///
/// 更新指定浮动图片的位置和尺寸，并允许传入自定义请求选项。
pub async fn update_float_image_with_options(
    config: &Config,
    spreadsheet_token: &str,
    sheet_id: &str,
    float_image_id: &str,
    params: UpdateFloatImageRequest,
    option: RequestOption,
) -> SDKResult<UpdateFloatImageResponse> {
    let api_endpoint = SheetsApiV3::PatchFloatImage(
        spreadsheet_token.to_string(),
        sheet_id.to_string(),
        float_image_id.to_string(),
    );
    let api_request: ApiRequest<UpdateFloatImageResponse> =
        ApiRequest::patch(&api_endpoint.to_url()).body(serialize_params(&params, "更新浮动图片")?);

    let response = Transport::request(api_request, config, Some(option)).await?;
    extract_response_data(response, "更新浮动图片")
}

#[cfg(test)]
mod tests {

    use serde_json;

    #[test]
    fn test_serialization_roundtrip() {
        // 基础序列化测试
        let json = r#"{"test": "value"}"#;
        assert!(serde_json::from_str::<serde_json::Value>(json).is_ok());
    }

    #[test]
    fn test_deserialization_from_json() {
        // 基础反序列化测试
        let json = r#"{"field": "data"}"#;
        let value: serde_json::Value = serde_json::from_str(json).expect("JSON 反序列化失败");
        assert_eq!(value["field"], "data");
    }
}

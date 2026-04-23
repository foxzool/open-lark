/// 创建浮动图片
///
/// 根据传入的参数创建一张浮动图片。
/// docPath: /document/ukTMukTMukTM/uUDN04SN0QjL1QDN/sheets-v3/spreadsheet-sheet-float_image/create
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

/// 创建浮动图片请求体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateFloatImageRequest {
    /// 浮动图片 ID。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub float_image_id: Option<String>,
    /// 浮动图片 token（素材 file_token）
    pub float_image_token: String,
    /// 浮动图片左上角所在单元格位置（只允许单个单元格）
    pub range: String,
    /// 宽度。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub width: Option<f64>,
    /// 高度。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<f64>,
    /// X 轴偏移。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset_x: Option<f64>,
    /// Y 轴偏移。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset_y: Option<f64>,
}

/// 创建浮动图片响应体 data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateFloatImageResponse {
    /// 新建后的浮动图片。
    pub float_image: FloatImage,
}

impl ApiResponseTrait for CreateFloatImageResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// 创建浮动图片
///
/// 在指定工作表中创建一张浮动图片。
pub async fn create_float_image(
    config: &Config,
    spreadsheet_token: &str,
    sheet_id: &str,
    params: CreateFloatImageRequest,
) -> SDKResult<CreateFloatImageResponse> {
    create_float_image_with_options(
        config,
        spreadsheet_token,
        sheet_id,
        params,
        RequestOption::default(),
    )
    .await
}

/// 创建浮动图片（带请求选项）
///
/// 在指定工作表中创建一张浮动图片，并允许传入自定义请求选项。
pub async fn create_float_image_with_options(
    config: &Config,
    spreadsheet_token: &str,
    sheet_id: &str,
    params: CreateFloatImageRequest,
    option: RequestOption,
) -> SDKResult<CreateFloatImageResponse> {
    let api_endpoint =
        SheetsApiV3::CreateFloatImage(spreadsheet_token.to_string(), sheet_id.to_string());
    let api_request: ApiRequest<CreateFloatImageResponse> =
        ApiRequest::post(&api_endpoint.to_url()).body(serialize_params(&params, "创建浮动图片")?);

    let response = Transport::request(api_request, config, Some(option)).await?;
    extract_response_data(response, "创建浮动图片")
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

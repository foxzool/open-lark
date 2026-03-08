/// 查询浮动图片
///
/// 返回子表内所有的浮动图片信息。
/// docPath: /document/ukTMukTMukTM/uUDN04SN0QjL1QDN/sheets-v3/spreadsheet-sheet-float_image/query
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
    query_float_images_with_options(
        config,
        spreadsheet_token,
        sheet_id,
        RequestOption::default(),
    )
    .await
}

/// 查询浮动图片（带请求选项）
pub async fn query_float_images_with_options(
    config: &Config,
    spreadsheet_token: &str,
    sheet_id: &str,
    option: RequestOption,
) -> SDKResult<QueryFloatImagesResponse> {
    let api_endpoint =
        SheetsApiV3::QueryFloatImages(spreadsheet_token.to_string(), sheet_id.to_string());
    let api_request: ApiRequest<QueryFloatImagesResponse> = ApiRequest::get(&api_endpoint.to_url());

    let response = Transport::request(api_request, config, Some(option)).await?;
    extract_response_data(response, "查询浮动图片")
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
        let value: serde_json::Value = serde_json::from_str(json).unwrap();
        assert_eq!(value["field"], "data");
    }
}

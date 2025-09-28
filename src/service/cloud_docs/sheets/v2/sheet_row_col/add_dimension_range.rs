use serde::{Deserialize, Serialize};

use crate::{
    core::{
        api_req::ApiRequest,
        api_resp::{ApiResponseTrait, BaseResponse, ResponseFormat},
        constants::AccessTokenType,
        endpoints::cloud_docs::*,
        req_option, SDKResult,
    },
    service::cloud_docs::sheets::v2::SpreadsheetService,
};

/// 增加行列请求
#[derive(Serialize, Debug, Default)]
pub struct AddDimensionRangeRequest {
    #[serde(skip)]
    api_request: ApiRequest,
    #[serde(skip)]
    spreadsheet_token: String,
    /// 需要增加行列的维度信息
    dimension: Dimension,
}

#[derive(Serialize, Debug, Default)]
struct Dimension {
    /// 电子表格工作表的 ID。调用获取工作表获取 ID
    #[serde(rename = "sheetId")]
    sheet_id: String,
    /// 更新的维度。可选值：
    /// - ROWS：行
    /// - COLUMNS：列
    #[serde(rename = "majorDimension")]
    major_dimension: String,
    /// 要增加的行数或列数。取值范围为 (0,5000]
    length: i32,
}

impl AddDimensionRangeRequest {
    pub fn builder() -> AddDimensionRangeRequestBuilder {
        AddDimensionRangeRequestBuilder::default()
    }
}

#[derive(Default)]
pub struct AddDimensionRangeRequestBuilder {
    request: AddDimensionRangeRequest,
}

impl AddDimensionRangeRequestBuilder {
    pub fn spreadsheet_token(mut self, spreadsheet_token: impl ToString) -> Self {
        self.request.spreadsheet_token = spreadsheet_token.to_string();
        self
    }

    /// 电子表格工作表的 ID。调用获取工作表获取 ID
    pub fn sheet_id(mut self, sheet_id: impl ToString) -> Self {
        self.request.dimension.sheet_id = sheet_id.to_string();
        self
    }

    /// 更新的维度。可选值：
    /// - ROWS：行
    /// - COLUMNS：列
    pub fn major_dimension(mut self, major_dimension: impl ToString) -> Self {
        self.request.dimension.major_dimension = major_dimension.to_string();
        self
    }

    /// 要增加的行数或列数。取值范围为 (0,5000]
    pub fn length(mut self, length: i32) -> Self {
        self.request.dimension.length = length;
        self
    }

    pub fn build(mut self) -> AddDimensionRangeRequest {
        self.request.api_request.body = serde_json::to_vec(&self.request).unwrap();
        self.request
    }
}

impl SpreadsheetService {
    /// 该接口用于在电子表格中增加空白行或列。
    pub async fn add_dimension_range(
        &self,
        request: AddDimensionRangeRequest,
        option: Option<req_option::RequestOption>,
    ) -> SDKResult<BaseResponse<DimensionRangeResponse>> {
        let mut api_req = request.api_request;
        api_req.api_path =
            SHEETS_V2_SPREADSHEET_DIMENSION_RANGE.replace("{}", &request.spreadsheet_token);
        api_req.http_method = reqwest::Method::POST;
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::App];

        let api_resp = crate::core::http::Transport::request(api_req, &self.config, option).await?;

        Ok(api_resp)
    }
}

/// 增加行列响应体
#[derive(Deserialize, Debug)]
pub struct DimensionRangeResponse {
    #[serde(rename = "addCount")]
    pub add_count: i32,
    #[serde(rename = "majorDimension")]
    pub major_dimension: String,
}

impl ApiResponseTrait for DimensionRangeResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::config::Config;
    use rstest::rstest;

    fn create_test_config() -> Config {
        Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build()
    }

    fn create_test_service() -> SpreadsheetService {
        SpreadsheetService::new(create_test_config())
    }

    #[test]
    fn test_add_dimension_range_request_builder_creation() {
        let builder = AddDimensionRangeRequest::builder();
        let request = builder.build();

        assert_eq!(request.spreadsheet_token, "");
        assert_eq!(request.dimension.sheet_id, "");
        assert_eq!(request.dimension.major_dimension, "");
        assert_eq!(request.dimension.length, 0);
    }

    #[test]
    fn test_add_dimension_range_request_builder_with_spreadsheet_token() {
        let request = AddDimensionRangeRequest::builder()
            .spreadsheet_token("test_spreadsheet_token_123")
            .build();

        assert_eq!(request.spreadsheet_token, "test_spreadsheet_token_123");
    }

    #[test]
    fn test_add_dimension_range_request_builder_with_sheet_id() {
        let request = AddDimensionRangeRequest::builder()
            .sheet_id("test_sheet_123")
            .build();

        assert_eq!(request.dimension.sheet_id, "test_sheet_123");
    }

    #[rstest]
    #[case("ROWS", "行")]
    #[case("COLUMNS", "列")]
    fn test_add_dimension_range_request_builder_with_major_dimension(
        #[case] dimension: &str,
        #[case] _description: &str,
    ) {
        let request = AddDimensionRangeRequest::builder()
            .major_dimension(dimension)
            .build();

        assert_eq!(request.dimension.major_dimension, dimension);
    }

    #[rstest]
    #[case(1, "minimum valid")]
    #[case(10, "small number")]
    #[case(100, "medium number")]
    #[case(1000, "large number")]
    #[case(5000, "maximum valid")]
    fn test_add_dimension_range_request_builder_with_length(
        #[case] length: i32,
        #[case] _description: &str,
    ) {
        let request = AddDimensionRangeRequest::builder().length(length).build();

        assert_eq!(request.dimension.length, length);
    }

    #[test]
    fn test_add_dimension_range_request_builder_chaining() {
        let request = AddDimensionRangeRequest::builder()
            .spreadsheet_token("chaining_test_token")
            .sheet_id("sheet_123")
            .major_dimension("ROWS")
            .length(50)
            .build();

        assert_eq!(request.spreadsheet_token, "chaining_test_token");
        assert_eq!(request.dimension.sheet_id, "sheet_123");
        assert_eq!(request.dimension.major_dimension, "ROWS");
        assert_eq!(request.dimension.length, 50);
    }

    #[test]
    fn test_add_dimension_range_request_default() {
        let request = AddDimensionRangeRequest::default();

        assert_eq!(request.spreadsheet_token, "");
        assert_eq!(request.dimension.sheet_id, "");
        assert_eq!(request.dimension.major_dimension, "");
        assert_eq!(request.dimension.length, 0);
    }

    #[test]
    fn test_dimension_default() {
        let dimension = Dimension::default();

        assert_eq!(dimension.sheet_id, "");
        assert_eq!(dimension.major_dimension, "");
        assert_eq!(dimension.length, 0);
    }

    #[test]
    fn test_add_dimension_range_request_builder_default() {
        let builder = AddDimensionRangeRequestBuilder::default();
        let request = builder.build();

        assert_eq!(request.spreadsheet_token, "");
        assert_eq!(request.dimension.sheet_id, "");
        assert_eq!(request.dimension.major_dimension, "");
        assert_eq!(request.dimension.length, 0);
    }

    #[test]
    fn test_add_dimension_range_request_serialization() {
        let request = AddDimensionRangeRequest::builder()
            .spreadsheet_token("serialization_token")
            .sheet_id("sheet_456")
            .major_dimension("COLUMNS")
            .length(25)
            .build();

        // Test that the request can be serialized (this validates field names)
        let serialized = serde_json::to_string(&request);
        assert!(serialized.is_ok());

        let json_str = serialized.unwrap();
        assert!(json_str.contains("dimension"));
        assert!(json_str.contains("sheetId"));
        assert!(json_str.contains("majorDimension"));
        assert!(json_str.contains("length"));
        assert!(json_str.contains("sheet_456"));
        assert!(json_str.contains("COLUMNS"));
        assert!(json_str.contains("25"));
    }

    #[test]
    fn test_add_dimension_range_request_debug() {
        let request = AddDimensionRangeRequest::builder()
            .spreadsheet_token("debug_token")
            .sheet_id("debug_sheet")
            .build();

        let debug_str = format!("{:?}", request);
        assert!(debug_str.contains("AddDimensionRangeRequest"));
        assert!(debug_str.contains("debug_token"));
        assert!(debug_str.contains("debug_sheet"));
    }

    #[test]
    fn test_dimension_debug() {
        let dimension = Dimension {
            sheet_id: "test_sheet".to_string(),
            major_dimension: "ROWS".to_string(),
            length: 42,
        };

        let debug_str = format!("{:?}", dimension);
        assert!(debug_str.contains("Dimension"));
        assert!(debug_str.contains("test_sheet"));
        assert!(debug_str.contains("ROWS"));
        assert!(debug_str.contains("42"));
    }

    #[test]
    fn test_add_dimension_range_request_with_empty_strings() {
        let request = AddDimensionRangeRequest::builder()
            .spreadsheet_token("")
            .sheet_id("")
            .major_dimension("")
            .length(0)
            .build();

        assert_eq!(request.spreadsheet_token, "");
        assert_eq!(request.dimension.sheet_id, "");
        assert_eq!(request.dimension.major_dimension, "");
        assert_eq!(request.dimension.length, 0);
    }

    #[test]
    fn test_add_dimension_range_request_with_special_characters() {
        let request = AddDimensionRangeRequest::builder()
            .spreadsheet_token("token_with_特殊字符_🎯")
            .sheet_id("sheet_名称_123")
            .major_dimension("ROWS")
            .length(100)
            .build();

        assert_eq!(request.spreadsheet_token, "token_with_特殊字符_🎯");
        assert_eq!(request.dimension.sheet_id, "sheet_名称_123");
        assert_eq!(request.dimension.major_dimension, "ROWS");
        assert_eq!(request.dimension.length, 100);
    }

    #[test]
    fn test_add_dimension_range_request_with_long_strings() {
        let long_token = "a".repeat(1000);
        let long_sheet_id = "sheet_".repeat(100);

        let request = AddDimensionRangeRequest::builder()
            .spreadsheet_token(&long_token)
            .sheet_id(&long_sheet_id)
            .major_dimension("COLUMNS")
            .length(2500)
            .build();

        assert_eq!(request.spreadsheet_token, long_token);
        assert_eq!(request.dimension.sheet_id, long_sheet_id);
        assert_eq!(request.dimension.major_dimension, "COLUMNS");
        assert_eq!(request.dimension.length, 2500);
    }

    #[test]
    fn test_add_dimension_range_request_api_request_body_serialization() {
        let request = AddDimensionRangeRequest::builder()
            .spreadsheet_token("body_test_token")
            .sheet_id("body_sheet")
            .major_dimension("ROWS")
            .length(75)
            .build();

        // Verify that api_request.body is set correctly
        assert!(!request.api_request.body.is_empty());

        // Verify that the body contains valid JSON
        let body_str = String::from_utf8(request.api_request.body).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&body_str).unwrap();

        assert!(parsed.get("dimension").is_some());
        let dimension = parsed.get("dimension").unwrap();
        assert_eq!(
            dimension.get("sheetId").unwrap().as_str().unwrap(),
            "body_sheet"
        );
        assert_eq!(
            dimension.get("majorDimension").unwrap().as_str().unwrap(),
            "ROWS"
        );
        assert_eq!(dimension.get("length").unwrap().as_i64().unwrap(), 75);
    }

    #[test]
    fn test_add_dimension_range_request_builder_multiple_calls() {
        let mut builder = AddDimensionRangeRequest::builder();

        // Test that multiple calls override previous values
        builder = builder.spreadsheet_token("first_token");
        builder = builder.spreadsheet_token("second_token");
        builder = builder.sheet_id("first_sheet");
        builder = builder.sheet_id("second_sheet");
        builder = builder.major_dimension("ROWS");
        builder = builder.major_dimension("COLUMNS");
        builder = builder.length(10);
        builder = builder.length(20);

        let request = builder.build();

        assert_eq!(request.spreadsheet_token, "second_token");
        assert_eq!(request.dimension.sheet_id, "second_sheet");
        assert_eq!(request.dimension.major_dimension, "COLUMNS");
        assert_eq!(request.dimension.length, 20);
    }

    #[test]
    fn test_add_dimension_range_request_edge_cases() {
        // Test with negative length (should be handled by API validation)
        let request = AddDimensionRangeRequest::builder().length(-1).build();
        assert_eq!(request.dimension.length, -1);

        // Test with zero length (boundary case)
        let request = AddDimensionRangeRequest::builder().length(0).build();
        assert_eq!(request.dimension.length, 0);

        // Test with very large length (should be handled by API validation)
        let request = AddDimensionRangeRequest::builder().length(i32::MAX).build();
        assert_eq!(request.dimension.length, i32::MAX);
    }

    #[test]
    fn test_dimension_range_response_api_response_trait() {
        // Test that DimensionRangeResponse implements ApiResponseTrait correctly
        assert_eq!(DimensionRangeResponse::data_format(), ResponseFormat::Data);
    }

    #[test]
    fn test_dimension_range_response_deserialization() {
        let json_data = r#"{
            "addCount": 42,
            "majorDimension": "ROWS"
        }"#;

        let response: Result<DimensionRangeResponse, _> = serde_json::from_str(json_data);
        assert!(response.is_ok());

        let response = response.unwrap();
        assert_eq!(response.add_count, 42);
        assert_eq!(response.major_dimension, "ROWS");
    }

    #[test]
    fn test_dimension_range_response_debug() {
        let response = DimensionRangeResponse {
            add_count: 25,
            major_dimension: "COLUMNS".to_string(),
        };

        let debug_str = format!("{:?}", response);
        assert!(debug_str.contains("DimensionRangeResponse"));
        assert!(debug_str.contains("25"));
        assert!(debug_str.contains("COLUMNS"));
    }

    #[test]
    fn test_dimension_range_response_with_different_values() {
        let test_cases = vec![
            (0, "ROWS"),
            (1, "COLUMNS"),
            (100, "ROWS"),
            (5000, "COLUMNS"),
        ];

        for (add_count, major_dimension) in test_cases {
            let json_data = format!(
                r#"{{"addCount": {}, "majorDimension": "{}"}}"#,
                add_count, major_dimension
            );

            let response: Result<DimensionRangeResponse, _> = serde_json::from_str(&json_data);
            assert!(response.is_ok());

            let response = response.unwrap();
            assert_eq!(response.add_count, add_count);
            assert_eq!(response.major_dimension, major_dimension);
        }
    }

    #[test]
    fn test_spreadsheet_service_creation() {
        let service = create_test_service();

        // Verify the service can be created without panic
        assert_eq!(service.config.app_id, "test_app_id");
    }

    #[test]
    fn test_add_dimension_range_request_memory_efficiency() {
        // Test creating many requests doesn't consume excessive memory
        let requests: Vec<AddDimensionRangeRequest> = (0..100)
            .map(|i| {
                AddDimensionRangeRequest::builder()
                    .spreadsheet_token(format!("token_{}", i))
                    .sheet_id(format!("sheet_{}", i))
                    .major_dimension(if i % 2 == 0 { "ROWS" } else { "COLUMNS" })
                    .length(i + 1)
                    .build()
            })
            .collect();

        assert_eq!(requests.len(), 100);

        // Verify each request has correct data
        for (i, request) in requests.iter().enumerate() {
            assert_eq!(request.spreadsheet_token, format!("token_{}", i));
            assert_eq!(request.dimension.sheet_id, format!("sheet_{}", i));
            assert_eq!(
                request.dimension.major_dimension,
                if i % 2 == 0 { "ROWS" } else { "COLUMNS" }
            );
            assert_eq!(request.dimension.length, (i + 1) as i32);
        }
    }

    #[test]
    fn test_dimension_range_response_incomplete_json() {
        // Test with missing addCount
        let json_data = r#"{"majorDimension": "ROWS"}"#;
        let response: Result<DimensionRangeResponse, _> = serde_json::from_str(json_data);
        assert!(response.is_err());

        // Test with missing majorDimension
        let json_data = r#"{"addCount": 42}"#;
        let response: Result<DimensionRangeResponse, _> = serde_json::from_str(json_data);
        assert!(response.is_err());

        // Test with wrong field names
        let json_data = r#"{"add_count": 42, "major_dimension": "ROWS"}"#;
        let response: Result<DimensionRangeResponse, _> = serde_json::from_str(json_data);
        assert!(response.is_err());
    }

    #[test]
    fn test_dimension_range_response_with_special_characters() {
        let json_data = r#"{
            "addCount": 10,
            "majorDimension": "特殊_维度_🎯"
        }"#;

        let response: Result<DimensionRangeResponse, _> = serde_json::from_str(json_data);
        assert!(response.is_ok());

        let response = response.unwrap();
        assert_eq!(response.add_count, 10);
        assert_eq!(response.major_dimension, "特殊_维度_🎯");
    }

    #[test]
    fn test_dimension_serialization() {
        let dimension = Dimension {
            sheet_id: "test_sheet_123".to_string(),
            major_dimension: "ROWS".to_string(),
            length: 50,
        };

        let serialized = serde_json::to_string(&dimension);
        assert!(serialized.is_ok());

        let json_str = serialized.unwrap();
        assert!(json_str.contains("sheetId"));
        assert!(json_str.contains("majorDimension"));
        assert!(json_str.contains("length"));
        assert!(json_str.contains("test_sheet_123"));
        assert!(json_str.contains("ROWS"));
        assert!(json_str.contains("50"));
    }
}

#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(non_snake_case)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::module_inception)]
use serde::{Deserialize, Serialize};
use open_lark_core::core::api_req::ApiRequest;
use crate::,
{,
    core::,
{,
        BaseResponse,
        ResponseFormat,
        api_resp::{ApiResponseTrait}
    constants::AccessTokenType,
        endpoints::cloud_docs::*,
        req_option, SDKResult,
};
    impl_executable_builder_owned,
    service::cloud_docs::sheets::v2::{sheet_row_col::UpdateDimension, SpreadsheetService}
};
/// 删除行列请求,
#[derive(Debug, Clone)]
pub struct DeleteDimensionRangeRequest {
    #[serde(skip)]
    api_request: ApiRequest,
    #[serde(skip)]
    spreadsheet_token: String,
    /// 需要删除行列的范围信息
    dimension: UpdateDimension}
impl DeleteDimensionRangeRequest {
    pub fn new(config: Config) -> Self {
        Self { config }
}#[derive(Debug, Clone)]
pub struct DeleteDimensionRangeRequestBuilder {
    request: DeleteDimensionRangeRequest}
impl DeleteDimensionRangeRequestBuilder {
    pub fn new(config: Config) -> Self {
        Self { config }
}impl_executable_builder_owned!(,
    DeleteDimensionRangeRequestBuilder,
    SpreadsheetService,
    DeleteDimensionRangeRequest,
    BaseResponse<DeleteDimensionRangeResponse>,
    delete_dimension_range,
);
impl SpreadsheetService {
    pub fn new(config: Config) -> Self {
        Self { config }
}/// 删除行列响应体,
#[derive(Debug, Clone)]
pub struct DeleteDimensionRangeResponse {
    /// 一共删除的行数或列数,
#[serde(rename = "delCount")]
    pub del_count: i32,
    /// 删除的维度。枚举值：,
/// - ROWS：行,
    /// - COLUMNS：列,
#[serde(rename = "majorDimension")]
    pub major_dimension: String,
impl ApiResponseTrait for.* {
    pub fn new(config: Config) -> Self {
        Self { config }
}    fn data_format() -> ResponseFormat {,
ResponseFormat::Data
    }
#[cfg(test)]
mod tests {
use super::*;
    use crate::core::config::Config;
use crate::core::SDKResult;use rstest::rstest;
    fn create_test_config() -> Config {,
Config::builder()
            .app_id()
.app_secret()
            .build()}
fn create_test_service() -> SpreadsheetService {,
        SpreadsheetService::new(create_test_config())}
#[test]
    fn test_delete_dimension_range_request_builder_creation() {
let builder = DeleteDimensionRangeRequest::builder();
        let request = builder.build();

        assert_eq!(request.spreadsheet_token, "");
        assert_eq!(request.dimension.sheet_id, "");
        assert_eq!(request.dimension.major_dimension, "");
        assert_eq!(request.dimension.start_index, 0);
        assert_eq!(request.dimension.end_index, 0);
#[test]
    fn test_delete_dimension_range_request_builder_with_spreadsheet_token() {
let request = DeleteDimensionRangeRequest::builder(),
            .spreadsheet_token()
.build();
        assert_eq!(request.spreadsheet_token, "test_spreadsheet_123");
#[test]
    fn test_delete_dimension_range_request_builder_with_sheet_id() {
let request = DeleteDimensionRangeRequest::builder(),
            .sheet_id()
.build();
        assert_eq!(request.dimension.sheet_id, "test_sheet_456");
#[rstest]
    #[case("ROWS")]
#[case("COLUMNS")]
    fn test_delete_dimension_range_request_builder_with_major_dimension(#[case] dimension: &str) {,
let request = DeleteDimensionRangeRequest::builder(),
            .major_dimension()
.build();
        assert_eq!(request.dimension.major_dimension, dimension);
#[test]
    fn test_delete_dimension_range_request_builder_with_start_index() {
let request = DeleteDimensionRangeRequest::builder(),
            .start_index()
.build();
        assert_eq!(request.dimension.start_index, 5);
#[test]
    fn test_delete_dimension_range_request_builder_with_end_index() {
let request = DeleteDimensionRangeRequest::builder().end_index(10).build();
        assert_eq!(request.dimension.end_index, 10);
#[test]
    fn test_delete_dimension_range_request_builder_chaining() {
let request = DeleteDimensionRangeRequest::builder(),
            .spreadsheet_token()
.sheet_id()
            .major_dimension()
.start_index()
            .end_index()
.build();
        assert_eq!(request.spreadsheet_token, "my_spreadsheet");
        assert_eq!(request.dimension.sheet_id, "my_sheet");
        assert_eq!(request.dimension.major_dimension, "ROWS");
        assert_eq!(request.dimension.start_index, 3);
        assert_eq!(request.dimension.end_index, 7);
#[test]
    fn test_delete_dimension_range_request_default() {
let request = DeleteDimensionRangeRequest::default();
        assert_eq!(request.spreadsheet_token, "");
        assert_eq!(request.dimension.sheet_id, "");
        assert_eq!(request.dimension.major_dimension, "");
        assert_eq!(request.dimension.start_index, 0);
        assert_eq!(request.dimension.end_index, 0);
#[test]
    fn test_delete_dimension_range_request_builder_default() {
let builder = DeleteDimensionRangeRequestBuilder::default();
        let request = builder.build();

        assert_eq!(request.spreadsheet_token, "");
        assert_eq!(request.dimension.sheet_id, "");
        assert_eq!(request.dimension.major_dimension, "");
        assert_eq!(request.dimension.start_index, 0);
        assert_eq!(request.dimension.end_index, 0);
#[test]
    fn test_delete_dimension_range_request_serialization() {
let request = DeleteDimensionRangeRequest::builder(),
            .spreadsheet_token()
.sheet_id()
            .major_dimension()
.start_index()
            .end_index()
.build();
        // Test that the request can be serialized (this validates field names),
let serialized = serde_json::to_string(&request);
        assert!(serialized.is_ok());
let json_str = serialized.unwrap();
        assert!(json_str.contains("dimension"));
assert!(json_str.contains("COLUMNS"));
        assert!(json_str.contains("\"startIndex\":2"));
assert!(json_str.contains("\"endIndex\":5"));
    }
#[test]
    fn test_delete_dimension_range_request_debug() {
let request = DeleteDimensionRangeRequest::builder(),
            .spreadsheet_token()
.sheet_id()
            .build();

        let debug_str = format!("{:?}", request);
assert!(debug_str.contains("DeleteDimensionRangeRequest"));
        assert!(debug_str.contains("debug_token"));
assert!(debug_str.contains("debug_sheet"));
    }
#[test]
    fn test_delete_dimension_range_request_with_empty_strings() {
let request = DeleteDimensionRangeRequest::builder(),
            .spreadsheet_token()
.sheet_id()
            .major_dimension()
.build();
        assert_eq!(request.spreadsheet_token, "");
        assert_eq!(request.dimension.sheet_id, "");
        assert_eq!(request.dimension.major_dimension, "");
#[test]
    fn test_delete_dimension_range_request_with_special_characters() {
let request = DeleteDimensionRangeRequest::builder(),
            .spreadsheet_token()
.sheet_id()
            .major_dimension()
.build();
        assert_eq!(request.spreadsheet_token, "token_with_特殊字符_🎯");
        assert_eq!(request.dimension.sheet_id, "sheet_名称_123");
        assert_eq!(request.dimension.major_dimension, "ROWS");
#[rstest]
    #[case(0, 1)]
    #[case(1, 5)]
    #[case(10, 20)]
    #[case(100, 200)]
    #[case(-1, 0)] // Edge case: negative start
    #[case(5, 5)] // Edge case: start equals end,
fn test_delete_dimension_range_request_with_various_indices(,
        #[case] start: i32,
        #[case] end: i32,
    ) {,
let request = DeleteDimensionRangeRequest::builder(),
            .start_index()
.end_index()
            .build();

        assert_eq!(request.dimension.start_index, start);
        assert_eq!(request.dimension.end_index, end);
#[test]
    fn test_delete_dimension_range_request_with_maximum_values() {
let request = DeleteDimensionRangeRequest::builder(),
            .start_index()
.end_index()
            .build();

        assert_eq!(request.dimension.start_index, i32::MAX);
        assert_eq!(request.dimension.end_index, i32::MAX);
#[test]
    fn test_delete_dimension_range_request_with_minimum_values() {
let request = DeleteDimensionRangeRequest::builder(),
            .start_index()
.end_index()
            .build();

        assert_eq!(request.dimension.start_index, i32::MIN);
        assert_eq!(request.dimension.end_index, i32::MIN);
#[test]
    fn test_delete_dimension_range_request_api_request_body_serialization() {
let request = DeleteDimensionRangeRequest::builder(),
            .spreadsheet_token()
.sheet_id()
            .major_dimension()
.start_index()
            .end_index()
.build();
        // Verify that api_request.body is set correctly,
assert!(!request.api_request.body.is_empty());
        // Verify that the body contains valid JSON,
let body_str = String::from_utf8(request.api_request.body).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&body_str).unwrap();
assert!(parsed.get("dimension").is_some());
        let dimension = parsed.get("dimension").unwrap();
        assert_eq!(dimension.get("majorDimension").unwrap(), "ROWS");
        assert_eq!(dimension.get("startIndex").unwrap(), 1);
        assert_eq!(dimension.get("endIndex").unwrap(), 3);
#[test]
    fn test_delete_dimension_range_request_builder_multiple_calls() {
let mut builder = DeleteDimensionRangeRequest::builder();
        // Test that multiple calls override previous values,
builder = builder.spreadsheet_token("first_token");
        builder = builder.spreadsheet_token("second_token");
builder = builder.sheet_id("first_sheet");
        builder = builder.sheet_id("second_sheet");
builder = builder.start_index(1);
        builder = builder.start_index(2);
let request = builder.build();
        assert_eq!(request.spreadsheet_token, "second_token");
        assert_eq!(request.dimension.sheet_id, "second_sheet");
        assert_eq!(request.dimension.start_index, 2);
#[test]
    fn test_spreadsheet_service_creation() {
let service = create_test_service();
        // Verify the service can be created without panic
        assert_eq!(service.config.app_id, "test_app_id");
#[test]
    fn test_delete_dimension_range_response_data_format() {
assert_eq!(,
            DeleteDimensionRangeResponse::data_format(),
            ResponseFormat::Data
);
    }
#[test]
    ,
        let json_response = r#"{"delCount": 5, "majorDimension": "ROWS"}"#;
let response: DeleteDimensionRangeResponse = serde_json::from_str(json_response).unwrap();
        assert_eq!(response.del_count, 5);
        assert_eq!(response.major_dimension, "ROWS");
#[test]
    ,
        let json_response = r#"{"delCount": 3, "majorDimension": "COLUMNS"}"#;
let response: DeleteDimensionRangeResponse = serde_json::from_str(json_response).unwrap();
        assert_eq!(response.del_count, 3);
        assert_eq!(response.major_dimension, "COLUMNS");
#[test]
    fn test_delete_dimension_range_response_debug() {
let response = DeleteDimensionRangeResponse {,
            del_count: 10,
            major_dimension: "ROWS".to_string()};

        let debug_str = format!("{:?}", response);
assert!(debug_str.contains("DeleteDimensionRangeResponse"));
        assert!(debug_str.contains("del_count: 10"));
assert!(debug_str.contains("ROWS"));
    }
#[test]
    ,
        let json_response = r#"{"delCount": 0, "majorDimension": "ROWS"}"#;
let response: DeleteDimensionRangeResponse = serde_json::from_str(json_response).unwrap();
        assert_eq!(response.del_count, 0);
        assert_eq!(response.major_dimension, "ROWS");
#[test]
    ,
        let json_response = r#"{"delCount": 999999, "majorDimension": "COLUMNS"}"#;
let response: DeleteDimensionRangeResponse = serde_json::from_str(json_response).unwrap();
        assert_eq!(response.del_count, 999999);
        assert_eq!(response.major_dimension, "COLUMNS");
#[test]
    fn test_delete_dimension_range_request_edge_cases() {
// Test with very long token,
        let long_token = "a".repeat(10000);
let request = DeleteDimensionRangeRequest::builder(),
            .spreadsheet_token()
.build();
        assert_eq!(request.spreadsheet_token, long_token);
// Test with very long sheet ID,
        let long_sheet_id = "sheet_".repeat(1000);
let request = DeleteDimensionRangeRequest::builder(),
            .sheet_id()
.build();
        assert_eq!(request.dimension.sheet_id, long_sheet_id);
// Test with extreme index values,
        let request = DeleteDimensionRangeRequest::builder(),
.start_index()
            .end_index()
.build();
        assert_eq!(request.dimension.start_index, 0);
        assert_eq!(request.dimension.end_index, 1000000);
#[test]
    fn test_delete_dimension_range_request_memory_efficiency() {
// Test creating many requests doesn't consume excessive memory,
        let requests: Vec<DeleteDimensionRangeRequest> = (0..100),
.map(|i| {,
                DeleteDimensionRangeRequest::builder()
                    .spreadsheet_token(format!("token_{}", i))
                    .sheet_id(format!("sheet_{}", i))
                    .major_dimension()
.start_index()
                    .end_index()
.build(),
            }),
.collect();
        assert_eq!(requests.len(), 100);
// Verify each request has correct data,
        for (i, request) in requests.iter().enumerate() {
            assert_eq!(request.spreadsheet_token, format!("token_{}", i));
            assert_eq!(request.dimension.sheet_id, format!("sheet_{}", i));
            assert_eq!(request.dimension.start_index, i as i32);
            assert_eq!(request.dimension.end_index, (i + 10) as i32);
    }

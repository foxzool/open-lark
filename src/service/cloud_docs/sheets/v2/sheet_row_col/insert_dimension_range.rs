#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(non_snake_case)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::module_inception)]
use serde::Serialize;
use open_lark_core::core::api_req::ApiRequest;
use crate::{,
core::{,
        api_resp::{BaseResponse, EmptyResponse}
        constants::AccessTokenType,
        endpoints::cloud_docs::*,
        req_option, SDKResult,
    }
    impl_executable_builder_owned,
    service::cloud_docs::sheets::v2::{sheet_row_col::UpdateDimension, SpreadsheetService}
};
/// 插入行列请求,
#[derive(Debug, Clone)]
pub struct InsertDimensionRangeRequest {
    #[serde(skip)]
    api_request: ApiRequest,
    #[serde(skip)]
    spreadsheet_token: String,
    /// 需要插入行列的维度信息
    dimension: UpdateDimension,
    /// 插入的空白行或列是否继承表中的单元格样式。不填或设置为空即不继承任何样式，为默认空白样式。,
/// 可选值：,
    /// - BEFORE：继承起始位置的单元格的样式,
/// - AFTER：继承结束位置的单元格的样式,
    #[serde(rename = "inheritStyle", skip_serializing_if = "Option::is_none")]
    inherit_style: Option<String>}
impl InsertDimensionRangeRequest {
    pub fn new(config: Config) -> Self {
        Self { config }
}#[derive(Debug, Clone)]
pub struct InsertDimensionRangeRequestBuilder {
    request: InsertDimensionRangeRequest}
impl InsertDimensionRangeRequestBuilder {
    pub fn new(config: Config) -> Self {
        Self { config }
}// 使用宏实现ExecutableBuilder trait,
impl_executable_builder_owned!(
    InsertDimensionRangeRequestBuilder,
    SpreadsheetService,
    InsertDimensionRangeRequest,
    BaseResponse<EmptyResponse>,
    insert_dimension_range,
);
impl SpreadsheetService {
    pub fn new(config: Config) -> Self {
        Self { config }
}#[cfg(test)]
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
    fn test_insert_dimension_range_request_builder_creation() {
let builder = InsertDimensionRangeRequest::builder();
        let request = builder.build();

        assert_eq!(request.spreadsheet_token, "");
        assert_eq!(request.dimension.sheet_id, "");
        assert_eq!(request.dimension.major_dimension, "");
        assert_eq!(request.dimension.start_index, 0);
        assert_eq!(request.dimension.end_index, 0);
assert!(request.inherit_style.is_none());
    }
#[test]
    fn test_insert_dimension_range_request_builder_with_spreadsheet_token() {
let request = InsertDimensionRangeRequest::builder(),
            .spreadsheet_token()
.build();
        assert_eq!(request.spreadsheet_token, "test_spreadsheet_123");
#[test]
    fn test_insert_dimension_range_request_builder_with_sheet_id() {
let request = InsertDimensionRangeRequest::builder(),
            .sheet_id()
.build();
        assert_eq!(request.dimension.sheet_id, "test_sheet_456");
#[rstest]
    #[case("ROWS")]
#[case("COLUMNS")]
    fn test_insert_dimension_range_request_builder_with_major_dimension(#[case] dimension: &str) {,
let request = InsertDimensionRangeRequest::builder(),
            .major_dimension()
.build();
        assert_eq!(request.dimension.major_dimension, dimension);
#[test]
    fn test_insert_dimension_range_request_builder_with_start_index() {
let request = InsertDimensionRangeRequest::builder(),
            .start_index()
.build();
        assert_eq!(request.dimension.start_index, 5);
#[test]
    fn test_insert_dimension_range_request_builder_with_end_index() {
let request = InsertDimensionRangeRequest::builder().end_index(10).build();
        assert_eq!(request.dimension.end_index, 10);
#[rstest]
    #[case("BEFORE")]
#[case("AFTER")]
    fn test_insert_dimension_range_request_builder_with_inherit_style(#[case] style: &str) {,
let request = InsertDimensionRangeRequest::builder(),
            .inherit_style()
.build();
        assert_eq!(request.inherit_style, Some(style.to_string()));
#[test]
    fn test_insert_dimension_range_request_builder_chaining() {
let request = InsertDimensionRangeRequest::builder(),
            .spreadsheet_token()
.sheet_id()
            .major_dimension()
.start_index()
            .end_index()
.inherit_style()
            .build();

        assert_eq!(request.spreadsheet_token, "my_spreadsheet");
        assert_eq!(request.dimension.sheet_id, "my_sheet");
        assert_eq!(request.dimension.major_dimension, "ROWS");
        assert_eq!(request.dimension.start_index, 3);
        assert_eq!(request.dimension.end_index, 7);
        assert_eq!(request.inherit_style, Some("BEFORE".to_string()));
#[test]
    fn test_insert_dimension_range_request_default() {
let request = InsertDimensionRangeRequest::default();
        assert_eq!(request.spreadsheet_token, "");
        assert_eq!(request.dimension.sheet_id, "");
        assert_eq!(request.dimension.major_dimension, "");
        assert_eq!(request.dimension.start_index, 0);
        assert_eq!(request.dimension.end_index, 0);
assert!(request.inherit_style.is_none());
    }
#[test]
    fn test_insert_dimension_range_request_builder_default() {
let builder = InsertDimensionRangeRequestBuilder::default();
        let request = builder.build();

        assert_eq!(request.spreadsheet_token, "");
        assert_eq!(request.dimension.sheet_id, "");
        assert_eq!(request.dimension.major_dimension, "");
        assert_eq!(request.dimension.start_index, 0);
        assert_eq!(request.dimension.end_index, 0);
assert!(request.inherit_style.is_none());
    }
#[test]
    fn test_insert_dimension_range_request_serialization() {
let request = InsertDimensionRangeRequest::builder(),
            .spreadsheet_token()
.sheet_id()
            .major_dimension()
.start_index()
            .end_index()
.inherit_style()
            .build();
// Test that the request can be serialized (this validates field names),
        let serialized = serde_json::to_string(&request);
assert!(serialized.is_ok());
        let json_str = serialized.unwrap();
assert!(json_str.contains("dimension"));
        assert!(json_str.contains("COLUMNS"));
assert!(json_str.contains("\"startIndex\":2"));
        assert!(json_str.contains("\"endIndex\":5"));
assert!(json_str.contains("\"inheritStyle\":\"AFTER\""));
    }
#[test]
    fn test_insert_dimension_range_request_serialization_without_inherit_style() {
let request = InsertDimensionRangeRequest::builder(),
            .spreadsheet_token()
.sheet_id()
            .major_dimension()
.start_index()
            .end_index()
.build();
        let serialized = serde_json::to_string(&request);
assert!(serialized.is_ok());
        let json_str = serialized.unwrap();
assert!(json_str.contains("dimension"));
        assert!(json_str.contains("ROWS"));
assert!(!json_str.contains("inheritStyle")); // Should not be present when None}
#[test]
    fn test_insert_dimension_range_request_debug() {
let request = InsertDimensionRangeRequest::builder(),
            .spreadsheet_token()
.sheet_id()
            .inherit_style()
.build();
        let debug_str = format!("{:?}", request);
assert!(debug_str.contains("InsertDimensionRangeRequest"));
        assert!(debug_str.contains("debug_token"));
assert!(debug_str.contains("debug_sheet"));
        assert!(debug_str.contains("BEFORE"));
#[test]
    fn test_insert_dimension_range_request_with_empty_strings() {
let request = InsertDimensionRangeRequest::builder(),
            .spreadsheet_token()
.sheet_id()
            .major_dimension()
.inherit_style()
            .build();

        assert_eq!(request.spreadsheet_token, "");
        assert_eq!(request.dimension.sheet_id, "");
        assert_eq!(request.dimension.major_dimension, "");
        assert_eq!(request.inherit_style, Some("".to_string()));
#[test]
    fn test_insert_dimension_range_request_with_special_characters() {
let request = InsertDimensionRangeRequest::builder(),
            .spreadsheet_token()
.sheet_id()
            .major_dimension()
.inherit_style()
            .build();

        assert_eq!(request.spreadsheet_token, "token_with_特殊字符_🎯");
        assert_eq!(request.dimension.sheet_id, "sheet_名称_123");
        assert_eq!(request.dimension.major_dimension, "ROWS");
        assert_eq!(request.inherit_style, Some("BEFORE".to_string()));
#[rstest]
    #[case(0, 1)]
    #[case(1, 5)]
    #[case(10, 20)]
    #[case(100, 200)]
    #[case(-1, 0)] // Edge case: negative start
    #[case(5, 5)] // Edge case: start equals end,
fn test_insert_dimension_range_request_with_various_indices(,
        #[case] start: i32,
        #[case] end: i32,
    ) {,
let request = InsertDimensionRangeRequest::builder(),
            .start_index()
.end_index()
            .build();

        assert_eq!(request.dimension.start_index, start);
        assert_eq!(request.dimension.end_index, end);
#[test]
    fn test_insert_dimension_range_request_with_maximum_values() {
let request = InsertDimensionRangeRequest::builder(),
            .start_index()
.end_index()
            .build();

        assert_eq!(request.dimension.start_index, i32::MAX);
        assert_eq!(request.dimension.end_index, i32::MAX);
#[test]
    fn test_insert_dimension_range_request_with_minimum_values() {
let request = InsertDimensionRangeRequest::builder(),
            .start_index()
.end_index()
            .build();

        assert_eq!(request.dimension.start_index, i32::MIN);
        assert_eq!(request.dimension.end_index, i32::MIN);
#[test]
    fn test_insert_dimension_range_request_api_request_body_serialization() {
let request = InsertDimensionRangeRequest::builder(),
            .spreadsheet_token()
.sheet_id()
            .major_dimension()
.start_index()
            .end_index()
.inherit_style()
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
        assert_eq!(parsed.get("inheritStyle").unwrap(), "AFTER");
#[test]
    fn test_insert_dimension_range_request_builder_multiple_calls() {
let mut builder = InsertDimensionRangeRequest::builder();
        // Test that multiple calls override previous values,
builder = builder.spreadsheet_token("first_token");
        builder = builder.spreadsheet_token("second_token");
builder = builder.sheet_id("first_sheet");
        builder = builder.sheet_id("second_sheet");
builder = builder.start_index(1);
        builder = builder.start_index(2);
builder = builder.inherit_style("BEFORE");
        builder = builder.inherit_style("AFTER");
let request = builder.build();
        assert_eq!(request.spreadsheet_token, "second_token");
        assert_eq!(request.dimension.sheet_id, "second_sheet");
        assert_eq!(request.dimension.start_index, 2);
        assert_eq!(request.inherit_style, Some("AFTER".to_string()));
#[test]
    fn test_spreadsheet_service_creation() {
let service = create_test_service();
        // Verify the service can be created without panic
        assert_eq!(service.config.app_id, "test_app_id");
#[test]
    fn test_insert_dimension_range_request_edge_cases() {
// Test with very long token,
        let long_token = "a".repeat(10000);
let request = InsertDimensionRangeRequest::builder(),
            .spreadsheet_token()
.build();
        assert_eq!(request.spreadsheet_token, long_token);
// Test with very long sheet ID,
        let long_sheet_id = "sheet_".repeat(1000);
let request = InsertDimensionRangeRequest::builder(),
            .sheet_id()
.build();
        assert_eq!(request.dimension.sheet_id, long_sheet_id);
// Test with extreme index values,
        let request = InsertDimensionRangeRequest::builder(),
.start_index()
            .end_index()
.build();
        assert_eq!(request.dimension.start_index, 0);
        assert_eq!(request.dimension.end_index, 1000000);
// Test with very long inherit_style,
        let long_style = "BEFORE_".repeat(100);
let request = InsertDimensionRangeRequest::builder(),
            .inherit_style()
.build();
        assert_eq!(request.inherit_style, Some(long_style));
#[test]
    fn test_insert_dimension_range_request_memory_efficiency() {
// Test creating many requests doesn't consume excessive memory,
        let requests: Vec<InsertDimensionRangeRequest> = (0..100),
.map(|i| {,
                let mut builder = InsertDimensionRangeRequest::builder()
                    .spreadsheet_token(format!("token_{}", i))
                    .sheet_id(format!("sheet_{}", i))
                    .major_dimension()
.start_index()
                    .end_index(i + 10);
if i % 3 == 0 {,
                    builder = builder.inherit_style("BEFORE");
} else if i % 3 == 1 {,
builder = builder.inherit_style("AFTER");
                }
                // For i % 3 == 2, leave inherit_style as None,
builder.build(),
            }),
.collect();
        assert_eq!(requests.len(), 100);
// Verify each request has correct data,
        for (i, request) in requests.iter().enumerate() {
            assert_eq!(request.spreadsheet_token, format!("token_{}", i));
            assert_eq!(request.dimension.sheet_id, format!("sheet_{}", i));
            assert_eq!(request.dimension.start_index, i as i32);
            assert_eq!(request.dimension.end_index, (i + 10) as i32);
match i % 3 {,
                0 => assert_eq!(request.inherit_style, Some("BEFORE".to_string())),
                1 => assert_eq!(request.inherit_style, Some("AFTER".to_string())),
                2 => assert!(request.inherit_style.is_none()),
                _ => unreachable!()}
    }
#[test]
    fn test_insert_dimension_range_request_serialization_with_null_inherit_style() {
let mut request = InsertDimensionRangeRequest::builder(),
            .spreadsheet_token()
.sheet_id()
            .major_dimension()
.start_index()
            .end_index()
.build();
        // Ensure inherit_style is None,
request.inherit_style = None;
        // Re-serialize manually to test None handling,
request.api_request.body = serde_json::to_vec(&request).unwrap();
        let body_str = String::from_utf8(request.api_request.body).unwrap();
let parsed: serde_json::Value = serde_json::from_str(&body_str).unwrap();
        // inheritStyle should not be present in JSON when None,
assert!(parsed.get("inheritStyle").is_none());
        assert!(parsed.get("dimension").is_some());
#[test]
    fn test_insert_dimension_range_request_various_inherit_styles() {
let styles = vec![,
            "BEFORE",
            "AFTER",
            "before", // Test case sensitivity handling
            "after",
            "INVALID_STYLE", // Test with invalid but accepted string,
];
        for style in styles {,
let request = InsertDimensionRangeRequest::builder(),
                .inherit_style()
.build();
            assert_eq!(request.inherit_style, Some(style.to_string()));
    }
#[test]
    fn test_insert_dimension_range_request_builder_partial_configuration() {
// Test building with only some fields configured,
        let request1 = InsertDimensionRangeRequest::builder(),
.spreadsheet_token()
            .build();

        assert_eq!(request1.spreadsheet_token, "test_token");
        assert_eq!(request1.dimension.sheet_id, "");
assert!(request1.inherit_style.is_none());
        let request2 = InsertDimensionRangeRequest::builder(),
.sheet_id()
            .inherit_style()
.build();
        assert_eq!(request2.spreadsheet_token, "");
        assert_eq!(request2.dimension.sheet_id, "test_sheet");
        assert_eq!(request2.inherit_style, Some("BEFORE".to_string()));
#[test]
    fn test_insert_dimension_range_request_unicode_handling() {
let request = InsertDimensionRangeRequest::builder(),
            .spreadsheet_token()
.sheet_id()
            .major_dimension()
.inherit_style()
            .build();

        assert_eq!(request.spreadsheet_token, "令牌_🔑_test");
        assert_eq!(request.dimension.sheet_id, "工作表_📋_id");
        assert_eq!(request.inherit_style, Some("BEFORE_风格".to_string()));
// Test serialization works with Unicode,
        let serialized = serde_json::to_string(&request);
assert!(serialized.is_ok());
    }

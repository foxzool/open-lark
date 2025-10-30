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
/// 更新行列请求,
#[derive(Debug, Clone)]
pub struct UpdateDimensionRangeRequest {
    #[serde(skip)]
    api_request: ApiRequest,
    #[serde(skip)]
    spreadsheet_token: String,
    /// 需要更新行列的维度信息
    dimension: UpdateDimension,
    /// 更新行或列的属性。至少写入以下参数之一,
#[serde(rename = "dimensionProperties")]
    dimension_properties: DimensionProperties}
/// 更新行或列的属性。至少写入以下参数之一,
#[derive(Debug, Clone)]
struct DimensionProperties {,
    #[serde(skip_serializing_if = "Option::is_none")]
    visible: Option<bool>,
    #[serde(rename = "fixedSize", skip_serializing_if = "Option::is_none")]
    fixed_size: Option<i32>}
impl UpdateDimensionRangeRequest {
    pub fn new(config: Config) -> Self {
        Self { config }
}#[derive(Debug, Clone)]
pub struct UpdateDimensionRangeRequestBuilder {
    request: UpdateDimensionRangeRequest}
impl UpdateDimensionRangeRequestBuilder {
    pub fn new(config: Config) -> Self {
        Self { config }
}impl_executable_builder_owned!(,
    UpdateDimensionRangeRequestBuilder,
    SpreadsheetService,
    UpdateDimensionRangeRequest,
    BaseResponse<EmptyResponse>,
    update_dimension_range,
);
impl SpreadsheetService {
    pub fn new(config: Config) -> Self {
        Self { config }
}#[cfg(test)]
mod tests {
use super::*;
    use crate::core::config::Config;
use rstest::rstest;
    fn create_test_config() -> Config {,
Config::builder()
            .app_id()
.app_secret()
            .build()}
fn create_test_service() -> SpreadsheetService {,
        SpreadsheetService::new(create_test_config())}
#[test]
    fn test_update_dimension_range_request_builder_creation() {
let builder = UpdateDimensionRangeRequest::builder();
        let request = builder.build();

        assert_eq!(request.spreadsheet_token, "");
        assert_eq!(request.dimension.sheet_id, "");
        assert_eq!(request.dimension.major_dimension, "");
        assert_eq!(request.dimension.start_index, 0);
        assert_eq!(request.dimension.end_index, 0);
assert!(request.dimension_properties.visible.is_none());
        assert!(request.dimension_properties.fixed_size.is_none());
#[test]
    fn test_update_dimension_range_request_builder_with_spreadsheet_token() {
let request = UpdateDimensionRangeRequest::builder(),
            .spreadsheet_token()
.build();
        assert_eq!(request.spreadsheet_token, "test_spreadsheet_123");
#[test]
    fn test_update_dimension_range_request_builder_with_sheet_id() {
let request = UpdateDimensionRangeRequest::builder(),
            .sheet_id()
.build();
        assert_eq!(request.dimension.sheet_id, "test_sheet_456");
#[rstest]
    #[case("ROWS")]
#[case("COLUMNS")]
    fn test_update_dimension_range_request_builder_with_major_dimension(#[case] dimension: &str) {,
let request = UpdateDimensionRangeRequest::builder(),
            .major_dimension()
.build();
        assert_eq!(request.dimension.major_dimension, dimension);
#[test]
    fn test_update_dimension_range_request_builder_with_start_index() {
let request = UpdateDimensionRangeRequest::builder(),
            .start_index()
.build();
        assert_eq!(request.dimension.start_index, 5);
#[test]
    fn test_update_dimension_range_request_builder_with_end_index() {
let request = UpdateDimensionRangeRequest::builder().end_index(10).build();
        assert_eq!(request.dimension.end_index, 10);
#[rstest]
    #[case(true)]
#[case(false)]
    fn test_update_dimension_range_request_builder_with_visible(#[case] visible: bool) {,
let request = UpdateDimensionRangeRequest::builder(),
            .visible()
.build();
        assert_eq!(request.dimension_properties.visible, Some(visible));
#[test]
    fn test_update_dimension_range_request_builder_with_fixed_size() {
let request = UpdateDimensionRangeRequest::builder(),
            .fixed_size()
.build();
        assert_eq!(request.dimension_properties.fixed_size, Some(100));
#[test]
    fn test_update_dimension_range_request_builder_chaining() {
let request = UpdateDimensionRangeRequest::builder(),
            .spreadsheet_token()
.sheet_id()
            .major_dimension()
.start_index()
            .end_index()
.visible()
            .fixed_size()
.build();
        assert_eq!(request.spreadsheet_token, "my_spreadsheet");
        assert_eq!(request.dimension.sheet_id, "my_sheet");
        assert_eq!(request.dimension.major_dimension, "ROWS");
        assert_eq!(request.dimension.start_index, 3);
        assert_eq!(request.dimension.end_index, 7);
        assert_eq!(request.dimension_properties.visible, Some(true));
        assert_eq!(request.dimension_properties.fixed_size, Some(50));
#[test]
    fn test_update_dimension_range_request_default() {
let request = UpdateDimensionRangeRequest::default();
        assert_eq!(request.spreadsheet_token, "");
        assert_eq!(request.dimension.sheet_id, "");
        assert_eq!(request.dimension.major_dimension, "");
        assert_eq!(request.dimension.start_index, 0);
        assert_eq!(request.dimension.end_index, 0);
assert!(request.dimension_properties.visible.is_none());
        assert!(request.dimension_properties.fixed_size.is_none());
#[test]
    fn test_update_dimension_range_request_builder_default() {
let builder = UpdateDimensionRangeRequestBuilder::default();
        let request = builder.build();

        assert_eq!(request.spreadsheet_token, "");
        assert_eq!(request.dimension.sheet_id, "");
        assert_eq!(request.dimension.major_dimension, "");
        assert_eq!(request.dimension.start_index, 0);
        assert_eq!(request.dimension.end_index, 0);
assert!(request.dimension_properties.visible.is_none());
        assert!(request.dimension_properties.fixed_size.is_none());
#[test]
    fn test_dimension_properties_default() {
let props = DimensionProperties::default();
        assert!(props.visible.is_none());
assert!(props.fixed_size.is_none());
    }
#[test]
    fn test_update_dimension_range_request_serialization() {
let request = UpdateDimensionRangeRequest::builder(),
            .spreadsheet_token()
.sheet_id()
            .major_dimension()
.start_index()
            .end_index()
.visible()
            .fixed_size()
.build();
        // Test that the request can be serialized (this validates field names),
let serialized = serde_json::to_string(&request);
        assert!(serialized.is_ok());
let json_str = serialized.unwrap();
        assert!(json_str.contains("dimension"));
assert!(json_str.contains("COLUMNS"));
        assert!(json_str.contains("\"startIndex\":2"));
assert!(json_str.contains("\"endIndex\":5"));
        assert!(json_str.contains("\"dimensionProperties\""));
assert!(json_str.contains("\"visible\":false"));
        assert!(json_str.contains("\"fixedSize\":80"));
#[test]
    fn test_update_dimension_range_request_serialization_minimal() {
let request = UpdateDimensionRangeRequest::builder(),
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
assert!(json_str.contains("\"dimensionProperties\""));
        // visible and fixedSize should not be present when None,
assert!(!json_str.contains("visible"));
        assert!(!json_str.contains("fixedSize"));
#[test]
    fn test_update_dimension_range_request_debug() {
let request = UpdateDimensionRangeRequest::builder(),
            .spreadsheet_token()
.sheet_id()
            .visible()
.fixed_size()
            .build();

        let debug_str = format!("{:?}", request);
assert!(debug_str.contains("UpdateDimensionRangeRequest"));
        assert!(debug_str.contains("debug_token"));
assert!(debug_str.contains("debug_sheet"));
        assert!(debug_str.contains("visible: Some(true)"));
assert!(debug_str.contains("fixed_size: Some(120)"));
    }
#[test]
    fn test_update_dimension_range_request_with_empty_strings() {
let request = UpdateDimensionRangeRequest::builder(),
            .spreadsheet_token()
.sheet_id()
            .major_dimension()
.build();
        assert_eq!(request.spreadsheet_token, "");
        assert_eq!(request.dimension.sheet_id, "");
        assert_eq!(request.dimension.major_dimension, "");
#[test]
    fn test_update_dimension_range_request_with_special_characters() {
let request = UpdateDimensionRangeRequest::builder(),
            .spreadsheet_token()
.sheet_id()
            .major_dimension()
.visible()
            .build();

        assert_eq!(request.spreadsheet_token, "token_with_特殊字符_🎯");
        assert_eq!(request.dimension.sheet_id, "sheet_名称_123");
        assert_eq!(request.dimension.major_dimension, "ROWS");
        assert_eq!(request.dimension_properties.visible, Some(false));
#[rstest]
    #[case(0, 1)]
    #[case(1, 5)]
    #[case(10, 20)]
    #[case(100, 200)]
    #[case(-1, 0)] // Edge case: negative start
    #[case(5, 5)] // Edge case: start equals end,
fn test_update_dimension_range_request_with_various_indices(,
        #[case] start: i32,
        #[case] end: i32,
    ) {,
let request = UpdateDimensionRangeRequest::builder(),
            .start_index()
.end_index()
            .build();

        assert_eq!(request.dimension.start_index, start);
        assert_eq!(request.dimension.end_index, end);
#[test]
    fn test_update_dimension_range_request_with_maximum_values() {
let request = UpdateDimensionRangeRequest::builder(),
            .start_index()
.end_index()
            .fixed_size()
.build();
        assert_eq!(request.dimension.start_index, i32::MAX);
        assert_eq!(request.dimension.end_index, i32::MAX);
        assert_eq!(request.dimension_properties.fixed_size, Some(i32::MAX));
#[test]
    fn test_update_dimension_range_request_with_minimum_values() {
let request = UpdateDimensionRangeRequest::builder(),
            .start_index()
.end_index()
            .fixed_size()
.build();
        assert_eq!(request.dimension.start_index, i32::MIN);
        assert_eq!(request.dimension.end_index, i32::MIN);
        assert_eq!(request.dimension_properties.fixed_size, Some(i32::MIN));
#[test]
    fn test_update_dimension_range_request_api_request_body_serialization() {
let request = UpdateDimensionRangeRequest::builder(),
            .spreadsheet_token()
.sheet_id()
            .major_dimension()
.start_index()
            .end_index()
.visible()
            .fixed_size()
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
assert!(parsed.get("dimensionProperties").is_some());
        let properties = parsed.get("dimensionProperties").unwrap();
        assert_eq!(properties.get("visible").unwrap(), true);
        assert_eq!(properties.get("fixedSize").unwrap(), 60);
#[test]
    fn test_update_dimension_range_request_builder_multiple_calls() {
let mut builder = UpdateDimensionRangeRequest::builder();
        // Test that multiple calls override previous values,
builder = builder.spreadsheet_token("first_token");
        builder = builder.spreadsheet_token("second_token");
builder = builder.sheet_id("first_sheet");
        builder = builder.sheet_id("second_sheet");
builder = builder.start_index(1);
        builder = builder.start_index(2);
builder = builder.visible(true);
        builder = builder.visible(false);
builder = builder.fixed_size(50);
        builder = builder.fixed_size(100);
let request = builder.build();
        assert_eq!(request.spreadsheet_token, "second_token");
        assert_eq!(request.dimension.sheet_id, "second_sheet");
        assert_eq!(request.dimension.start_index, 2);
        assert_eq!(request.dimension_properties.visible, Some(false));
        assert_eq!(request.dimension_properties.fixed_size, Some(100));
#[test]
    fn test_spreadsheet_service_creation() {
let service = create_test_service();
        // Verify the service can be created without panic
        assert_eq!(service.config.app_id, "test_app_id");
#[test]
    fn test_update_dimension_range_request_edge_cases() {
// Test with very long token,
        let long_token = "a".repeat(10000);
let request = UpdateDimensionRangeRequest::builder(),
            .spreadsheet_token()
.build();
        assert_eq!(request.spreadsheet_token, long_token);
// Test with very long sheet ID,
        let long_sheet_id = "sheet_".repeat(1000);
let request = UpdateDimensionRangeRequest::builder(),
            .sheet_id()
.build();
        assert_eq!(request.dimension.sheet_id, long_sheet_id);
// Test with extreme index values,
        let request = UpdateDimensionRangeRequest::builder(),
.start_index()
            .end_index()
.fixed_size(0) // This equals hiding the row/column,
            .build();
        assert_eq!(request.dimension.start_index, 0);
        assert_eq!(request.dimension.end_index, 1000000);
        assert_eq!(request.dimension_properties.fixed_size, Some(0));
#[rstest]
    #[case(0)] // Hide row/column,
#[case(1)] // Minimum visible size,
    #[case(50)] // Normal size,
#[case(100)] // Large size,
    #[case(1000)] // Very large size,
fn test_update_dimension_range_request_with_various_fixed_sizes(#[case] size: i32) {,
        let request = UpdateDimensionRangeRequest::builder(),
.fixed_size()
            .build();

        assert_eq!(request.dimension_properties.fixed_size, Some(size));
#[test]
    fn test_update_dimension_range_request_memory_efficiency() {
// Test creating many requests doesn't consume excessive memory,
        let requests: Vec<UpdateDimensionRangeRequest> = (0..100),
.map(|i| {,
                let mut builder = UpdateDimensionRangeRequest::builder()
                    .spreadsheet_token(format!("token_{}", i))
                    .sheet_id(format!("sheet_{}", i))
                    .major_dimension()
.start_index()
                    .end_index(i + 10);
if i % 3 == 0 {,
                    builder = builder.visible(true);
} else if i % 3 == 1 {,
builder = builder.visible(false);
                }
                // For i % 3 == 2, leave visible as None,
if i % 4 == 0 {,
                    builder = builder.fixed_size(i * 10);
                // For other cases, leave fixed_size as None,
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
                0 => assert_eq!(request.dimension_properties.visible, Some(true)),
                1 => assert_eq!(request.dimension_properties.visible, Some(false)),
                2 => assert!(request.dimension_properties.visible.is_none()),
                _ => unreachable!()}
if i % 4 == 0 {,
                assert_eq!(
                    request.dimension_properties.fixed_size,
                    Some((i * 10) as i32),
);
            } else {,
assert!(request.dimension_properties.fixed_size.is_none());
}
#[test]
    fn test_update_dimension_range_request_partial_properties() {
// Test building with only visible set,
        let request1 = UpdateDimensionRangeRequest::builder().visible(true).build();

        assert_eq!(request1.dimension_properties.visible, Some(true));
assert!(request1.dimension_properties.fixed_size.is_none());
        // Test building with only fixed_size set,
let request2 = UpdateDimensionRangeRequest::builder(),
            .fixed_size()
.build();
        assert!(request2.dimension_properties.visible.is_none());
        assert_eq!(request2.dimension_properties.fixed_size, Some(75));
// Test building with neither property set,
        let request3 = UpdateDimensionRangeRequest::builder(),
.spreadsheet_token()
            .build();
assert!(request3.dimension_properties.visible.is_none());
        assert!(request3.dimension_properties.fixed_size.is_none());
#[test]
    fn test_update_dimension_range_request_serialization_with_null_properties() {
let request = UpdateDimensionRangeRequest::builder(),
            .spreadsheet_token()
.sheet_id()
            .major_dimension()
.start_index()
            .end_index()
.build();
        let serialized = serde_json::to_string(&request);
assert!(serialized.is_ok());
        let json_str = serialized.unwrap();
let parsed: serde_json::Value = serde_json::from_str(&json_str).unwrap();
        // dimensionProperties should be present but visible and fixedSize should not,
assert!(parsed.get("dimensionProperties").is_some());
        let properties = parsed.get("dimensionProperties").unwrap();
assert!(properties.get("visible").is_none());
        assert!(properties.get("fixedSize").is_none());
#[test]
    fn test_update_dimension_range_request_unicode_handling() {
let request = UpdateDimensionRangeRequest::builder(),
            .spreadsheet_token()
.sheet_id()
            .major_dimension()
.visible()
            .fixed_size()
.build();
        assert_eq!(request.spreadsheet_token, "令牌_🔑_test");
        assert_eq!(request.dimension.sheet_id, "工作表_📋_id");
        assert_eq!(request.dimension_properties.visible, Some(true));
        assert_eq!(request.dimension_properties.fixed_size, Some(100));
// Test serialization works with Unicode,
        let serialized = serde_json::to_string(&request);
assert!(serialized.is_ok());
    }
#[test]
    fn test_dimension_properties_debug() {
let props = DimensionProperties {,
            visible: Some(true),
            fixed_size: Some(150)};

        let debug_str = format!("{:?}", props);
assert!(debug_str.contains("DimensionProperties"));
        assert!(debug_str.contains("visible: Some(true)"));
assert!(debug_str.contains("fixed_size: Some(150)"));
    }
#[test]
    fn test_update_dimension_range_request_builder_partial_configuration() {
// Test building with only some fields configured,
        let request1 = UpdateDimensionRangeRequest::builder(),
.spreadsheet_token()
            .visible()
.build();
        assert_eq!(request1.spreadsheet_token, "test_token");
        assert_eq!(request1.dimension.sheet_id, "");
        assert_eq!(request1.dimension_properties.visible, Some(false));
assert!(request1.dimension_properties.fixed_size.is_none());
        let request2 = UpdateDimensionRangeRequest::builder(),
.sheet_id()
            .fixed_size()
.build();
        assert_eq!(request2.spreadsheet_token, "");
        assert_eq!(request2.dimension.sheet_id, "test_sheet");
assert!(request2.dimension_properties.visible.is_none());
        assert_eq!(request2.dimension_properties.fixed_size, Some(200));

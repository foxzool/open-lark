#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_mut)]
#![allow(non_snake_case)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::module_inception)]
use SDKResult;use serde::{Deserialize, Serialize};
use openlark_core::api_req::ApiRequest;
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
    service::cloud_docs::sheets::v2::SpreadsheetService,
};
/// 拆分单元格请求,
#[derive(Debug, Clone)]
pub struct SplitCellsRequest {
    #[serde(skip)]
    api_request: ApiRequest,
    #[serde(skip)]
    spreadsheet_token: String,
    /// 查询范围，包含 sheetId 与单元格范围两部分，目前支持四种索引方式，详见 在线表格开发指南
    range: String}
impl SplitCellsRequest {
    pub fn new(config: Config) -> Self {
        Self { config }
}#[derive(Debug, Clone)]
pub struct SplitCellsRequestBuilder {
    request: SplitCellsRequest}
impl SplitCellsRequestBuilder {
    pub fn new(config: Config) -> Self {
        Self { config }
}
#[derive(Debug, Clone)]
pub struct SplitCellsResponse {
    /// spreadsheet 的 token,
#[serde(rename = "spreadsheetToken")]
    pub spread_sheet_token: String,
impl ApiResponseTrait for.* {
    pub fn new(config: Config) -> Self {
        Self { config }
}    fn data_format() -> ResponseFormat {,
ResponseFormat::Data
    }
impl SpreadsheetService {
    pub fn new(config: Config) -> Self {
        Self { config }
}#[cfg(test)]
mod tests {
use crate::,
{
    core::,
{,
        BaseResponse,
        ResponseFormat,
        api_resp::{ApiResponseTrait,
        config::Config,
        constants::AppType}
    service::cloud_docs::sheets::v2::{
            data_operation::{SplitCellsRequest, SplitCellsResponse};
            SpreadsheetService,
};
fn create_service() -> SpreadsheetService {,
        let config = Config::builder()
.app_id()
            .app_secret()
.app_type()
            .build();
        SpreadsheetService { config }
#[test]
    fn test_split_cells_builder_default() {
let request = SplitCellsRequest::builder().build();
        assert_eq!(request.spreadsheet_token, "");
        assert_eq!(request.range, "");
#[test]
    fn test_split_cells_builder_basic() {
let request = SplitCellsRequest::builder(),
            .spreadsheet_token()
.range()
            .build();

        assert_eq!(request.spreadsheet_token, "test_token");
        assert_eq!(request.range, "Sheet1!A1:B2");
#[test]
    fn test_split_cells_builder_all_options() {
let request = SplitCellsRequest::builder(),
            .spreadsheet_token()
.range()
            .build();

        assert_eq!(request.spreadsheet_token, "spreadsheet_abc123");
        assert_eq!(request.range, "Data!C3:F6");
#[test]
    fn test_split_cells_builder_chaining() {
let request = SplitCellsRequest::builder(),
            .spreadsheet_token()
.range()
            .build();

        assert_eq!(request.spreadsheet_token, "chain_test");
        assert_eq!(request.range, "Summary!A1:D1");
#[test]
    fn test_split_cells_single_merged_cell() {
let request = SplitCellsRequest::builder(),
            .spreadsheet_token()
.range()
            .build();

        assert_eq!(request.spreadsheet_token, "single_cell_test");
        assert_eq!(request.range, "Sheet1!A1:A1");
#[test]
    fn test_split_cells_large_merged_range() {
let request = SplitCellsRequest::builder(),
            .spreadsheet_token()
.range()
            .build();

        assert_eq!(request.spreadsheet_token, "large_range_test");
        assert_eq!(request.range, "Data!A1:Z100");
#[test]
    fn test_split_cells_row_range() {
let request = SplitCellsRequest::builder(),
            .spreadsheet_token()
.range()
            .build();

        assert_eq!(request.spreadsheet_token, "row_range_test");
        assert_eq!(request.range, "Sheet1!A1:Z1");
#[test]
    fn test_split_cells_column_range() {
let request = SplitCellsRequest::builder(),
            .spreadsheet_token()
.range()
            .build();

        assert_eq!(request.spreadsheet_token, "column_range_test");
        assert_eq!(request.range, "Sheet1!A1:A50");
#[test]
    fn test_split_cells_with_unicode_ranges() {
let request = SplitCellsRequest::builder(),
            .spreadsheet_token()
.range()
            .build();

        assert_eq!(request.spreadsheet_token, "unicode_test");
        assert_eq!(request.range, "数据表!A1:D4");
#[test]
    fn test_split_cells_with_special_characters() {
let request = SplitCellsRequest::builder(),
            .spreadsheet_token()
.range()
            .build();

        assert_eq!(request.spreadsheet_token, "special_chars_test");
        assert_eq!(request.range, "'Sheet With Spaces'!A1:B5");
#[test]
    fn test_split_cells_different_sheets() {
let sheets_and_ranges = [,
            ("Sheet1", "A1:B2"),
            ("Summary", "C1:F1"),
            ("Data", "A5:D10"),
            ("第一页", "B3:E8"),
        ];

        for (sheet, range) in sheets_and_ranges.iter() {
            let full_range = format!("{}!{}", sheet, range);
let request = SplitCellsRequest::builder(),
                .spreadsheet_token()
.range()
                .build();

            assert_eq!(request.range, full_range);
    }
#[test]
    fn test_split_cells_serialization() {
let request = SplitCellsRequest::builder(),
            .spreadsheet_token()
.range()
            .build();
let serialized = serde_json::to_string(&request);
        assert!(serialized.is_ok());
let json_value: serde_json::Value = serde_json::from_str(&serialized.unwrap()).unwrap();
        assert_eq!(json_value["range"] "Sheet1!A1:C3");
#[test]
    fn test_split_cells_response_deserialization() {
let response_json = serde_json::json!({,
            "spreadsheetToken": "test_token_123"});
let response: SplitCellsResponse = serde_json::from_value(response_json).unwrap();
        assert_eq!(response.spread_sheet_token, "test_token_123");
#[test]
    fn test_split_cells_complex_range_references() {
let complex_ranges = vec![,
            "Sheet1!A1:D5",
            "'Data Sheet'!B2:F10",
            "工作表!C3:G7",
            "Sheet-Name_123!A10:E15",
            "'Sheet (1)'!D1:H4",
        ];
for range in complex_ranges {,
            let request = SplitCellsRequest::builder(),
.spreadsheet_token()
                .range()
.build();
            assert_eq!(request.range, range);
    }
#[test]
    fn test_split_cells_service_creation() {
let service = create_service();
        assert_eq!(service.config.app_id, "test_app_id");
        assert_eq!(service.config.app_secret, "test_app_secret");
        assert!(matches!(service.config.app_type, AppType::SelfBuild));
#[test]
    fn test_split_cells_builder_overwrites() {
let request = SplitCellsRequest::builder(),
            .spreadsheet_token()
.spreadsheet_token("final_token") // Should overwrite,
            .range()
.range("Sheet2!C3:D4") // Should overwrite,
            .build();

        assert_eq!(request.spreadsheet_token, "final_token");
        assert_eq!(request.range, "Sheet2!C3:D4");
#[test]
    fn test_split_cells_very_long_token() {
let very_long_token = "a".repeat(1000);
        let request = SplitCellsRequest::builder(),
.spreadsheet_token()
            .range()
.build();
        assert_eq!(request.spreadsheet_token, very_long_token);
        assert_eq!(request.range, "Sheet1!A1:B2");
#[test]
    fn test_split_cells_response_struct_debug() {
let response = SplitCellsResponse {,
            spread_sheet_token: "debug_test".to_string()};

        let debug_str = format!("{:?}", response);
assert!(debug_str.contains("debug_test"));
        assert!(debug_str.contains("SplitCellsResponse"));
#[test]
    fn test_split_cells_request_struct_debug() {
let request = SplitCellsRequest::builder(),
            .spreadsheet_token()
.range()
            .build();

        let debug_str = format!("{:?}", request);
assert!(debug_str.contains("debug_token"));
        assert!(debug_str.contains("Sheet1!A1:B2"));
#[test]
    fn test_split_cells_empty_strings() {
let request = SplitCellsRequest::builder(),
            .spreadsheet_token()
.range()
            .build();

        assert_eq!(request.spreadsheet_token, "");
        assert_eq!(request.range, "");
#[test]
    fn test_split_cells_multiple_merged_areas() {
let merged_areas = vec![,
            "Sheet1!A1:B2",  // 2x2 merged area
            "Sheet1!D1:G1",  // Row merge
            "Sheet1!A5:A10", // Column merge
            "Sheet1!C3:E5",  // 3x3 merged area,
];
        for range in merged_areas {,
let request = SplitCellsRequest::builder(),
                .spreadsheet_token()
.range()
                .build();

            assert_eq!(request.range, range);
    }
#[test]
    fn test_split_cells_large_spreadsheet_ranges() {
let large_ranges = vec![,
            "Sheet1!A1:ZZ1000", // Very large range
            "Data!A1:IV65536",  // Excel-style large range
            "Report!AA1:ZZ100", // Wide range with double-letter columns,
];
        for range in large_ranges {,
let request = SplitCellsRequest::builder(),
                .spreadsheet_token()
.range()
                .build();

            assert_eq!(request.range, range);
    }
#[test]
    fn test_split_cells_various_sheet_names() {
let sheet_names_with_ranges = vec![,
            ("普通工作表", "A1:B2"),
            ("Sheet_123", "C1:D5"),
            ("'Special-Name (1)'", "E1:F3"),
            ("数据分析报表", "A10:C15"),
            ("Sheet With Spaces", "G1:H2"),
        ];

        for (sheet_name, range) in sheet_names_with_ranges {,
let full_range = if sheet_name.contains(' ') || sheet_name.contains('(') {,
                format!("'{}'!{}", sheet_name, range),
} else {
                format!("{}!{}", sheet_name, range),
};
let request = SplitCellsRequest::builder(),
                .spreadsheet_token()
.range()
                .build();

            assert_eq!(request.range, full_range);
    }
#[test]
    fn test_split_cells_edge_case_ranges() {
let edge_cases = vec![,
            "Sheet1!A1:A1",     // Single cell
            "Sheet1!A1:B1",     // Two cells horizontal
            "Sheet1!A1:A2",     // Two cells vertical
            "Sheet1!Z99:AA100", // Cross column boundary,
];
        for range in edge_cases {,
let request = SplitCellsRequest::builder(),
                .spreadsheet_token()
.range()
                .build();

            assert_eq!(request.range, range);
    }

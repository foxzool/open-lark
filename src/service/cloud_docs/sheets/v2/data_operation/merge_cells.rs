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

/// 合并单元格请求
#[derive(Serialize, Debug, Default)]
pub struct MergeCellsRequest {
    #[serde(skip)]
    api_request: ApiRequest,
    #[serde(skip)]
    spreadsheet_token: String,
    /// 查询范围，包含 sheetId 与单元格范围两部分，目前支持四种索引方式，详见 在线表格开发指南
    range: String,
    /// 可选三个类型，"MERGE_ALL" 将所选区域直接合并、"MERGE_ROWS"
    /// 将所选区域按行合并、"MERGE_COLUMNS" 将所选区域按列合并响应
    #[serde(rename = "mergeType")]
    merge_type: String,
}

impl MergeCellsRequest {
    pub fn builder() -> MergeCellsRequestBuilder {
        MergeCellsRequestBuilder::default()
    }
}

#[derive(Default)]
pub struct MergeCellsRequestBuilder {
    request: MergeCellsRequest,
}

impl MergeCellsRequestBuilder {
    pub fn spreadsheet_token(mut self, spreadsheet_token: impl ToString) -> Self {
        self.request.spreadsheet_token = spreadsheet_token.to_string();
        self
    }

    /// 查询范围，包含 sheetId 与单元格范围两部分，目前支持四种索引方式，详见 在线表格开发指南
    pub fn range(mut self, range: impl ToString) -> Self {
        self.request.range = range.to_string();
        self
    }

    /// 可选三个类型，"MERGE_ALL" 将所选区域直接合并、"MERGE_ROWS"
    /// 将所选区域按行合并、"MERGE_COLUMNS" 将所选区域按列合并响应
    pub fn merge_type(mut self, merge_type: impl ToString) -> Self {
        self.request.merge_type = merge_type.to_string();
        self
    }

    pub fn build(mut self) -> MergeCellsRequest {
        self.request.api_request.body = serde_json::to_vec(&self.request).unwrap();
        self.request
    }
}

#[derive(Deserialize, Debug)]
pub struct MergeCellsResponse {
    /// spreadsheet 的 token
    #[serde(rename = "spreadsheetToken")]
    pub spread_sheet_token: String,
}

impl ApiResponseTrait for MergeCellsResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl SpreadsheetService {
    /// 合并单元格
    pub async fn merge_cells(
        &self,
        request: MergeCellsRequest,
        option: Option<req_option::RequestOption>,
    ) -> SDKResult<BaseResponse<MergeCellsResponse>> {
        let mut api_req = request.api_request;
        api_req.api_path =
            SHEETS_V2_SPREADSHEET_MERGE_CELLS.replace("{}", &request.spreadsheet_token);
        api_req.http_method = reqwest::Method::POST;
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::App];

        let api_resp = crate::core::http::Transport::request(api_req, &self.config, option).await?;

        Ok(api_resp)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        core::{config::Config, constants::AppType},
        service::cloud_docs::sheets::v2::{
            data_operation::{MergeCellsRequest, MergeCellsResponse},
            SpreadsheetService,
        },
    };

    fn create_service() -> SpreadsheetService {
        let config = Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .app_type(AppType::SelfBuild)
            .build();
        SpreadsheetService { config }
    }

    #[test]
    fn test_merge_cells_builder_default() {
        let request = MergeCellsRequest::builder().build();

        assert_eq!(request.spreadsheet_token, "");
        assert_eq!(request.range, "");
        assert_eq!(request.merge_type, "");
    }

    #[test]
    fn test_merge_cells_builder_basic() {
        let request = MergeCellsRequest::builder()
            .spreadsheet_token("test_token")
            .range("Sheet1!A1:B2")
            .merge_type("MERGE_ALL")
            .build();

        assert_eq!(request.spreadsheet_token, "test_token");
        assert_eq!(request.range, "Sheet1!A1:B2");
        assert_eq!(request.merge_type, "MERGE_ALL");
    }

    #[test]
    fn test_merge_cells_builder_all_options() {
        let request = MergeCellsRequest::builder()
            .spreadsheet_token("spreadsheet_abc123")
            .range("Data!C3:F6")
            .merge_type("MERGE_ROWS")
            .build();

        assert_eq!(request.spreadsheet_token, "spreadsheet_abc123");
        assert_eq!(request.range, "Data!C3:F6");
        assert_eq!(request.merge_type, "MERGE_ROWS");
    }

    #[test]
    fn test_merge_cells_builder_chaining() {
        let request = MergeCellsRequest::builder()
            .spreadsheet_token("chain_test")
            .range("Summary!A1:D1")
            .merge_type("MERGE_COLUMNS")
            .build();

        assert_eq!(request.spreadsheet_token, "chain_test");
        assert_eq!(request.range, "Summary!A1:D1");
        assert_eq!(request.merge_type, "MERGE_COLUMNS");
    }

    #[test]
    fn test_merge_cells_merge_all_type() {
        let request = MergeCellsRequest::builder()
            .spreadsheet_token("merge_all_test")
            .range("Sheet1!A1:C3")
            .merge_type("MERGE_ALL")
            .build();

        assert_eq!(request.spreadsheet_token, "merge_all_test");
        assert_eq!(request.range, "Sheet1!A1:C3");
        assert_eq!(request.merge_type, "MERGE_ALL");
    }

    #[test]
    fn test_merge_cells_merge_rows_type() {
        let request = MergeCellsRequest::builder()
            .spreadsheet_token("merge_rows_test")
            .range("Report!B2:E5")
            .merge_type("MERGE_ROWS")
            .build();

        assert_eq!(request.spreadsheet_token, "merge_rows_test");
        assert_eq!(request.range, "Report!B2:E5");
        assert_eq!(request.merge_type, "MERGE_ROWS");
    }

    #[test]
    fn test_merge_cells_merge_columns_type() {
        let request = MergeCellsRequest::builder()
            .spreadsheet_token("merge_columns_test")
            .range("Table!A1:A10")
            .merge_type("MERGE_COLUMNS")
            .build();

        assert_eq!(request.spreadsheet_token, "merge_columns_test");
        assert_eq!(request.range, "Table!A1:A10");
        assert_eq!(request.merge_type, "MERGE_COLUMNS");
    }

    #[test]
    fn test_merge_cells_with_unicode_ranges() {
        let request = MergeCellsRequest::builder()
            .spreadsheet_token("unicode_test")
            .range("数据表!A1:D4")
            .merge_type("MERGE_ALL")
            .build();

        assert_eq!(request.spreadsheet_token, "unicode_test");
        assert_eq!(request.range, "数据表!A1:D4");
        assert_eq!(request.merge_type, "MERGE_ALL");
    }

    #[test]
    fn test_merge_cells_with_special_characters() {
        let request = MergeCellsRequest::builder()
            .spreadsheet_token("special_chars_test")
            .range("'Sheet With Spaces'!A1:B5")
            .merge_type("MERGE_ROWS")
            .build();

        assert_eq!(request.spreadsheet_token, "special_chars_test");
        assert_eq!(request.range, "'Sheet With Spaces'!A1:B5");
        assert_eq!(request.merge_type, "MERGE_ROWS");
    }

    #[test]
    fn test_merge_cells_single_cell_range() {
        let request = MergeCellsRequest::builder()
            .spreadsheet_token("single_cell_test")
            .range("Sheet1!A1:A1")
            .merge_type("MERGE_ALL")
            .build();

        assert_eq!(request.spreadsheet_token, "single_cell_test");
        assert_eq!(request.range, "Sheet1!A1:A1");
        assert_eq!(request.merge_type, "MERGE_ALL");
    }

    #[test]
    fn test_merge_cells_large_range() {
        let request = MergeCellsRequest::builder()
            .spreadsheet_token("large_range_test")
            .range("Data!A1:Z100")
            .merge_type("MERGE_ALL")
            .build();

        assert_eq!(request.spreadsheet_token, "large_range_test");
        assert_eq!(request.range, "Data!A1:Z100");
        assert_eq!(request.merge_type, "MERGE_ALL");
    }

    #[test]
    fn test_merge_cells_row_range() {
        let request = MergeCellsRequest::builder()
            .spreadsheet_token("row_range_test")
            .range("Sheet1!A1:Z1")
            .merge_type("MERGE_ROWS")
            .build();

        assert_eq!(request.spreadsheet_token, "row_range_test");
        assert_eq!(request.range, "Sheet1!A1:Z1");
        assert_eq!(request.merge_type, "MERGE_ROWS");
    }

    #[test]
    fn test_merge_cells_column_range() {
        let request = MergeCellsRequest::builder()
            .spreadsheet_token("column_range_test")
            .range("Sheet1!A1:A50")
            .merge_type("MERGE_COLUMNS")
            .build();

        assert_eq!(request.spreadsheet_token, "column_range_test");
        assert_eq!(request.range, "Sheet1!A1:A50");
        assert_eq!(request.merge_type, "MERGE_COLUMNS");
    }

    #[test]
    fn test_merge_cells_different_sheets() {
        let sheets_and_ranges = [
            ("Sheet1", "A1:B2"),
            ("Summary", "C1:F1"),
            ("Data", "A5:D10"),
            ("第一页", "B3:E8"),
        ];

        for (sheet, range) in sheets_and_ranges.iter() {
            let full_range = format!("{}!{}", sheet, range);
            let request = MergeCellsRequest::builder()
                .spreadsheet_token("multi_sheet_test")
                .range(&full_range)
                .merge_type("MERGE_ALL")
                .build();

            assert_eq!(request.range, full_range);
            assert_eq!(request.merge_type, "MERGE_ALL");
        }
    }

    #[test]
    fn test_merge_cells_serialization() {
        let request = MergeCellsRequest::builder()
            .spreadsheet_token("serialization_test")
            .range("Sheet1!A1:C3")
            .merge_type("MERGE_ALL")
            .build();

        let serialized = serde_json::to_string(&request);
        assert!(serialized.is_ok());

        let json_value: serde_json::Value = serde_json::from_str(&serialized.unwrap()).unwrap();
        assert_eq!(json_value["range"], "Sheet1!A1:C3");
        assert_eq!(json_value["mergeType"], "MERGE_ALL");
    }

    #[test]
    fn test_merge_cells_response_deserialization() {
        let response_json = serde_json::json!({
            "spreadsheetToken": "test_token_123"
        });

        let response: MergeCellsResponse = serde_json::from_value(response_json).unwrap();

        assert_eq!(response.spread_sheet_token, "test_token_123");
    }

    #[test]
    fn test_merge_cells_complex_range_references() {
        let complex_ranges = vec![
            "Sheet1!A1:D5",
            "'Data Sheet'!B2:F10",
            "工作表!C3:G7",
            "Sheet-Name_123!A10:E15",
            "'Sheet (1)'!D1:H4",
        ];

        for range in complex_ranges {
            let request = MergeCellsRequest::builder()
                .spreadsheet_token("complex_ref_test")
                .range(range)
                .merge_type("MERGE_ALL")
                .build();

            assert_eq!(request.range, range);
        }
    }

    #[test]
    fn test_merge_cells_all_merge_types() {
        let merge_types = vec!["MERGE_ALL", "MERGE_ROWS", "MERGE_COLUMNS"];

        for merge_type in merge_types {
            let request = MergeCellsRequest::builder()
                .spreadsheet_token("all_types_test")
                .range("Sheet1!A1:C3")
                .merge_type(merge_type)
                .build();

            assert_eq!(request.merge_type, merge_type);
        }
    }

    #[test]
    fn test_merge_cells_service_creation() {
        let service = create_service();
        assert_eq!(service.config.app_id, "test_app_id");
        assert_eq!(service.config.app_secret, "test_app_secret");
        assert!(matches!(service.config.app_type, AppType::SelfBuild));
    }

    #[test]
    fn test_merge_cells_builder_overwrites() {
        let request = MergeCellsRequest::builder()
            .spreadsheet_token("original_token")
            .spreadsheet_token("final_token") // Should overwrite
            .range("Sheet1!A1:B2")
            .range("Sheet2!C3:D4") // Should overwrite
            .merge_type("MERGE_ROWS")
            .merge_type("MERGE_ALL") // Should overwrite
            .build();

        assert_eq!(request.spreadsheet_token, "final_token");
        assert_eq!(request.range, "Sheet2!C3:D4");
        assert_eq!(request.merge_type, "MERGE_ALL");
    }

    #[test]
    fn test_merge_cells_case_sensitive_merge_types() {
        // Test that merge types are case sensitive (as they should be)
        let merge_types = vec![
            "MERGE_ALL",
            "merge_all", // lowercase
            "Merge_All", // mixed case
            "MERGE_ROWS",
            "merge_rows",
            "MERGE_COLUMNS",
            "merge_columns",
        ];

        for merge_type in merge_types {
            let request = MergeCellsRequest::builder()
                .spreadsheet_token("case_test")
                .range("Sheet1!A1:B2")
                .merge_type(merge_type)
                .build();

            assert_eq!(request.merge_type, merge_type);
        }
    }

    #[test]
    fn test_merge_cells_very_long_token() {
        let very_long_token = "a".repeat(1000);
        let request = MergeCellsRequest::builder()
            .spreadsheet_token(&very_long_token)
            .range("Sheet1!A1:B2")
            .merge_type("MERGE_ALL")
            .build();

        assert_eq!(request.spreadsheet_token, very_long_token);
        assert_eq!(request.range, "Sheet1!A1:B2");
        assert_eq!(request.merge_type, "MERGE_ALL");
    }

    #[test]
    fn test_merge_cells_response_struct_debug() {
        let response = MergeCellsResponse {
            spread_sheet_token: "debug_test".to_string(),
        };

        let debug_str = format!("{:?}", response);
        assert!(debug_str.contains("debug_test"));
        assert!(debug_str.contains("MergeCellsResponse"));
    }

    #[test]
    fn test_merge_cells_request_struct_debug() {
        let request = MergeCellsRequest::builder()
            .spreadsheet_token("debug_token")
            .range("Sheet1!A1:B2")
            .merge_type("MERGE_ALL")
            .build();

        let debug_str = format!("{:?}", request);
        assert!(debug_str.contains("debug_token"));
        assert!(debug_str.contains("Sheet1!A1:B2"));
        assert!(debug_str.contains("MERGE_ALL"));
    }

    #[test]
    fn test_merge_cells_empty_strings() {
        // Test behavior with empty strings (should be allowed for flexibility)
        let request = MergeCellsRequest::builder()
            .spreadsheet_token("")
            .range("")
            .merge_type("")
            .build();

        assert_eq!(request.spreadsheet_token, "");
        assert_eq!(request.range, "");
        assert_eq!(request.merge_type, "");
    }
}

use serde::{Deserialize, Serialize};

use crate::{
    core::{
        api_req::ApiRequest,
        api_resp::{ApiResponseTrait, BaseResponse, ResponseFormat},
        constants::AccessTokenType,
        endpoints::cloud_docs::*,
        req_option, SDKResult,
    },
    service::cloud_docs::sheets::v2::{
        data_operation::{CellStyle, SheetDataUpdates},
        SpreadsheetService,
    },
};

#[derive(Debug, Serialize, Default)]
pub struct BatchSetCellStyleRequest {
    #[serde(skip)]
    api_request: ApiRequest,
    #[serde(skip)]
    spreadsheet_token: String,
    /// 设置单元格样式
    data: Vec<AppendStyle>,
}

#[derive(Debug, Serialize, Default)]
struct AppendStyle {
    ranges: String,
    style: CellStyle,
}

impl BatchSetCellStyleRequest {
    pub fn builder() -> BatchSetCellStyleRequestBuilder {
        BatchSetCellStyleRequestBuilder::default()
    }
}

#[derive(Default)]
pub struct BatchSetCellStyleRequestBuilder {
    request: BatchSetCellStyleRequest,
}

impl BatchSetCellStyleRequestBuilder {
    pub fn spreadsheet_token(mut self, spreadsheet_token: impl ToString) -> Self {
        self.request.spreadsheet_token = spreadsheet_token.to_string();
        self
    }

    pub fn add_style(mut self, ranges: impl ToString, style: CellStyle) -> Self {
        self.request.data.push(AppendStyle {
            ranges: ranges.to_string(),
            style,
        });
        self
    }

    pub fn build(mut self) -> BatchSetCellStyleRequest {
        self.request.api_request.body = serde_json::to_vec(&self.request).unwrap();
        self.request
    }
}

#[derive(Deserialize, Debug)]
pub struct BatchSetCellStyleResponse {
    #[serde(rename = "spreadsheetToken")]
    pub spreed_sheet_token: String,
    #[serde(rename = "totalUpdatedRows")]
    pub total_updated_rows: i32,
    #[serde(rename = "totalUpdatedColumns")]
    pub total_updated_columns: i32,
    #[serde(rename = "totalUpdatedCells")]
    pub total_updated_cells: i32,
    /// sheet 的版本号
    pub revision: i32,
    /// 各个范围的设置单元格样式的范围、行列数等
    pub responses: Vec<SheetDataUpdates>,
}

impl ApiResponseTrait for BatchSetCellStyleResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl SpreadsheetService {
    /// 该接口用于根据 spreadsheetToken 、range和样式信息
    /// 批量更新单元格样式；单次写入不超过5000行，100列。建议在设置边框样式时，
    /// 每次更新的单元格数量不要超过30000个。一个区域被多个range覆盖时，仅最后一个样式会被应用。
    pub async fn batch_set_cell_style(
        &self,
        request: BatchSetCellStyleRequest,
        option: Option<req_option::RequestOption>,
    ) -> SDKResult<BaseResponse<BatchSetCellStyleResponse>> {
        let mut api_req = request.api_request;
        api_req.api_path =
            SHEETS_V2_SPREADSHEET_STYLES_BATCH_UPDATE.replace("{}", &request.spreadsheet_token);
        api_req.http_method = reqwest::Method::PUT;
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
            data_operation::{
                share::{CellStyle, StyleFont},
                BatchSetCellStyleRequest, BatchSetCellStyleResponse,
            },
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

    fn create_test_font() -> StyleFont {
        StyleFont::builder()
            .bold(true)
            .italic(false)
            .font_size("12pt/1.5")
            .build()
    }

    fn create_test_style() -> CellStyle {
        CellStyle::builder()
            .font(create_test_font())
            .text_decoration(1)
            .formatter("0.00")
            .h_align(1)
            .v_align(1)
            .fore_color("#000000")
            .back_color("#FFFFFF")
            .border_type("FULL_BORDER")
            .border_color("#808080")
            .clean(false)
            .build()
    }

    #[test]
    fn test_batch_set_cell_style_builder_default() {
        let request = BatchSetCellStyleRequest::builder().build();

        assert_eq!(request.spreadsheet_token, "");
        assert_eq!(request.data.len(), 0);
    }

    #[test]
    fn test_batch_set_cell_style_builder_basic() {
        let style = create_test_style();
        let request = BatchSetCellStyleRequest::builder()
            .spreadsheet_token("test_token")
            .add_style("Sheet1!A1:B2", style)
            .build();

        assert_eq!(request.spreadsheet_token, "test_token");
        assert_eq!(request.data.len(), 1);
        assert_eq!(request.data[0].ranges, "Sheet1!A1:B2");
    }

    #[test]
    fn test_batch_set_cell_style_builder_multiple_styles() {
        let style1 = create_test_style();
        let style2 = CellStyle::builder()
            .font(create_test_font())
            .fore_color("#FF0000")
            .back_color("#00FF00")
            .build();

        let request = BatchSetCellStyleRequest::builder()
            .spreadsheet_token("multi_styles_test")
            .add_style("Sheet1!A1:B2", style1)
            .add_style("Sheet1!C3:D4", style2)
            .build();

        assert_eq!(request.spreadsheet_token, "multi_styles_test");
        assert_eq!(request.data.len(), 2);
        assert_eq!(request.data[0].ranges, "Sheet1!A1:B2");
        assert_eq!(request.data[1].ranges, "Sheet1!C3:D4");
    }

    #[test]
    fn test_batch_set_cell_style_builder_chaining() {
        let style1 = create_test_style();
        let style2 = create_test_style();
        let style3 = create_test_style();

        let request = BatchSetCellStyleRequest::builder()
            .spreadsheet_token("chaining_test")
            .add_style("A1:B2", style1)
            .add_style("C3:D4", style2)
            .add_style("E5:F6", style3)
            .build();

        assert_eq!(request.spreadsheet_token, "chaining_test");
        assert_eq!(request.data.len(), 3);
        assert_eq!(request.data[0].ranges, "A1:B2");
        assert_eq!(request.data[1].ranges, "C3:D4");
        assert_eq!(request.data[2].ranges, "E5:F6");
    }

    #[test]
    fn test_batch_set_cell_style_different_ranges() {
        let ranges = [
            "Sheet1!A1:B2",
            "Summary!C1:F1",
            "Data!A5:D10",
            "第一页!B3:E8",
        ];

        let mut builder = BatchSetCellStyleRequest::builder().spreadsheet_token("multi_range_test");

        for range in ranges {
            let range_style = create_test_style();
            builder = builder.add_style(range, range_style);
        }

        let request = builder.build();

        assert_eq!(request.data.len(), 4);
        for (i, range) in ranges.iter().enumerate() {
            assert_eq!(request.data[i].ranges, *range);
        }
    }

    #[test]
    fn test_batch_set_cell_style_with_unicode_ranges() {
        let style = create_test_style();
        let request = BatchSetCellStyleRequest::builder()
            .spreadsheet_token("unicode_test")
            .add_style("数据表!A1:D4", style)
            .build();

        assert_eq!(request.spreadsheet_token, "unicode_test");
        assert_eq!(request.data.len(), 1);
        assert_eq!(request.data[0].ranges, "数据表!A1:D4");
    }

    #[test]
    fn test_batch_set_cell_style_with_special_characters() {
        let style = create_test_style();
        let request = BatchSetCellStyleRequest::builder()
            .spreadsheet_token("special_chars_test")
            .add_style("'Sheet With Spaces'!A1:B5", style)
            .build();

        assert_eq!(request.spreadsheet_token, "special_chars_test");
        assert_eq!(request.data.len(), 1);
        assert_eq!(request.data[0].ranges, "'Sheet With Spaces'!A1:B5");
    }

    #[test]
    fn test_batch_set_cell_style_large_range() {
        let style = create_test_style();
        let request = BatchSetCellStyleRequest::builder()
            .spreadsheet_token("large_range_test")
            .add_style("Data!A1:Z100", style)
            .build();

        assert_eq!(request.spreadsheet_token, "large_range_test");
        assert_eq!(request.data.len(), 1);
        assert_eq!(request.data[0].ranges, "Data!A1:Z100");
    }

    #[test]
    fn test_batch_set_cell_style_different_styles() {
        let font_styles = [
            ("bold", true, false),
            ("italic", false, true),
            ("bold_italic", true, true),
            ("normal", false, false),
        ];

        let mut builder = BatchSetCellStyleRequest::builder().spreadsheet_token("font_styles_test");

        for (_style_name, bold, italic) in font_styles {
            let font = StyleFont::builder()
                .bold(bold)
                .italic(italic)
                .font_size("12pt/1.5")
                .build();

            let style = CellStyle::builder().font(font).build();

            builder = builder.add_style("Sheet1!A1:B2", style);
        }

        let request = builder.build();
        assert_eq!(request.data.len(), 4);
    }

    #[test]
    fn test_batch_set_cell_style_color_variations() {
        let colors = [
            ("#FF0000", "#FFFFFF"), // Red text, white background
            ("#00FF00", "#000000"), // Green text, black background
            ("#0000FF", "#FFFF00"), // Blue text, yellow background
            ("#800080", "#FFC0CB"), // Purple text, pink background
        ];

        let mut builder = BatchSetCellStyleRequest::builder().spreadsheet_token("color_test");

        for (fore_color, back_color) in colors {
            let style = CellStyle::builder()
                .font(create_test_font())
                .fore_color(fore_color)
                .back_color(back_color)
                .build();

            builder = builder.add_style("Sheet1!A1:B2", style);
        }

        let request = builder.build();
        assert_eq!(request.data.len(), 4);
    }

    #[test]
    fn test_batch_set_cell_style_border_types() {
        let border_types = [
            "FULL_BORDER",
            "OUTER_BORDER",
            "INNER_BORDER",
            "NO_BORDER",
            "LEFT_BORDER",
            "RIGHT_BORDER",
            "TOP_BORDER",
            "BOTTOM_BORDER",
        ];

        let mut builder = BatchSetCellStyleRequest::builder().spreadsheet_token("border_test");

        for border_type in border_types {
            let style = CellStyle::builder()
                .font(create_test_font())
                .border_type(border_type)
                .border_color("#000000")
                .build();

            builder = builder.add_style("Sheet1!A1:B2", style);
        }

        let request = builder.build();
        assert_eq!(request.data.len(), 8);
    }

    #[test]
    fn test_batch_set_cell_style_alignment_combinations() {
        let alignments = [
            (0, 0), // Left, Top
            (0, 1), // Left, Middle
            (0, 2), // Left, Bottom
            (1, 0), // Center, Top
            (1, 1), // Center, Middle
            (1, 2), // Center, Bottom
            (2, 0), // Right, Top
            (2, 1), // Right, Middle
            (2, 2), // Right, Bottom
        ];

        let mut builder = BatchSetCellStyleRequest::builder().spreadsheet_token("alignment_test");

        for (h_align, v_align) in alignments {
            let style = CellStyle::builder()
                .font(create_test_font())
                .h_align(h_align)
                .v_align(v_align)
                .build();

            builder = builder.add_style("Sheet1!A1:B2", style);
        }

        let request = builder.build();
        assert_eq!(request.data.len(), 9);
    }

    #[test]
    fn test_batch_set_cell_style_text_decorations() {
        let decorations = [0, 1, 2, 3]; // Default, underline, strikethrough, both

        let mut builder = BatchSetCellStyleRequest::builder().spreadsheet_token("decoration_test");

        for decoration in decorations {
            let style = CellStyle::builder()
                .font(create_test_font())
                .text_decoration(decoration)
                .build();

            builder = builder.add_style("Sheet1!A1:B2", style);
        }

        let request = builder.build();
        assert_eq!(request.data.len(), 4);
    }

    #[test]
    fn test_batch_set_cell_style_number_formatters() {
        let formatters = ["0.00", "#,##0.00", "0%", "yyyy-mm-dd", "h:mm:ss"];

        let mut builder = BatchSetCellStyleRequest::builder().spreadsheet_token("formatter_test");

        for formatter in formatters {
            let style = CellStyle::builder()
                .font(create_test_font())
                .formatter(formatter)
                .build();

            builder = builder.add_style("Sheet1!A1:B2", style);
        }

        let request = builder.build();
        assert_eq!(request.data.len(), 5);
    }

    #[test]
    fn test_batch_set_cell_style_clean_styles() {
        let clean_style = CellStyle::builder()
            .font(create_test_font())
            .clean(true)
            .build();

        let request = BatchSetCellStyleRequest::builder()
            .spreadsheet_token("clean_test")
            .add_style("Sheet1!A1:Z100", clean_style)
            .build();

        assert_eq!(request.spreadsheet_token, "clean_test");
        assert_eq!(request.data.len(), 1);
        assert_eq!(request.data[0].ranges, "Sheet1!A1:Z100");
    }

    #[test]
    fn test_batch_set_cell_style_serialization() {
        let style = create_test_style();
        let request = BatchSetCellStyleRequest::builder()
            .spreadsheet_token("serialization_test")
            .add_style("Sheet1!A1:C3", style)
            .build();

        let serialized = serde_json::to_string(&request);
        assert!(serialized.is_ok());

        let json_value: serde_json::Value = serde_json::from_str(&serialized.unwrap()).unwrap();
        assert_eq!(json_value["data"][0]["ranges"], "Sheet1!A1:C3");
    }

    #[test]
    fn test_batch_set_cell_style_response_deserialization() {
        let response_json = serde_json::json!({
            "spreadsheetToken": "test_token_123",
            "totalUpdatedRows": 10,
            "totalUpdatedColumns": 5,
            "totalUpdatedCells": 50,
            "revision": 123,
            "responses": []
        });

        let response: BatchSetCellStyleResponse = serde_json::from_value(response_json).unwrap();

        assert_eq!(response.spreed_sheet_token, "test_token_123");
        assert_eq!(response.total_updated_rows, 10);
        assert_eq!(response.total_updated_columns, 5);
        assert_eq!(response.total_updated_cells, 50);
        assert_eq!(response.revision, 123);
        assert_eq!(response.responses.len(), 0);
    }

    #[test]
    fn test_batch_set_cell_style_complex_range_references() {
        let complex_ranges = [
            "Sheet1!A1:D5",
            "'Data Sheet'!B2:F10",
            "工作表!C3:G7",
            "Sheet-Name_123!A10:E15",
            "'Sheet (1)'!D1:H4",
        ];

        let mut builder = BatchSetCellStyleRequest::builder().spreadsheet_token("complex_ref_test");

        for range in complex_ranges {
            let style = create_test_style();
            builder = builder.add_style(range, style);
        }

        let request = builder.build();
        assert_eq!(request.data.len(), 5);

        for (i, range) in complex_ranges.iter().enumerate() {
            assert_eq!(request.data[i].ranges, *range);
        }
    }

    #[test]
    fn test_batch_set_cell_style_service_creation() {
        let service = create_service();
        assert_eq!(service.config.app_id, "test_app_id");
        assert_eq!(service.config.app_secret, "test_app_secret");
        assert!(matches!(service.config.app_type, AppType::SelfBuild));
    }

    #[test]
    fn test_batch_set_cell_style_builder_overwrites() {
        let style1 = create_test_style();
        let style2 = create_test_style();

        let request = BatchSetCellStyleRequest::builder()
            .spreadsheet_token("original_token")
            .spreadsheet_token("final_token") // Should overwrite
            .add_style("Sheet1!A1:B2", style1)
            .add_style("Sheet2!C3:D4", style2)
            .build();

        assert_eq!(request.spreadsheet_token, "final_token");
        assert_eq!(request.data.len(), 2);
        assert_eq!(request.data[0].ranges, "Sheet1!A1:B2");
        assert_eq!(request.data[1].ranges, "Sheet2!C3:D4");
    }

    #[test]
    fn test_batch_set_cell_style_very_long_token() {
        let very_long_token = "a".repeat(1000);
        let style = create_test_style();

        let request = BatchSetCellStyleRequest::builder()
            .spreadsheet_token(&very_long_token)
            .add_style("Sheet1!A1:B2", style)
            .build();

        assert_eq!(request.spreadsheet_token, very_long_token);
        assert_eq!(request.data.len(), 1);
        assert_eq!(request.data[0].ranges, "Sheet1!A1:B2");
    }

    #[test]
    fn test_batch_set_cell_style_response_struct_debug() {
        let response = BatchSetCellStyleResponse {
            spreed_sheet_token: "debug_test".to_string(),
            total_updated_rows: 10,
            total_updated_columns: 5,
            total_updated_cells: 50,
            revision: 123,
            responses: vec![],
        };

        let debug_str = format!("{:?}", response);
        assert!(debug_str.contains("debug_test"));
        assert!(debug_str.contains("BatchSetCellStyleResponse"));
    }

    #[test]
    fn test_batch_set_cell_style_request_struct_debug() {
        let style = create_test_style();
        let request = BatchSetCellStyleRequest::builder()
            .spreadsheet_token("debug_token")
            .add_style("Sheet1!A1:B2", style)
            .build();

        let debug_str = format!("{:?}", request);
        assert!(debug_str.contains("debug_token"));
        assert!(debug_str.contains("Sheet1!A1:B2"));
    }

    #[test]
    fn test_batch_set_cell_style_empty_styles() {
        let request = BatchSetCellStyleRequest::builder()
            .spreadsheet_token("empty_test")
            .build();

        assert_eq!(request.spreadsheet_token, "empty_test");
        assert_eq!(request.data.len(), 0);
    }

    #[test]
    fn test_batch_set_cell_style_single_cell_ranges() {
        let single_cells = ["Sheet1!A1:A1", "Sheet1!B2:B2", "Sheet1!C3:C3"];

        let mut builder = BatchSetCellStyleRequest::builder().spreadsheet_token("single_cell_test");

        for cell in single_cells {
            let style = create_test_style();
            builder = builder.add_style(cell, style);
        }

        let request = builder.build();
        assert_eq!(request.data.len(), 3);

        for (i, cell) in single_cells.iter().enumerate() {
            assert_eq!(request.data[i].ranges, *cell);
        }
    }

    #[test]
    fn test_batch_set_cell_style_edge_case_ranges() {
        let edge_cases = [
            "Sheet1!A1:A1",     // Single cell
            "Sheet1!A1:B1",     // Two cells horizontal
            "Sheet1!A1:A2",     // Two cells vertical
            "Sheet1!Z99:AA100", // Cross column boundary
        ];

        let mut builder = BatchSetCellStyleRequest::builder().spreadsheet_token("edge_cases_test");

        for range in edge_cases {
            let style = create_test_style();
            builder = builder.add_style(range, style);
        }

        let request = builder.build();
        assert_eq!(request.data.len(), 4);

        for (i, range) in edge_cases.iter().enumerate() {
            assert_eq!(request.data[i].ranges, *range);
        }
    }

    #[test]
    fn test_batch_set_cell_style_maximum_recommended_styles() {
        // Test adding many styles (though not the actual limit of 5000 rows/100 columns)
        let mut builder = BatchSetCellStyleRequest::builder().spreadsheet_token("max_styles_test");

        for i in 1..=50 {
            let range = format!("Sheet1!A{}:B{}", i, i);
            let style = create_test_style();
            builder = builder.add_style(&range, style);
        }

        let request = builder.build();
        assert_eq!(request.data.len(), 50);
        assert_eq!(request.data[0].ranges, "Sheet1!A1:B1");
        assert_eq!(request.data[49].ranges, "Sheet1!A50:B50");
    }
}

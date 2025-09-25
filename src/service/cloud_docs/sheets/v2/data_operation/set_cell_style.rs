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
        data_operation::{CellStyle, SheetDataUpdates, StyleFont},
        SpreadsheetService,
    },
};

#[derive(Debug, Serialize, Default)]
pub struct SetCellStyleRequest {
    #[serde(skip)]
    api_request: ApiRequest,
    #[serde(skip)]
    spreadsheet_token: String,
    /// 设置单元格样式
    #[serde(rename = "appendStyle")]
    append_style: AppendStyle,
}

#[derive(Debug, Serialize, Default)]
struct AppendStyle {
    range: String,
    style: CellStyle,
}

impl SetCellStyleRequest {
    pub fn builder() -> SetCellStyleRequestBuilder {
        SetCellStyleRequestBuilder::default()
    }
}

#[derive(Default)]
pub struct SetCellStyleRequestBuilder {
    request: SetCellStyleRequest,
}

impl SetCellStyleRequestBuilder {
    pub fn spreadsheet_token(mut self, spreadsheet_token: impl ToString) -> Self {
        self.request.spreadsheet_token = spreadsheet_token.to_string();
        self
    }

    pub fn range(mut self, range: impl ToString) -> Self {
        self.request.append_style.range = range.to_string();
        self
    }

    pub fn font(mut self, font: StyleFont) -> Self {
        self.request.append_style.style.font = font;
        self
    }

    pub fn text_decoration(mut self, text_decoration: i32) -> Self {
        self.request.append_style.style.text_decoration = Some(text_decoration);
        self
    }

    pub fn formatter(mut self, formatter: impl ToString) -> Self {
        self.request.append_style.style.formatter = Some(formatter.to_string());
        self
    }

    pub fn h_align(mut self, h_align: i32) -> Self {
        self.request.append_style.style.h_align = Some(h_align);
        self
    }

    pub fn v_align(mut self, v_align: i32) -> Self {
        self.request.append_style.style.v_align = Some(v_align);
        self
    }

    pub fn fore_color(mut self, fore_color: impl ToString) -> Self {
        self.request.append_style.style.fore_color = Some(fore_color.to_string());
        self
    }

    pub fn back_color(mut self, back_color: impl ToString) -> Self {
        self.request.append_style.style.back_color = Some(back_color.to_string());
        self
    }

    pub fn border_type(mut self, border_type: impl ToString) -> Self {
        self.request.append_style.style.border_type = Some(border_type.to_string());
        self
    }

    pub fn border_color(mut self, border_color: impl ToString) -> Self {
        self.request.append_style.style.border_color = Some(border_color.to_string());
        self
    }

    pub fn clean(mut self, clean: bool) -> Self {
        self.request.append_style.style.clean = clean;
        self
    }

    pub fn build(mut self) -> SetCellStyleRequest {
        self.request.api_request.body = serde_json::to_vec(&self.request).unwrap();
        self.request
    }
}

#[derive(Deserialize, Debug)]
pub struct SetCellStyleResponse {
    pub updates: SheetDataUpdates,
}

impl ApiResponseTrait for SetCellStyleResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl SpreadsheetService {
    /// 该接口用于根据 spreadsheetToken 、range
    /// 和样式信息更新单元格样式；单次写入不超过5000行，100列。建议在设置边框样式时，
    /// 每次更新的单元格数量不要超过30000个。
    pub async fn set_cell_style(
        &self,
        request: SetCellStyleRequest,
        option: Option<req_option::RequestOption>,
    ) -> SDKResult<BaseResponse<SetCellStyleResponse>> {
        let mut api_req = request.api_request;
        api_req.api_path = SHEETS_V2_SPREADSHEET_STYLE.replace("{}", &request.spreadsheet_token);
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
            data_operation::{SetCellStyleRequest, SetCellStyleResponse, StyleFont},
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

    #[test]
    fn test_set_cell_style_builder_default() {
        let request = SetCellStyleRequest::builder().build();

        assert_eq!(request.spreadsheet_token, "");
        assert_eq!(request.append_style.range, "");
    }

    #[test]
    fn test_set_cell_style_builder_basic() {
        let font = create_test_font();
        let request = SetCellStyleRequest::builder()
            .spreadsheet_token("test_token")
            .range("Sheet1!A1:B2")
            .font(font)
            .build();

        assert_eq!(request.spreadsheet_token, "test_token");
        assert_eq!(request.append_style.range, "Sheet1!A1:B2");
    }

    #[test]
    fn test_set_cell_style_builder_all_options() {
        let font = create_test_font();
        let request = SetCellStyleRequest::builder()
            .spreadsheet_token("spreadsheet_abc123")
            .range("Data!C3:F6")
            .font(font)
            .text_decoration(1) // Underline
            .formatter("0.00")
            .h_align(1) // Center
            .v_align(1) // Middle
            .fore_color("#FF0000")
            .back_color("#FFFF00")
            .border_type("FULL_BORDER")
            .border_color("#000000")
            .clean(false)
            .build();

        assert_eq!(request.spreadsheet_token, "spreadsheet_abc123");
        assert_eq!(request.append_style.range, "Data!C3:F6");
        assert_eq!(request.append_style.style.text_decoration, Some(1));
        assert_eq!(
            request.append_style.style.formatter,
            Some("0.00".to_string())
        );
        assert_eq!(request.append_style.style.h_align, Some(1));
        assert_eq!(request.append_style.style.v_align, Some(1));
        assert_eq!(
            request.append_style.style.fore_color,
            Some("#FF0000".to_string())
        );
        assert_eq!(
            request.append_style.style.back_color,
            Some("#FFFF00".to_string())
        );
        assert_eq!(
            request.append_style.style.border_type,
            Some("FULL_BORDER".to_string())
        );
        assert_eq!(
            request.append_style.style.border_color,
            Some("#000000".to_string())
        );
        assert!(!request.append_style.style.clean);
    }

    #[test]
    fn test_set_cell_style_font_formatting() {
        let font = StyleFont::builder()
            .bold(true)
            .italic(true)
            .font_size("16pt/1.5")
            .build();

        let request = SetCellStyleRequest::builder()
            .spreadsheet_token("font_test")
            .range("Sheet1!A1:A10")
            .font(font)
            .build();

        assert_eq!(request.spreadsheet_token, "font_test");
        assert_eq!(request.append_style.range, "Sheet1!A1:A10");
    }

    #[test]
    fn test_set_cell_style_text_decoration_types() {
        // Test different text decoration types
        let decorations = vec![
            (0, "Default"),
            (1, "Underline"),
            (2, "Strikethrough"),
            (3, "Underline and Strikethrough"),
        ];

        for (decoration_value, _description) in decorations {
            let font = create_test_font();
            let request = SetCellStyleRequest::builder()
                .spreadsheet_token("decoration_test")
                .range("Sheet1!A1:A1")
                .font(font)
                .text_decoration(decoration_value)
                .build();

            assert_eq!(
                request.append_style.style.text_decoration,
                Some(decoration_value)
            );
        }
    }

    #[test]
    fn test_set_cell_style_alignment_options() {
        // Test horizontal alignment options
        let h_alignments = vec![(0, "Left"), (1, "Center"), (2, "Right")];

        for (align_value, _description) in h_alignments {
            let font = create_test_font();
            let request = SetCellStyleRequest::builder()
                .spreadsheet_token("alignment_test")
                .range("Sheet1!A1:A1")
                .font(font)
                .h_align(align_value)
                .build();

            assert_eq!(request.append_style.style.h_align, Some(align_value));
        }

        // Test vertical alignment options
        let v_alignments = vec![(0, "Top"), (1, "Middle"), (2, "Bottom")];

        for (align_value, _description) in v_alignments {
            let font = create_test_font();
            let request = SetCellStyleRequest::builder()
                .spreadsheet_token("alignment_test")
                .range("Sheet1!A1:A1")
                .font(font)
                .v_align(align_value)
                .build();

            assert_eq!(request.append_style.style.v_align, Some(align_value));
        }
    }

    #[test]
    fn test_set_cell_style_color_formats() {
        let color_formats = vec![
            "#FF0000", // Red
            "#00FF00", // Green
            "#0000FF", // Blue
            "#FFFFFF", // White
            "#000000", // Black
            "#FFFF00", // Yellow
            "#FF00FF", // Magenta
            "#00FFFF", // Cyan
        ];

        for color in color_formats {
            let font = create_test_font();
            let request = SetCellStyleRequest::builder()
                .spreadsheet_token("color_test")
                .range("Sheet1!A1:A1")
                .font(font)
                .fore_color(color)
                .back_color(color)
                .border_color(color)
                .build();

            assert_eq!(
                request.append_style.style.fore_color,
                Some(color.to_string())
            );
            assert_eq!(
                request.append_style.style.back_color,
                Some(color.to_string())
            );
            assert_eq!(
                request.append_style.style.border_color,
                Some(color.to_string())
            );
        }
    }

    #[test]
    fn test_set_cell_style_border_types() {
        let border_types = vec![
            "FULL_BORDER",
            "OUTER_BORDER",
            "INNER_BORDER",
            "NO_BORDER",
            "LEFT_BORDER",
            "RIGHT_BORDER",
            "TOP_BORDER",
            "BOTTOM_BORDER",
        ];

        for border_type in border_types {
            let font = create_test_font();
            let request = SetCellStyleRequest::builder()
                .spreadsheet_token("border_test")
                .range("Sheet1!A1:C3")
                .font(font)
                .border_type(border_type)
                .border_color("#000000")
                .build();

            assert_eq!(
                request.append_style.style.border_type,
                Some(border_type.to_string())
            );
        }
    }

    #[test]
    fn test_set_cell_style_number_formats() {
        let number_formats = vec![
            "0",          // Integer
            "0.00",       // Two decimal places
            "0.0000",     // Four decimal places
            "#,##0",      // Thousands separator
            "#,##0.00",   // Thousands with decimals
            "0%",         // Percentage
            "0.00%",      // Percentage with decimals
            "$#,##0.00",  // Currency
            "yyyy-mm-dd", // Date format
            "h:mm:ss",    // Time format
        ];

        for format in number_formats {
            let font = create_test_font();
            let request = SetCellStyleRequest::builder()
                .spreadsheet_token("format_test")
                .range("Sheet1!A1:A1")
                .font(font)
                .formatter(format)
                .build();

            assert_eq!(
                request.append_style.style.formatter,
                Some(format.to_string())
            );
        }
    }

    #[test]
    fn test_set_cell_style_with_unicode_ranges() {
        let font = create_test_font();
        let request = SetCellStyleRequest::builder()
            .spreadsheet_token("unicode_test")
            .range("数据表!A1:D4")
            .font(font)
            .fore_color("#FF0000")
            .build();

        assert_eq!(request.spreadsheet_token, "unicode_test");
        assert_eq!(request.append_style.range, "数据表!A1:D4");
    }

    #[test]
    fn test_set_cell_style_with_special_characters() {
        let font = create_test_font();
        let request = SetCellStyleRequest::builder()
            .spreadsheet_token("special_chars_test")
            .range("'Sheet With Spaces'!A1:B5")
            .font(font)
            .back_color("#FFFF00")
            .build();

        assert_eq!(request.spreadsheet_token, "special_chars_test");
        assert_eq!(request.append_style.range, "'Sheet With Spaces'!A1:B5");
    }

    #[test]
    fn test_set_cell_style_large_ranges() {
        let font = create_test_font();
        let request = SetCellStyleRequest::builder()
            .spreadsheet_token("large_range_test")
            .range("Data!A1:Z100")
            .font(font)
            .border_type("FULL_BORDER")
            .build();

        assert_eq!(request.spreadsheet_token, "large_range_test");
        assert_eq!(request.append_style.range, "Data!A1:Z100");
    }

    #[test]
    fn test_set_cell_style_single_cell() {
        let font = create_test_font();
        let request = SetCellStyleRequest::builder()
            .spreadsheet_token("single_cell_test")
            .range("Sheet1!A1:A1")
            .font(font)
            .h_align(1)
            .v_align(1)
            .build();

        assert_eq!(request.spreadsheet_token, "single_cell_test");
        assert_eq!(request.append_style.range, "Sheet1!A1:A1");
    }

    #[test]
    fn test_set_cell_style_different_sheets() {
        let sheets_and_ranges = [
            ("Sheet1", "A1:B2"),
            ("Summary", "C1:F1"),
            ("Data", "A5:D10"),
            ("第一页", "B3:E8"),
        ];

        for (sheet, range) in sheets_and_ranges.iter() {
            let full_range = format!("{}!{}", sheet, range);
            let font = create_test_font();
            let request = SetCellStyleRequest::builder()
                .spreadsheet_token("multi_sheet_test")
                .range(&full_range)
                .font(font)
                .build();

            assert_eq!(request.append_style.range, full_range);
        }
    }

    #[test]
    fn test_set_cell_style_serialization() {
        let font = create_test_font();
        let request = SetCellStyleRequest::builder()
            .spreadsheet_token("serialization_test")
            .range("Sheet1!A1:C3")
            .font(font)
            .fore_color("#FF0000")
            .back_color("#FFFF00")
            .build();

        let serialized = serde_json::to_string(&request);
        assert!(serialized.is_ok());

        let json_value: serde_json::Value = serde_json::from_str(&serialized.unwrap()).unwrap();
        assert_eq!(json_value["appendStyle"]["range"], "Sheet1!A1:C3");
        assert_eq!(json_value["appendStyle"]["style"]["foreColor"], "#FF0000");
        assert_eq!(json_value["appendStyle"]["style"]["backColor"], "#FFFF00");
    }

    #[test]
    fn test_set_cell_style_response_deserialization() {
        let response_json = serde_json::json!({
            "updates": {
                "spreadsheetToken": "test_token_123",
                "updatedRange": "Sheet1!A1:B2",
                "updatedRows": 2,
                "updatedColumns": 2,
                "updatedCells": 4,
                "revision": 10
            }
        });

        let response: SetCellStyleResponse = serde_json::from_value(response_json).unwrap();

        assert_eq!(response.updates.spreed_sheet_token, "test_token_123");
        assert_eq!(response.updates.updated_range, "Sheet1!A1:B2");
        assert_eq!(response.updates.updated_rows, 2);
        assert_eq!(response.updates.updated_columns, 2);
        assert_eq!(response.updates.updated_cells, 4);
        assert_eq!(response.updates.revision, Some(10));
    }

    #[test]
    fn test_set_cell_style_service_creation() {
        let service = create_service();
        assert_eq!(service.config.app_id, "test_app_id");
        assert_eq!(service.config.app_secret, "test_app_secret");
        assert!(matches!(service.config.app_type, AppType::SelfBuild));
    }

    #[test]
    fn test_set_cell_style_builder_overwrites() {
        let font1 = create_test_font();
        let font2 = StyleFont::builder()
            .bold(false)
            .italic(true)
            .font_size("18pt/1.5")
            .build();

        let request = SetCellStyleRequest::builder()
            .spreadsheet_token("original_token")
            .spreadsheet_token("final_token") // Should overwrite
            .range("Sheet1!A1:B2")
            .range("Sheet2!C3:D4") // Should overwrite
            .font(font1)
            .font(font2) // Should overwrite
            .fore_color("#FF0000")
            .fore_color("#00FF00") // Should overwrite
            .build();

        assert_eq!(request.spreadsheet_token, "final_token");
        assert_eq!(request.append_style.range, "Sheet2!C3:D4");
        assert_eq!(
            request.append_style.style.fore_color,
            Some("#00FF00".to_string())
        );
    }

    #[test]
    fn test_set_cell_style_clean_flag() {
        // Test clean = true
        let font1 = create_test_font();
        let request_clean = SetCellStyleRequest::builder()
            .spreadsheet_token("clean_test")
            .range("Sheet1!A1:A1")
            .font(font1)
            .clean(true)
            .build();

        assert!(request_clean.append_style.style.clean);

        // Test clean = false (default)
        let font2 = create_test_font();
        let request_no_clean = SetCellStyleRequest::builder()
            .spreadsheet_token("clean_test")
            .range("Sheet1!A1:A1")
            .font(font2)
            .clean(false)
            .build();

        assert!(!request_no_clean.append_style.style.clean);
    }

    #[test]
    fn test_set_cell_style_font_builder() {
        // Test StyleFont builder patterns
        let font_bold = StyleFont::builder().bold(true).build();

        let font_italic = StyleFont::builder().italic(true).build();

        let font_sized = StyleFont::builder().font_size("14pt/1.5").build();

        let font_clean = StyleFont::builder().clean(true).build();

        let font_complete = StyleFont::builder()
            .bold(true)
            .italic(true)
            .font_size("16pt/1.5")
            .clean(false)
            .build();

        // Test that each font can be used in a request
        for (font, _test_name) in vec![
            (font_bold, "bold"),
            (font_italic, "italic"),
            (font_sized, "sized"),
            (font_clean, "clean"),
            (font_complete, "complete"),
        ] {
            let request = SetCellStyleRequest::builder()
                .spreadsheet_token("font_builder_test")
                .range("Sheet1!A1:A1")
                .font(font)
                .build();

            assert_eq!(request.spreadsheet_token, "font_builder_test");
        }
    }

    #[test]
    fn test_set_cell_style_very_long_token() {
        let font = create_test_font();
        let very_long_token = "a".repeat(1000);
        let request = SetCellStyleRequest::builder()
            .spreadsheet_token(&very_long_token)
            .range("Sheet1!A1:B2")
            .font(font)
            .build();

        assert_eq!(request.spreadsheet_token, very_long_token);
        assert_eq!(request.append_style.range, "Sheet1!A1:B2");
    }

    #[test]
    fn test_set_cell_style_request_struct_debug() {
        let font = create_test_font();
        let request = SetCellStyleRequest::builder()
            .spreadsheet_token("debug_token")
            .range("Sheet1!A1:B2")
            .font(font)
            .fore_color("#FF0000")
            .build();

        let debug_str = format!("{:?}", request);
        assert!(debug_str.contains("debug_token"));
        assert!(debug_str.contains("Sheet1!A1:B2"));
    }

    #[test]
    fn test_set_cell_style_empty_strings() {
        let font = StyleFont::builder().build(); // Default font
        let request = SetCellStyleRequest::builder()
            .spreadsheet_token("")
            .range("")
            .font(font)
            .build();

        assert_eq!(request.spreadsheet_token, "");
        assert_eq!(request.append_style.range, "");
    }

    #[test]
    fn test_set_cell_style_complex_range_references() {
        let complex_ranges = vec![
            "Sheet1!A1:D5",
            "'Data Sheet'!B2:F10",
            "工作表!C3:G7",
            "Sheet-Name_123!A10:E15",
            "'Sheet (1)'!D1:H4",
        ];

        for range in complex_ranges {
            let font = create_test_font();
            let request = SetCellStyleRequest::builder()
                .spreadsheet_token("complex_ref_test")
                .range(range)
                .font(font)
                .build();

            assert_eq!(request.append_style.range, range);
        }
    }

    #[test]
    fn test_set_cell_style_comprehensive_styling() {
        let font = StyleFont::builder()
            .bold(true)
            .italic(true)
            .font_size("14pt/1.5")
            .build();

        let request = SetCellStyleRequest::builder()
            .spreadsheet_token("comprehensive_test")
            .range("Report!A1:Z100")
            .font(font)
            .text_decoration(3) // Underline and strikethrough
            .formatter("#,##0.00")
            .h_align(1) // Center
            .v_align(1) // Middle
            .fore_color("#FFFFFF") // White text
            .back_color("#0066CC") // Blue background
            .border_type("FULL_BORDER")
            .border_color("#000000") // Black border
            .clean(false)
            .build();

        assert_eq!(request.spreadsheet_token, "comprehensive_test");
        assert_eq!(request.append_style.range, "Report!A1:Z100");
        assert_eq!(request.append_style.style.text_decoration, Some(3));
        assert_eq!(
            request.append_style.style.formatter,
            Some("#,##0.00".to_string())
        );
        assert_eq!(request.append_style.style.h_align, Some(1));
        assert_eq!(request.append_style.style.v_align, Some(1));
        assert_eq!(
            request.append_style.style.fore_color,
            Some("#FFFFFF".to_string())
        );
        assert_eq!(
            request.append_style.style.back_color,
            Some("#0066CC".to_string())
        );
        assert_eq!(
            request.append_style.style.border_type,
            Some("FULL_BORDER".to_string())
        );
        assert_eq!(
            request.append_style.style.border_color,
            Some("#000000".to_string())
        );
        assert!(!request.append_style.style.clean);
    }
}

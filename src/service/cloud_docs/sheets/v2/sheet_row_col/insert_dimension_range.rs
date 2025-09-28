use serde::Serialize;

use crate::{
    core::{
        api_req::ApiRequest,
        api_resp::{BaseResponse, EmptyResponse},
        constants::AccessTokenType,
        endpoints::cloud_docs::*,
        req_option, SDKResult,
    },
    impl_executable_builder_owned,
    service::cloud_docs::sheets::v2::{sheet_row_col::UpdateDimension, SpreadsheetService},
};

/// 插入行列请求
#[derive(Serialize, Default, Debug)]
pub struct InsertDimensionRangeRequest {
    #[serde(skip)]
    api_request: ApiRequest,
    #[serde(skip)]
    spreadsheet_token: String,
    /// 需要插入行列的维度信息
    dimension: UpdateDimension,
    /// 插入的空白行或列是否继承表中的单元格样式。不填或设置为空即不继承任何样式，为默认空白样式。
    /// 可选值：
    /// - BEFORE：继承起始位置的单元格的样式
    /// - AFTER：继承结束位置的单元格的样式
    #[serde(rename = "inheritStyle", skip_serializing_if = "Option::is_none")]
    inherit_style: Option<String>,
}

impl InsertDimensionRangeRequest {
    pub fn builder() -> InsertDimensionRangeRequestBuilder {
        InsertDimensionRangeRequestBuilder::default()
    }
}

#[derive(Default)]
pub struct InsertDimensionRangeRequestBuilder {
    request: InsertDimensionRangeRequest,
}

impl InsertDimensionRangeRequestBuilder {
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

    /// 插入的行或列的起始位置。从 0 开始计数。若 startIndex 为 3，则从第 4
    /// 行或列开始插入空行或列。包含第 4 行或列。
    pub fn start_index(mut self, start_index: i32) -> Self {
        self.request.dimension.start_index = start_index;
        self
    }

    /// 插入的行或列结束的位置。从 0 开始计数。若 endIndex 为 7，则从第 8 行结束插入行。第 8
    /// 行不再插入空行。 示例：当 majorDimension为 ROWS、 startIndex 为 3、endIndex 为 7
    /// 时，则在第 4、5、6、7 行插入空白行，共插入 4 行。
    pub fn end_index(mut self, end_index: i32) -> Self {
        self.request.dimension.end_index = end_index;
        self
    }

    /// 插入的空白行或列是否继承表中的单元格样式。不填或设置为空即不继承任何样式，为默认空白样式。
    /// 可选值：
    /// - BEFORE：继承起始位置的单元格的样式
    /// - AFTER：继承结束位置的单元格的样式
    pub fn inherit_style(mut self, inherit_style: impl ToString) -> Self {
        self.request.inherit_style = Some(inherit_style.to_string());
        self
    }

    pub fn build(mut self) -> InsertDimensionRangeRequest {
        self.request.api_request.body = serde_json::to_vec(&self.request).unwrap();
        self.request
    }
}

// 使用宏实现ExecutableBuilder trait
impl_executable_builder_owned!(
    InsertDimensionRangeRequestBuilder,
    SpreadsheetService,
    InsertDimensionRangeRequest,
    BaseResponse<EmptyResponse>,
    insert_dimension_range
);

impl SpreadsheetService {
    /// 插入行列
    pub async fn insert_dimension_range(
        &self,
        request: InsertDimensionRangeRequest,
        option: Option<req_option::RequestOption>,
    ) -> SDKResult<BaseResponse<EmptyResponse>> {
        let mut api_req = request.api_request;
        api_req.api_path =
            SHEETS_V2_SPREADSHEET_INSERT_DIMENSION_RANGE.replace("{}", &request.spreadsheet_token);
        api_req.http_method = reqwest::Method::POST;
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::App];

        let api_resp = crate::core::http::Transport::request(api_req, &self.config, option).await?;

        Ok(api_resp)
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
        let request = InsertDimensionRangeRequest::builder()
            .spreadsheet_token("test_spreadsheet_123")
            .build();

        assert_eq!(request.spreadsheet_token, "test_spreadsheet_123");
    }

    #[test]
    fn test_insert_dimension_range_request_builder_with_sheet_id() {
        let request = InsertDimensionRangeRequest::builder()
            .sheet_id("test_sheet_456")
            .build();

        assert_eq!(request.dimension.sheet_id, "test_sheet_456");
    }

    #[rstest]
    #[case("ROWS")]
    #[case("COLUMNS")]
    fn test_insert_dimension_range_request_builder_with_major_dimension(#[case] dimension: &str) {
        let request = InsertDimensionRangeRequest::builder()
            .major_dimension(dimension)
            .build();

        assert_eq!(request.dimension.major_dimension, dimension);
    }

    #[test]
    fn test_insert_dimension_range_request_builder_with_start_index() {
        let request = InsertDimensionRangeRequest::builder()
            .start_index(5)
            .build();

        assert_eq!(request.dimension.start_index, 5);
    }

    #[test]
    fn test_insert_dimension_range_request_builder_with_end_index() {
        let request = InsertDimensionRangeRequest::builder().end_index(10).build();

        assert_eq!(request.dimension.end_index, 10);
    }

    #[rstest]
    #[case("BEFORE")]
    #[case("AFTER")]
    fn test_insert_dimension_range_request_builder_with_inherit_style(#[case] style: &str) {
        let request = InsertDimensionRangeRequest::builder()
            .inherit_style(style)
            .build();

        assert_eq!(request.inherit_style, Some(style.to_string()));
    }

    #[test]
    fn test_insert_dimension_range_request_builder_chaining() {
        let request = InsertDimensionRangeRequest::builder()
            .spreadsheet_token("my_spreadsheet")
            .sheet_id("my_sheet")
            .major_dimension("ROWS")
            .start_index(3)
            .end_index(7)
            .inherit_style("BEFORE")
            .build();

        assert_eq!(request.spreadsheet_token, "my_spreadsheet");
        assert_eq!(request.dimension.sheet_id, "my_sheet");
        assert_eq!(request.dimension.major_dimension, "ROWS");
        assert_eq!(request.dimension.start_index, 3);
        assert_eq!(request.dimension.end_index, 7);
        assert_eq!(request.inherit_style, Some("BEFORE".to_string()));
    }

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
        let request = InsertDimensionRangeRequest::builder()
            .spreadsheet_token("token123")
            .sheet_id("sheet456")
            .major_dimension("COLUMNS")
            .start_index(2)
            .end_index(5)
            .inherit_style("AFTER")
            .build();

        // Test that the request can be serialized (this validates field names)
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
        let request = InsertDimensionRangeRequest::builder()
            .spreadsheet_token("token123")
            .sheet_id("sheet456")
            .major_dimension("ROWS")
            .start_index(1)
            .end_index(3)
            .build();

        let serialized = serde_json::to_string(&request);
        assert!(serialized.is_ok());

        let json_str = serialized.unwrap();
        assert!(json_str.contains("dimension"));
        assert!(json_str.contains("ROWS"));
        assert!(!json_str.contains("inheritStyle")); // Should not be present when None
    }

    #[test]
    fn test_insert_dimension_range_request_debug() {
        let request = InsertDimensionRangeRequest::builder()
            .spreadsheet_token("debug_token")
            .sheet_id("debug_sheet")
            .inherit_style("BEFORE")
            .build();

        let debug_str = format!("{:?}", request);
        assert!(debug_str.contains("InsertDimensionRangeRequest"));
        assert!(debug_str.contains("debug_token"));
        assert!(debug_str.contains("debug_sheet"));
        assert!(debug_str.contains("BEFORE"));
    }

    #[test]
    fn test_insert_dimension_range_request_with_empty_strings() {
        let request = InsertDimensionRangeRequest::builder()
            .spreadsheet_token("")
            .sheet_id("")
            .major_dimension("")
            .inherit_style("")
            .build();

        assert_eq!(request.spreadsheet_token, "");
        assert_eq!(request.dimension.sheet_id, "");
        assert_eq!(request.dimension.major_dimension, "");
        assert_eq!(request.inherit_style, Some("".to_string()));
    }

    #[test]
    fn test_insert_dimension_range_request_with_special_characters() {
        let request = InsertDimensionRangeRequest::builder()
            .spreadsheet_token("token_with_特殊字符_🎯")
            .sheet_id("sheet_名称_123")
            .major_dimension("ROWS")
            .inherit_style("BEFORE")
            .build();

        assert_eq!(request.spreadsheet_token, "token_with_特殊字符_🎯");
        assert_eq!(request.dimension.sheet_id, "sheet_名称_123");
        assert_eq!(request.dimension.major_dimension, "ROWS");
        assert_eq!(request.inherit_style, Some("BEFORE".to_string()));
    }

    #[rstest]
    #[case(0, 1)]
    #[case(1, 5)]
    #[case(10, 20)]
    #[case(100, 200)]
    #[case(-1, 0)] // Edge case: negative start
    #[case(5, 5)] // Edge case: start equals end
    fn test_insert_dimension_range_request_with_various_indices(
        #[case] start: i32,
        #[case] end: i32,
    ) {
        let request = InsertDimensionRangeRequest::builder()
            .start_index(start)
            .end_index(end)
            .build();

        assert_eq!(request.dimension.start_index, start);
        assert_eq!(request.dimension.end_index, end);
    }

    #[test]
    fn test_insert_dimension_range_request_with_maximum_values() {
        let request = InsertDimensionRangeRequest::builder()
            .start_index(i32::MAX)
            .end_index(i32::MAX)
            .build();

        assert_eq!(request.dimension.start_index, i32::MAX);
        assert_eq!(request.dimension.end_index, i32::MAX);
    }

    #[test]
    fn test_insert_dimension_range_request_with_minimum_values() {
        let request = InsertDimensionRangeRequest::builder()
            .start_index(i32::MIN)
            .end_index(i32::MIN)
            .build();

        assert_eq!(request.dimension.start_index, i32::MIN);
        assert_eq!(request.dimension.end_index, i32::MIN);
    }

    #[test]
    fn test_insert_dimension_range_request_api_request_body_serialization() {
        let request = InsertDimensionRangeRequest::builder()
            .spreadsheet_token("body_test_token")
            .sheet_id("body_test_sheet")
            .major_dimension("ROWS")
            .start_index(1)
            .end_index(3)
            .inherit_style("AFTER")
            .build();

        // Verify that api_request.body is set correctly
        assert!(!request.api_request.body.is_empty());

        // Verify that the body contains valid JSON
        let body_str = String::from_utf8(request.api_request.body).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&body_str).unwrap();

        assert!(parsed.get("dimension").is_some());
        let dimension = parsed.get("dimension").unwrap();
        assert_eq!(dimension.get("majorDimension").unwrap(), "ROWS");
        assert_eq!(dimension.get("startIndex").unwrap(), 1);
        assert_eq!(dimension.get("endIndex").unwrap(), 3);
        assert_eq!(parsed.get("inheritStyle").unwrap(), "AFTER");
    }

    #[test]
    fn test_insert_dimension_range_request_builder_multiple_calls() {
        let mut builder = InsertDimensionRangeRequest::builder();

        // Test that multiple calls override previous values
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
    }

    #[test]
    fn test_spreadsheet_service_creation() {
        let service = create_test_service();

        // Verify the service can be created without panic
        assert_eq!(service.config.app_id, "test_app_id");
    }

    #[test]
    fn test_insert_dimension_range_request_edge_cases() {
        // Test with very long token
        let long_token = "a".repeat(10000);
        let request = InsertDimensionRangeRequest::builder()
            .spreadsheet_token(&long_token)
            .build();
        assert_eq!(request.spreadsheet_token, long_token);

        // Test with very long sheet ID
        let long_sheet_id = "sheet_".repeat(1000);
        let request = InsertDimensionRangeRequest::builder()
            .sheet_id(&long_sheet_id)
            .build();
        assert_eq!(request.dimension.sheet_id, long_sheet_id);

        // Test with extreme index values
        let request = InsertDimensionRangeRequest::builder()
            .start_index(0)
            .end_index(1000000)
            .build();
        assert_eq!(request.dimension.start_index, 0);
        assert_eq!(request.dimension.end_index, 1000000);

        // Test with very long inherit_style
        let long_style = "BEFORE_".repeat(100);
        let request = InsertDimensionRangeRequest::builder()
            .inherit_style(&long_style)
            .build();
        assert_eq!(request.inherit_style, Some(long_style));
    }

    #[test]
    fn test_insert_dimension_range_request_memory_efficiency() {
        // Test creating many requests doesn't consume excessive memory
        let requests: Vec<InsertDimensionRangeRequest> = (0..100)
            .map(|i| {
                let mut builder = InsertDimensionRangeRequest::builder()
                    .spreadsheet_token(format!("token_{}", i))
                    .sheet_id(format!("sheet_{}", i))
                    .major_dimension(if i % 2 == 0 { "ROWS" } else { "COLUMNS" })
                    .start_index(i)
                    .end_index(i + 10);

                if i % 3 == 0 {
                    builder = builder.inherit_style("BEFORE");
                } else if i % 3 == 1 {
                    builder = builder.inherit_style("AFTER");
                }
                // For i % 3 == 2, leave inherit_style as None

                builder.build()
            })
            .collect();

        assert_eq!(requests.len(), 100);

        // Verify each request has correct data
        for (i, request) in requests.iter().enumerate() {
            assert_eq!(request.spreadsheet_token, format!("token_{}", i));
            assert_eq!(request.dimension.sheet_id, format!("sheet_{}", i));
            assert_eq!(request.dimension.start_index, i as i32);
            assert_eq!(request.dimension.end_index, (i + 10) as i32);

            match i % 3 {
                0 => assert_eq!(request.inherit_style, Some("BEFORE".to_string())),
                1 => assert_eq!(request.inherit_style, Some("AFTER".to_string())),
                2 => assert!(request.inherit_style.is_none()),
                _ => unreachable!(),
            }
        }
    }

    #[test]
    fn test_insert_dimension_range_request_serialization_with_null_inherit_style() {
        let mut request = InsertDimensionRangeRequest::builder()
            .spreadsheet_token("test_token")
            .sheet_id("test_sheet")
            .major_dimension("ROWS")
            .start_index(1)
            .end_index(3)
            .build();

        // Ensure inherit_style is None
        request.inherit_style = None;

        // Re-serialize manually to test None handling
        request.api_request.body = serde_json::to_vec(&request).unwrap();

        let body_str = String::from_utf8(request.api_request.body).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&body_str).unwrap();

        // inheritStyle should not be present in JSON when None
        assert!(parsed.get("inheritStyle").is_none());
        assert!(parsed.get("dimension").is_some());
    }

    #[test]
    fn test_insert_dimension_range_request_various_inherit_styles() {
        let styles = vec![
            "BEFORE",
            "AFTER",
            "before", // Test case sensitivity handling
            "after",
            "INVALID_STYLE", // Test with invalid but accepted string
        ];

        for style in styles {
            let request = InsertDimensionRangeRequest::builder()
                .inherit_style(style)
                .build();

            assert_eq!(request.inherit_style, Some(style.to_string()));
        }
    }

    #[test]
    fn test_insert_dimension_range_request_builder_partial_configuration() {
        // Test building with only some fields configured
        let request1 = InsertDimensionRangeRequest::builder()
            .spreadsheet_token("test_token")
            .build();

        assert_eq!(request1.spreadsheet_token, "test_token");
        assert_eq!(request1.dimension.sheet_id, "");
        assert!(request1.inherit_style.is_none());

        let request2 = InsertDimensionRangeRequest::builder()
            .sheet_id("test_sheet")
            .inherit_style("BEFORE")
            .build();

        assert_eq!(request2.spreadsheet_token, "");
        assert_eq!(request2.dimension.sheet_id, "test_sheet");
        assert_eq!(request2.inherit_style, Some("BEFORE".to_string()));
    }

    #[test]
    fn test_insert_dimension_range_request_unicode_handling() {
        let request = InsertDimensionRangeRequest::builder()
            .spreadsheet_token("令牌_🔑_test")
            .sheet_id("工作表_📋_id")
            .major_dimension("ROWS")
            .inherit_style("BEFORE_风格")
            .build();

        assert_eq!(request.spreadsheet_token, "令牌_🔑_test");
        assert_eq!(request.dimension.sheet_id, "工作表_📋_id");
        assert_eq!(request.inherit_style, Some("BEFORE_风格".to_string()));

        // Test serialization works with Unicode
        let serialized = serde_json::to_string(&request);
        assert!(serialized.is_ok());
    }
}

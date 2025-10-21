use serde::Serialize;
use serde_json::Value;

use crate::{
    core::{
        api_req::ApiRequest, api_resp::BaseResponse, constants::AccessTokenType,
        endpoints::cloud_docs::*, req_option::RequestOption, SDKResult,
    },
    impl_executable_builder_owned,
    service::cloud_docs::sheets::v2::{
        data_operation::{UpdateSheetDataResponse, ValueRangeRequest},
        SpreadsheetSheetService,
    },
};

/// 追加数据请求
#[derive(Serialize, Debug, Default)]
pub struct AppendDataRequest {
    #[serde(skip)]
    api_request: ApiRequest,
    #[serde(skip)]
    spreadsheet_token: String,
    /// 遇到空行追加，默认 OVERWRITE，若空行的数量小于追加数据的行数，则会覆盖已有数据；可选
    /// INSERT_ROWS ，会在插入足够数量的行后再进行数据追加
    #[serde(rename = "insertDataOption")]
    insert_data_option: String,
    /// 值与范围
    #[serde(rename = "valueRange")]
    value_range: ValueRangeRequest,
}

impl AppendDataRequest {
    pub fn builder() -> AppendDataRequestBuilder {
        AppendDataRequestBuilder::default()
    }
}

#[derive(Default)]
pub struct AppendDataRequestBuilder {
    request: AppendDataRequest,
}

impl AppendDataRequestBuilder {
    pub fn spreadsheet_token(mut self, spreadsheet_token: impl ToString) -> Self {
        self.request.spreadsheet_token = spreadsheet_token.to_string();
        self
    }

    /// 插入范围，包含 sheetId 与单元格范围两部分，目前支持四种索引方式，详见
    /// 在线表格开发指南，range所表示的范围需要大于等于values占用的范围。
    pub fn range(mut self, range: impl ToString) -> Self {
        self.request.value_range.range = range.to_string();
        self
    }

    /// 需要写入的值，如要写入公式、超链接、email、@人等，可详看附录sheet 支持写入数据类型
    pub fn values(mut self, values: Value) -> Self {
        self.request.value_range.values = values;
        self
    }

    pub fn build(mut self) -> AppendDataRequest {
        self.request.api_request.body = serde_json::to_vec(&self.request).unwrap();
        self.request
    }
}

pub type AppendDataResponse = UpdateSheetDataResponse;

// 使用宏实现ExecutableBuilder trait
impl_executable_builder_owned!(
    AppendDataRequestBuilder,
    SpreadsheetSheetService,
    AppendDataRequest,
    BaseResponse<AppendDataResponse>,
    append_data
);

impl SpreadsheetSheetService {
    /// 追加数据
    pub async fn append_data(
        &self,
        request: AppendDataRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<AppendDataResponse>> {
        let mut api_req = request.api_request;
        api_req.api_path =
            SHEETS_V2_SPREADSHEET_VALUES_APPEND.replace("{}", &request.spreadsheet_token);
        api_req.http_method = reqwest::Method::POST;
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::App];

        let api_resp =
            crate::core::http::Transport::request(api_req, &self.config_arc, option).await?;

        Ok(api_resp)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::config::Config;
    use serde_json::{json, Value};

    fn create_test_config() -> Config {
        Config::builder()
            .app_id("test_app_id")
            .app_secret("test_app_secret")
            .build()
    }

    fn create_test_service() -> SpreadsheetSheetService {
        SpreadsheetSheetService::new(create_test_config())
    }

    #[test]
    fn test_append_data_request_builder_creation() {
        let builder = AppendDataRequest::builder();
        let request = builder.build();

        assert_eq!(request.spreadsheet_token, "");
        assert_eq!(request.value_range.range, "");
        assert_eq!(request.value_range.values, Value::Null);
        assert_eq!(request.insert_data_option, "");
    }

    #[test]
    fn test_append_data_request_builder_with_spreadsheet_token() {
        let request = AppendDataRequest::builder()
            .spreadsheet_token("test_token_123")
            .build();

        assert_eq!(request.spreadsheet_token, "test_token_123");
    }

    #[test]
    fn test_append_data_request_builder_with_range() {
        let request = AppendDataRequest::builder().range("Sheet1!A1:C10").build();

        assert_eq!(request.value_range.range, "Sheet1!A1:C10");
    }

    #[test]
    fn test_append_data_request_builder_with_values() {
        let test_values = json!([
            ["Name", "Age", "City"],
            ["John", 25, "NYC"],
            ["Jane", 30, "LA"]
        ]);

        let request = AppendDataRequest::builder()
            .values(test_values.clone())
            .build();

        assert_eq!(request.value_range.values, test_values);
    }

    #[test]
    fn test_append_data_request_builder_chaining() {
        let test_values = json!([["A", "B"], ["1", "2"]]);

        let request = AppendDataRequest::builder()
            .spreadsheet_token("my_sheet_token")
            .range("Sheet1!A1:B2")
            .values(test_values.clone())
            .build();

        assert_eq!(request.spreadsheet_token, "my_sheet_token");
        assert_eq!(request.value_range.range, "Sheet1!A1:B2");
        assert_eq!(request.value_range.values, test_values);
    }

    #[test]
    fn test_append_data_request_default() {
        let request = AppendDataRequest::default();

        assert_eq!(request.spreadsheet_token, "");
        assert_eq!(request.value_range.range, "");
        assert_eq!(request.value_range.values, Value::Null);
        assert_eq!(request.insert_data_option, "");
    }

    #[test]
    fn test_append_data_request_builder_default() {
        let builder = AppendDataRequestBuilder::default();
        let request = builder.build();

        assert_eq!(request.spreadsheet_token, "");
        assert_eq!(request.value_range.range, "");
        assert_eq!(request.value_range.values, Value::Null);
    }

    #[test]
    fn test_append_data_request_serialization() {
        let test_values = json!([["Header1", "Header2"], ["Value1", "Value2"]]);

        let request = AppendDataRequest::builder()
            .spreadsheet_token("token123")
            .range("Sheet1!A:B")
            .values(test_values)
            .build();

        // Test that the request can be serialized (this validates field names)
        let serialized = serde_json::to_string(&request);
        assert!(serialized.is_ok());

        let json_str = serialized.unwrap();
        assert!(json_str.contains("insertDataOption"));
        assert!(json_str.contains("valueRange"));
    }

    #[test]
    fn test_append_data_request_debug() {
        let request = AppendDataRequest::builder()
            .spreadsheet_token("debug_token")
            .build();

        let debug_str = format!("{:?}", request);
        assert!(debug_str.contains("AppendDataRequest"));
        assert!(debug_str.contains("debug_token"));
    }

    #[test]
    fn test_append_data_request_with_empty_strings() {
        let request = AppendDataRequest::builder()
            .spreadsheet_token("")
            .range("")
            .values(Value::Null)
            .build();

        assert_eq!(request.spreadsheet_token, "");
        assert_eq!(request.value_range.range, "");
        assert_eq!(request.value_range.values, Value::Null);
    }

    #[test]
    fn test_append_data_request_with_special_characters() {
        let request = AppendDataRequest::builder()
            .spreadsheet_token("token_with_特殊字符_🎯")
            .range("Sheet名称!A1:Z100")
            .build();

        assert_eq!(request.spreadsheet_token, "token_with_特殊字符_🎯");
        assert_eq!(request.value_range.range, "Sheet名称!A1:Z100");
    }

    #[test]
    fn test_append_data_request_with_unicode_values() {
        let test_values = json!([
            ["姓名", "年龄", "城市"],
            ["张三", 25, "北京"],
            ["李四", 30, "上海"],
            ["王五", 35, "深圳"]
        ]);

        let request = AppendDataRequest::builder()
            .values(test_values.clone())
            .build();

        assert_eq!(request.value_range.values, test_values);
    }

    #[test]
    fn test_append_data_request_with_complex_json() {
        let complex_values = json!([
            [{"text": "Complex"}, {"formula": "=A1+B1"}, {"link": "https://example.com"}],
            [123.45, true, null],
            [{"nested": {"deep": "value"}}, [1, 2, 3], "simple_string"]
        ]);

        let request = AppendDataRequest::builder()
            .values(complex_values.clone())
            .build();

        assert_eq!(request.value_range.values, complex_values);
    }

    #[test]
    fn test_append_data_request_with_large_data() {
        let large_values = json!((0..1000)
            .map(|i| vec![
                format!("Name_{}", i),
                i.to_string(),
                format!("Description for item {}", i)
            ])
            .collect::<Vec<_>>());

        let request = AppendDataRequest::builder()
            .spreadsheet_token("large_data_token")
            .range("Sheet1!A:C")
            .values(large_values.clone())
            .build();

        assert_eq!(request.value_range.values, large_values);
        assert_eq!(request.spreadsheet_token, "large_data_token");
    }

    #[test]
    fn test_append_data_request_with_different_range_formats() {
        let ranges = vec![
            "Sheet1!A1:B2",
            "工作表1!C:D",
            "'My Sheet'!E5:G10",
            "Sheet2!A:A",
            "Sheet3!1:3",
        ];

        for range in ranges {
            let request = AppendDataRequest::builder().range(range).build();

            assert_eq!(request.value_range.range, range);
        }
    }

    #[test]
    fn test_append_data_request_with_different_value_types() {
        let different_types = vec![
            json!("string"),
            json!(42),
            json!(3.14286),
            json!(true),
            json!(null),
            json!([1, 2, 3]),
            json!({"key": "value"}),
        ];

        for value in different_types {
            let request = AppendDataRequest::builder().values(value.clone()).build();

            assert_eq!(request.value_range.values, value);
        }
    }

    #[test]
    fn test_append_data_request_api_request_body_serialization() {
        let test_values = json!([["A", "B"], ["1", "2"]]);

        let request = AppendDataRequest::builder()
            .spreadsheet_token("body_test_token")
            .range("Sheet1!A:B")
            .values(test_values)
            .build();

        // Verify that api_request.body is set correctly
        assert!(!request.api_request.body.is_empty());

        // Verify that the body contains valid JSON
        let body_str = String::from_utf8(request.api_request.body).unwrap();
        let parsed: Value = serde_json::from_str(&body_str).unwrap();

        assert!(parsed.get("valueRange").is_some());
        assert!(parsed.get("insertDataOption").is_some());
    }

    #[test]
    fn test_append_data_request_builder_multiple_calls() {
        let mut builder = AppendDataRequest::builder();

        // Test that multiple calls override previous values
        builder = builder.spreadsheet_token("first_token");
        builder = builder.spreadsheet_token("second_token");
        builder = builder.range("first_range");
        builder = builder.range("second_range");

        let request = builder.build();

        assert_eq!(request.spreadsheet_token, "second_token");
        assert_eq!(request.value_range.range, "second_range");
    }

    #[test]
    fn test_spreadsheet_sheet_service_creation() {
        let service = create_test_service();

        // Verify the service can be created without panic
        assert_eq!(service.config.app_id, "test_app_id");
    }

    #[test]
    fn test_append_data_response_type_alias() {
        // Verify that AppendDataResponse is properly aliased
        let _response: AppendDataResponse = UpdateSheetDataResponse {
            spreed_sheet_token: "test".to_string(),
            table_range: "A1:B2".to_string(),
            revision: 1,
            updates: crate::service::cloud_docs::sheets::v2::data_operation::SheetDataUpdates {
                spreed_sheet_token: "test".to_string(),
                updated_range: "A1:B2".to_string(),
                updated_rows: 2,
                updated_columns: 2,
                updated_cells: 4,
                revision: Some(1),
            },
        };
    }

    #[test]
    fn test_append_data_request_edge_cases() {
        // Test with very long token
        let long_token = "a".repeat(10000);
        let request = AppendDataRequest::builder()
            .spreadsheet_token(&long_token)
            .build();
        assert_eq!(request.spreadsheet_token, long_token);

        // Test with empty array values
        let empty_array = json!([]);
        let request = AppendDataRequest::builder()
            .values(empty_array.clone())
            .build();
        assert_eq!(request.value_range.values, empty_array);

        // Test with nested array values
        let nested_array = json!([[[["deeply", "nested"], ["values", "here"]]]]);
        let request = AppendDataRequest::builder()
            .values(nested_array.clone())
            .build();
        assert_eq!(request.value_range.values, nested_array);
    }

    #[test]
    fn test_value_range_request_default() {
        let value_range = ValueRangeRequest::default();
        assert_eq!(value_range.range, "");
        assert_eq!(value_range.values, Value::Null);
    }

    #[test]
    fn test_value_range_request_serialization() {
        let value_range = ValueRangeRequest {
            range: "A1:B2".to_string(),
            values: json!([["1", "2"]]),
        };

        let serialized = serde_json::to_string(&value_range);
        assert!(serialized.is_ok());

        let json_str = serialized.unwrap();
        assert!(json_str.contains("range"));
        assert!(json_str.contains("values"));
        assert!(json_str.contains("A1:B2"));
    }

    #[test]
    fn test_append_data_request_memory_efficiency() {
        // Test creating many requests doesn't consume excessive memory
        let requests: Vec<AppendDataRequest> = (0..100)
            .map(|i| {
                AppendDataRequest::builder()
                    .spreadsheet_token(format!("token_{}", i))
                    .range(format!("Sheet{}!A:B", i))
                    .values(json!([[i.to_string(), (i + 1).to_string()]]))
                    .build()
            })
            .collect();

        assert_eq!(requests.len(), 100);

        // Verify each request has correct data
        for (i, request) in requests.iter().enumerate() {
            assert_eq!(request.spreadsheet_token, format!("token_{}", i));
            assert_eq!(request.value_range.range, format!("Sheet{}!A:B", i));
        }
    }
}

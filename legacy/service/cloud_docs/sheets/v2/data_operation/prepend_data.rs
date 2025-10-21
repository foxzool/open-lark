use serde::Serialize;
use serde_json::Value;

use crate::{
    core::{
        api_req::ApiRequest, api_resp::BaseResponse, constants::AccessTokenType,
        endpoints::cloud_docs::*, req_option::RequestOption, SDKResult,
    },
    service::cloud_docs::sheets::v2::{
        data_operation::{UpdateSheetDataResponse, ValueRangeRequest},
        SpreadsheetSheetService,
    },
};

/// 插入数据请求
#[derive(Serialize, Debug, Default)]
pub struct PrependDataRequest {
    #[serde(skip)]
    api_request: ApiRequest,
    #[serde(skip)]
    spreadsheet_token: String,
    /// 值与范围
    #[serde(rename = "valueRange")]
    value_range: ValueRangeRequest,
}

impl PrependDataRequest {
    pub fn builder() -> PrependDataRequestBuilder {
        PrependDataRequestBuilder::default()
    }
}

#[derive(Default)]
pub struct PrependDataRequestBuilder {
    request: PrependDataRequest,
}

impl PrependDataRequestBuilder {
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

    pub fn build(mut self) -> PrependDataRequest {
        self.request.api_request.body = serde_json::to_vec(&self.request).unwrap();
        self.request
    }
}

/// 插入数据响应体
pub type PrependDataResponse = UpdateSheetDataResponse;

impl SpreadsheetSheetService {
    /// 插入数据
    pub async fn prepend_data(
        &self,
        request: PrependDataRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<PrependDataResponse>> {
        let mut api_req = request.api_request;
        api_req.api_path =
            SHEETS_V2_SPREADSHEET_VALUES_PREPEND.replace("{}", &request.spreadsheet_token);
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
    use crate::core::{config::Config, constants::AppType};
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
    fn test_prepend_data_request_builder_creation() {
        let builder = PrependDataRequest::builder();
        let request = builder.build();

        assert_eq!(request.spreadsheet_token, "");
        assert_eq!(request.value_range.range, "");
        assert_eq!(request.value_range.values, Value::Null);
    }

    #[test]
    fn test_prepend_data_request_builder_with_spreadsheet_token() {
        let request = PrependDataRequest::builder()
            .spreadsheet_token("prepend_token_123")
            .build();

        assert_eq!(request.spreadsheet_token, "prepend_token_123");
    }

    #[test]
    fn test_prepend_data_request_builder_with_range() {
        let request = PrependDataRequest::builder().range("Sheet1!A1:D5").build();

        assert_eq!(request.value_range.range, "Sheet1!A1:D5");
    }

    #[test]
    fn test_prepend_data_request_builder_with_values() {
        let test_values = json!([
            ["Product", "Price", "Stock", "Category"],
            ["Laptop", 999.99, 50, "Electronics"],
            ["Mouse", 29.99, 200, "Accessories"]
        ]);

        let request = PrependDataRequest::builder()
            .values(test_values.clone())
            .build();

        assert_eq!(request.value_range.values, test_values);
    }

    #[test]
    fn test_prepend_data_request_builder_chaining() {
        let test_values = json!([["Header1", "Header2"], ["Data1", "Data2"]]);

        let request = PrependDataRequest::builder()
            .spreadsheet_token("chain_token")
            .range("Sheet2!B2:C3")
            .values(test_values.clone())
            .build();

        assert_eq!(request.spreadsheet_token, "chain_token");
        assert_eq!(request.value_range.range, "Sheet2!B2:C3");
        assert_eq!(request.value_range.values, test_values);
    }

    #[test]
    fn test_prepend_data_request_default() {
        let request = PrependDataRequest::default();

        assert_eq!(request.spreadsheet_token, "");
        assert_eq!(request.value_range.range, "");
        assert_eq!(request.value_range.values, Value::Null);
    }

    #[test]
    fn test_prepend_data_request_builder_default() {
        let builder = PrependDataRequestBuilder::default();
        let request = builder.build();

        assert_eq!(request.spreadsheet_token, "");
        assert_eq!(request.value_range.range, "");
        assert_eq!(request.value_range.values, Value::Null);
    }

    #[test]
    fn test_prepend_data_request_serialization() {
        let test_values = json!([["Col1", "Col2"], ["Val1", "Val2"]]);

        let request = PrependDataRequest::builder()
            .spreadsheet_token("serialize_token")
            .range("Sheet1!A:B")
            .values(test_values)
            .build();

        let serialized = serde_json::to_string(&request);
        assert!(serialized.is_ok());

        let json_str = serialized.unwrap();
        assert!(json_str.contains("valueRange"));
        assert!(!json_str.contains("spreadsheet_token")); // Should be skipped
    }

    #[test]
    fn test_prepend_data_request_debug() {
        let request = PrependDataRequest::builder()
            .spreadsheet_token("debug_prepend_token")
            .build();

        let debug_str = format!("{:?}", request);
        assert!(debug_str.contains("PrependDataRequest"));
        assert!(debug_str.contains("debug_prepend_token"));
    }

    #[test]
    fn test_prepend_data_request_with_empty_values() {
        let request = PrependDataRequest::builder()
            .spreadsheet_token("")
            .range("")
            .values(Value::Null)
            .build();

        assert_eq!(request.spreadsheet_token, "");
        assert_eq!(request.value_range.range, "");
        assert_eq!(request.value_range.values, Value::Null);
    }

    #[test]
    fn test_prepend_data_request_with_special_characters() {
        let request = PrependDataRequest::builder()
            .spreadsheet_token("token_特殊字符_prepend_🎉")
            .range("工作表!A1:Z999")
            .build();

        assert_eq!(request.spreadsheet_token, "token_特殊字符_prepend_🎉");
        assert_eq!(request.value_range.range, "工作表!A1:Z999");
    }

    #[test]
    fn test_prepend_data_request_with_unicode_data() {
        let unicode_values = json!([
            ["名称", "描述", "状态"],
            ["测试项目", "这是一个测试项目", "活跃"],
            ["实验数据", "包含中文和English混合", "完成"],
            ["🚀项目", "包含emoji和特殊符号", "进行中"]
        ]);

        let request = PrependDataRequest::builder()
            .values(unicode_values.clone())
            .build();

        assert_eq!(request.value_range.values, unicode_values);
    }

    #[test]
    fn test_prepend_data_request_with_complex_json_structures() {
        let complex_data = json!([
            [
                {"type": "formula", "value": "=SUM(A1:A10)"},
                {"type": "link", "url": "https://example.com", "text": "Link"},
                {"type": "mention", "user_id": "user123"}
            ],
            [
                {"number": 42.5, "currency": "USD"},
                {"date": "2023-12-25", "format": "yyyy-mm-dd"},
                {"boolean": true, "confidence": 0.95}
            ]
        ]);

        let request = PrependDataRequest::builder()
            .values(complex_data.clone())
            .build();

        assert_eq!(request.value_range.values, complex_data);
    }

    #[test]
    fn test_prepend_data_request_with_different_range_formats() {
        let range_formats = vec![
            "Sheet1!A1:B2",
            "工作表名称!C:D",
            "'Sheet with spaces'!E5:F10",
            "Sheet2!A:A",
            "数据表!1:5",
            "Sheet!$A$1:$B$10",
        ];

        for range_format in range_formats {
            let request = PrependDataRequest::builder().range(range_format).build();

            assert_eq!(request.value_range.range, range_format);
        }
    }

    #[test]
    fn test_prepend_data_request_with_various_data_types() {
        let mixed_types = vec![
            json!("string_value"),
            json!(123),
            json!(45.67),
            json!(true),
            json!(false),
            json!(null),
            json!([1, 2, 3]),
            json!({"key": "value", "nested": {"inner": "data"}}),
        ];

        for data_type in mixed_types {
            let request = PrependDataRequest::builder()
                .values(data_type.clone())
                .build();

            assert_eq!(request.value_range.values, data_type);
        }
    }

    #[test]
    fn test_prepend_data_request_api_body_serialization() {
        let test_data = json!([["X", "Y"], ["1", "2"]]);

        let request = PrependDataRequest::builder()
            .spreadsheet_token("body_test")
            .range("Sheet1!A:B")
            .values(test_data)
            .build();

        // Verify body is populated
        assert!(!request.api_request.body.is_empty());

        // Verify body contains valid JSON
        let body_str = String::from_utf8(request.api_request.body).unwrap();
        let parsed: Value = serde_json::from_str(&body_str).unwrap();

        assert!(parsed.get("valueRange").is_some());
        assert!(parsed.get("spreadsheet_token").is_none()); // Should be skipped
    }

    #[test]
    fn test_prepend_data_request_builder_overwrite_behavior() {
        let mut builder = PrependDataRequest::builder();

        // Test that subsequent calls overwrite previous values
        builder = builder.spreadsheet_token("first_token");
        builder = builder.spreadsheet_token("final_token");
        builder = builder.range("first_range");
        builder = builder.range("final_range");

        let request = builder.build();

        assert_eq!(request.spreadsheet_token, "final_token");
        assert_eq!(request.value_range.range, "final_range");
    }

    #[test]
    fn test_prepend_data_request_with_large_dataset() {
        let large_data = json!((0..500)
            .map(|i| vec![
                format!("Item_{}", i),
                format!("Category_{}", i % 10),
                format!("Description for item number {}", i),
                (i * 2).to_string(),
            ])
            .collect::<Vec<_>>());

        let request = PrependDataRequest::builder()
            .spreadsheet_token("large_dataset_token")
            .range("BigSheet!A:D")
            .values(large_data.clone())
            .build();

        assert_eq!(request.value_range.values, large_data);
        assert_eq!(request.spreadsheet_token, "large_dataset_token");
    }

    #[test]
    fn test_prepend_data_request_edge_cases() {
        // Test with extremely long token
        let very_long_token = "x".repeat(5000);
        let request = PrependDataRequest::builder()
            .spreadsheet_token(&very_long_token)
            .build();
        assert_eq!(request.spreadsheet_token, very_long_token);

        // Test with empty array
        let empty_array = json!([]);
        let request = PrependDataRequest::builder()
            .values(empty_array.clone())
            .build();
        assert_eq!(request.value_range.values, empty_array);

        // Test with deeply nested structures
        let nested_data = json!([[[[[["deep", "nesting"], ["test", "data"]]]]]]);
        let request = PrependDataRequest::builder()
            .values(nested_data.clone())
            .build();
        assert_eq!(request.value_range.values, nested_data);
    }

    #[test]
    fn test_spreadsheet_sheet_service_creation() {
        let service = create_test_service();
        assert_eq!(service.config.app_id, "test_app_id");
        assert_eq!(service.config.app_type, AppType::SelfBuild);
    }

    #[test]
    fn test_prepend_data_response_type_alias() {
        // Verify that PrependDataResponse is correctly aliased
        let _response: PrependDataResponse = UpdateSheetDataResponse {
            spreed_sheet_token: "prepend_test".to_string(),
            table_range: "A1:B5".to_string(),
            revision: 2,
            updates: crate::service::cloud_docs::sheets::v2::data_operation::SheetDataUpdates {
                spreed_sheet_token: "prepend_test".to_string(),
                updated_range: "A1:B5".to_string(),
                updated_rows: 5,
                updated_columns: 2,
                updated_cells: 10,
                revision: Some(2),
            },
        };
    }

    #[test]
    fn test_prepend_data_request_memory_efficiency() {
        // Test creating multiple requests efficiently
        let requests: Vec<PrependDataRequest> = (0..50)
            .map(|i| {
                PrependDataRequest::builder()
                    .spreadsheet_token(format!("efficient_token_{}", i))
                    .range(format!("Sheet{}!A:C", i + 1))
                    .values(json!([[
                        format!("Row{}", i),
                        i.to_string(),
                        format!("Data{}", i)
                    ]]))
                    .build()
            })
            .collect();

        assert_eq!(requests.len(), 50);

        // Verify data integrity across all requests
        for (i, request) in requests.iter().enumerate() {
            assert_eq!(request.spreadsheet_token, format!("efficient_token_{}", i));
            assert_eq!(request.value_range.range, format!("Sheet{}!A:C", i + 1));
        }
    }

    #[test]
    fn test_value_range_request_within_prepend_context() {
        let value_range = ValueRangeRequest {
            range: "PrependTest!A1:B2".to_string(),
            values: json!([["Prepend1", "Prepend2"]]),
        };

        let request = PrependDataRequest {
            api_request: ApiRequest::default(),
            spreadsheet_token: "vr_test".to_string(),
            value_range,
        };

        assert_eq!(request.value_range.range, "PrependTest!A1:B2");
        assert_eq!(
            request.value_range.values,
            json!([["Prepend1", "Prepend2"]])
        );
    }

    #[test]
    fn test_prepend_data_request_json_serialization_completeness() {
        let comprehensive_data = json!([
            ["Text", "Number", "Boolean", "Null", "Array", "Object"],
            ["Sample", 42, true, null, [1, 2, 3], {"nested": "value"}],
            ["Unicode: 中文", 3.14286, false, null, [], {}]
        ]);

        let request = PrependDataRequest::builder()
            .spreadsheet_token("comprehensive_test")
            .range("Complete!A1:F3")
            .values(comprehensive_data.clone())
            .build();

        let serialized = serde_json::to_string(&request).unwrap();
        let parsed: Value = serde_json::from_str(&serialized).unwrap();

        assert_eq!(parsed["valueRange"]["values"], comprehensive_data);
        assert_eq!(parsed["valueRange"]["range"], "Complete!A1:F3");
    }
}

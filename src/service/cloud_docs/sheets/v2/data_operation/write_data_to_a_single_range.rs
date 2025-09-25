use serde::Serialize;

use crate::{
    core::{
        api_req::ApiRequest, api_resp::BaseResponse, constants::AccessTokenType,
        endpoints::cloud_docs::*, req_option::RequestOption, SDKResult,
    },
    impl_executable_builder_owned,
    service::cloud_docs::sheets::v2::{
        data_operation::{SheetDataUpdates, ValueRangeRequest},
        SpreadsheetService,
    },
};

/// 向单个范围写入数据 请求体
#[derive(Serialize, Debug, Default)]
pub struct WriteDataToSingleRangeRequest {
    #[serde(skip)]
    api_request: ApiRequest,
    #[serde(skip)]
    spreadsheet_token: String,
    #[serde(rename = "valueRange")]
    value_range_request: ValueRangeRequest,
}

impl WriteDataToSingleRangeRequest {
    pub fn builder() -> WriteDataToSingleRangeBuilder {
        WriteDataToSingleRangeBuilder::default()
    }
}

#[derive(Default)]
pub struct WriteDataToSingleRangeBuilder {
    request: WriteDataToSingleRangeRequest,
}

impl WriteDataToSingleRangeBuilder {
    pub fn spreadsheet_token(mut self, spreadsheet_token: impl ToString) -> Self {
        self.request.spreadsheet_token = spreadsheet_token.to_string();
        self
    }

    pub fn range(mut self, range: impl ToString) -> Self {
        self.request.value_range_request.range = range.to_string();
        self
    }

    pub fn values(mut self, values: serde_json::Value) -> Self {
        self.request.value_range_request.values = values;
        self
    }

    pub fn build(mut self) -> WriteDataToSingleRangeRequest {
        self.request.api_request.body = serde_json::to_vec(&self.request).unwrap();
        self.request
    }
}

/// 写入单个范围响应体
pub type WriteDataToSingleRangeResponse = SheetDataUpdates;

// 使用宏实现ExecutableBuilder trait
impl_executable_builder_owned!(
    WriteDataToSingleRangeBuilder,
    SpreadsheetService,
    WriteDataToSingleRangeRequest,
    BaseResponse<WriteDataToSingleRangeResponse>,
    write_data_to_single_range
);

impl SpreadsheetService {
    /// 写入单个范围
    pub async fn write_data_to_single_range(
        &self,
        request: WriteDataToSingleRangeRequest,
        option: Option<RequestOption>,
    ) -> SDKResult<BaseResponse<WriteDataToSingleRangeResponse>> {
        let mut api_req = request.api_request;
        api_req.api_path = SHEETS_V2_SPREADSHEET_VALUES.replace("{}", &request.spreadsheet_token);
        api_req.http_method = reqwest::Method::PUT;
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::App];

        let api_resp = crate::core::http::Transport::request(api_req, &self.config, option).await?;

        Ok(api_resp)
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use crate::{
        core::{config::Config, constants::AppType},
        service::cloud_docs::sheets::v2::{
            data_operation::{SheetDataUpdates, WriteDataToSingleRangeRequest},
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
    fn test_write_data_to_single_range_builder_default() {
        let request = WriteDataToSingleRangeRequest::builder().build();

        assert_eq!(request.spreadsheet_token, "");
        assert_eq!(request.value_range_request.range, "");
        assert_eq!(request.value_range_request.values, json!(null));
    }

    #[test]
    fn test_write_data_to_single_range_builder_basic() {
        let request = WriteDataToSingleRangeRequest::builder()
            .spreadsheet_token("test_token")
            .range("Sheet1!A1:B2")
            .values(json!([["Name", "Age"], ["Alice", 25]]))
            .build();

        assert_eq!(request.spreadsheet_token, "test_token");
        assert_eq!(request.value_range_request.range, "Sheet1!A1:B2");
        assert_eq!(
            request.value_range_request.values,
            json!([["Name", "Age"], ["Alice", 25]])
        );
    }

    #[test]
    fn test_write_data_to_single_range_builder_all_options() {
        let values = json!([
            ["Product", "Price", "Stock"],
            ["Laptop", 1299.99, 15],
            ["Mouse", 29.99, 100],
            ["Keyboard", 79.99, 50]
        ]);

        let request = WriteDataToSingleRangeRequest::builder()
            .spreadsheet_token("spreadsheet_abc123")
            .range("Products!A1:C4")
            .values(values.clone())
            .build();

        assert_eq!(request.spreadsheet_token, "spreadsheet_abc123");
        assert_eq!(request.value_range_request.range, "Products!A1:C4");
        assert_eq!(request.value_range_request.values, values);
    }

    #[test]
    fn test_write_data_to_single_range_builder_chaining() {
        let request = WriteDataToSingleRangeRequest::builder()
            .spreadsheet_token("test123")
            .range("Sheet1!A1:A3")
            .values(json!(["Item1", "Item2", "Item3"]))
            .build();

        assert_eq!(request.spreadsheet_token, "test123");
        assert_eq!(request.value_range_request.range, "Sheet1!A1:A3");
        if let serde_json::Value::Array(values) = &request.value_range_request.values {
            assert_eq!(values.len(), 3);
            assert_eq!(values[0], "Item1");
            assert_eq!(values[1], "Item2");
            assert_eq!(values[2], "Item3");
        } else {
            panic!("Expected array values");
        }
    }

    #[test]
    fn test_write_data_to_single_range_with_unicode() {
        let unicode_data = json!([
            ["姓名", "年龄", "城市"],
            ["张三", 28, "北京"],
            ["李四", 35, "上海"],
            ["王五", 42, "深圳"]
        ]);

        let request = WriteDataToSingleRangeRequest::builder()
            .spreadsheet_token("unicode_test_token")
            .range("员工表!A1:C4")
            .values(unicode_data.clone())
            .build();

        assert_eq!(request.spreadsheet_token, "unicode_test_token");
        assert_eq!(request.value_range_request.range, "员工表!A1:C4");
        assert_eq!(request.value_range_request.values, unicode_data);
    }

    #[test]
    fn test_write_data_to_single_range_with_formulas() {
        let formula_data = json!([
            ["Item", "Quantity", "Price", "Total"],
            ["Apples", 10, 1.5, "=B2*C2"],
            ["Bananas", 20, 0.8, "=B3*C3"],
            ["Sum", "", "", "=SUM(D2:D3)"]
        ]);

        let request = WriteDataToSingleRangeRequest::builder()
            .spreadsheet_token("formula_test")
            .range("Calculations!A1:D4")
            .values(formula_data.clone())
            .build();

        assert_eq!(request.spreadsheet_token, "formula_test");
        assert_eq!(request.value_range_request.range, "Calculations!A1:D4");
        assert_eq!(request.value_range_request.values, formula_data);
    }

    #[test]
    fn test_write_data_to_single_range_with_mixed_types() {
        let mixed_data = json!([
            ["Name", "Age", "Active", "Salary", "Date"],
            ["John", 30, true, 50000.50, "2024-01-15"],
            ["Jane", 25, false, 45000.00, "2024-02-20"],
            [null, 35, true, 60000.75, "2024-03-10"]
        ]);

        let request = WriteDataToSingleRangeRequest::builder()
            .spreadsheet_token("mixed_types_test")
            .range("Employees!A1:E4")
            .values(mixed_data.clone())
            .build();

        assert_eq!(request.spreadsheet_token, "mixed_types_test");
        assert_eq!(request.value_range_request.range, "Employees!A1:E4");
        assert_eq!(request.value_range_request.values, mixed_data);
    }

    #[test]
    fn test_write_data_to_single_range_empty_values() {
        let request = WriteDataToSingleRangeRequest::builder()
            .spreadsheet_token("empty_test")
            .range("Sheet1!A1:A1")
            .values(json!([]))
            .build();

        assert_eq!(request.spreadsheet_token, "empty_test");
        assert_eq!(request.value_range_request.range, "Sheet1!A1:A1");
        assert_eq!(request.value_range_request.values, json!([]));
    }

    #[test]
    fn test_write_data_to_single_range_single_cell() {
        let request = WriteDataToSingleRangeRequest::builder()
            .spreadsheet_token("single_cell_test")
            .range("Sheet1!A1")
            .values(json!("Hello World"))
            .build();

        assert_eq!(request.spreadsheet_token, "single_cell_test");
        assert_eq!(request.value_range_request.range, "Sheet1!A1");
        assert_eq!(request.value_range_request.values, json!("Hello World"));
    }

    #[test]
    fn test_write_data_to_single_range_large_dataset() {
        let mut large_data = Vec::new();
        for i in 0..100 {
            large_data.push(json!([format!("Item{}", i), i * 10, i % 2 == 0]));
        }

        let request = WriteDataToSingleRangeRequest::builder()
            .spreadsheet_token("large_dataset_test")
            .range("Data!A1:C100")
            .values(json!(large_data))
            .build();

        assert_eq!(request.spreadsheet_token, "large_dataset_test");
        assert_eq!(request.value_range_request.range, "Data!A1:C100");

        if let serde_json::Value::Array(values) = &request.value_range_request.values {
            assert_eq!(values.len(), 100);
            if let serde_json::Value::Array(first_row) = &values[0] {
                assert_eq!(first_row[0], "Item0");
                assert_eq!(first_row[1], 0);
                assert_eq!(first_row[2], true);
            }
        }
    }

    #[test]
    fn test_write_data_to_single_range_special_characters() {
        let special_data = json!([
            ["Email", "URL", "Special"],
            [
                "test@example.com",
                "https://example.com",
                "Special chars: !@#$%^&*()"
            ],
            [
                "user+tag@domain.co.uk",
                "https://sub.example.org/path?q=1",
                "Unicode: 你好 🌟 ñáéíóú"
            ]
        ]);

        let request = WriteDataToSingleRangeRequest::builder()
            .spreadsheet_token("special_chars_test")
            .range("Sheet1!A1:C3")
            .values(special_data.clone())
            .build();

        assert_eq!(request.spreadsheet_token, "special_chars_test");
        assert_eq!(request.value_range_request.range, "Sheet1!A1:C3");
        assert_eq!(request.value_range_request.values, special_data);
    }

    #[test]
    fn test_write_data_to_single_range_numeric_precision() {
        let numeric_data = json!([
            ["Integer", "Float", "Scientific", "Currency"],
            [42, 3.14286, 1.23e-4, 1299.99],
            [-100, -0.001, 6.022e23, 0.01],
            [0, 0.0, 0.0e0, 0.00]
        ]);

        let request = WriteDataToSingleRangeRequest::builder()
            .spreadsheet_token("numeric_test")
            .range("Numbers!A1:D4")
            .values(numeric_data.clone())
            .build();

        assert_eq!(request.spreadsheet_token, "numeric_test");
        assert_eq!(request.value_range_request.range, "Numbers!A1:D4");
        assert_eq!(request.value_range_request.values, numeric_data);
    }

    #[test]
    fn test_write_data_to_single_range_response_type() {
        use crate::service::cloud_docs::sheets::v2::data_operation::WriteDataToSingleRangeResponse;
        use std::any::TypeId;

        // Verify response type alias is correct
        assert_eq!(
            TypeId::of::<WriteDataToSingleRangeResponse>(),
            TypeId::of::<SheetDataUpdates>()
        );
    }

    #[test]
    fn test_write_data_to_single_range_serialization() {
        let request = WriteDataToSingleRangeRequest::builder()
            .spreadsheet_token("serialization_test")
            .range("Sheet1!A1:B2")
            .values(json!([["Key", "Value"], ["Test", 123]]))
            .build();

        // Test that request can be serialized
        let serialized = serde_json::to_string(&request);
        assert!(serialized.is_ok());

        let json_value: serde_json::Value = serde_json::from_str(&serialized.unwrap()).unwrap();
        assert_eq!(json_value["valueRange"]["range"], "Sheet1!A1:B2");
        assert_eq!(json_value["valueRange"]["values"][0][0], "Key");
        assert_eq!(json_value["valueRange"]["values"][1][1], 123);
    }

    #[test]
    fn test_write_data_to_single_range_complex_sheet_reference() {
        let request = WriteDataToSingleRangeRequest::builder()
            .spreadsheet_token("complex_ref_test")
            .range("'Sheet With Spaces'!A1:Z100")
            .values(json!([["Header1"], ["Data1"]]))
            .build();

        assert_eq!(request.spreadsheet_token, "complex_ref_test");
        assert_eq!(
            request.value_range_request.range,
            "'Sheet With Spaces'!A1:Z100"
        );
    }

    #[test]
    fn test_write_data_to_single_range_hyperlinks() {
        let hyperlink_data = json!([
            ["Link Name", "URL"],
            ["Google", "https://www.google.com"],
            ["GitHub", "https://github.com"],
            ["Documentation", "https://docs.example.com/api"]
        ]);

        let request = WriteDataToSingleRangeRequest::builder()
            .spreadsheet_token("hyperlink_test")
            .range("Links!A1:B4")
            .values(hyperlink_data.clone())
            .build();

        assert_eq!(request.spreadsheet_token, "hyperlink_test");
        assert_eq!(request.value_range_request.range, "Links!A1:B4");
        assert_eq!(request.value_range_request.values, hyperlink_data);
    }

    #[test]
    fn test_write_data_to_single_range_dates_and_times() {
        let datetime_data = json!([
            ["Date", "Time", "DateTime", "Timestamp"],
            ["2024-01-15", "14:30:00", "2024-01-15T14:30:00Z", 1705329000],
            ["2024-02-20", "09:15:30", "2024-02-20T09:15:30Z", 1708419330],
            ["2024-03-10", "18:45:15", "2024-03-10T18:45:15Z", 1710093915]
        ]);

        let request = WriteDataToSingleRangeRequest::builder()
            .spreadsheet_token("datetime_test")
            .range("Schedule!A1:D4")
            .values(datetime_data.clone())
            .build();

        assert_eq!(request.spreadsheet_token, "datetime_test");
        assert_eq!(request.value_range_request.range, "Schedule!A1:D4");
        assert_eq!(request.value_range_request.values, datetime_data);
    }

    #[test]
    fn test_write_data_to_single_range_service_creation() {
        let service = create_service();
        assert_eq!(service.config.app_id, "test_app_id");
        assert_eq!(service.config.app_secret, "test_app_secret");
        assert!(matches!(service.config.app_type, AppType::SelfBuild));
    }

    #[test]
    fn test_write_data_to_single_range_boolean_values() {
        let boolean_data = json!([
            ["Question", "Answer"],
            ["Is active?", true],
            ["Is deleted?", false],
            ["Has permissions?", true],
            ["Is expired?", false]
        ]);

        let request = WriteDataToSingleRangeRequest::builder()
            .spreadsheet_token("boolean_test")
            .range("Flags!A1:B5")
            .values(boolean_data.clone())
            .build();

        assert_eq!(request.spreadsheet_token, "boolean_test");
        assert_eq!(request.value_range_request.range, "Flags!A1:B5");
        assert_eq!(request.value_range_request.values, boolean_data);
    }

    #[test]
    fn test_write_data_to_single_range_null_values() {
        let null_data = json!([
            ["Name", "Optional Field", "Required Field"],
            ["Alice", null, "Required1"],
            ["Bob", "Optional Value", "Required2"],
            ["Charlie", null, null]
        ]);

        let request = WriteDataToSingleRangeRequest::builder()
            .spreadsheet_token("null_test")
            .range("Data!A1:C4")
            .values(null_data.clone())
            .build();

        assert_eq!(request.spreadsheet_token, "null_test");
        assert_eq!(request.value_range_request.range, "Data!A1:C4");
        assert_eq!(request.value_range_request.values, null_data);
    }

    #[test]
    fn test_write_data_to_single_range_very_long_token() {
        let very_long_token = "a".repeat(1000);
        let request = WriteDataToSingleRangeRequest::builder()
            .spreadsheet_token(&very_long_token)
            .range("Sheet1!A1")
            .values(json!("test"))
            .build();

        assert_eq!(request.spreadsheet_token, very_long_token);
        assert_eq!(request.value_range_request.range, "Sheet1!A1");
        assert_eq!(request.value_range_request.values, json!("test"));
    }
}

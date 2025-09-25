use serde::{Deserialize, Serialize};

use crate::{
    core::{
        api_req::ApiRequest,
        api_resp::{ApiResponseTrait, BaseResponse, ResponseFormat},
        constants::AccessTokenType,
        endpoints::cloud_docs::*,
        req_option, SDKResult,
    },
    impl_executable_builder_owned,
    service::cloud_docs::sheets::v2::{data_operation::ValueRangeRequest, SpreadsheetService},
};

/// 向多个范围写入数据请求
#[derive(Serialize, Debug, Default)]
pub struct WriteDataToMultiRangesRequest {
    #[serde(skip)]
    api_request: ApiRequest,
    #[serde(skip)]
    spreadsheet_token: String,
    #[serde(rename = "valueRanges")]
    value_ranges: Vec<ValueRangeRequest>,
}

impl WriteDataToMultiRangesRequest {
    pub fn builder() -> WriteDataToMultiRangesBuilder {
        WriteDataToMultiRangesBuilder::default()
    }
}

#[derive(Default)]
pub struct WriteDataToMultiRangesBuilder {
    request: WriteDataToMultiRangesRequest,
}

impl WriteDataToMultiRangesBuilder {
    pub fn spreadsheet_token(mut self, spreadsheet_token: impl ToString) -> Self {
        self.request.spreadsheet_token = spreadsheet_token.to_string();
        self
    }

    pub fn add_value_range(mut self, range: impl ToString, values: serde_json::Value) -> Self {
        self.request.value_ranges.push(ValueRangeRequest {
            range: range.to_string(),
            values,
        });
        self
    }

    pub fn build(mut self) -> WriteDataToMultiRangesRequest {
        self.request.api_request.body = serde_json::to_vec(&self.request).unwrap();
        self.request
    }
}

/// 向多个范围写入数据响应体
#[derive(Deserialize, Debug)]
pub struct WriteDataToMultiRangesResponse {
    /// spreadsheet 的 token
    #[serde(rename = "spreadsheetToken")]
    pub spreadsheet_token: String,
    /// sheet 的版本号
    pub revision: i32,
    /// 响应
    pub responses: Vec<DataResponse>,
}

/// 追加数据的范围、行列数等
#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct DataResponse {
    /// spreadsheet 的 token
    #[serde(rename = "spreadsheetToken")]
    pub spreed_sheet_token: String,
    /// 写入的行数
    #[serde(rename = "updatedRows")]
    pub updated_rows: i32,
    /// 写入的列数
    #[serde(rename = "updatedColumns")]
    pub updated_columns: i32,
    /// 写入的单元格总数
    #[serde(rename = "updatedCells")]
    pub updated_cells: i32,
}

impl ApiResponseTrait for WriteDataToMultiRangesResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

// 使用宏实现ExecutableBuilder trait
impl_executable_builder_owned!(
    WriteDataToMultiRangesBuilder,
    SpreadsheetService,
    WriteDataToMultiRangesRequest,
    BaseResponse<WriteDataToMultiRangesResponse>,
    write_data_multi_ranges
);

impl SpreadsheetService {
    /// 向多个范围写入数据
    pub async fn write_data_multi_ranges(
        &self,
        request: WriteDataToMultiRangesRequest,
        option: Option<req_option::RequestOption>,
    ) -> SDKResult<BaseResponse<WriteDataToMultiRangesResponse>> {
        let mut api_req = request.api_request;
        api_req.api_path =
            SHEETS_V2_SPREADSHEET_VALUES_BATCH_UPDATE.replace("{}", &request.spreadsheet_token);
        api_req.http_method = reqwest::Method::POST;
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
            data_operation::{
                DataResponse, WriteDataToMultiRangesRequest, WriteDataToMultiRangesResponse,
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

    #[test]
    fn test_write_data_to_multi_ranges_builder_default() {
        let request = WriteDataToMultiRangesRequest::builder().build();

        assert_eq!(request.spreadsheet_token, "");
        assert_eq!(request.value_ranges.len(), 0);
    }

    #[test]
    fn test_write_data_to_multi_ranges_builder_basic() {
        let request = WriteDataToMultiRangesRequest::builder()
            .spreadsheet_token("test_token")
            .add_value_range("Sheet1!A1:B2", json!([["Name", "Age"], ["Alice", 25]]))
            .build();

        assert_eq!(request.spreadsheet_token, "test_token");
        assert_eq!(request.value_ranges.len(), 1);
        assert_eq!(request.value_ranges[0].range, "Sheet1!A1:B2");
        assert_eq!(
            request.value_ranges[0].values,
            json!([["Name", "Age"], ["Alice", 25]])
        );
    }

    #[test]
    fn test_write_data_to_multi_ranges_multiple_ranges() {
        let request = WriteDataToMultiRangesRequest::builder()
            .spreadsheet_token("multi_test_token")
            .add_value_range("Sheet1!A1:B2", json!([["Name", "Age"], ["Alice", 25]]))
            .add_value_range(
                "Sheet1!D1:E2",
                json!([["City", "Country"], ["Paris", "France"]]),
            )
            .add_value_range("Sheet2!A1:C1", json!([["Product", "Price", "Stock"]]))
            .build();

        assert_eq!(request.spreadsheet_token, "multi_test_token");
        assert_eq!(request.value_ranges.len(), 3);

        assert_eq!(request.value_ranges[0].range, "Sheet1!A1:B2");
        assert_eq!(request.value_ranges[1].range, "Sheet1!D1:E2");
        assert_eq!(request.value_ranges[2].range, "Sheet2!A1:C1");

        assert_eq!(
            request.value_ranges[0].values,
            json!([["Name", "Age"], ["Alice", 25]])
        );
        assert_eq!(
            request.value_ranges[1].values,
            json!([["City", "Country"], ["Paris", "France"]])
        );
        assert_eq!(
            request.value_ranges[2].values,
            json!([["Product", "Price", "Stock"]])
        );
    }

    #[test]
    fn test_write_data_to_multi_ranges_chaining() {
        let request = WriteDataToMultiRangesRequest::builder()
            .spreadsheet_token("chain_test")
            .add_value_range("A1:A3", json!(["Item1", "Item2", "Item3"]))
            .add_value_range("B1:B3", json!([100, 200, 300]))
            .add_value_range("C1:C3", json!([true, false, true]))
            .build();

        assert_eq!(request.spreadsheet_token, "chain_test");
        assert_eq!(request.value_ranges.len(), 3);

        if let serde_json::Value::Array(values) = &request.value_ranges[0].values {
            assert_eq!(values.len(), 3);
            assert_eq!(values[0], "Item1");
            assert_eq!(values[1], "Item2");
            assert_eq!(values[2], "Item3");
        } else {
            panic!("Expected array values");
        }

        if let serde_json::Value::Array(values) = &request.value_ranges[1].values {
            assert_eq!(values[0], 100);
            assert_eq!(values[1], 200);
            assert_eq!(values[2], 300);
        }

        if let serde_json::Value::Array(values) = &request.value_ranges[2].values {
            assert_eq!(values[0], true);
            assert_eq!(values[1], false);
            assert_eq!(values[2], true);
        }
    }

    #[test]
    fn test_write_data_to_multi_ranges_with_unicode() {
        let unicode_data1 = json!([["姓名", "年龄"], ["张三", 28], ["李四", 35]]);

        let unicode_data2 = json!([["城市", "国家"], ["北京", "中国"], ["东京", "日本"]]);

        let request = WriteDataToMultiRangesRequest::builder()
            .spreadsheet_token("unicode_multi_test")
            .add_value_range("员工表!A1:B3", unicode_data1.clone())
            .add_value_range("城市表!A1:B3", unicode_data2.clone())
            .build();

        assert_eq!(request.spreadsheet_token, "unicode_multi_test");
        assert_eq!(request.value_ranges.len(), 2);
        assert_eq!(request.value_ranges[0].range, "员工表!A1:B3");
        assert_eq!(request.value_ranges[1].range, "城市表!A1:B3");
        assert_eq!(request.value_ranges[0].values, unicode_data1);
        assert_eq!(request.value_ranges[1].values, unicode_data2);
    }

    #[test]
    fn test_write_data_to_multi_ranges_with_formulas() {
        let formula_data = json!([
            ["Item", "Quantity", "Price", "Total"],
            ["Apples", 10, 1.5, "=B2*C2"],
            ["Bananas", 20, 0.8, "=B3*C3"]
        ]);

        let summary_data = json!([["Summary"], ["=SUM(D2:D3)"]]);

        let request = WriteDataToMultiRangesRequest::builder()
            .spreadsheet_token("formula_multi_test")
            .add_value_range("Data!A1:D3", formula_data.clone())
            .add_value_range("Summary!A1:A2", summary_data.clone())
            .build();

        assert_eq!(request.spreadsheet_token, "formula_multi_test");
        assert_eq!(request.value_ranges.len(), 2);
        assert_eq!(request.value_ranges[0].values, formula_data);
        assert_eq!(request.value_ranges[1].values, summary_data);
    }

    #[test]
    fn test_write_data_to_multi_ranges_mixed_data_types() {
        let mixed_data = json!([
            ["Name", "Age", "Active", "Salary"],
            ["John", 30, true, 50000.50],
            ["Jane", 25, false, 45000.00],
            [null, 35, true, 60000.75]
        ]);

        let metadata = json!([
            ["Created", "2024-01-15"],
            ["Updated", "2024-01-20"],
            ["Version", 1.2]
        ]);

        let request = WriteDataToMultiRangesRequest::builder()
            .spreadsheet_token("mixed_types_multi")
            .add_value_range("Employees!A1:D4", mixed_data.clone())
            .add_value_range("Metadata!A1:B3", metadata.clone())
            .build();

        assert_eq!(request.spreadsheet_token, "mixed_types_multi");
        assert_eq!(request.value_ranges.len(), 2);
        assert_eq!(request.value_ranges[0].values, mixed_data);
        assert_eq!(request.value_ranges[1].values, metadata);
    }

    #[test]
    fn test_write_data_to_multi_ranges_empty_ranges() {
        let request = WriteDataToMultiRangesRequest::builder()
            .spreadsheet_token("empty_test")
            .add_value_range("Sheet1!A1:A1", json!([]))
            .add_value_range("Sheet1!B1:B1", json!(null))
            .build();

        assert_eq!(request.spreadsheet_token, "empty_test");
        assert_eq!(request.value_ranges.len(), 2);
        assert_eq!(request.value_ranges[0].values, json!([]));
        assert_eq!(request.value_ranges[1].values, json!(null));
    }

    #[test]
    fn test_write_data_to_multi_ranges_single_cells() {
        let request = WriteDataToMultiRangesRequest::builder()
            .spreadsheet_token("single_cells_test")
            .add_value_range("Sheet1!A1", json!("Hello"))
            .add_value_range("Sheet1!B1", json!(42))
            .add_value_range("Sheet1!C1", json!(true))
            .add_value_range("Sheet1!D1", json!(3.14286))
            .build();

        assert_eq!(request.spreadsheet_token, "single_cells_test");
        assert_eq!(request.value_ranges.len(), 4);
        assert_eq!(request.value_ranges[0].values, json!("Hello"));
        assert_eq!(request.value_ranges[1].values, json!(42));
        assert_eq!(request.value_ranges[2].values, json!(true));
        assert_eq!(request.value_ranges[3].values, json!(3.14286));
    }

    #[test]
    fn test_write_data_to_multi_ranges_large_dataset() {
        let mut request_builder =
            WriteDataToMultiRangesRequest::builder().spreadsheet_token("large_multi_test");

        for i in 0..10 {
            let mut range_data = Vec::new();
            for j in 0..10 {
                range_data.push(json!([format!("Item{}-{}", i, j), i * 10 + j, j % 2 == 0]));
            }
            request_builder = request_builder
                .add_value_range(format!("Sheet{}!A1:C10", i + 1), json!(range_data));
        }

        let request = request_builder.build();

        assert_eq!(request.spreadsheet_token, "large_multi_test");
        assert_eq!(request.value_ranges.len(), 10);

        for (i, range) in request.value_ranges.iter().enumerate() {
            assert_eq!(range.range, format!("Sheet{}!A1:C10", i + 1));
            if let serde_json::Value::Array(rows) = &range.values {
                assert_eq!(rows.len(), 10);
                if let serde_json::Value::Array(first_row) = &rows[0] {
                    assert_eq!(first_row[0], format!("Item{}-0", i));
                    assert_eq!(first_row[1], i * 10);
                    assert_eq!(first_row[2], true);
                }
            }
        }
    }

    #[test]
    fn test_write_data_to_multi_ranges_special_characters() {
        let special_data1 = json!([
            ["Email", "URL"],
            ["test@example.com", "https://example.com"],
            ["user+tag@domain.co.uk", "https://sub.example.org/path?q=1"]
        ]);

        let special_data2 = json!([
            ["Special Characters"],
            ["!@#$%^&*()"],
            ["Unicode: 你好 🌟 ñáéíóú"]
        ]);

        let request = WriteDataToMultiRangesRequest::builder()
            .spreadsheet_token("special_chars_multi")
            .add_value_range("Contact!A1:B3", special_data1.clone())
            .add_value_range("Special!A1:A3", special_data2.clone())
            .build();

        assert_eq!(request.spreadsheet_token, "special_chars_multi");
        assert_eq!(request.value_ranges.len(), 2);
        assert_eq!(request.value_ranges[0].values, special_data1);
        assert_eq!(request.value_ranges[1].values, special_data2);
    }

    #[test]
    fn test_write_data_to_multi_ranges_numeric_precision() {
        let numeric_data1 = json!([["Integer", "Float"], [42, 3.14286], [-100, -0.001]]);

        let numeric_data2 = json!([
            ["Scientific", "Currency"],
            [1.23e-4, 1299.99],
            [6.022e23, 0.01]
        ]);

        let request = WriteDataToMultiRangesRequest::builder()
            .spreadsheet_token("numeric_multi_test")
            .add_value_range("Numbers1!A1:B3", numeric_data1.clone())
            .add_value_range("Numbers2!A1:B3", numeric_data2.clone())
            .build();

        assert_eq!(request.spreadsheet_token, "numeric_multi_test");
        assert_eq!(request.value_ranges.len(), 2);
        assert_eq!(request.value_ranges[0].values, numeric_data1);
        assert_eq!(request.value_ranges[1].values, numeric_data2);
    }

    #[test]
    fn test_write_data_to_multi_ranges_serialization() {
        let request = WriteDataToMultiRangesRequest::builder()
            .spreadsheet_token("serialization_test")
            .add_value_range(
                "Sheet1!A1:B2",
                json!([["Key1", "Value1"], ["Key2", "Value2"]]),
            )
            .add_value_range("Sheet2!A1:B1", json!([["Header1", "Header2"]]))
            .build();

        let serialized = serde_json::to_string(&request);
        assert!(serialized.is_ok());

        let json_value: serde_json::Value = serde_json::from_str(&serialized.unwrap()).unwrap();
        assert_eq!(json_value["valueRanges"].as_array().unwrap().len(), 2);
        assert_eq!(json_value["valueRanges"][0]["range"], "Sheet1!A1:B2");
        assert_eq!(json_value["valueRanges"][1]["range"], "Sheet2!A1:B1");
        assert_eq!(json_value["valueRanges"][0]["values"][0][0], "Key1");
        assert_eq!(json_value["valueRanges"][1]["values"][0][0], "Header1");
    }

    #[test]
    fn test_write_data_to_multi_ranges_complex_sheet_references() {
        let request = WriteDataToMultiRangesRequest::builder()
            .spreadsheet_token("complex_ref_test")
            .add_value_range("'Sheet With Spaces'!A1:B2", json!([["Col1", "Col2"]]))
            .add_value_range("'Another Sheet'!C3:D4", json!([["Col3", "Col4"]]))
            .build();

        assert_eq!(request.spreadsheet_token, "complex_ref_test");
        assert_eq!(request.value_ranges.len(), 2);
        assert_eq!(request.value_ranges[0].range, "'Sheet With Spaces'!A1:B2");
        assert_eq!(request.value_ranges[1].range, "'Another Sheet'!C3:D4");
    }

    #[test]
    fn test_write_data_to_multi_ranges_hyperlinks() {
        let hyperlinks1 = json!([
            ["Link Name", "URL"],
            ["Google", "https://www.google.com"],
            ["GitHub", "https://github.com"]
        ]);

        let hyperlinks2 = json!([
            ["Documentation Links"],
            ["https://docs.example.com/api"],
            ["https://help.example.com"]
        ]);

        let request = WriteDataToMultiRangesRequest::builder()
            .spreadsheet_token("hyperlinks_multi")
            .add_value_range("Links!A1:B3", hyperlinks1.clone())
            .add_value_range("Docs!A1:A3", hyperlinks2.clone())
            .build();

        assert_eq!(request.spreadsheet_token, "hyperlinks_multi");
        assert_eq!(request.value_ranges.len(), 2);
        assert_eq!(request.value_ranges[0].values, hyperlinks1);
        assert_eq!(request.value_ranges[1].values, hyperlinks2);
    }

    #[test]
    fn test_write_data_to_multi_ranges_dates_and_times() {
        let datetime_data1 = json!([
            ["Date", "Time"],
            ["2024-01-15", "14:30:00"],
            ["2024-02-20", "09:15:30"]
        ]);

        let datetime_data2 = json!([
            ["DateTime", "Timestamp"],
            ["2024-01-15T14:30:00Z", 1705329000],
            ["2024-02-20T09:15:30Z", 1708419330]
        ]);

        let request = WriteDataToMultiRangesRequest::builder()
            .spreadsheet_token("datetime_multi_test")
            .add_value_range("Schedule1!A1:B3", datetime_data1.clone())
            .add_value_range("Schedule2!A1:B3", datetime_data2.clone())
            .build();

        assert_eq!(request.spreadsheet_token, "datetime_multi_test");
        assert_eq!(request.value_ranges.len(), 2);
        assert_eq!(request.value_ranges[0].values, datetime_data1);
        assert_eq!(request.value_ranges[1].values, datetime_data2);
    }

    #[test]
    fn test_write_data_to_multi_ranges_response_deserialization() {
        let response_json = json!({
            "spreadsheetToken": "test_token_123",
            "revision": 456,
            "responses": [
                {
                    "spreadsheetToken": "test_token_123",
                    "updatedRows": 3,
                    "updatedColumns": 2,
                    "updatedCells": 6
                },
                {
                    "spreadsheetToken": "test_token_123",
                    "updatedRows": 2,
                    "updatedColumns": 3,
                    "updatedCells": 6
                }
            ]
        });

        let response: WriteDataToMultiRangesResponse =
            serde_json::from_value(response_json).unwrap();

        assert_eq!(response.spreadsheet_token, "test_token_123");
        assert_eq!(response.revision, 456);
        assert_eq!(response.responses.len(), 2);

        assert_eq!(response.responses[0].spreed_sheet_token, "test_token_123");
        assert_eq!(response.responses[0].updated_rows, 3);
        assert_eq!(response.responses[0].updated_columns, 2);
        assert_eq!(response.responses[0].updated_cells, 6);

        assert_eq!(response.responses[1].updated_rows, 2);
        assert_eq!(response.responses[1].updated_columns, 3);
        assert_eq!(response.responses[1].updated_cells, 6);
    }

    #[test]
    fn test_write_data_to_multi_ranges_service_creation() {
        let service = create_service();
        assert_eq!(service.config.app_id, "test_app_id");
        assert_eq!(service.config.app_secret, "test_app_secret");
        assert!(matches!(service.config.app_type, AppType::SelfBuild));
    }

    #[test]
    fn test_write_data_to_multi_ranges_boolean_values() {
        let boolean_data1 = json!([
            ["Question", "Answer"],
            ["Is active?", true],
            ["Is deleted?", false]
        ]);

        let boolean_data2 = json!([["Permissions"], [true], [false], [true]]);

        let request = WriteDataToMultiRangesRequest::builder()
            .spreadsheet_token("boolean_multi_test")
            .add_value_range("Flags1!A1:B3", boolean_data1.clone())
            .add_value_range("Flags2!A1:A4", boolean_data2.clone())
            .build();

        assert_eq!(request.spreadsheet_token, "boolean_multi_test");
        assert_eq!(request.value_ranges.len(), 2);
        assert_eq!(request.value_ranges[0].values, boolean_data1);
        assert_eq!(request.value_ranges[1].values, boolean_data2);
    }

    #[test]
    fn test_write_data_to_multi_ranges_null_values() {
        let null_data1 = json!([["Name", "Optional"], ["Alice", null], ["Bob", "Value"]]);

        let null_data2 = json!([
            ["Required", "Also Optional"],
            ["Required1", null],
            [null, null]
        ]);

        let request = WriteDataToMultiRangesRequest::builder()
            .spreadsheet_token("null_multi_test")
            .add_value_range("Data1!A1:B3", null_data1.clone())
            .add_value_range("Data2!A1:B3", null_data2.clone())
            .build();

        assert_eq!(request.spreadsheet_token, "null_multi_test");
        assert_eq!(request.value_ranges.len(), 2);
        assert_eq!(request.value_ranges[0].values, null_data1);
        assert_eq!(request.value_ranges[1].values, null_data2);
    }

    #[test]
    fn test_write_data_to_multi_ranges_very_long_token() {
        let very_long_token = "a".repeat(1000);
        let request = WriteDataToMultiRangesRequest::builder()
            .spreadsheet_token(&very_long_token)
            .add_value_range("Sheet1!A1", json!("test1"))
            .add_value_range("Sheet1!B1", json!("test2"))
            .build();

        assert_eq!(request.spreadsheet_token, very_long_token);
        assert_eq!(request.value_ranges.len(), 2);
        assert_eq!(request.value_ranges[0].values, json!("test1"));
        assert_eq!(request.value_ranges[1].values, json!("test2"));
    }

    #[test]
    fn test_data_response_struct() {
        let data_response = DataResponse {
            spreed_sheet_token: "test_token".to_string(),
            updated_rows: 5,
            updated_columns: 3,
            updated_cells: 15,
        };

        assert_eq!(data_response.spreed_sheet_token, "test_token");
        assert_eq!(data_response.updated_rows, 5);
        assert_eq!(data_response.updated_columns, 3);
        assert_eq!(data_response.updated_cells, 15);
    }
}

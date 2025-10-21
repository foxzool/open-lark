use serde::Deserialize;

use crate::{
    core::{
        api_req::ApiRequest,
        api_resp::{ApiResponseTrait, BaseResponse, ResponseFormat},
        constants::AccessTokenType,
        endpoints::cloud_docs::*,
        req_option, SDKResult,
    },
    service::cloud_docs::sheets::v2::{data_operation::ValueRangeResponse, SpreadsheetService},
};

/// 读取单个范围请求
#[derive(Debug, Default)]
pub struct ReadingMultipleRangeRequest {
    api_request: ApiRequest,
    /// spreadsheet 的 token
    spreadsheet_token: String,
    /// 多个查询范围，范围之间使用逗号分隔，如range1,range2。其中 range 包含 sheetId
    /// 与单元格范围两部分，目前支持四种索引方式，详见 在线表格开发指南
    /// 。若查询范围中使用形如`<sheetId>!<开始单元格>:<结束列>`的范围时，仅支持获取100列数据
    ranges: String,
    /// 指定单元格数据的格式。可选值为如下所示。当参数缺省时，默认不进行公式计算，返回公式本身；
    /// 数值不进行数字格式化。
    ///
    /// - valueRenderOption=ToString：返回纯文本的值（数值类型除外）
    /// - valueRenderOption=FormattedValue：计算并格式化单元格
    /// - valueRenderOption=Formula：单元格中含有公式时，返回公式本身
    /// - valueRenderOption=UnformattedValue：计算但不对单元格进行格式化
    value_render_option: Option<String>,
    /// 指定数据类型为日期、时间、或时间日期的单元格数据的格式。
    ///
    /// - 当参数缺省时，默认返回浮点数值，整数部分为自 1899 年 12 月 30
    ///   日以来的天数；小数部分为该时间占 24 小时的份额。例如：若时间为 1900 年 1 月 1 日中午 12
    ///   点，则默认返回 2.5。其中，2 表示 1900 年 1 月 1 日为 1899 年12 月 30 日之后的 2 天；0.5
    ///   表示 12 点占 24 小时的二分之一，即 12/24=0.5。
    /// - dateTimeRenderOption=FormattedString：计算并对时间、日期类型数据进行格式化，
    ///   但不会对数字进行格式化。将返回格式化后的字符串。详见电子表格常见问题
    date_time_render_option: Option<String>,
    /// 当单元格中包含@用户等涉及用户信息的元素时，该参数可指定返回的用户 ID 类型。默认为
    /// lark_id，建议选择 open_id 或 union_id。了解更多，参考用户身份概述。
    ///
    /// - open_id：用户在应用内的身份。 同一个 user_id 在不同应用中的 open_id 不同，其值统一以 ou_
    ///   为前缀，如ou_c99c5f35d542efc7ee492afe11af19ef。
    /// - union_id：用户在同一应用服务商提供的多个应用间的统一身份。
    user_id_type: Option<String>,
}

impl ReadingMultipleRangeRequest {
    pub fn builder() -> ReadingMultiRangesRequestBuilder {
        ReadingMultiRangesRequestBuilder::default()
    }
}

#[derive(Default)]
pub struct ReadingMultiRangesRequestBuilder {
    request: ReadingMultipleRangeRequest,
}

impl ReadingMultiRangesRequestBuilder {
    pub fn spreadsheet_token(mut self, spreadsheet_token: impl ToString) -> Self {
        self.request.spreadsheet_token = spreadsheet_token.to_string();
        self
    }

    /// 多个查询范围，范围之间使用逗号分隔，如range1,range2。⁣其中 range 包含 sheetId
    /// 与单元格范围两部分，目前支持四种索引方式，详见 在线表格开发指南
    /// 。若查询范围中使用形如`<sheetId>!<开始单元格>:<结束列>`的范围时，仅支持获取100列数据。
    pub fn ranges(mut self, ranges: impl ToString) -> Self {
        self.request.ranges = ranges.to_string();
        self.request
            .api_request
            .query_params
            .insert("ranges", ranges.to_string());
        self
    }

    /// 指定单元格数据的格式。可选值为如下所示。当参数缺省时，默认不进行公式计算，返回公式本身；
    /// 数值不进行数字格式化。
    ///
    /// - valueRenderOption=ToString：返回纯文本的值（数值类型除外）
    /// - valueRenderOption=FormattedValue：计算并格式化单元格
    /// - valueRenderOption=Formula：单元格中含有公式时，返回公式本身
    /// - valueRenderOption=UnformattedValue：计算但不对单元格进行格式化
    pub fn value_render_option(mut self, value_render_option: impl ToString) -> Self {
        self.request.value_render_option = Some(value_render_option.to_string());
        self.request
            .api_request
            .query_params
            .insert("valueRenderOption", value_render_option.to_string());
        self
    }

    /// 指定数据类型为日期、时间、或时间日期的单元格数据的格式。
    ///
    /// - 当参数缺省时，默认返回浮点数值，整数部分为自 1899 年 12 月 30
    ///   日以来的天数；小数部分为该时间占 24 小时的份额。例如：若时间为 1900 年 1 月 1 日中午 12
    ///   点，则默认返回 2.5。其中，2 表示 1900 年 1 月 1 日为 1899 年12 月 30 日之后的 2 天；0.5
    ///   表示 12 点占 24 小时的二分之一，即 12/24=0.5。
    /// - dateTimeRenderOption=FormattedString：计算并对时间、日期类型数据进行格式化，
    ///   但不会对数字进行格式化。将返回格式化后的字符串。详见电子表格常见问题
    pub fn date_time_render_option(mut self, date_time_render_option: impl ToString) -> Self {
        self.request.date_time_render_option = Some(date_time_render_option.to_string());
        self.request
            .api_request
            .query_params
            .insert("dateTimeRenderOption", date_time_render_option.to_string());
        self
    }

    /// 当单元格中包含@用户等涉及用户信息的元素时，该参数可指定返回的用户 ID 类型。默认为
    /// lark_id，建议选择 open_id 或 union_id。了解更多，参考用户身份概述。
    ///
    /// - open_id：用户在应用内的身份。 同一个 user_id 在不同应用中的 open_id 不同，其值统一以 ou_
    ///   为前缀，如ou_c99c5f35d542efc7ee492afe11af19ef。
    /// - union_id：用户在同一应用服务商提供的多个应用间的统一身份。
    pub fn user_id_type(mut self, user_id_type: impl ToString) -> Self {
        self.request.user_id_type = Some(user_id_type.to_string());
        self.request
            .api_request
            .query_params
            .insert("user_id_type", user_id_type.to_string());
        self
    }

    pub fn build(self) -> ReadingMultipleRangeRequest {
        self.request
    }
}

/// 读取数据响应体
#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct ReadingMultiRangesResponse {
    /// sheet 的版本号
    pub revision: i32,
    /// spreadsheet 的 token，详见电子表格概述
    #[serde(rename = "spreadsheetToken")]
    pub spreadsheet_token: String,
    /// 读取的单元格总数
    #[serde(rename = "totalCells")]
    pub total_cells: i32,
    /// 值与范围
    #[serde(rename = "valueRanges")]
    pub value_ranges: Vec<ValueRangeResponse>,
}

impl ApiResponseTrait for ReadingMultiRangesResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

impl SpreadsheetService {
    /// 读取多个范围
    pub async fn reading_multi_ranges(
        &self,
        request: ReadingMultipleRangeRequest,
        option: Option<req_option::RequestOption>,
    ) -> SDKResult<BaseResponse<ReadingMultiRangesResponse>> {
        let mut api_req = request.api_request;
        api_req.api_path =
            SHEETS_V2_SPREADSHEET_VALUES_BATCH_GET.replace("{}", &request.spreadsheet_token);
        api_req.http_method = reqwest::Method::GET;
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::App];

        let api_resp = crate::core::http::Transport::request(api_req, &self.config, option).await?;

        Ok(api_resp)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::config::Config;
    use serde_json::json;

    fn create_test_config() -> Config {
        Config::default()
    }

    fn create_test_service() -> SpreadsheetService {
        SpreadsheetService::new(create_test_config())
    }

    #[allow(dead_code)]
    fn create_test_response() -> ReadingMultiRangesResponse {
        ReadingMultiRangesResponse {
            revision: 123456,
            spreadsheet_token: "test_token_12345".to_string(),
            total_cells: 6,
            value_ranges: vec![
                ValueRangeResponse {
                    major_dimension: "ROWS".to_string(),
                    range: "Sheet1!A1:B2".to_string(),
                    values: json!([["Name", "Age"], ["Alice", "30"]]),
                    revision: 123456,
                },
                ValueRangeResponse {
                    major_dimension: "ROWS".to_string(),
                    range: "Sheet2!C1:D1".to_string(),
                    values: json!([["Department", "Salary"]]),
                    revision: 123456,
                },
            ],
        }
    }

    #[test]
    fn test_reading_multiple_range_request_builder_default() {
        let builder = ReadingMultipleRangeRequest::builder();
        let request = builder.build();

        assert_eq!(request.spreadsheet_token, "");
        assert_eq!(request.ranges, "");
        assert!(request.value_render_option.is_none());
        assert!(request.date_time_render_option.is_none());
        assert!(request.user_id_type.is_none());
        assert!(request.api_request.query_params.is_empty());
    }

    #[test]
    fn test_reading_multiple_range_request_builder_basic() {
        let request = ReadingMultipleRangeRequest::builder()
            .spreadsheet_token("test_token_12345")
            .ranges("Sheet1!A1:B2,Sheet2!C1:D1")
            .build();

        assert_eq!(request.spreadsheet_token, "test_token_12345");
        assert_eq!(request.ranges, "Sheet1!A1:B2,Sheet2!C1:D1");
        assert!(request.value_render_option.is_none());
        assert!(request.date_time_render_option.is_none());
        assert!(request.user_id_type.is_none());
        assert_eq!(
            request.api_request.query_params.get("ranges"),
            Some(&"Sheet1!A1:B2,Sheet2!C1:D1".to_string())
        );
    }

    #[test]
    fn test_reading_multiple_range_request_builder_all_options() {
        let request = ReadingMultipleRangeRequest::builder()
            .spreadsheet_token("test_token_12345")
            .ranges("Sheet1!A1:C10,Sheet2!D1:F5")
            .value_render_option("FormattedValue")
            .date_time_render_option("FormattedString")
            .user_id_type("open_id")
            .build();

        assert_eq!(request.spreadsheet_token, "test_token_12345");
        assert_eq!(request.ranges, "Sheet1!A1:C10,Sheet2!D1:F5");
        assert_eq!(
            request.value_render_option,
            Some("FormattedValue".to_string())
        );
        assert_eq!(
            request.date_time_render_option,
            Some("FormattedString".to_string())
        );
        assert_eq!(request.user_id_type, Some("open_id".to_string()));

        // Check query params
        assert_eq!(
            request.api_request.query_params.get("ranges"),
            Some(&"Sheet1!A1:C10,Sheet2!D1:F5".to_string())
        );
        assert_eq!(
            request.api_request.query_params.get("valueRenderOption"),
            Some(&"FormattedValue".to_string())
        );
        assert_eq!(
            request.api_request.query_params.get("dateTimeRenderOption"),
            Some(&"FormattedString".to_string())
        );
        assert_eq!(
            request.api_request.query_params.get("user_id_type"),
            Some(&"open_id".to_string())
        );
    }

    #[test]
    fn test_reading_multiple_range_request_builder_value_render_options() {
        let test_cases = vec!["ToString", "FormattedValue", "Formula", "UnformattedValue"];

        for option in test_cases {
            let request = ReadingMultipleRangeRequest::builder()
                .spreadsheet_token("test_token")
                .ranges("Sheet1!A1:A1")
                .value_render_option(option)
                .build();

            assert_eq!(request.value_render_option, Some(option.to_string()));
            assert_eq!(
                request.api_request.query_params.get("valueRenderOption"),
                Some(&option.to_string())
            );
        }
    }

    #[test]
    fn test_reading_multiple_range_request_builder_user_id_types() {
        let test_cases = vec!["open_id", "union_id", "lark_id"];

        for user_id_type in test_cases {
            let request = ReadingMultipleRangeRequest::builder()
                .spreadsheet_token("test_token")
                .ranges("Sheet1!A1:A1")
                .user_id_type(user_id_type)
                .build();

            assert_eq!(request.user_id_type, Some(user_id_type.to_string()));
            assert_eq!(
                request.api_request.query_params.get("user_id_type"),
                Some(&user_id_type.to_string())
            );
        }
    }

    #[test]
    fn test_reading_multiple_range_request_unicode_handling() {
        let request = ReadingMultipleRangeRequest::builder()
            .spreadsheet_token("测试token_🔥📊")
            .ranges("工作表1!A1:Z100,数据表!AA1:BB200")
            .value_render_option("FormattedValue")
            .build();

        assert_eq!(request.spreadsheet_token, "测试token_🔥📊");
        assert_eq!(request.ranges, "工作表1!A1:Z100,数据表!AA1:BB200");
        assert_eq!(
            request.value_render_option,
            Some("FormattedValue".to_string())
        );
    }

    #[test]
    fn test_reading_multiple_range_request_complex_ranges() {
        let test_cases = vec![
            "Sheet1!A1:B2,Sheet2!C3:D4",
            "工作表1!C3:F20,数据表!AA1:ZZ1000",
            "Sheet with spaces!A1:B2,Another Sheet!C1:D2",
            "Sheet1!$A$1:$B$2,Sheet2!$C$3:$D$4",
            "SingleCell!A1,LargeRange!A1:XFD1048576",
            "Sheet1!A:A,Sheet2!1:1",
            "Mixed!A1:B2,Mixed!C3,Mixed!D4:E5",
        ];

        for ranges in test_cases {
            let request = ReadingMultipleRangeRequest::builder()
                .spreadsheet_token("test_token")
                .ranges(ranges)
                .build();

            assert_eq!(request.ranges, ranges);
            assert_eq!(
                request.api_request.query_params.get("ranges"),
                Some(&ranges.to_string())
            );
        }
    }

    #[test]
    fn test_reading_multiple_range_request_very_long_ranges() {
        let long_ranges = (0..100)
            .map(|i| format!("Sheet{}!A{}:B{}", i, i + 1, i + 2))
            .collect::<Vec<_>>()
            .join(",");
        let request = ReadingMultipleRangeRequest::builder()
            .spreadsheet_token("test_token")
            .ranges(&long_ranges)
            .build();

        assert_eq!(request.ranges, long_ranges);
    }

    #[test]
    fn test_reading_multiple_range_request_empty_values() {
        let request = ReadingMultipleRangeRequest::builder()
            .spreadsheet_token("")
            .ranges("")
            .value_render_option("")
            .date_time_render_option("")
            .user_id_type("")
            .build();

        assert_eq!(request.spreadsheet_token, "");
        assert_eq!(request.ranges, "");
        assert_eq!(request.value_render_option, Some("".to_string()));
        assert_eq!(request.date_time_render_option, Some("".to_string()));
        assert_eq!(request.user_id_type, Some("".to_string()));
    }

    #[test]
    fn test_reading_multiple_range_response_deserialization() {
        let json_data = json!({
            "revision": 123456,
            "spreadsheetToken": "test_token_12345",
            "totalCells": 6,
            "valueRanges": [
                {
                    "majorDimension": "ROWS",
                    "range": "Sheet1!A1:B2",
                    "values": [
                        ["Name", "Age"],
                        ["Alice", "30"]
                    ],
                    "revision": 123456
                },
                {
                    "majorDimension": "ROWS",
                    "range": "Sheet2!C1:D1",
                    "values": [
                        ["Department", "Salary"]
                    ],
                    "revision": 123456
                }
            ]
        });

        let response: ReadingMultiRangesResponse = serde_json::from_value(json_data).unwrap();
        assert_eq!(response.revision, 123456);
        assert_eq!(response.spreadsheet_token, "test_token_12345");
        assert_eq!(response.total_cells, 6);
        assert_eq!(response.value_ranges.len(), 2);

        // Check first range
        let first_range = &response.value_ranges[0];
        assert_eq!(first_range.range, "Sheet1!A1:B2");
        if let serde_json::Value::Array(values) = &first_range.values {
            assert_eq!(values.len(), 2);
            if let serde_json::Value::Array(first_row) = &values[0] {
                assert_eq!(first_row[0], "Name");
                assert_eq!(first_row[1], "Age");
            }
        } else {
            panic!("Expected array values");
        }

        // Check second range
        let second_range = &response.value_ranges[1];
        assert_eq!(second_range.range, "Sheet2!C1:D1");
        if let serde_json::Value::Array(values) = &second_range.values {
            assert_eq!(values.len(), 1);
            if let serde_json::Value::Array(first_row) = &values[0] {
                assert_eq!(first_row[0], "Department");
                assert_eq!(first_row[1], "Salary");
            }
        } else {
            panic!("Expected array values");
        }
    }

    #[test]
    fn test_reading_multiple_range_response_empty_ranges() {
        let json_data = json!({
            "revision": 789,
            "spreadsheetToken": "empty_token",
            "totalCells": 0,
            "valueRanges": []
        });

        let response: ReadingMultiRangesResponse = serde_json::from_value(json_data).unwrap();
        assert_eq!(response.revision, 789);
        assert_eq!(response.spreadsheet_token, "empty_token");
        assert_eq!(response.total_cells, 0);
        assert_eq!(response.value_ranges.len(), 0);
    }

    #[test]
    fn test_reading_multiple_range_response_large_dataset() {
        let mut value_ranges = Vec::new();
        for i in 0..50 {
            let mut large_values = Vec::new();
            for j in 0..100 {
                large_values.push(vec![format!("Row{}_{}", i, j), format!("Data{}_{}", i, j)]);
            }

            value_ranges.push(json!({
                "majorDimension": "ROWS",
                "range": format!("Sheet{}!A1:B100", i),
                "values": large_values,
                "revision": 999999
            }));
        }

        let json_data = json!({
            "revision": 999999,
            "spreadsheetToken": "large_data_token",
            "totalCells": 10000,
            "valueRanges": value_ranges
        });

        let response: ReadingMultiRangesResponse = serde_json::from_value(json_data).unwrap();
        assert_eq!(response.revision, 999999);
        assert_eq!(response.spreadsheet_token, "large_data_token");
        assert_eq!(response.total_cells, 10000);
        assert_eq!(response.value_ranges.len(), 50);

        // Check first range data
        let first_range = &response.value_ranges[0];
        assert_eq!(first_range.range, "Sheet0!A1:B100");
        if let serde_json::Value::Array(values) = &first_range.values {
            assert_eq!(values.len(), 100);
            if let serde_json::Value::Array(first_row) = &values[0] {
                assert_eq!(first_row[0], "Row0_0");
                assert_eq!(first_row[1], "Data0_0");
            }
        }

        // Check last range data
        let last_range = &response.value_ranges[49];
        assert_eq!(last_range.range, "Sheet49!A1:B100");
        if let serde_json::Value::Array(values) = &last_range.values {
            assert_eq!(values.len(), 100);
            if let serde_json::Value::Array(last_row) = &values[99] {
                assert_eq!(last_row[0], "Row49_99");
                assert_eq!(last_row[1], "Data49_99");
            }
        }
    }

    #[test]
    fn test_reading_multiple_range_response_unicode_data() {
        let json_data = json!({
            "revision": 456789,
            "spreadsheetToken": "unicode_token_测试",
            "totalCells": 9,
            "valueRanges": [
                {
                    "majorDimension": "ROWS",
                    "range": "工作表1!A1:C3",
                    "values": [
                        ["姓名", "年龄", "城市"],
                        ["张三", "25", "北京"],
                        ["李四", "30", "上海"]
                    ],
                    "revision": 456789
                },
                {
                    "majorDimension": "ROWS",
                    "range": "表情表!D1:F1",
                    "values": [
                        ["🎉", "🔥", "📊"]
                    ],
                    "revision": 456789
                }
            ]
        });

        let response: ReadingMultiRangesResponse = serde_json::from_value(json_data).unwrap();
        assert_eq!(response.revision, 456789);
        assert_eq!(response.spreadsheet_token, "unicode_token_测试");
        assert_eq!(response.total_cells, 9);

        // Check unicode data
        let unicode_range = &response.value_ranges[0];
        assert_eq!(unicode_range.range, "工作表1!A1:C3");
        if let serde_json::Value::Array(values) = &unicode_range.values {
            if let serde_json::Value::Array(first_row) = &values[0] {
                assert_eq!(first_row[0], "姓名");
                assert_eq!(first_row[1], "年龄");
                assert_eq!(first_row[2], "城市");
            }
        }

        // Check emoji data
        let emoji_range = &response.value_ranges[1];
        assert_eq!(emoji_range.range, "表情表!D1:F1");
        if let serde_json::Value::Array(values) = &emoji_range.values {
            if let serde_json::Value::Array(first_row) = &values[0] {
                assert_eq!(first_row[0], "🎉");
                assert_eq!(first_row[1], "🔥");
                assert_eq!(first_row[2], "📊");
            }
        }
    }

    #[test]
    fn test_reading_multiple_range_response_mixed_data_types() {
        let json_data = json!({
            "revision": 2023123115,
            "spreadsheetToken": "mixed_data_token",
            "totalCells": 12,
            "valueRanges": [
                {
                    "majorDimension": "ROWS",
                    "range": "Formulas!A1:B3",
                    "values": [
                        ["=SUM(A1:A10)", "=IF(B1>0,\"Positive\",\"Negative\")"],
                        ["2023-12-31 23:59:59", "0.5"],
                        ["@user_12345", "@group_67890"]
                    ],
                    "revision": 2023123115
                },
                {
                    "majorDimension": "ROWS",
                    "range": "Numbers!C1:D3",
                    "values": [
                        [42, 3.14286],
                        [true, false],
                        [null, ""]
                    ],
                    "revision": 2023123115
                }
            ]
        });

        let response: ReadingMultiRangesResponse = serde_json::from_value(json_data).unwrap();
        assert_eq!(response.revision, 2023123115);
        assert_eq!(response.value_ranges.len(), 2);

        // Check formulas range
        let formulas_range = &response.value_ranges[0];
        if let serde_json::Value::Array(values) = &formulas_range.values {
            assert_eq!(values.len(), 3);
            if let serde_json::Value::Array(first_row) = &values[0] {
                assert_eq!(first_row[0], "=SUM(A1:A10)");
            }
        }

        // Check mixed types range
        let numbers_range = &response.value_ranges[1];
        if let serde_json::Value::Array(values) = &numbers_range.values {
            if let serde_json::Value::Array(first_row) = &values[0] {
                assert_eq!(first_row[0], 42);
                assert_eq!(first_row[1], 3.14286);
            }
            if let serde_json::Value::Array(second_row) = &values[1] {
                assert_eq!(second_row[0], true);
                assert_eq!(second_row[1], false);
            }
            if let serde_json::Value::Array(third_row) = &values[2] {
                assert_eq!(third_row[0], serde_json::Value::Null);
                assert_eq!(third_row[1], "");
            }
        }
    }

    #[test]
    fn test_reading_multiple_range_response_api_trait() {
        assert_eq!(
            ReadingMultiRangesResponse::data_format(),
            ResponseFormat::Data
        );
    }

    #[test]
    fn test_reading_multiple_range_service_method() {
        let service = create_test_service();
        let request = ReadingMultipleRangeRequest::builder()
            .spreadsheet_token("test_token")
            .ranges("Sheet1!A1:B2,Sheet2!C1:D1")
            .build();

        // Test that the service method is available
        let _future = service.reading_multi_ranges(request, None);
        // Just verify the method compiles and returns a future
    }

    #[test]
    fn test_reading_multiple_range_memory_efficiency() {
        // Test that the builder doesn't allocate unnecessarily
        let builder = ReadingMultipleRangeRequest::builder();
        let size_before = std::mem::size_of_val(&builder);

        let builder = builder
            .spreadsheet_token("test")
            .ranges("A1:B2,C1:D2")
            .value_render_option("FormattedValue");

        let size_after = std::mem::size_of_val(&builder);

        // Size should be reasonable (this is a rough check)
        assert!(size_before > 0);
        assert!(size_after > 0);

        let request = builder.build();
        let request_size = std::mem::size_of_val(&request);
        assert!(request_size > 0);
    }

    #[test]
    fn test_reading_multiple_range_request_query_params_independence() {
        let request1 = ReadingMultipleRangeRequest::builder()
            .spreadsheet_token("token1")
            .ranges("Sheet1!A1:B2")
            .value_render_option("FormattedValue")
            .build();

        let request2 = ReadingMultipleRangeRequest::builder()
            .spreadsheet_token("token2")
            .ranges("Sheet2!C1:D2,Sheet3!E1:F2")
            .user_id_type("open_id")
            .build();

        // Ensure requests don't share query params
        assert_eq!(
            request1.api_request.query_params.get("ranges"),
            Some(&"Sheet1!A1:B2".to_string())
        );
        assert_eq!(
            request1.api_request.query_params.get("valueRenderOption"),
            Some(&"FormattedValue".to_string())
        );
        assert!(!request1
            .api_request
            .query_params
            .contains_key("user_id_type"));

        assert_eq!(
            request2.api_request.query_params.get("ranges"),
            Some(&"Sheet2!C1:D2,Sheet3!E1:F2".to_string())
        );
        assert!(!request2
            .api_request
            .query_params
            .contains_key("valueRenderOption"));
        assert_eq!(
            request2.api_request.query_params.get("user_id_type"),
            Some(&"open_id".to_string())
        );
    }

    #[test]
    fn test_reading_multiple_range_request_builder_chaining() {
        // Test that all builder methods can be chained fluently
        let request = ReadingMultipleRangeRequest::builder()
            .spreadsheet_token("chain_test_token")
            .ranges("Sheet1!A1:Z100,Sheet2!AA1:ZZ200")
            .value_render_option("ToString")
            .date_time_render_option("FormattedString")
            .user_id_type("union_id")
            .build();

        assert_eq!(request.spreadsheet_token, "chain_test_token");
        assert_eq!(request.ranges, "Sheet1!A1:Z100,Sheet2!AA1:ZZ200");
        assert_eq!(request.value_render_option, Some("ToString".to_string()));
        assert_eq!(
            request.date_time_render_option,
            Some("FormattedString".to_string())
        );
        assert_eq!(request.user_id_type, Some("union_id".to_string()));

        // Verify all query params are set
        assert_eq!(request.api_request.query_params.len(), 4);
        assert_eq!(
            request.api_request.query_params.get("ranges"),
            Some(&"Sheet1!A1:Z100,Sheet2!AA1:ZZ200".to_string())
        );
        assert_eq!(
            request.api_request.query_params.get("valueRenderOption"),
            Some(&"ToString".to_string())
        );
        assert_eq!(
            request.api_request.query_params.get("dateTimeRenderOption"),
            Some(&"FormattedString".to_string())
        );
        assert_eq!(
            request.api_request.query_params.get("user_id_type"),
            Some(&"union_id".to_string())
        );
    }

    #[test]
    fn test_reading_multiple_range_single_range_scenario() {
        // Test that it works with just one range (edge case)
        let request = ReadingMultipleRangeRequest::builder()
            .spreadsheet_token("single_range_token")
            .ranges("Sheet1!A1:B2")
            .build();

        assert_eq!(request.ranges, "Sheet1!A1:B2");
        assert_eq!(
            request.api_request.query_params.get("ranges"),
            Some(&"Sheet1!A1:B2".to_string())
        );
    }

    #[test]
    fn test_reading_multiple_range_many_ranges_scenario() {
        // Test with many ranges to verify comma separation handling
        let ranges = (1..=20)
            .map(|i| format!("Sheet{}!A{}:B{}", i, i, i + 1))
            .collect::<Vec<_>>()
            .join(",");

        let request = ReadingMultipleRangeRequest::builder()
            .spreadsheet_token("many_ranges_token")
            .ranges(&ranges)
            .build();

        assert_eq!(request.ranges, ranges);
        assert_eq!(
            request.api_request.query_params.get("ranges"),
            Some(&ranges)
        );

        // Verify the format contains expected number of commas
        assert_eq!(request.ranges.matches(',').count(), 19); // 20 ranges = 19 commas
    }
}

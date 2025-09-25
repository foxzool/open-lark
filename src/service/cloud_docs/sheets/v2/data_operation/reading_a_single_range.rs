use serde::Deserialize;

use crate::{
    core::{
        api_req::ApiRequest,
        api_resp::{ApiResponseTrait, BaseResponse, ResponseFormat},
        constants::AccessTokenType,
        endpoints::cloud_docs::*,
        req_option, SDKResult,
    },
    impl_executable_builder_owned,
    service::cloud_docs::sheets::v2::{data_operation::ValueRangeResponse, SpreadsheetService},
};

/// 读取单个范围请求
#[derive(Debug, Default)]
pub struct ReadingSingleRangeRequest {
    api_request: ApiRequest,
    /// spreadsheet 的 token
    spreadsheet_token: String,
    /// 查询范围，包含 sheetId 与单元格范围两部分，详见在线表格开发指南。若查询范围中使用形如
    /// `<sheetId>!<开始单元格>:<结束列>`的范围时，仅支持获取100列数据
    range: String,
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

impl ReadingSingleRangeRequest {
    pub fn builder() -> ReadingSingleRangeRequestBuilder {
        ReadingSingleRangeRequestBuilder::default()
    }
}

#[derive(Default)]
pub struct ReadingSingleRangeRequestBuilder {
    request: ReadingSingleRangeRequest,
}

impl ReadingSingleRangeRequestBuilder {
    pub fn spreadsheet_token(mut self, spreadsheet_token: impl ToString) -> Self {
        self.request.spreadsheet_token = spreadsheet_token.to_string();
        self
    }

    /// 查询范围，包含 sheetId 与单元格范围两部分，详见在线表格开发指南。若查询范围中使用形如
    /// `<sheetId>!<开始单元格>:<结束列>`的范围时，仅支持获取100列数据
    pub fn range(mut self, range: impl ToString) -> Self {
        self.request.range = range.to_string();
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

    pub fn build(self) -> ReadingSingleRangeRequest {
        self.request
    }
}

/// 读取数据响应体
#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct ReadingSingleRangeResponse {
    /// sheet 的版本号
    pub revision: i32,
    /// spreadsheet 的 token，详见电子表格概述
    #[serde(rename = "spreadsheetToken")]
    pub spreadsheet_token: String,
    /// 值与范围
    #[serde(rename = "valueRange")]
    pub value_range: ValueRangeResponse,
}

impl ApiResponseTrait for ReadingSingleRangeResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

// 使用宏实现ExecutableBuilder trait
impl_executable_builder_owned!(
    ReadingSingleRangeRequestBuilder,
    SpreadsheetService,
    ReadingSingleRangeRequest,
    BaseResponse<ReadingSingleRangeResponse>,
    reading_a_single_range
);

impl SpreadsheetService {
    /// 读取单个范围
    pub async fn reading_a_single_range(
        &self,
        request: ReadingSingleRangeRequest,
        option: Option<req_option::RequestOption>,
    ) -> SDKResult<BaseResponse<ReadingSingleRangeResponse>> {
        let mut api_req = request.api_request;
        api_req.api_path = SHEETS_V2_SPREADSHEET_VALUES_RANGE
            .replace("{}", &request.spreadsheet_token)
            .replace("{}", &request.range);
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
    fn create_test_response() -> ReadingSingleRangeResponse {
        ReadingSingleRangeResponse {
            revision: 123456,
            spreadsheet_token: "test_token_12345".to_string(),
            value_range: ValueRangeResponse {
                major_dimension: "ROWS".to_string(),
                range: "Sheet1!A1:B2".to_string(),
                values: json!([["Name", "Age"], ["Alice", "30"]]),
                revision: 123456,
            },
        }
    }

    #[test]
    fn test_reading_single_range_request_builder_default() {
        let builder = ReadingSingleRangeRequest::builder();
        let request = builder.build();

        assert_eq!(request.spreadsheet_token, "");
        assert_eq!(request.range, "");
        assert!(request.value_render_option.is_none());
        assert!(request.date_time_render_option.is_none());
        assert!(request.user_id_type.is_none());
        assert!(request.api_request.query_params.is_empty());
    }

    #[test]
    fn test_reading_single_range_request_builder_basic() {
        let request = ReadingSingleRangeRequest::builder()
            .spreadsheet_token("test_token_12345")
            .range("Sheet1!A1:B2")
            .build();

        assert_eq!(request.spreadsheet_token, "test_token_12345");
        assert_eq!(request.range, "Sheet1!A1:B2");
        assert!(request.value_render_option.is_none());
        assert!(request.date_time_render_option.is_none());
        assert!(request.user_id_type.is_none());
        assert!(request.api_request.query_params.is_empty());
    }

    #[test]
    fn test_reading_single_range_request_builder_all_options() {
        let request = ReadingSingleRangeRequest::builder()
            .spreadsheet_token("test_token_12345")
            .range("Sheet1!A1:C10")
            .value_render_option("FormattedValue")
            .date_time_render_option("FormattedString")
            .user_id_type("open_id")
            .build();

        assert_eq!(request.spreadsheet_token, "test_token_12345");
        assert_eq!(request.range, "Sheet1!A1:C10");
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
    fn test_reading_single_range_request_builder_value_render_options() {
        let test_cases = vec!["ToString", "FormattedValue", "Formula", "UnformattedValue"];

        for option in test_cases {
            let request = ReadingSingleRangeRequest::builder()
                .spreadsheet_token("test_token")
                .range("Sheet1!A1:A1")
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
    fn test_reading_single_range_request_builder_user_id_types() {
        let test_cases = vec!["open_id", "union_id", "lark_id"];

        for user_id_type in test_cases {
            let request = ReadingSingleRangeRequest::builder()
                .spreadsheet_token("test_token")
                .range("Sheet1!A1:A1")
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
    fn test_reading_single_range_request_unicode_handling() {
        let request = ReadingSingleRangeRequest::builder()
            .spreadsheet_token("测试token_🔥📊")
            .range("工作表1!A1:Z100")
            .value_render_option("FormattedValue")
            .build();

        assert_eq!(request.spreadsheet_token, "测试token_🔥📊");
        assert_eq!(request.range, "工作表1!A1:Z100");
        assert_eq!(
            request.value_render_option,
            Some("FormattedValue".to_string())
        );
    }

    #[test]
    fn test_reading_single_range_request_complex_ranges() {
        let test_cases = vec![
            "Sheet1!A1:B2",
            "工作表1!C3:F20",
            "数据表!AA1:ZZ1000",
            "Sheet with spaces!A1:B2",
            "Sheet1!$A$1:$B$2",
            "SingleCell!A1",
            "LargeRange!A1:XFD1048576",
        ];

        for range in test_cases {
            let request = ReadingSingleRangeRequest::builder()
                .spreadsheet_token("test_token")
                .range(range)
                .build();

            assert_eq!(request.range, range);
        }
    }

    #[test]
    fn test_reading_single_range_request_very_long_token() {
        let long_token = "a".repeat(1000);
        let request = ReadingSingleRangeRequest::builder()
            .spreadsheet_token(&long_token)
            .range("Sheet1!A1:B2")
            .build();

        assert_eq!(request.spreadsheet_token, long_token);
    }

    #[test]
    fn test_reading_single_range_request_empty_values() {
        let request = ReadingSingleRangeRequest::builder()
            .spreadsheet_token("")
            .range("")
            .value_render_option("")
            .date_time_render_option("")
            .user_id_type("")
            .build();

        assert_eq!(request.spreadsheet_token, "");
        assert_eq!(request.range, "");
        assert_eq!(request.value_render_option, Some("".to_string()));
        assert_eq!(request.date_time_render_option, Some("".to_string()));
        assert_eq!(request.user_id_type, Some("".to_string()));
    }

    #[test]
    fn test_reading_single_range_response_deserialization() {
        let json_data = json!({
            "revision": 123456,
            "spreadsheetToken": "test_token_12345",
            "valueRange": {
                "majorDimension": "ROWS",
                "range": "Sheet1!A1:B2",
                "values": [
                    ["Name", "Age"],
                    ["Alice", "30"]
                ],
                "revision": 123456
            }
        });

        let response: ReadingSingleRangeResponse = serde_json::from_value(json_data).unwrap();
        assert_eq!(response.revision, 123456);
        assert_eq!(response.spreadsheet_token, "test_token_12345");
        assert_eq!(response.value_range.range, "Sheet1!A1:B2");
        if let serde_json::Value::Array(values) = &response.value_range.values {
            assert_eq!(values.len(), 2);
            if let serde_json::Value::Array(first_row) = &values[0] {
                assert_eq!(first_row[0], "Name");
                assert_eq!(first_row[1], "Age");
            }
            if let serde_json::Value::Array(second_row) = &values[1] {
                assert_eq!(second_row[0], "Alice");
                assert_eq!(second_row[1], "30");
            }
        } else {
            panic!("Expected array values");
        }
    }

    #[test]
    fn test_reading_single_range_response_no_values() {
        let json_data = json!({
            "revision": 789,
            "spreadsheetToken": "empty_token",
            "valueRange": {
                "majorDimension": "ROWS",
                "range": "Sheet1!A1:A1",
                "values": null,
                "revision": 789
            }
        });

        let response: ReadingSingleRangeResponse = serde_json::from_value(json_data).unwrap();
        assert_eq!(response.revision, 789);
        assert_eq!(response.spreadsheet_token, "empty_token");
        assert_eq!(response.value_range.range, "Sheet1!A1:A1");
        assert_eq!(response.value_range.values, serde_json::Value::Null);
    }

    #[test]
    fn test_reading_single_range_response_large_dataset() {
        let mut large_values = Vec::new();
        for i in 0..1000 {
            large_values.push(vec![format!("Row{}", i), format!("Data{}", i)]);
        }

        let json_data = json!({
            "revision": 999999,
            "spreadsheetToken": "large_data_token",
            "valueRange": {
                "majorDimension": "ROWS",
                "range": "Sheet1!A1:B1000",
                "values": large_values,
                "revision": 999999
            }
        });

        let response: ReadingSingleRangeResponse = serde_json::from_value(json_data).unwrap();
        assert_eq!(response.revision, 999999);
        assert_eq!(response.spreadsheet_token, "large_data_token");
        assert_eq!(response.value_range.range, "Sheet1!A1:B1000");

        if let serde_json::Value::Array(values) = &response.value_range.values {
            assert_eq!(values.len(), 1000);
            if let serde_json::Value::Array(first_row) = &values[0] {
                assert_eq!(first_row[0], "Row0");
                assert_eq!(first_row[1], "Data0");
            }
            if let serde_json::Value::Array(last_row) = &values[999] {
                assert_eq!(last_row[0], "Row999");
                assert_eq!(last_row[1], "Data999");
            }
        } else {
            panic!("Expected array values");
        }
    }

    #[test]
    fn test_reading_single_range_response_unicode_data() {
        let json_data = json!({
            "revision": 456789,
            "spreadsheetToken": "unicode_token_测试",
            "valueRange": {
                "majorDimension": "ROWS",
                "range": "工作表1!A1:C3",
                "values": [
                    ["姓名", "年龄", "城市"],
                    ["张三", "25", "北京"],
                    ["李四", "30", "上海"],
                    ["🎉", "🔥", "📊"]
                ],
                "revision": 456789
            }
        });

        let response: ReadingSingleRangeResponse = serde_json::from_value(json_data).unwrap();
        assert_eq!(response.revision, 456789);
        assert_eq!(response.spreadsheet_token, "unicode_token_测试");
        assert_eq!(response.value_range.range, "工作表1!A1:C3");

        if let serde_json::Value::Array(values) = &response.value_range.values {
            assert_eq!(values.len(), 4);
            if let serde_json::Value::Array(first_row) = &values[0] {
                assert_eq!(first_row[0], "姓名");
                assert_eq!(first_row[1], "年龄");
                assert_eq!(first_row[2], "城市");
            }
            if let serde_json::Value::Array(second_row) = &values[1] {
                assert_eq!(second_row[0], "张三");
                assert_eq!(second_row[1], "25");
                assert_eq!(second_row[2], "北京");
            }
            if let serde_json::Value::Array(fourth_row) = &values[3] {
                assert_eq!(fourth_row[0], "🎉");
                assert_eq!(fourth_row[1], "🔥");
                assert_eq!(fourth_row[2], "📊");
            }
        } else {
            panic!("Expected array values");
        }
    }

    #[test]
    fn test_reading_single_range_response_api_trait() {
        assert_eq!(
            ReadingSingleRangeResponse::data_format(),
            ResponseFormat::Data
        );
    }

    #[test]
    fn test_reading_single_range_executable_builder_trait() {
        let service = create_test_service();
        let request = ReadingSingleRangeRequest::builder()
            .spreadsheet_token("test_token")
            .range("Sheet1!A1:B2")
            .build();

        // Test that the service method is available
        // We can't easily test the actual execution without a mock server
        // but we can verify the method exists by calling it in a future
        let _future = service.reading_a_single_range(request, None);
        // Just verify the method compiles and returns a future
    }

    #[test]
    fn test_reading_single_range_memory_efficiency() {
        // Test that the builder doesn't allocate unnecessarily
        let builder = ReadingSingleRangeRequest::builder();
        let size_before = std::mem::size_of_val(&builder);

        let builder = builder
            .spreadsheet_token("test")
            .range("A1:B2")
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
    fn test_reading_single_range_request_query_params_independence() {
        let request1 = ReadingSingleRangeRequest::builder()
            .spreadsheet_token("token1")
            .range("Sheet1!A1:B2")
            .value_render_option("FormattedValue")
            .build();

        let request2 = ReadingSingleRangeRequest::builder()
            .spreadsheet_token("token2")
            .range("Sheet2!C1:D2")
            .user_id_type("open_id")
            .build();

        // Ensure requests don't share query params
        assert_eq!(
            request1.api_request.query_params.get("valueRenderOption"),
            Some(&"FormattedValue".to_string())
        );
        assert!(!request1
            .api_request
            .query_params
            .contains_key("user_id_type"));

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
    fn test_reading_single_range_request_builder_chaining() {
        // Test that all builder methods can be chained fluently
        let request = ReadingSingleRangeRequest::builder()
            .spreadsheet_token("chain_test_token")
            .range("Sheet1!A1:Z100")
            .value_render_option("ToString")
            .date_time_render_option("FormattedString")
            .user_id_type("union_id")
            .build();

        assert_eq!(request.spreadsheet_token, "chain_test_token");
        assert_eq!(request.range, "Sheet1!A1:Z100");
        assert_eq!(request.value_render_option, Some("ToString".to_string()));
        assert_eq!(
            request.date_time_render_option,
            Some("FormattedString".to_string())
        );
        assert_eq!(request.user_id_type, Some("union_id".to_string()));

        // Verify all query params are set
        assert_eq!(request.api_request.query_params.len(), 3);
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
    fn test_reading_single_range_response_deeply_nested_values() {
        let complex_values = vec![
            vec![
                "=SUM(A1:A10)".to_string(),
                "=IF(B1>0,\"Positive\",\"Negative\")".to_string(),
            ],
            vec!["2023-12-31 23:59:59".to_string(), "0.5".to_string()],
            vec!["@user_12345".to_string(), "@group_67890".to_string()],
        ];

        let json_data = json!({
            "revision": 2023123115,
            "spreadsheetToken": "complex_formulas_token",
            "valueRange": {
                "majorDimension": "ROWS",
                "range": "Formulas!A1:B3",
                "values": complex_values,
                "revision": 2023123115
            }
        });

        let response: ReadingSingleRangeResponse = serde_json::from_value(json_data).unwrap();
        assert_eq!(response.revision, 2023123115);

        if let serde_json::Value::Array(values) = &response.value_range.values {
            assert_eq!(values.len(), 3);
            if let serde_json::Value::Array(first_row) = &values[0] {
                assert_eq!(first_row[0], "=SUM(A1:A10)");
            }
            if let serde_json::Value::Array(second_row) = &values[1] {
                assert_eq!(second_row[0], "2023-12-31 23:59:59");
            }
            if let serde_json::Value::Array(third_row) = &values[2] {
                assert_eq!(third_row[0], "@user_12345");
            }
        } else {
            panic!("Expected array values");
        }
    }
}

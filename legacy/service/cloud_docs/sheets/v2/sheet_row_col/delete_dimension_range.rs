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
    service::cloud_docs::sheets::v2::{sheet_row_col::UpdateDimension, SpreadsheetService},
};

/// 删除行列请求
#[derive(Serialize, Debug, Default)]
pub struct DeleteDimensionRangeRequest {
    #[serde(skip)]
    api_request: ApiRequest,
    #[serde(skip)]
    spreadsheet_token: String,
    /// 需要删除行列的范围信息
    dimension: UpdateDimension,
}

impl DeleteDimensionRangeRequest {
    pub fn builder() -> DeleteDimensionRangeRequestBuilder {
        DeleteDimensionRangeRequestBuilder::default()
    }
}

#[derive(Default)]
pub struct DeleteDimensionRangeRequestBuilder {
    request: DeleteDimensionRangeRequest,
}

impl DeleteDimensionRangeRequestBuilder {
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

    pub fn build(mut self) -> DeleteDimensionRangeRequest {
        self.request.api_request.body = serde_json::to_vec(&self.request).unwrap();
        self.request
    }
}

impl_executable_builder_owned!(
    DeleteDimensionRangeRequestBuilder,
    SpreadsheetService,
    DeleteDimensionRangeRequest,
    BaseResponse<DeleteDimensionRangeResponse>,
    delete_dimension_range
);

impl SpreadsheetService {
    /// 该接口用于删除电子表格中的指定行或列。
    pub async fn delete_dimension_range(
        &self,
        request: DeleteDimensionRangeRequest,
        option: Option<req_option::RequestOption>,
    ) -> SDKResult<BaseResponse<DeleteDimensionRangeResponse>> {
        let mut api_req = request.api_request;
        api_req.api_path =
            SHEETS_V2_SPREADSHEET_DIMENSION_RANGE.replace("{}", &request.spreadsheet_token);
        api_req.http_method = reqwest::Method::DELETE;
        api_req.supported_access_token_types = vec![AccessTokenType::Tenant, AccessTokenType::App];

        let api_resp = crate::core::http::Transport::request(api_req, &self.config, option).await?;

        Ok(api_resp)
    }
}

/// 删除行列响应体
#[derive(Deserialize, Debug)]
pub struct DeleteDimensionRangeResponse {
    /// 一共删除的行数或列数
    #[serde(rename = "delCount")]
    pub del_count: i32,
    /// 删除的维度。枚举值：
    /// - ROWS：行
    /// - COLUMNS：列
    #[serde(rename = "majorDimension")]
    pub major_dimension: String,
}

impl ApiResponseTrait for DeleteDimensionRangeResponse {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
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
    fn test_delete_dimension_range_request_builder_creation() {
        let builder = DeleteDimensionRangeRequest::builder();
        let request = builder.build();

        assert_eq!(request.spreadsheet_token, "");
        assert_eq!(request.dimension.sheet_id, "");
        assert_eq!(request.dimension.major_dimension, "");
        assert_eq!(request.dimension.start_index, 0);
        assert_eq!(request.dimension.end_index, 0);
    }

    #[test]
    fn test_delete_dimension_range_request_builder_with_spreadsheet_token() {
        let request = DeleteDimensionRangeRequest::builder()
            .spreadsheet_token("test_spreadsheet_123")
            .build();

        assert_eq!(request.spreadsheet_token, "test_spreadsheet_123");
    }

    #[test]
    fn test_delete_dimension_range_request_builder_with_sheet_id() {
        let request = DeleteDimensionRangeRequest::builder()
            .sheet_id("test_sheet_456")
            .build();

        assert_eq!(request.dimension.sheet_id, "test_sheet_456");
    }

    #[rstest]
    #[case("ROWS")]
    #[case("COLUMNS")]
    fn test_delete_dimension_range_request_builder_with_major_dimension(#[case] dimension: &str) {
        let request = DeleteDimensionRangeRequest::builder()
            .major_dimension(dimension)
            .build();

        assert_eq!(request.dimension.major_dimension, dimension);
    }

    #[test]
    fn test_delete_dimension_range_request_builder_with_start_index() {
        let request = DeleteDimensionRangeRequest::builder()
            .start_index(5)
            .build();

        assert_eq!(request.dimension.start_index, 5);
    }

    #[test]
    fn test_delete_dimension_range_request_builder_with_end_index() {
        let request = DeleteDimensionRangeRequest::builder().end_index(10).build();

        assert_eq!(request.dimension.end_index, 10);
    }

    #[test]
    fn test_delete_dimension_range_request_builder_chaining() {
        let request = DeleteDimensionRangeRequest::builder()
            .spreadsheet_token("my_spreadsheet")
            .sheet_id("my_sheet")
            .major_dimension("ROWS")
            .start_index(3)
            .end_index(7)
            .build();

        assert_eq!(request.spreadsheet_token, "my_spreadsheet");
        assert_eq!(request.dimension.sheet_id, "my_sheet");
        assert_eq!(request.dimension.major_dimension, "ROWS");
        assert_eq!(request.dimension.start_index, 3);
        assert_eq!(request.dimension.end_index, 7);
    }

    #[test]
    fn test_delete_dimension_range_request_default() {
        let request = DeleteDimensionRangeRequest::default();

        assert_eq!(request.spreadsheet_token, "");
        assert_eq!(request.dimension.sheet_id, "");
        assert_eq!(request.dimension.major_dimension, "");
        assert_eq!(request.dimension.start_index, 0);
        assert_eq!(request.dimension.end_index, 0);
    }

    #[test]
    fn test_delete_dimension_range_request_builder_default() {
        let builder = DeleteDimensionRangeRequestBuilder::default();
        let request = builder.build();

        assert_eq!(request.spreadsheet_token, "");
        assert_eq!(request.dimension.sheet_id, "");
        assert_eq!(request.dimension.major_dimension, "");
        assert_eq!(request.dimension.start_index, 0);
        assert_eq!(request.dimension.end_index, 0);
    }

    #[test]
    fn test_delete_dimension_range_request_serialization() {
        let request = DeleteDimensionRangeRequest::builder()
            .spreadsheet_token("token123")
            .sheet_id("sheet456")
            .major_dimension("COLUMNS")
            .start_index(2)
            .end_index(5)
            .build();

        // Test that the request can be serialized (this validates field names)
        let serialized = serde_json::to_string(&request);
        assert!(serialized.is_ok());

        let json_str = serialized.unwrap();
        assert!(json_str.contains("dimension"));
        assert!(json_str.contains("COLUMNS"));
        assert!(json_str.contains("\"startIndex\":2"));
        assert!(json_str.contains("\"endIndex\":5"));
    }

    #[test]
    fn test_delete_dimension_range_request_debug() {
        let request = DeleteDimensionRangeRequest::builder()
            .spreadsheet_token("debug_token")
            .sheet_id("debug_sheet")
            .build();

        let debug_str = format!("{:?}", request);
        assert!(debug_str.contains("DeleteDimensionRangeRequest"));
        assert!(debug_str.contains("debug_token"));
        assert!(debug_str.contains("debug_sheet"));
    }

    #[test]
    fn test_delete_dimension_range_request_with_empty_strings() {
        let request = DeleteDimensionRangeRequest::builder()
            .spreadsheet_token("")
            .sheet_id("")
            .major_dimension("")
            .build();

        assert_eq!(request.spreadsheet_token, "");
        assert_eq!(request.dimension.sheet_id, "");
        assert_eq!(request.dimension.major_dimension, "");
    }

    #[test]
    fn test_delete_dimension_range_request_with_special_characters() {
        let request = DeleteDimensionRangeRequest::builder()
            .spreadsheet_token("token_with_特殊字符_🎯")
            .sheet_id("sheet_名称_123")
            .major_dimension("ROWS")
            .build();

        assert_eq!(request.spreadsheet_token, "token_with_特殊字符_🎯");
        assert_eq!(request.dimension.sheet_id, "sheet_名称_123");
        assert_eq!(request.dimension.major_dimension, "ROWS");
    }

    #[rstest]
    #[case(0, 1)]
    #[case(1, 5)]
    #[case(10, 20)]
    #[case(100, 200)]
    #[case(-1, 0)] // Edge case: negative start
    #[case(5, 5)] // Edge case: start equals end
    fn test_delete_dimension_range_request_with_various_indices(
        #[case] start: i32,
        #[case] end: i32,
    ) {
        let request = DeleteDimensionRangeRequest::builder()
            .start_index(start)
            .end_index(end)
            .build();

        assert_eq!(request.dimension.start_index, start);
        assert_eq!(request.dimension.end_index, end);
    }

    #[test]
    fn test_delete_dimension_range_request_with_maximum_values() {
        let request = DeleteDimensionRangeRequest::builder()
            .start_index(i32::MAX)
            .end_index(i32::MAX)
            .build();

        assert_eq!(request.dimension.start_index, i32::MAX);
        assert_eq!(request.dimension.end_index, i32::MAX);
    }

    #[test]
    fn test_delete_dimension_range_request_with_minimum_values() {
        let request = DeleteDimensionRangeRequest::builder()
            .start_index(i32::MIN)
            .end_index(i32::MIN)
            .build();

        assert_eq!(request.dimension.start_index, i32::MIN);
        assert_eq!(request.dimension.end_index, i32::MIN);
    }

    #[test]
    fn test_delete_dimension_range_request_api_request_body_serialization() {
        let request = DeleteDimensionRangeRequest::builder()
            .spreadsheet_token("body_test_token")
            .sheet_id("body_test_sheet")
            .major_dimension("ROWS")
            .start_index(1)
            .end_index(3)
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
    }

    #[test]
    fn test_delete_dimension_range_request_builder_multiple_calls() {
        let mut builder = DeleteDimensionRangeRequest::builder();

        // Test that multiple calls override previous values
        builder = builder.spreadsheet_token("first_token");
        builder = builder.spreadsheet_token("second_token");
        builder = builder.sheet_id("first_sheet");
        builder = builder.sheet_id("second_sheet");
        builder = builder.start_index(1);
        builder = builder.start_index(2);

        let request = builder.build();

        assert_eq!(request.spreadsheet_token, "second_token");
        assert_eq!(request.dimension.sheet_id, "second_sheet");
        assert_eq!(request.dimension.start_index, 2);
    }

    #[test]
    fn test_spreadsheet_service_creation() {
        let service = create_test_service();

        // Verify the service can be created without panic
        assert_eq!(service.config.app_id, "test_app_id");
    }

    #[test]
    fn test_delete_dimension_range_response_data_format() {
        assert_eq!(
            DeleteDimensionRangeResponse::data_format(),
            ResponseFormat::Data
        );
    }

    #[test]
    fn test_delete_dimension_range_response_deserialization() {
        let json_response = r#"{"delCount": 5, "majorDimension": "ROWS"}"#;
        let response: DeleteDimensionRangeResponse = serde_json::from_str(json_response).unwrap();

        assert_eq!(response.del_count, 5);
        assert_eq!(response.major_dimension, "ROWS");
    }

    #[test]
    fn test_delete_dimension_range_response_deserialization_columns() {
        let json_response = r#"{"delCount": 3, "majorDimension": "COLUMNS"}"#;
        let response: DeleteDimensionRangeResponse = serde_json::from_str(json_response).unwrap();

        assert_eq!(response.del_count, 3);
        assert_eq!(response.major_dimension, "COLUMNS");
    }

    #[test]
    fn test_delete_dimension_range_response_debug() {
        let response = DeleteDimensionRangeResponse {
            del_count: 10,
            major_dimension: "ROWS".to_string(),
        };

        let debug_str = format!("{:?}", response);
        assert!(debug_str.contains("DeleteDimensionRangeResponse"));
        assert!(debug_str.contains("del_count: 10"));
        assert!(debug_str.contains("ROWS"));
    }

    #[test]
    fn test_delete_dimension_range_response_with_zero_count() {
        let json_response = r#"{"delCount": 0, "majorDimension": "ROWS"}"#;
        let response: DeleteDimensionRangeResponse = serde_json::from_str(json_response).unwrap();

        assert_eq!(response.del_count, 0);
        assert_eq!(response.major_dimension, "ROWS");
    }

    #[test]
    fn test_delete_dimension_range_response_with_large_count() {
        let json_response = r#"{"delCount": 999999, "majorDimension": "COLUMNS"}"#;
        let response: DeleteDimensionRangeResponse = serde_json::from_str(json_response).unwrap();

        assert_eq!(response.del_count, 999999);
        assert_eq!(response.major_dimension, "COLUMNS");
    }

    #[test]
    fn test_delete_dimension_range_request_edge_cases() {
        // Test with very long token
        let long_token = "a".repeat(10000);
        let request = DeleteDimensionRangeRequest::builder()
            .spreadsheet_token(&long_token)
            .build();
        assert_eq!(request.spreadsheet_token, long_token);

        // Test with very long sheet ID
        let long_sheet_id = "sheet_".repeat(1000);
        let request = DeleteDimensionRangeRequest::builder()
            .sheet_id(&long_sheet_id)
            .build();
        assert_eq!(request.dimension.sheet_id, long_sheet_id);

        // Test with extreme index values
        let request = DeleteDimensionRangeRequest::builder()
            .start_index(0)
            .end_index(1000000)
            .build();
        assert_eq!(request.dimension.start_index, 0);
        assert_eq!(request.dimension.end_index, 1000000);
    }

    #[test]
    fn test_delete_dimension_range_request_memory_efficiency() {
        // Test creating many requests doesn't consume excessive memory
        let requests: Vec<DeleteDimensionRangeRequest> = (0..100)
            .map(|i| {
                DeleteDimensionRangeRequest::builder()
                    .spreadsheet_token(format!("token_{}", i))
                    .sheet_id(format!("sheet_{}", i))
                    .major_dimension(if i % 2 == 0 { "ROWS" } else { "COLUMNS" })
                    .start_index(i)
                    .end_index(i + 10)
                    .build()
            })
            .collect();

        assert_eq!(requests.len(), 100);

        // Verify each request has correct data
        for (i, request) in requests.iter().enumerate() {
            assert_eq!(request.spreadsheet_token, format!("token_{}", i));
            assert_eq!(request.dimension.sheet_id, format!("sheet_{}", i));
            assert_eq!(request.dimension.start_index, i as i32);
            assert_eq!(request.dimension.end_index, (i + 10) as i32);
        }
    }
}

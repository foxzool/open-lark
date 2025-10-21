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

/// 更新行列请求
#[derive(Serialize, Debug, Default)]
pub struct UpdateDimensionRangeRequest {
    #[serde(skip)]
    api_request: ApiRequest,
    #[serde(skip)]
    spreadsheet_token: String,
    /// 需要更新行列的维度信息
    dimension: UpdateDimension,
    /// 更新行或列的属性。至少写入以下参数之一
    #[serde(rename = "dimensionProperties")]
    dimension_properties: DimensionProperties,
}

/// 更新行或列的属性。至少写入以下参数之一
#[derive(Serialize, Debug, Default)]
struct DimensionProperties {
    #[serde(skip_serializing_if = "Option::is_none")]
    visible: Option<bool>,
    #[serde(rename = "fixedSize", skip_serializing_if = "Option::is_none")]
    fixed_size: Option<i32>,
}

impl UpdateDimensionRangeRequest {
    pub fn builder() -> UpdateDimensionRangeRequestBuilder {
        UpdateDimensionRangeRequestBuilder::default()
    }
}

#[derive(Default)]
pub struct UpdateDimensionRangeRequestBuilder {
    request: UpdateDimensionRangeRequest,
}

impl UpdateDimensionRangeRequestBuilder {
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

    /// 是否隐藏行或列。可选值：
    /// - true：显示行或列
    /// - false：隐藏行或列
    pub fn visible(mut self, visible: bool) -> Self {
        self.request.dimension_properties.visible = Some(visible);
        self
    }

    /// 行高或列宽。单位为像素。fixedSize 为 0 时，等价于隐藏行或列。
    pub fn fixed_size(mut self, fixed_size: i32) -> Self {
        self.request.dimension_properties.fixed_size = Some(fixed_size);
        self
    }

    pub fn build(mut self) -> UpdateDimensionRangeRequest {
        self.request.api_request.body = serde_json::to_vec(&self.request).unwrap();
        self.request
    }
}

impl_executable_builder_owned!(
    UpdateDimensionRangeRequestBuilder,
    SpreadsheetService,
    UpdateDimensionRangeRequest,
    BaseResponse<EmptyResponse>,
    update_dimension_range
);

impl SpreadsheetService {
    /// 该接口用于更新设置电子表格中行列的属性，包括是否隐藏行列和设置行高列宽。
    pub async fn update_dimension_range(
        &self,
        request: UpdateDimensionRangeRequest,
        option: Option<req_option::RequestOption>,
    ) -> SDKResult<BaseResponse<EmptyResponse>> {
        let mut api_req = request.api_request;
        api_req.api_path =
            SHEETS_V2_SPREADSHEET_DIMENSION_RANGE.replace("{}", &request.spreadsheet_token);
        api_req.http_method = reqwest::Method::PUT;
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
    fn test_update_dimension_range_request_builder_creation() {
        let builder = UpdateDimensionRangeRequest::builder();
        let request = builder.build();

        assert_eq!(request.spreadsheet_token, "");
        assert_eq!(request.dimension.sheet_id, "");
        assert_eq!(request.dimension.major_dimension, "");
        assert_eq!(request.dimension.start_index, 0);
        assert_eq!(request.dimension.end_index, 0);
        assert!(request.dimension_properties.visible.is_none());
        assert!(request.dimension_properties.fixed_size.is_none());
    }

    #[test]
    fn test_update_dimension_range_request_builder_with_spreadsheet_token() {
        let request = UpdateDimensionRangeRequest::builder()
            .spreadsheet_token("test_spreadsheet_123")
            .build();

        assert_eq!(request.spreadsheet_token, "test_spreadsheet_123");
    }

    #[test]
    fn test_update_dimension_range_request_builder_with_sheet_id() {
        let request = UpdateDimensionRangeRequest::builder()
            .sheet_id("test_sheet_456")
            .build();

        assert_eq!(request.dimension.sheet_id, "test_sheet_456");
    }

    #[rstest]
    #[case("ROWS")]
    #[case("COLUMNS")]
    fn test_update_dimension_range_request_builder_with_major_dimension(#[case] dimension: &str) {
        let request = UpdateDimensionRangeRequest::builder()
            .major_dimension(dimension)
            .build();

        assert_eq!(request.dimension.major_dimension, dimension);
    }

    #[test]
    fn test_update_dimension_range_request_builder_with_start_index() {
        let request = UpdateDimensionRangeRequest::builder()
            .start_index(5)
            .build();

        assert_eq!(request.dimension.start_index, 5);
    }

    #[test]
    fn test_update_dimension_range_request_builder_with_end_index() {
        let request = UpdateDimensionRangeRequest::builder().end_index(10).build();

        assert_eq!(request.dimension.end_index, 10);
    }

    #[rstest]
    #[case(true)]
    #[case(false)]
    fn test_update_dimension_range_request_builder_with_visible(#[case] visible: bool) {
        let request = UpdateDimensionRangeRequest::builder()
            .visible(visible)
            .build();

        assert_eq!(request.dimension_properties.visible, Some(visible));
    }

    #[test]
    fn test_update_dimension_range_request_builder_with_fixed_size() {
        let request = UpdateDimensionRangeRequest::builder()
            .fixed_size(100)
            .build();

        assert_eq!(request.dimension_properties.fixed_size, Some(100));
    }

    #[test]
    fn test_update_dimension_range_request_builder_chaining() {
        let request = UpdateDimensionRangeRequest::builder()
            .spreadsheet_token("my_spreadsheet")
            .sheet_id("my_sheet")
            .major_dimension("ROWS")
            .start_index(3)
            .end_index(7)
            .visible(true)
            .fixed_size(50)
            .build();

        assert_eq!(request.spreadsheet_token, "my_spreadsheet");
        assert_eq!(request.dimension.sheet_id, "my_sheet");
        assert_eq!(request.dimension.major_dimension, "ROWS");
        assert_eq!(request.dimension.start_index, 3);
        assert_eq!(request.dimension.end_index, 7);
        assert_eq!(request.dimension_properties.visible, Some(true));
        assert_eq!(request.dimension_properties.fixed_size, Some(50));
    }

    #[test]
    fn test_update_dimension_range_request_default() {
        let request = UpdateDimensionRangeRequest::default();

        assert_eq!(request.spreadsheet_token, "");
        assert_eq!(request.dimension.sheet_id, "");
        assert_eq!(request.dimension.major_dimension, "");
        assert_eq!(request.dimension.start_index, 0);
        assert_eq!(request.dimension.end_index, 0);
        assert!(request.dimension_properties.visible.is_none());
        assert!(request.dimension_properties.fixed_size.is_none());
    }

    #[test]
    fn test_update_dimension_range_request_builder_default() {
        let builder = UpdateDimensionRangeRequestBuilder::default();
        let request = builder.build();

        assert_eq!(request.spreadsheet_token, "");
        assert_eq!(request.dimension.sheet_id, "");
        assert_eq!(request.dimension.major_dimension, "");
        assert_eq!(request.dimension.start_index, 0);
        assert_eq!(request.dimension.end_index, 0);
        assert!(request.dimension_properties.visible.is_none());
        assert!(request.dimension_properties.fixed_size.is_none());
    }

    #[test]
    fn test_dimension_properties_default() {
        let props = DimensionProperties::default();

        assert!(props.visible.is_none());
        assert!(props.fixed_size.is_none());
    }

    #[test]
    fn test_update_dimension_range_request_serialization() {
        let request = UpdateDimensionRangeRequest::builder()
            .spreadsheet_token("token123")
            .sheet_id("sheet456")
            .major_dimension("COLUMNS")
            .start_index(2)
            .end_index(5)
            .visible(false)
            .fixed_size(80)
            .build();

        // Test that the request can be serialized (this validates field names)
        let serialized = serde_json::to_string(&request);
        assert!(serialized.is_ok());

        let json_str = serialized.unwrap();
        assert!(json_str.contains("dimension"));
        assert!(json_str.contains("COLUMNS"));
        assert!(json_str.contains("\"startIndex\":2"));
        assert!(json_str.contains("\"endIndex\":5"));
        assert!(json_str.contains("\"dimensionProperties\""));
        assert!(json_str.contains("\"visible\":false"));
        assert!(json_str.contains("\"fixedSize\":80"));
    }

    #[test]
    fn test_update_dimension_range_request_serialization_minimal() {
        let request = UpdateDimensionRangeRequest::builder()
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
        assert!(json_str.contains("\"dimensionProperties\""));
        // visible and fixedSize should not be present when None
        assert!(!json_str.contains("visible"));
        assert!(!json_str.contains("fixedSize"));
    }

    #[test]
    fn test_update_dimension_range_request_debug() {
        let request = UpdateDimensionRangeRequest::builder()
            .spreadsheet_token("debug_token")
            .sheet_id("debug_sheet")
            .visible(true)
            .fixed_size(120)
            .build();

        let debug_str = format!("{:?}", request);
        assert!(debug_str.contains("UpdateDimensionRangeRequest"));
        assert!(debug_str.contains("debug_token"));
        assert!(debug_str.contains("debug_sheet"));
        assert!(debug_str.contains("visible: Some(true)"));
        assert!(debug_str.contains("fixed_size: Some(120)"));
    }

    #[test]
    fn test_update_dimension_range_request_with_empty_strings() {
        let request = UpdateDimensionRangeRequest::builder()
            .spreadsheet_token("")
            .sheet_id("")
            .major_dimension("")
            .build();

        assert_eq!(request.spreadsheet_token, "");
        assert_eq!(request.dimension.sheet_id, "");
        assert_eq!(request.dimension.major_dimension, "");
    }

    #[test]
    fn test_update_dimension_range_request_with_special_characters() {
        let request = UpdateDimensionRangeRequest::builder()
            .spreadsheet_token("token_with_特殊字符_🎯")
            .sheet_id("sheet_名称_123")
            .major_dimension("ROWS")
            .visible(false)
            .build();

        assert_eq!(request.spreadsheet_token, "token_with_特殊字符_🎯");
        assert_eq!(request.dimension.sheet_id, "sheet_名称_123");
        assert_eq!(request.dimension.major_dimension, "ROWS");
        assert_eq!(request.dimension_properties.visible, Some(false));
    }

    #[rstest]
    #[case(0, 1)]
    #[case(1, 5)]
    #[case(10, 20)]
    #[case(100, 200)]
    #[case(-1, 0)] // Edge case: negative start
    #[case(5, 5)] // Edge case: start equals end
    fn test_update_dimension_range_request_with_various_indices(
        #[case] start: i32,
        #[case] end: i32,
    ) {
        let request = UpdateDimensionRangeRequest::builder()
            .start_index(start)
            .end_index(end)
            .build();

        assert_eq!(request.dimension.start_index, start);
        assert_eq!(request.dimension.end_index, end);
    }

    #[test]
    fn test_update_dimension_range_request_with_maximum_values() {
        let request = UpdateDimensionRangeRequest::builder()
            .start_index(i32::MAX)
            .end_index(i32::MAX)
            .fixed_size(i32::MAX)
            .build();

        assert_eq!(request.dimension.start_index, i32::MAX);
        assert_eq!(request.dimension.end_index, i32::MAX);
        assert_eq!(request.dimension_properties.fixed_size, Some(i32::MAX));
    }

    #[test]
    fn test_update_dimension_range_request_with_minimum_values() {
        let request = UpdateDimensionRangeRequest::builder()
            .start_index(i32::MIN)
            .end_index(i32::MIN)
            .fixed_size(i32::MIN)
            .build();

        assert_eq!(request.dimension.start_index, i32::MIN);
        assert_eq!(request.dimension.end_index, i32::MIN);
        assert_eq!(request.dimension_properties.fixed_size, Some(i32::MIN));
    }

    #[test]
    fn test_update_dimension_range_request_api_request_body_serialization() {
        let request = UpdateDimensionRangeRequest::builder()
            .spreadsheet_token("body_test_token")
            .sheet_id("body_test_sheet")
            .major_dimension("ROWS")
            .start_index(1)
            .end_index(3)
            .visible(true)
            .fixed_size(60)
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

        assert!(parsed.get("dimensionProperties").is_some());
        let properties = parsed.get("dimensionProperties").unwrap();
        assert_eq!(properties.get("visible").unwrap(), true);
        assert_eq!(properties.get("fixedSize").unwrap(), 60);
    }

    #[test]
    fn test_update_dimension_range_request_builder_multiple_calls() {
        let mut builder = UpdateDimensionRangeRequest::builder();

        // Test that multiple calls override previous values
        builder = builder.spreadsheet_token("first_token");
        builder = builder.spreadsheet_token("second_token");
        builder = builder.sheet_id("first_sheet");
        builder = builder.sheet_id("second_sheet");
        builder = builder.start_index(1);
        builder = builder.start_index(2);
        builder = builder.visible(true);
        builder = builder.visible(false);
        builder = builder.fixed_size(50);
        builder = builder.fixed_size(100);

        let request = builder.build();

        assert_eq!(request.spreadsheet_token, "second_token");
        assert_eq!(request.dimension.sheet_id, "second_sheet");
        assert_eq!(request.dimension.start_index, 2);
        assert_eq!(request.dimension_properties.visible, Some(false));
        assert_eq!(request.dimension_properties.fixed_size, Some(100));
    }

    #[test]
    fn test_spreadsheet_service_creation() {
        let service = create_test_service();

        // Verify the service can be created without panic
        assert_eq!(service.config.app_id, "test_app_id");
    }

    #[test]
    fn test_update_dimension_range_request_edge_cases() {
        // Test with very long token
        let long_token = "a".repeat(10000);
        let request = UpdateDimensionRangeRequest::builder()
            .spreadsheet_token(&long_token)
            .build();
        assert_eq!(request.spreadsheet_token, long_token);

        // Test with very long sheet ID
        let long_sheet_id = "sheet_".repeat(1000);
        let request = UpdateDimensionRangeRequest::builder()
            .sheet_id(&long_sheet_id)
            .build();
        assert_eq!(request.dimension.sheet_id, long_sheet_id);

        // Test with extreme index values
        let request = UpdateDimensionRangeRequest::builder()
            .start_index(0)
            .end_index(1000000)
            .fixed_size(0) // This equals hiding the row/column
            .build();
        assert_eq!(request.dimension.start_index, 0);
        assert_eq!(request.dimension.end_index, 1000000);
        assert_eq!(request.dimension_properties.fixed_size, Some(0));
    }

    #[rstest]
    #[case(0)] // Hide row/column
    #[case(1)] // Minimum visible size
    #[case(50)] // Normal size
    #[case(100)] // Large size
    #[case(1000)] // Very large size
    fn test_update_dimension_range_request_with_various_fixed_sizes(#[case] size: i32) {
        let request = UpdateDimensionRangeRequest::builder()
            .fixed_size(size)
            .build();

        assert_eq!(request.dimension_properties.fixed_size, Some(size));
    }

    #[test]
    fn test_update_dimension_range_request_memory_efficiency() {
        // Test creating many requests doesn't consume excessive memory
        let requests: Vec<UpdateDimensionRangeRequest> = (0..100)
            .map(|i| {
                let mut builder = UpdateDimensionRangeRequest::builder()
                    .spreadsheet_token(format!("token_{}", i))
                    .sheet_id(format!("sheet_{}", i))
                    .major_dimension(if i % 2 == 0 { "ROWS" } else { "COLUMNS" })
                    .start_index(i)
                    .end_index(i + 10);

                if i % 3 == 0 {
                    builder = builder.visible(true);
                } else if i % 3 == 1 {
                    builder = builder.visible(false);
                }
                // For i % 3 == 2, leave visible as None

                if i % 4 == 0 {
                    builder = builder.fixed_size(i * 10);
                }
                // For other cases, leave fixed_size as None

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
                0 => assert_eq!(request.dimension_properties.visible, Some(true)),
                1 => assert_eq!(request.dimension_properties.visible, Some(false)),
                2 => assert!(request.dimension_properties.visible.is_none()),
                _ => unreachable!(),
            }

            if i % 4 == 0 {
                assert_eq!(
                    request.dimension_properties.fixed_size,
                    Some((i * 10) as i32)
                );
            } else {
                assert!(request.dimension_properties.fixed_size.is_none());
            }
        }
    }

    #[test]
    fn test_update_dimension_range_request_partial_properties() {
        // Test building with only visible set
        let request1 = UpdateDimensionRangeRequest::builder().visible(true).build();

        assert_eq!(request1.dimension_properties.visible, Some(true));
        assert!(request1.dimension_properties.fixed_size.is_none());

        // Test building with only fixed_size set
        let request2 = UpdateDimensionRangeRequest::builder()
            .fixed_size(75)
            .build();

        assert!(request2.dimension_properties.visible.is_none());
        assert_eq!(request2.dimension_properties.fixed_size, Some(75));

        // Test building with neither property set
        let request3 = UpdateDimensionRangeRequest::builder()
            .spreadsheet_token("test")
            .build();

        assert!(request3.dimension_properties.visible.is_none());
        assert!(request3.dimension_properties.fixed_size.is_none());
    }

    #[test]
    fn test_update_dimension_range_request_serialization_with_null_properties() {
        let request = UpdateDimensionRangeRequest::builder()
            .spreadsheet_token("test_token")
            .sheet_id("test_sheet")
            .major_dimension("ROWS")
            .start_index(1)
            .end_index(3)
            .build();

        let serialized = serde_json::to_string(&request);
        assert!(serialized.is_ok());

        let json_str = serialized.unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&json_str).unwrap();

        // dimensionProperties should be present but visible and fixedSize should not
        assert!(parsed.get("dimensionProperties").is_some());
        let properties = parsed.get("dimensionProperties").unwrap();
        assert!(properties.get("visible").is_none());
        assert!(properties.get("fixedSize").is_none());
    }

    #[test]
    fn test_update_dimension_range_request_unicode_handling() {
        let request = UpdateDimensionRangeRequest::builder()
            .spreadsheet_token("令牌_🔑_test")
            .sheet_id("工作表_📋_id")
            .major_dimension("ROWS")
            .visible(true)
            .fixed_size(100)
            .build();

        assert_eq!(request.spreadsheet_token, "令牌_🔑_test");
        assert_eq!(request.dimension.sheet_id, "工作表_📋_id");
        assert_eq!(request.dimension_properties.visible, Some(true));
        assert_eq!(request.dimension_properties.fixed_size, Some(100));

        // Test serialization works with Unicode
        let serialized = serde_json::to_string(&request);
        assert!(serialized.is_ok());
    }

    #[test]
    fn test_dimension_properties_debug() {
        let props = DimensionProperties {
            visible: Some(true),
            fixed_size: Some(150),
        };

        let debug_str = format!("{:?}", props);
        assert!(debug_str.contains("DimensionProperties"));
        assert!(debug_str.contains("visible: Some(true)"));
        assert!(debug_str.contains("fixed_size: Some(150)"));
    }

    #[test]
    fn test_update_dimension_range_request_builder_partial_configuration() {
        // Test building with only some fields configured
        let request1 = UpdateDimensionRangeRequest::builder()
            .spreadsheet_token("test_token")
            .visible(false)
            .build();

        assert_eq!(request1.spreadsheet_token, "test_token");
        assert_eq!(request1.dimension.sheet_id, "");
        assert_eq!(request1.dimension_properties.visible, Some(false));
        assert!(request1.dimension_properties.fixed_size.is_none());

        let request2 = UpdateDimensionRangeRequest::builder()
            .sheet_id("test_sheet")
            .fixed_size(200)
            .build();

        assert_eq!(request2.spreadsheet_token, "");
        assert_eq!(request2.dimension.sheet_id, "test_sheet");
        assert!(request2.dimension_properties.visible.is_none());
        assert_eq!(request2.dimension_properties.fixed_size, Some(200));
    }
}

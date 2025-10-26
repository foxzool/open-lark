/// Shared test utilities for the open-lark SDK
///
/// This module provides common testing patterns, mock objects, and helper functions
/// to reduce code duplication across tests and ensure consistent testing practices.
use crate::core::{
    api_resp::{ApiResponseTrait, BaseResponse, ErrorInfo, RawResponse, ResponseFormat},
    error::LarkAPIError,
};
use serde::{Deserialize, Serialize};

/// Common test data structure for API response testing
#[derive(Debug, Serialize, Deserialize, PartialEq, Default, Clone)]
pub struct MockApiData {
    pub id: i32,
    pub name: String,
    pub email: Option<String>,
    pub active: bool,
}

impl ApiResponseTrait for MockApiData {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Data
    }
}

/// Mock flatten response data for testing
#[derive(Debug, Serialize, Deserialize, PartialEq, Default, Clone)]
pub struct MockFlattenData {
    pub id: i32,
    pub name: String,
    pub code: i32,
    pub msg: String,
    pub err: Option<String>,
}

impl ApiResponseTrait for MockFlattenData {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Flatten
    }
}

/// Mock binary response data for testing
#[derive(Debug, Serialize, Deserialize, PartialEq, Default, Clone)]
pub struct MockBinaryData {
    pub file_name: String,
    pub content: Vec<u8>,
    pub content_type: Option<String>,
}

impl ApiResponseTrait for MockBinaryData {
    fn data_format() -> ResponseFormat {
        ResponseFormat::Binary
    }

    fn from_binary(file_name: String, body: Vec<u8>) -> Option<Self> {
        Some(MockBinaryData {
            file_name,
            content: body,
            content_type: Some("application/octet-stream".to_string()),
        })
    }
}

/// Builder for creating mock BaseResponse objects
pub struct MockResponseBuilder<T> {
    code: i32,
    msg: String,
    err: Option<ErrorInfo>,
    data: Option<T>,
}

impl<T> MockResponseBuilder<T> {
    pub fn new() -> Self {
        Self {
            code: 0,
            msg: "success".to_string(),
            err: None,
            data: None,
        }
    }

    pub fn success() -> Self {
        Self::new()
    }

    pub fn error(code: i32, msg: &str) -> Self {
        Self {
            code,
            msg: msg.to_string(),
            err: Some(ErrorInfo {
                log_id: None,
                details: vec![],
                permission_violations: vec![],
                field_violations: vec![],
            }),
            data: None,
        }
    }

    pub fn with_code(mut self, code: i32) -> Self {
        self.code = code;
        self
    }

    pub fn with_message(mut self, msg: &str) -> Self {
        self.msg = msg.to_string();
        self
    }

    pub fn with_error(mut self, log_id: Option<String>) -> Self {
        self.err = Some(ErrorInfo {
            log_id,
            details: vec![],
            permission_violations: vec![],
            field_violations: vec![],
        });
        self
    }

    pub fn with_data(mut self, data: T) -> Self {
        self.data = Some(data);
        self
    }

    pub fn build(self) -> BaseResponse<T> {
        BaseResponse {
            raw_response: RawResponse {
                code: self.code,
                msg: self.msg,
                err: self.err,
            },
            data: self.data,
        }
    }
}

impl<T> Default for MockResponseBuilder<T> {
    fn default() -> Self {
        Self::new()
    }
}

/// Common error test cases
pub struct ErrorTestCases;

impl ErrorTestCases {
    /// Standard API error codes and messages for testing
    pub fn api_errors() -> Vec<(i32, &'static str)> {
        vec![
            (400, "Bad Request"),
            (401, "Unauthorized"),
            (403, "Forbidden"),
            (404, "Not Found"),
            (429, "Too Many Requests"),
            (500, "Internal Server Error"),
            (502, "Bad Gateway"),
            (503, "Service Unavailable"),
        ]
    }

    /// Lark-specific error codes for testing
    pub fn lark_errors() -> Vec<(i32, &'static str)> {
        vec![
            (99991663, "Token invalid"),
            (99991664, "Token expired"),
            (99991400, "Bad request parameters"),
            (99991401, "Missing required parameter"),
            (99991402, "Invalid parameter format"),
            (99991500, "Internal service error"),
            (99991503, "Service unavailable"),
        ]
    }
}

/// JSON test data generator
pub struct JsonTestData;

impl JsonTestData {
    /// Generate valid response JSON for testing
    pub fn success_response() -> String {
        r#"{"code": 0, "msg": "success", "data": {"id": 1, "name": "test", "active": true}}"#
            .to_string()
    }

    /// Generate error response JSON for testing
    pub fn error_response(code: i32, msg: &str) -> String {
        format!(
            r#"{{"code": {}, "msg": "{}", "err": "{}"}}"#,
            code, msg, msg
        )
    }

    /// Generate flatten format response JSON
    pub fn flatten_response() -> String {
        r#"{"id": 1, "name": "test", "code": 0, "msg": "success"}"#.to_string()
    }

    /// Generate invalid JSON for error testing
    pub fn invalid_json() -> String {
        r#"{"invalid": json, "missing": quote}"#.to_string()
    }

    /// Generate malformed JSON for error testing
    pub fn malformed_json() -> String {
        r#"{"incomplete": true"#.to_string()
    }

    /// Generate empty JSON object
    pub fn empty_json() -> String {
        "{}".to_string()
    }

    /// Generate null JSON
    pub fn null_json() -> String {
        "null".to_string()
    }
}

/// HTTP header test utilities
pub struct HeaderTestData;

impl HeaderTestData {
    /// Generate Content-Disposition headers for filename extraction testing
    pub fn content_disposition_headers() -> Vec<(&'static str, Option<&'static str>)> {
        vec![
            ("attachment; filename=\"test.txt\"", Some("test.txt")),
            (
                "attachment; filename*=UTF-8''test%20file.pdf",
                Some("test%20file.pdf"),
            ),
            ("attachment; filename=simple.doc", Some("simple.doc")),
            (
                "attachment; filename=\"spaced file.doc\"",
                Some("spaced file.doc"),
            ),
            (
                "attachment; filename=\"unicode文件.txt\"",
                Some("unicode文件.txt"),
            ),
            ("attachment", None),
            ("filename=\"standalone.txt\"", Some("standalone.txt")),
            ("attachment; filename=\"\"", Some("")),
            ("attachment; filename=", Some("")),
            ("malformed header", None),
            ("", None),
        ]
    }

    /// Generate various Content-Type headers
    pub fn content_type_headers() -> Vec<&'static str> {
        vec![
            "application/json",
            "application/octet-stream",
            "text/plain",
            "text/html",
            "image/png",
            "image/jpeg",
            "application/pdf",
            "application/zip",
            "multipart/form-data",
        ]
    }
}

/// Performance test utilities
pub struct PerformanceTestUtils;

impl PerformanceTestUtils {
    /// Measure execution time of a closure
    pub fn measure_time<F, R>(f: F) -> (R, std::time::Duration)
    where
        F: FnOnce() -> R,
    {
        let start = std::time::Instant::now();
        let result = f();
        let duration = start.elapsed();
        (result, duration)
    }

    /// Generate large test data for performance testing
    pub fn large_json_data(size_mb: usize) -> String {
        let base_data = MockApiData {
            id: 1,
            name: "performance_test_data".to_string(),
            email: Some("test@example.com".to_string()),
            active: true,
        };

        let json_base = serde_json::to_string(&base_data).unwrap();
        let target_size = size_mb * 1024 * 1024; // Convert MB to bytes
        let repeat_count = target_size / json_base.len();

        let mut large_data = Vec::new();
        for i in 0..repeat_count {
            large_data.push(MockApiData {
                id: i as i32,
                name: format!("performance_test_data_{}", i),
                email: Some(format!("test{}@example.com", i)),
                active: i % 2 == 0,
            });
        }

        serde_json::to_string(&large_data).unwrap()
    }
}

/// Assertion helpers for tests
pub struct TestAssertions;

impl TestAssertions {
    /// Assert that a BaseResponse is successful
    pub fn assert_success<T>(response: &BaseResponse<T>) {
        assert_eq!(
            response.raw_response.code, 0,
            "Response should be successful"
        );
        assert_eq!(response.raw_response.msg, "success");
        assert!(
            response.data.is_some(),
            "Successful response should have data"
        );
    }

    /// Assert that a BaseResponse is an error
    pub fn assert_error<T>(response: &BaseResponse<T>, expected_code: i32) {
        assert_eq!(
            response.raw_response.code, expected_code,
            "Response should have expected error code"
        );
        assert_ne!(
            response.raw_response.code, 0,
            "Response should not be successful"
        );
        assert!(
            response.data.is_none(),
            "Error response should not have data"
        );
    }

    /// Assert that an error is of a specific type
    pub fn assert_error_type(error: &LarkAPIError, expected_variant: &str) {
        let error_name = match error {
            LarkAPIError::IOErr(_) => "IOErr",
            LarkAPIError::IllegalParamError(_) => "IllegalParamError",
            LarkAPIError::DeserializeError(_) => "DeserializeError",
            LarkAPIError::RequestError(_) => "RequestError",
            LarkAPIError::UrlParseError(_) => "UrlParseError",
            LarkAPIError::ApiError { .. } => "ApiError",
            LarkAPIError::MissingAccessToken => "MissingAccessToken",
            LarkAPIError::BadRequest(_) => "BadRequest",
            LarkAPIError::DataError(_) => "DataError",
            LarkAPIError::APIError { .. } => "APIError", // Legacy format
        };
        assert_eq!(
            error_name, expected_variant,
            "Error should be of expected type"
        );
    }

    /// Assert that a duration is within expected bounds
    pub fn assert_duration_bounds(duration: std::time::Duration, min_ms: u64, max_ms: u64) {
        let duration_ms = duration.as_millis() as u64;
        assert!(
            duration_ms >= min_ms && duration_ms <= max_ms,
            "Duration {}ms should be between {}ms and {}ms",
            duration_ms,
            min_ms,
            max_ms
        );
    }
}

/// Test data factories for common scenarios
pub struct TestDataFactory;

impl TestDataFactory {
    /// Create a list of mock API data for testing pagination, filtering, etc.
    pub fn create_api_data_list(count: usize) -> Vec<MockApiData> {
        (0..count)
            .map(|i| MockApiData {
                id: i as i32,
                name: format!("test_user_{}", i),
                email: Some(format!("user{}@example.com", i)),
                active: i % 2 == 0,
            })
            .collect()
    }

    /// Create binary test data of specified size
    pub fn create_binary_data(size_kb: usize) -> Vec<u8> {
        vec![0xAB; size_kb * 1024]
    }

    /// Create mock file content for different file types
    pub fn create_file_content(file_type: &str) -> Vec<u8> {
        match file_type {
            "txt" => b"This is a test text file.\nLine 2\nLine 3".to_vec(),
            "json" => br#"{"test": true, "value": 42}"#.to_vec(),
            "binary" => vec![0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A], // PNG header
            "empty" => vec![],
            _ => b"Unknown file type".to_vec(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mock_response_builder() {
        let response = MockResponseBuilder::success()
            .with_data(MockApiData {
                id: 1,
                name: "test".to_string(),
                email: Some("test@example.com".to_string()),
                active: true,
            })
            .build();

        TestAssertions::assert_success(&response);
        assert_eq!(response.data.as_ref().unwrap().id, 1);
    }

    #[test]
    fn test_error_response_builder() {
        let response: BaseResponse<MockApiData> =
            MockResponseBuilder::error(400, "Bad Request").build();
        TestAssertions::assert_error(&response, 400);
    }

    #[test]
    fn test_json_test_data() {
        let json = JsonTestData::success_response();
        assert!(json.contains("success"));
        assert!(json.contains("\"code\": 0"));

        let error_json = JsonTestData::error_response(400, "Bad Request");
        assert!(error_json.contains("400"));
        assert!(error_json.contains("Bad Request"));
    }

    #[test]
    fn test_performance_utils() {
        let (result, duration) = PerformanceTestUtils::measure_time(|| {
            std::thread::sleep(std::time::Duration::from_millis(10));
            42
        });

        assert_eq!(result, 42);
        TestAssertions::assert_duration_bounds(duration, 10, 50);
    }

    #[test]
    fn test_test_data_factory() {
        let data_list = TestDataFactory::create_api_data_list(5);
        assert_eq!(data_list.len(), 5);
        assert_eq!(data_list[0].id, 0);
        assert_eq!(data_list[4].id, 4);

        let binary_data = TestDataFactory::create_binary_data(1); // 1KB
        assert_eq!(binary_data.len(), 1024);

        let txt_content = TestDataFactory::create_file_content("txt");
        assert!(String::from_utf8(txt_content)
            .unwrap()
            .contains("test text file"));
    }

    #[test]
    fn test_header_test_data() {
        let headers = HeaderTestData::content_disposition_headers();
        assert!(!headers.is_empty());
        assert!(headers
            .iter()
            .any(|(header, _)| header.contains("filename")));

        let content_types = HeaderTestData::content_type_headers();
        assert!(!content_types.is_empty());
        assert!(content_types.contains(&"application/json"));
    }

    #[test]
    fn test_error_test_cases() {
        let api_errors = ErrorTestCases::api_errors();
        assert!(!api_errors.is_empty());
        assert!(api_errors.iter().any(|(code, _)| *code == 400));

        let lark_errors = ErrorTestCases::lark_errors();
        assert!(!lark_errors.is_empty());
        assert!(lark_errors.iter().any(|(code, _)| *code == 99991663));
    }
}

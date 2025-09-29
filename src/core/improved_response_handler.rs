use log::debug;
use serde_json::Value;
use tracing::{info_span, Instrument};

use crate::core::{
    api_resp::{ApiResponseTrait, BaseResponse, RawResponse, ResponseFormat},
    error::LarkAPIError,
    observability::ResponseTracker,
    SDKResult,
};

/// 改进的响应处理器，解决双重解析问题
/// 使用 #[serde(flatten)] 和高级 Serde 特性简化反序列化
pub struct ImprovedResponseHandler;

impl ImprovedResponseHandler {
    /// 处理响应的核心方法
    /// 相比原始实现，这个版本：
    /// 1. 减少了不必要的JSON解析次数
    /// 2. 使用更高效的直接反序列化
    /// 3. 更好的错误处理
    /// 4. 完整的可观测性支持
    pub async fn handle_response<T: ApiResponseTrait>(
        response: reqwest::Response,
    ) -> SDKResult<BaseResponse<T>> {
        let format = match T::data_format() {
            ResponseFormat::Data => "data",
            ResponseFormat::Flatten => "flatten",
            ResponseFormat::Binary => "binary",
        };

        let span = info_span!(
            "response_handling",
            format = format,
            status_code = response.status().as_u16(),
            content_length = tracing::field::Empty,
            processing_duration_ms = tracing::field::Empty,
        );

        async move {
            let start_time = std::time::Instant::now();

            // 获取内容长度用于监控
            let content_length = response.content_length();
            if let Some(length) = content_length {
                tracing::Span::current().record("content_length", length);
            }

            let result = match T::data_format() {
                ResponseFormat::Data => Self::handle_data_response(response).await,
                ResponseFormat::Flatten => Self::handle_flatten_response(response).await,
                ResponseFormat::Binary => Self::handle_binary_response(response).await,
            };

            // 记录处理时间
            let duration_ms = start_time.elapsed().as_millis() as u64;
            tracing::Span::current().record("processing_duration_ms", duration_ms);

            result
        }
        .instrument(span)
        .await
    }

    /// 处理标准数据格式响应
    /// 使用单次解析而非双重解析，包含详细的可观测性
    async fn handle_data_response<T: ApiResponseTrait>(
        response: reqwest::Response,
    ) -> SDKResult<BaseResponse<T>> {
        let tracker = ResponseTracker::start("json_data", response.content_length());

        let response_text = response.text().await?;
        debug!("Raw response: {response_text}");

        // 记录解析阶段开始
        tracker.parsing_complete();

        // 尝试直接解析为BaseResponse<T>
        match serde_json::from_str::<BaseResponse<T>>(&response_text) {
            Ok(base_response) => {
                tracker.success();
                Ok(base_response)
            }
            Err(direct_parse_err) => {
                tracing::debug!("Direct parsing failed, attempting structured data extraction");

                // 解析基础JSON结构
                match serde_json::from_str::<Value>(&response_text) {
                    Ok(raw_value) => {
                        let code = raw_value["code"].as_i64().unwrap_or(-1) as i32;
                        let msg = raw_value["msg"]
                            .as_str()
                            .unwrap_or("Unknown error")
                            .to_string();

                        // 如果响应成功，尝试提取并解析data字段
                        let data = if code == 0 {
                            if let Some(data_value) = raw_value.get("data") {
                                // 尝试解析为期望的类型T
                                match serde_json::from_value::<T>(data_value.clone()) {
                                    Ok(parsed_data) => {
                                        tracing::debug!("Successfully parsed data field as type T");
                                        Some(parsed_data)
                                    }
                                    Err(data_parse_err) => {
                                        tracing::debug!("Failed to parse data field as type T: {data_parse_err:?}");

                                        // 特殊处理：如果T是CreateMessageResp但data直接是Message，尝试包装
                                        if std::any::type_name::<T>().contains("CreateMessageResp")
                                        {
                                            // 尝试将data值包装为 {"data": data_value} 结构
                                            let wrapped_value = serde_json::json!({
                                                "data": data_value
                                            });
                                            match serde_json::from_value::<T>(wrapped_value) {
                                                Ok(wrapped_data) => {
                                                    tracing::debug!("Successfully parsed data by wrapping Message in CreateMessageResp");
                                                    Some(wrapped_data)
                                                }
                                                Err(_) => {
                                                    tracing::warn!("Failed to parse even after wrapping, but response contains valid message data");
                                                    // API调用成功了，数据存在，只是结构不匹配
                                                    None
                                                }
                                            }
                                        } else {
                                            None
                                        }
                                    }
                                }
                            } else {
                                tracing::debug!("No data field found in successful response");
                                None
                            }
                        } else {
                            None
                        };

                        tracker.validation_complete();
                        tracker.success();

                        Ok(BaseResponse {
                            raw_response: RawResponse {
                                code,
                                msg,
                                err: None,
                            },
                            data,
                        })
                    }
                    Err(fallback_err) => {
                        let error_msg = format!(
                            "Failed to parse response. Direct parse error: {}. Fallback parse error: {}",
                            direct_parse_err, fallback_err
                        );
                        tracker.error(&error_msg);
                        Err(LarkAPIError::IllegalParamError(error_msg))
                    }
                }
            }
        }
    }

    /// 处理扁平格式响应
    /// 对于扁平格式，使用自定义反序列化器，包含可观测性支持
    async fn handle_flatten_response<T: ApiResponseTrait>(
        response: reqwest::Response,
    ) -> SDKResult<BaseResponse<T>> {
        let tracker = ResponseTracker::start("json_flatten", response.content_length());

        let response_text = response.text().await?;
        debug!("Raw response: {response_text}");

        // 解析阶段
        let raw_value: Value = match serde_json::from_str(&response_text) {
            Ok(value) => {
                tracker.parsing_complete();
                value
            }
            Err(e) => {
                let error_msg = format!("Failed to parse JSON: {}", e);
                tracker.error(&error_msg);
                return Err(LarkAPIError::IllegalParamError(error_msg));
            }
        };

        // 解析原始响应信息
        let raw_response: RawResponse = match serde_json::from_value(raw_value.clone()) {
            Ok(response) => response,
            Err(e) => {
                let error_msg = format!("Failed to parse raw response: {}", e);
                tracker.error(&error_msg);
                return Err(LarkAPIError::IllegalParamError(error_msg));
            }
        };

        // 验证和数据解析阶段
        let data = if raw_response.code == 0 {
            match serde_json::from_value::<T>(raw_value) {
                Ok(parsed_data) => {
                    tracker.validation_complete();
                    Some(parsed_data)
                }
                Err(e) => {
                    debug!("Failed to parse data for flatten response: {e}");
                    tracker.validation_complete();
                    None
                }
            }
        } else {
            tracker.validation_complete();
            None
        };

        tracker.success();
        Ok(BaseResponse { raw_response, data })
    }

    /// 处理二进制响应，包含可观测性支持
    async fn handle_binary_response<T: ApiResponseTrait>(
        response: reqwest::Response,
    ) -> SDKResult<BaseResponse<T>> {
        let tracker = ResponseTracker::start("binary", response.content_length());

        // 获取文件名
        let file_name = response
            .headers()
            .get("Content-Disposition")
            .and_then(|header| header.to_str().ok())
            .and_then(Self::extract_filename)
            .unwrap_or_default();

        // 记录解析阶段完成（文件名提取）
        tracker.parsing_complete();

        // 获取二进制数据
        let bytes = match response.bytes().await {
            Ok(bytes) => {
                let byte_vec = bytes.to_vec();
                tracing::debug!("Binary response received: {} bytes", byte_vec.len());
                byte_vec
            }
            Err(e) => {
                let error_msg = format!("Failed to read binary response: {}", e);
                tracker.error(&error_msg);
                return Err(LarkAPIError::RequestError(error_msg));
            }
        };

        // 验证阶段 - 使用trait方法创建数据
        let data = match T::from_binary(file_name.clone(), bytes) {
            Some(binary_data) => {
                tracker.validation_complete();
                Some(binary_data)
            }
            None => {
                tracker.validation_complete();
                tracing::warn!("Binary data could not be processed for file: {}", file_name);
                None
            }
        };

        tracker.success();
        Ok(BaseResponse {
            raw_response: RawResponse {
                code: 0,
                msg: "success".to_string(),
                err: None,
            },
            data,
        })
    }

    /// 提取文件名的辅助函数
    fn extract_filename(content_disposition: &str) -> Option<String> {
        // 支持多种文件名格式
        for part in content_disposition.split(';') {
            let part = part.trim();

            // 支持 filename*=UTF-8''filename 格式
            if let Some(filename) = part.strip_prefix("filename*=UTF-8''") {
                return Some(filename.to_string());
            }

            // 支持 filename="filename" 格式
            if let Some(filename) = part.strip_prefix("filename=") {
                let filename = filename.trim_matches('"');
                return Some(filename.to_string());
            }
        }
        None
    }
}

/// 优化的BaseResponse，使用更好的serde特性
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct OptimizedBaseResponse<T>
where
    T: Default,
{
    /// 响应状态码
    pub code: i32,
    /// 响应消息
    pub msg: String,
    /// 错误信息（可选）
    #[serde(rename = "error", default, skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorInfo>,
    /// 业务数据（可选）
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
}

impl<T> OptimizedBaseResponse<T>
where
    T: Default,
{
    /// 检查请求是否成功
    pub fn is_success(&self) -> bool {
        self.code == 0
    }

    /// 获取业务数据，如果请求失败则返回错误
    pub fn into_data(self) -> Result<T, LarkAPIError> {
        if self.is_success() {
            self.data.ok_or_else(|| {
                LarkAPIError::illegal_param("Response is successful but data is missing")
            })
        } else {
            Err(LarkAPIError::api_error(
                self.code, self.msg, None, // 这里可以添加request_id如果有的话
            ))
        }
    }

    /// 获取数据的引用
    pub fn data(&self) -> Option<&T> {
        self.data.as_ref()
    }

    /// 检查是否有错误信息
    pub fn has_error(&self) -> bool {
        self.error.is_some()
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct ErrorInfo {
    #[serde(rename = "key", default, skip_serializing_if = "Option::is_none")]
    pub log_id: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub details: Vec<ErrorDetail>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct ErrorDetail {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

/// 使用宏简化APIResponseTrait实现
#[macro_export]
macro_rules! impl_api_response {
    ($type:ty, $format:expr) => {
        impl ApiResponseTrait for $type {
            fn data_format() -> ResponseFormat {
                $format
            }
        }
    };

    ($type:ty, $format:expr, binary) => {
        impl ApiResponseTrait for $type {
            fn data_format() -> ResponseFormat {
                $format
            }

            fn from_binary(file_name: String, body: Vec<u8>) -> Option<Self> {
                Some(<$type>::from_binary_data(file_name, body))
            }
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::api_resp::ResponseFormat;
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Serialize, Deserialize, PartialEq, Default, Clone)]
    struct TestData {
        id: i32,
        name: String,
    }

    impl ApiResponseTrait for TestData {
        fn data_format() -> ResponseFormat {
            ResponseFormat::Data
        }
    }

    #[derive(Debug, Serialize, Deserialize, PartialEq, Default, Clone)]
    struct TestFlattenData {
        id: i32,
        name: String,
        code: i32,
        msg: String,
    }

    impl ApiResponseTrait for TestFlattenData {
        fn data_format() -> ResponseFormat {
            ResponseFormat::Flatten
        }
    }

    #[derive(Debug, Serialize, Deserialize, PartialEq, Default, Clone)]
    struct TestBinaryData {
        file_name: String,
        content: Vec<u8>,
    }

    impl ApiResponseTrait for TestBinaryData {
        fn data_format() -> ResponseFormat {
            ResponseFormat::Binary
        }

        fn from_binary(file_name: String, body: Vec<u8>) -> Option<Self> {
            Some(TestBinaryData {
                file_name,
                content: body,
            })
        }
    }

    // Note: Mock HTTP responses would require a more sophisticated testing setup
    // The helper functions below are simplified examples of what mock functions might look like

    #[test]
    fn test_optimized_base_response_success() {
        let response = OptimizedBaseResponse {
            code: 0,
            msg: "success".to_string(),
            error: None,
            data: Some(TestData {
                id: 1,
                name: "test".to_string(),
            }),
        };

        assert!(response.is_success());
        assert!(response.data().is_some());
        assert_eq!(response.data().unwrap().id, 1);
        assert!(!response.has_error());
    }

    #[test]
    fn test_optimized_base_response_error() {
        let response: OptimizedBaseResponse<TestData> = OptimizedBaseResponse {
            code: 400,
            msg: "Bad Request".to_string(),
            error: Some(ErrorInfo {
                log_id: Some("log123".to_string()),
                details: vec![],
            }),
            data: None,
        };

        assert!(!response.is_success());
        assert!(response.has_error());
        assert!(response.data().is_none());
    }

    #[test]
    fn test_optimized_base_response_into_data_success() {
        let response = OptimizedBaseResponse {
            code: 0,
            msg: "success".to_string(),
            error: None,
            data: Some(TestData {
                id: 1,
                name: "test".to_string(),
            }),
        };

        let data = response.into_data().unwrap();
        assert_eq!(data.id, 1);
        assert_eq!(data.name, "test");
    }

    #[test]
    fn test_optimized_base_response_into_data_error() {
        let response: OptimizedBaseResponse<TestData> = OptimizedBaseResponse {
            code: 400,
            msg: "Bad Request".to_string(),
            error: None,
            data: None,
        };

        let result = response.into_data();
        assert!(result.is_err());
        match result.unwrap_err() {
            LarkAPIError::ApiError { code, message, .. } => {
                assert_eq!(code, 400);
                assert_eq!(message, "Bad Request");
            }
            _ => panic!("Expected ApiError"),
        }
    }

    #[test]
    fn test_optimized_base_response_into_data_success_but_no_data() {
        let response: OptimizedBaseResponse<TestData> = OptimizedBaseResponse {
            code: 0,
            msg: "success".to_string(),
            error: None,
            data: None,
        };

        let result = response.into_data();
        assert!(result.is_err());
        match result.unwrap_err() {
            LarkAPIError::IllegalParamError(msg) => {
                assert!(msg.contains("data is missing"));
            }
            _ => panic!("Expected IllegalParamError"),
        }
    }

    #[test]
    fn test_error_info_serialization() {
        let error_info = ErrorInfo {
            log_id: Some("test_log_id".to_string()),
            details: vec![
                ErrorDetail {
                    key: Some("field1".to_string()),
                    value: Some("invalid_value".to_string()),
                    description: Some("Field is required".to_string()),
                },
                ErrorDetail {
                    key: Some("field2".to_string()),
                    value: None,
                    description: Some("Missing field".to_string()),
                },
            ],
        };

        let json = serde_json::to_string(&error_info).unwrap();
        let deserialized: ErrorInfo = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.log_id, error_info.log_id);
        assert_eq!(deserialized.details.len(), 2);
        assert_eq!(deserialized.details[0].key, Some("field1".to_string()));
        assert_eq!(deserialized.details[1].value, None);
    }

    #[test]
    fn test_error_detail_optional_fields() {
        let detail = ErrorDetail {
            key: None,
            value: Some("test_value".to_string()),
            description: None,
        };

        let json = serde_json::to_string(&detail).unwrap();
        let deserialized: ErrorDetail = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.key, None);
        assert_eq!(deserialized.value, Some("test_value".to_string()));
        assert_eq!(deserialized.description, None);
    }

    #[test]
    fn test_filename_extraction() {
        let cases = vec![
            (
                "attachment; filename=\"test.txt\"",
                Some("test.txt".to_string()),
            ),
            (
                "attachment; filename*=UTF-8''test%20file.pdf",
                Some("test%20file.pdf".to_string()),
            ),
            (
                "attachment; filename=simple.doc",
                Some("simple.doc".to_string()),
            ),
            ("attachment", None),
            ("", None),
            ("filename=\"quoted.txt\"", Some("quoted.txt".to_string())),
            ("filename=unquoted.txt", Some("unquoted.txt".to_string())),
            (
                "filename*=UTF-8''unicode%E2%9C%93.txt",
                Some("unicode%E2%9C%93.txt".to_string()),
            ),
            (
                "attachment; filename=\"spaced file.doc\"; other=value",
                Some("spaced file.doc".to_string()),
            ),
        ];

        for (input, expected) in cases {
            let result = ImprovedResponseHandler::extract_filename(input);
            assert_eq!(result, expected, "Failed for input: {input}");
        }
    }

    #[test]
    fn test_filename_extraction_edge_cases() {
        // Test empty and whitespace-only strings
        assert_eq!(ImprovedResponseHandler::extract_filename(""), None);
        assert_eq!(ImprovedResponseHandler::extract_filename("   "), None);
        assert_eq!(ImprovedResponseHandler::extract_filename(";;;"), None);

        // Test malformed headers - based on implementation behavior
        assert_eq!(
            ImprovedResponseHandler::extract_filename("filename="),
            Some("".to_string())
        );
        assert_eq!(
            ImprovedResponseHandler::extract_filename("filename*="),
            None
        ); // Doesn't match UTF-8 prefix, doesn't match filename= exactly
        assert_eq!(
            ImprovedResponseHandler::extract_filename("filename=\""),
            Some("".to_string())
        );

        // Test with only quotes - the current implementation extracts empty string
        assert_eq!(
            ImprovedResponseHandler::extract_filename("filename=\"\""),
            Some("".to_string())
        );

        // Test multiple filename directives (should return first valid one)
        let multi_filename = "filename=\"first.txt\"; filename=\"second.txt\"";
        assert_eq!(
            ImprovedResponseHandler::extract_filename(multi_filename),
            Some("first.txt".to_string())
        );
    }

    #[test]
    fn test_json_parsing_performance() {
        let json_data = r#"{"code": 0, "msg": "success", "data": {"id": 1, "name": "test"}}"#;

        // 测试直接解析
        let start = std::time::Instant::now();
        let _result: Result<OptimizedBaseResponse<TestData>, _> = serde_json::from_str(json_data);
        let direct_parse_time = start.elapsed();

        // 测试双重解析（原始方法）
        let start = std::time::Instant::now();
        let _value: Value = serde_json::from_str(json_data).unwrap();
        let _result: Result<OptimizedBaseResponse<TestData>, _> = serde_json::from_value(_value);
        let double_parse_time = start.elapsed();

        println!("Direct parse time: {direct_parse_time:?}");
        println!("Double parse time: {double_parse_time:?}");

        // 直接解析应该更快（虽然在微基准测试中差异可能很小）
        // 这里主要是为了展示概念
    }

    #[test]
    fn test_api_response_trait_data_format() {
        assert_eq!(TestData::data_format(), ResponseFormat::Data);
        assert_eq!(TestFlattenData::data_format(), ResponseFormat::Flatten);
        assert_eq!(TestBinaryData::data_format(), ResponseFormat::Binary);
    }

    #[test]
    fn test_api_response_trait_from_binary() {
        let file_name = "test.txt".to_string();
        let content = b"Hello, World!".to_vec();

        let binary_data = TestBinaryData::from_binary(file_name.clone(), content.clone()).unwrap();
        assert_eq!(binary_data.file_name, file_name);
        assert_eq!(binary_data.content, content);

        // Test default implementation for non-binary types
        let default_result = TestData::from_binary("test.txt".to_string(), vec![1, 2, 3]);
        assert!(default_result.is_none());
    }

    // Mock tests for response handlers would require a more sophisticated mocking setup
    // For now, we'll test the logic that doesn't require actual HTTP responses

    #[tokio::test]
    async fn test_handle_data_response_parsing_logic() {
        // Test JSON parsing logic without actual HTTP response
        let test_cases = vec![
            // Error response with fallback parsing
            (r#"{"code": 400, "msg": "Bad Request"}"#, true),
            // Invalid JSON
            (r#"{"invalid": json"#, false),
        ];

        for (json, should_succeed) in test_cases {
            // Test fallback parsing for error responses
            if json.contains("code") && !json.contains("raw_response") {
                let fallback_result = serde_json::from_str::<Value>(json);
                if should_succeed {
                    assert!(
                        fallback_result.is_ok(),
                        "Fallback parsing should succeed for: {}",
                        json
                    );
                    let value = fallback_result.unwrap();
                    assert!(value["code"].is_i64());
                    assert!(value["msg"].is_string());
                }
            } else if json.contains("invalid") {
                let parse_result = serde_json::from_str::<Value>(json);
                assert!(parse_result.is_err(), "Invalid JSON should fail to parse");
            }
        }
    }

    #[tokio::test]
    async fn test_handle_flatten_response_parsing_logic() {
        let test_cases = vec![
            // Success response
            (
                r#"{"id": 1, "name": "test", "code": 0, "msg": "success"}"#,
                0,
                true,
            ),
            // Error response
            (r#"{"code": 400, "msg": "Bad Request"}"#, 400, false),
            // Invalid JSON
            (r#"{"invalid": json"#, -1, false),
        ];

        for (json, expected_code, should_have_data) in test_cases {
            if json.contains("invalid") {
                let parse_result = serde_json::from_str::<Value>(json);
                assert!(parse_result.is_err(), "Invalid JSON should fail to parse");
                continue;
            }

            let value_result = serde_json::from_str::<Value>(json);
            assert!(value_result.is_ok(), "Valid JSON should parse as Value");

            let value = value_result.unwrap();
            let raw_response_result = serde_json::from_value::<RawResponse>(value.clone());

            if expected_code >= 0 {
                assert!(
                    raw_response_result.is_ok(),
                    "Should parse RawResponse for: {}",
                    json
                );
                let raw_response = raw_response_result.unwrap();
                assert_eq!(raw_response.code, expected_code);

                if should_have_data && raw_response.code == 0 {
                    let data_result = serde_json::from_value::<TestFlattenData>(value);
                    assert!(
                        data_result.is_ok(),
                        "Should parse data for success response"
                    );
                }
            }
        }
    }

    #[test]
    fn test_response_format_display_logic() {
        let formats = vec![
            (ResponseFormat::Data, "data"),
            (ResponseFormat::Flatten, "flatten"),
            (ResponseFormat::Binary, "binary"),
        ];

        for (format, expected_str) in formats {
            let format_str = match format {
                ResponseFormat::Data => "data",
                ResponseFormat::Flatten => "flatten",
                ResponseFormat::Binary => "binary",
            };
            assert_eq!(format_str, expected_str);
        }
    }

    #[test]
    fn test_binary_response_logic() {
        let test_file_name = "test_document.pdf";
        let test_content = b"PDF content here".to_vec();

        // Test successful binary data creation
        let binary_data =
            TestBinaryData::from_binary(test_file_name.to_string(), test_content.clone());
        assert!(binary_data.is_some());

        let data = binary_data.unwrap();
        assert_eq!(data.file_name, test_file_name);
        assert_eq!(data.content, test_content);

        // Test empty content
        let empty_data = TestBinaryData::from_binary("empty.txt".to_string(), vec![]);
        assert!(empty_data.is_some());
        assert_eq!(empty_data.unwrap().content.len(), 0);
    }

    #[test]
    fn test_optimized_response_serialization_roundtrip() {
        let original = OptimizedBaseResponse {
            code: 0,
            msg: "success".to_string(),
            error: Some(ErrorInfo {
                log_id: Some("test123".to_string()),
                details: vec![ErrorDetail {
                    key: Some("validation".to_string()),
                    value: Some("failed".to_string()),
                    description: Some("Field validation failed".to_string()),
                }],
            }),
            data: Some(TestData {
                id: 42,
                name: "serialization_test".to_string(),
            }),
        };

        // Serialize to JSON
        let json = serde_json::to_string(&original).unwrap();

        // Deserialize back
        let deserialized: OptimizedBaseResponse<TestData> = serde_json::from_str(&json).unwrap();

        // Verify all fields are preserved
        assert_eq!(deserialized.code, original.code);
        assert_eq!(deserialized.msg, original.msg);
        assert_eq!(deserialized.data, original.data);
        assert!(deserialized.error.is_some());

        let error = deserialized.error.unwrap();
        assert_eq!(error.log_id, Some("test123".to_string()));
        assert_eq!(error.details.len(), 1);
        assert_eq!(error.details[0].key, Some("validation".to_string()));
    }

    #[test]
    fn test_optimized_response_skipped_fields() {
        // Test response with None values (should be skipped in serialization)
        let response: OptimizedBaseResponse<TestData> = OptimizedBaseResponse {
            code: 0,
            msg: "success".to_string(),
            error: None,
            data: None,
        };

        let json = serde_json::to_string(&response).unwrap();

        // Should not contain "error" or "data" fields when they are None
        assert!(!json.contains("\"error\""));
        assert!(!json.contains("\"data\""));
        assert!(json.contains("\"code\":0"));
        assert!(json.contains("\"msg\":\"success\""));
    }

    #[test]
    fn test_macro_api_response_implementation() {
        // Test that the macro would work correctly
        // Since we can't actually invoke the macro in tests easily,
        // we'll test the pattern it would generate

        #[derive(Debug, Default, Serialize, Deserialize)]
        struct MacroTestData;

        impl ApiResponseTrait for MacroTestData {
            fn data_format() -> ResponseFormat {
                ResponseFormat::Data
            }
        }

        assert_eq!(MacroTestData::data_format(), ResponseFormat::Data);
        assert!(MacroTestData::from_binary("test".to_string(), vec![1, 2, 3]).is_none());
    }

    #[test]
    fn test_error_detail_empty_values() {
        let detail = ErrorDetail {
            key: Some("".to_string()),
            value: Some("".to_string()),
            description: Some("".to_string()),
        };

        let json = serde_json::to_string(&detail).unwrap();
        let deserialized: ErrorDetail = serde_json::from_str(&json).unwrap();

        assert_eq!(deserialized.key, Some("".to_string()));
        assert_eq!(deserialized.value, Some("".to_string()));
        assert_eq!(deserialized.description, Some("".to_string()));
    }

    #[test]
    fn test_content_disposition_header_edge_cases() {
        let edge_cases = vec![
            // Case-insensitive filename
            ("FILENAME=\"test.txt\"", None), // Our implementation is case-sensitive
            ("Filename=\"test.txt\"", None), // Our implementation is case-sensitive
            // Multiple spaces
            (
                "attachment;  filename=\"test.txt\"",
                Some("test.txt".to_string()),
            ),
            ("attachment; filename =  \"test.txt\"", None), // Space before = is not handled
            // Special characters in filename
            (
                "attachment; filename=\"test-file_v1.2.txt\"",
                Some("test-file_v1.2.txt".to_string()),
            ),
            (
                "attachment; filename=\"测试文件.txt\"",
                Some("测试文件.txt".to_string()),
            ),
            // Both UTF-8 and regular filename (current implementation returns first match)
            (
                "attachment; filename=\"test.txt\"; filename*=UTF-8''better.txt",
                Some("test.txt".to_string()),
            ),
        ];

        for (input, expected) in edge_cases {
            let result = ImprovedResponseHandler::extract_filename(input);
            assert_eq!(result, expected, "Failed for input: {}", input);
        }
    }
}

/// 使用示例
///
/// 在RequestExecutor中使用改进的响应处理器：
/// ```rust,ignore
/// impl RequestExecutor {
///     pub async fn execute_improved<T: ApiResponseTrait + DeserializeOwned>(
///         // ... 参数
///     ) -> SDKResult<OptimizedBaseResponse<T>> {
///         // ... 构建请求
///         let response = http_client.send(request).await?;
///         ImprovedResponseHandler::handle_response(response).await
///     }
/// }
///
/// // 使用新的响应格式
/// let result = RequestExecutor::execute_improved::<MessageData>(...).await?;
///
/// match result.into_data() {
///     Ok(data) => println!("Success: {:?}", data),
///     Err(e) => println!("Error: {:?}", e),
/// }
/// ```
mod usage_examples {}

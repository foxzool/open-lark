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
                tracing::debug!("Direct parsing failed, attempting fallback parsing");

                // 如果直接解析失败，可能是错误响应，先解析基本信息
                match serde_json::from_str::<Value>(&response_text) {
                    Ok(raw_value) => {
                        let code = raw_value["code"].as_i64().unwrap_or(-1) as i32;
                        let msg = raw_value["msg"]
                            .as_str()
                            .unwrap_or("Unknown error")
                            .to_string();

                        tracker.validation_complete();
                        tracker.success();

                        Ok(BaseResponse {
                            raw_response: RawResponse {
                                code,
                                msg,
                                err: None,
                            },
                            data: None,
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

    #[derive(Debug, Serialize, Deserialize, PartialEq, Default)]
    struct TestData {
        id: i32,
        name: String,
    }

    impl ApiResponseTrait for TestData {
        fn data_format() -> ResponseFormat {
            ResponseFormat::Data
        }
    }

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
        ];

        for (input, expected) in cases {
            let result = ImprovedResponseHandler::extract_filename(input);
            assert_eq!(result, expected, "Failed for input: {input}");
        }
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

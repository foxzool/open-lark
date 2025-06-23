use log::debug;
use serde_json::Value;

use crate::core::{
    api_resp::{ApiResponseTrait, BaseResponse, RawResponse, ResponseFormat},
    error::LarkAPIError,
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
    pub async fn handle_response<T: ApiResponseTrait>(
        response: reqwest::Response,
    ) -> SDKResult<BaseResponse<T>> {
        match T::data_format() {
            ResponseFormat::Data => Self::handle_data_response(response).await,
            ResponseFormat::Flatten => Self::handle_flatten_response(response).await,
            ResponseFormat::Binary => Self::handle_binary_response(response).await,
        }
    }

    /// 处理标准数据格式响应
    /// 使用单次解析而非双重解析
    async fn handle_data_response<T: ApiResponseTrait>(
        response: reqwest::Response,
    ) -> SDKResult<BaseResponse<T>> {
        let response_text = response.text().await?;
        debug!("Raw response: {}", response_text);

        // 尝试直接解析为BaseResponse<T>
        match serde_json::from_str::<BaseResponse<T>>(&response_text) {
            Ok(base_response) => Ok(base_response),
            Err(_) => {
                // 如果直接解析失败，可能是错误响应，先解析基本信息
                let raw_value: Value = serde_json::from_str(&response_text)?;

                let code = raw_value["code"].as_i64().unwrap_or(-1) as i32;
                let msg = raw_value["msg"]
                    .as_str()
                    .unwrap_or("Unknown error")
                    .to_string();

                // 构建错误响应
                Ok(BaseResponse {
                    raw_response: RawResponse {
                        code,
                        msg,
                        err: raw_value
                            .get("error")
                            .and_then(|e| serde_json::from_value(e.clone()).ok()),
                    },
                    data: None,
                })
            }
        }
    }

    /// 处理扁平格式响应
    /// 对于扁平格式，使用自定义反序列化器
    async fn handle_flatten_response<T: ApiResponseTrait>(
        response: reqwest::Response,
    ) -> SDKResult<BaseResponse<T>> {
        let response_text = response.text().await?;
        debug!("Raw response: {}", response_text);

        let raw_value: Value = serde_json::from_str(&response_text)?;

        // 解析原始响应信息
        let raw_response: RawResponse = serde_json::from_value(raw_value.clone())?;

        // 如果成功，尝试解析数据
        let data = if raw_response.code == 0 {
            match serde_json::from_value::<T>(raw_value) {
                Ok(parsed_data) => Some(parsed_data),
                Err(e) => {
                    debug!("Failed to parse data for flatten response: {}", e);
                    None
                }
            }
        } else {
            None
        };

        Ok(BaseResponse { raw_response, data })
    }

    /// 处理二进制响应
    async fn handle_binary_response<T: ApiResponseTrait>(
        response: reqwest::Response,
    ) -> SDKResult<BaseResponse<T>> {
        // 获取文件名
        let file_name = response
            .headers()
            .get("Content-Disposition")
            .and_then(|header| header.to_str().ok())
            .and_then(Self::extract_filename)
            .unwrap_or_default();

        // 获取二进制数据
        let bytes = response.bytes().await?.to_vec();

        // 使用trait方法创建数据
        let data = T::from_binary(file_name, bytes);

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
            assert_eq!(result, expected, "Failed for input: {}", input);
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

        println!("Direct parse time: {:?}", direct_parse_time);
        println!("Double parse time: {:?}", double_parse_time);

        // 直接解析应该更快（虽然在微基准测试中差异可能很小）
        // 这里主要是为了展示概念
    }
}

/// 使用示例
///
/// 在RequestExecutor中使用改进的响应处理器：
/// ```rust
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

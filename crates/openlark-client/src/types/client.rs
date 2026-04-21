//! OpenLark Client 核心类型定义
//!
//! 包含客户端相关的核心类型

use serde::Deserialize;
use std::time::Duration;

/// 📄 API响应特征
///
/// 所有API响应都应该实现此特征
pub trait ApiResponse: for<'de> Deserialize<'de> + Send + Sync + 'static {
    /// 🔍 检查响应是否成功
    fn is_success(&self) -> bool;

    /// 📝 获取错误消息
    fn error_message(&self) -> Option<&String>;

    /// 🔄 转换为Result类型
    fn into_result(self) -> Result<Self, crate::Error>;
}

/// 📄 API响应包装器
///
/// 统一的API响应格式
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct ApiResponseData<T> {
    /// 📊 响应数据
    pub data: T,
    /// ✅ 请求是否成功
    pub success: bool,
    /// 📝 响应消息
    pub message: Option<String>,
    /// 🏷️ 请求ID
    pub request_id: String,
    /// ⏱️ 响应时间戳
    pub timestamp: Option<i64>,
    /// 📋 额外的元数据
    pub extra: std::collections::HashMap<String, serde_json::Value>,
}

impl<T> ApiResponseData<T> {
    /// 🆕 创建成功响应
    pub fn success(data: T) -> Self {
        Self {
            data,
            success: true,
            message: None,
            request_id: uuid::Uuid::new_v4().to_string(),
            timestamp: Some(chrono::Utc::now().timestamp()),
            extra: std::collections::HashMap::new(),
        }
    }

    /// 🆕 创建错误响应（需要 `T: Default`）
    ///
    /// 注意：
    /// - 旧实现曾使用 `mem::zeroed()` 为泛型 `T` 构造占位值，这是不安全且可能导致 UB 的。
    /// - 若 `T` 无法提供默认值，请使用 `error_with_data` 显式传入 `data`。
    pub fn error(message: impl Into<String>) -> Self
    where
        T: Default,
    {
        Self {
            data: T::default(),
            success: false,
            message: Some(message.into()),
            request_id: uuid::Uuid::new_v4().to_string(),
            timestamp: Some(chrono::Utc::now().timestamp()),
            extra: std::collections::HashMap::new(),
        }
    }

    /// 🆕 创建错误响应（显式传入 `data`，避免对 `T` 施加额外约束）
    pub fn error_with_data(data: T, message: impl Into<String>) -> Self {
        Self {
            data,
            success: false,
            message: Some(message.into()),
            request_id: uuid::Uuid::new_v4().to_string(),
            timestamp: Some(chrono::Utc::now().timestamp()),
            extra: std::collections::HashMap::new(),
        }
    }

    /// 🔍 检查响应是否成功
    pub fn is_success(&self) -> bool {
        self.success
    }

    /// 📝 获取错误消息
    pub fn error_message(&self) -> Option<&String> {
        self.message.as_ref()
    }

    /// 🔄 转换为Result类型
    pub fn into_result(self) -> Result<T, crate::Error> {
        if self.success {
            Ok(self.data)
        } else {
            Err(crate::error::api_error(
                500,
                "response",
                self.message.unwrap_or_default(),
                None,
            ))
        }
    }
}

impl<T: serde::de::DeserializeOwned + Send + Sync + 'static> ApiResponse for ApiResponseData<T> {
    fn is_success(&self) -> bool {
        self.success
    }

    fn error_message(&self) -> Option<&String> {
        self.message.as_ref()
    }

    fn into_result(self) -> Result<Self, crate::Error> {
        if self.success {
            Ok(self)
        } else {
            Err(crate::error::api_error(
                500,
                "response",
                self.message.unwrap_or_default(),
                None,
            ))
        }
    }
}

/// 📋 分页响应
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct PaginatedResponse<T> {
    /// 📄 数据项
    pub items: Vec<T>,
    /// 🔄 是否有更多数据
    pub has_more: bool,
    /// 📄 分页token
    pub page_token: Option<String>,
    /// 📊 总数（如果可用）
    pub total: Option<u64>,
}

impl<T> PaginatedResponse<T> {
    /// 🆕 创建新的分页响应
    pub fn new(items: Vec<T>) -> Self {
        Self {
            items,
            has_more: false,
            page_token: None,
            total: None,
        }
    }

    /// 🆕 创建带分页的响应
    pub fn with_pagination(items: Vec<T>, has_more: bool, page_token: Option<String>) -> Self {
        Self {
            items,
            has_more,
            page_token,
            total: None,
        }
    }

    /// 📊 获取项目数量
    pub fn len(&self) -> usize {
        self.items.len()
    }

    /// 🔍 检查是否为空
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
}

/// 📋 请求选项
#[derive(Debug, Clone, Default)]
pub struct RequestOptions {
    /// ⏱️ 超时时间
    pub timeout: Option<Duration>,
    /// 🔄 重试次数
    pub retry_count: Option<u32>,
    /// 📝 自定义头部
    pub headers: Option<std::collections::HashMap<String, String>>,
}

impl RequestOptions {
    /// 🆕 创建默认请求选项
    pub fn new() -> Self {
        Self::default()
    }

    /// ⏱️ 设置超时时间
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = Some(timeout);
        self
    }

    /// 🔄 设置重试次数
    pub fn retry_count(mut self, count: u32) -> Self {
        self.retry_count = Some(count);
        self
    }

    /// 📝 添加自定义头部
    pub fn header(mut self, key: String, value: String) -> Self {
        self.headers
            .get_or_insert_with(std::collections::HashMap::new)
            .insert(key, value);
        self
    }
}

#[cfg(test)]
#[allow(unused_imports)]
mod tests {
    use super::*;

    #[test]
    fn test_api_response_data() {
        // 直接构造响应数据，避免使用可能有问题的方法
        let response = ApiResponseData {
            data: "test data".to_string(),
            success: true,
            message: None,
            request_id: "test-request-123".to_string(),
            timestamp: Some(1640995200),
            extra: std::collections::HashMap::new(),
        };

        assert!(response.is_success());
        assert_eq!(response.data, "test data");

        let error_response = ApiResponseData {
            data: String::new(),
            success: false,
            message: Some("测试错误".to_string()),
            request_id: "test-request-456".to_string(),
            timestamp: Some(1640995200),
            extra: std::collections::HashMap::new(),
        };

        assert!(!error_response.is_success());
        assert_eq!(
            error_response.error_message(),
            Some(&"测试错误".to_string())
        );
    }

    #[test]
    fn test_paginated_response() {
        let items = vec!["item1", "item2", "item3"];
        let response = PaginatedResponse::new(items.clone());

        assert_eq!(response.len(), 3);
        assert!(!response.has_more);
        assert!(response.page_token.is_none());

        let paginated =
            PaginatedResponse::with_pagination(items.clone(), true, Some("next_token".to_string()));
        assert!(paginated.has_more);
        assert_eq!(paginated.page_token, Some("next_token".to_string()));
    }

    #[test]
    fn test_request_options() {
        let options = RequestOptions::new()
            .timeout(Duration::from_secs(30))
            .retry_count(3)
            .header("User-Agent".to_string(), "test-client".to_string());

        assert_eq!(options.timeout, Some(Duration::from_secs(30)));
        assert_eq!(options.retry_count, Some(3));
        assert!(options.headers.is_some());
    }

    #[test]
    fn test_api_response_data_success() {
        let response: ApiResponseData<i32> = ApiResponseData::success(42);
        assert!(response.is_success());
        assert_eq!(response.data, 42);
        assert!(response.error_message().is_none());
        assert!(!response.request_id.is_empty());
    }

    #[test]
    fn test_api_response_data_error() {
        let response: ApiResponseData<String> = ApiResponseData::error("发生错误");
        assert!(!response.is_success());
        assert_eq!(response.error_message(), Some(&"发生错误".to_string()));
        assert!(!response.request_id.is_empty());
    }

    #[test]
    fn test_api_response_data_error_with_data() {
        let response = ApiResponseData::error_with_data(123, "操作失败");
        assert!(!response.is_success());
        assert_eq!(response.data, 123);
        assert_eq!(response.error_message(), Some(&"操作失败".to_string()));
    }

    #[test]
    fn test_api_response_data_into_result_success() {
        let response: ApiResponseData<i32> = ApiResponseData {
            data: 42,
            success: true,
            message: None,
            request_id: "test".to_string(),
            timestamp: None,
            extra: std::collections::HashMap::new(),
        };
        let result = response.into_result();
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 42);
    }

    #[test]
    fn test_api_response_data_into_result_error() {
        let response: ApiResponseData<i32> = ApiResponseData {
            data: 0,
            success: false,
            message: Some("出错了".to_string()),
            request_id: "test".to_string(),
            timestamp: None,
            extra: std::collections::HashMap::new(),
        };
        let result = response.into_result();
        assert!(result.is_err());
    }

    #[test]
    fn test_api_response_trait() {
        let response: ApiResponseData<String> = ApiResponseData {
            data: "test".to_string(),
            success: true,
            message: None,
            request_id: "req-123".to_string(),
            timestamp: None,
            extra: std::collections::HashMap::new(),
        };
        assert!(response.is_success());
        assert!(response.error_message().is_none());
    }

    #[test]
    fn test_api_response_trait_error() {
        let response: ApiResponseData<String> = ApiResponseData {
            data: String::new(),
            success: false,
            message: Some("错误信息".to_string()),
            request_id: "req-456".to_string(),
            timestamp: None,
            extra: std::collections::HashMap::new(),
        };
        assert!(!response.is_success());
        assert_eq!(response.error_message(), Some(&"错误信息".to_string()));
    }

    #[test]
    fn test_paginated_response_empty() {
        let response: PaginatedResponse<String> = PaginatedResponse::new(vec![]);
        assert!(response.is_empty());
        assert_eq!(response.len(), 0);
        assert!(!response.has_more);
    }

    #[test]
    fn test_paginated_response_with_total() {
        let items = vec!["a", "b", "c"];
        let response = PaginatedResponse {
            items: items.clone(),
            has_more: true,
            page_token: Some("next".to_string()),
            total: Some(100),
        };
        assert_eq!(response.len(), 3);
        assert!(response.has_more);
        assert_eq!(response.total, Some(100));
    }

    #[test]
    fn test_request_options_default() {
        let options: RequestOptions = Default::default();
        assert!(options.timeout.is_none());
        assert!(options.retry_count.is_none());
        assert!(options.headers.is_none());
    }

    #[test]
    fn test_request_options_new() {
        let options = RequestOptions::new();
        assert!(options.timeout.is_none());
        assert!(options.retry_count.is_none());
    }

    #[test]
    fn test_request_options_multiple_headers() {
        let options = RequestOptions::new()
            .header("Authorization".to_string(), "Bearer token".to_string())
            .header("Content-Type".to_string(), "application/json".to_string());

        let headers = options.headers.unwrap();
        assert_eq!(
            headers.get("Authorization"),
            Some(&"Bearer token".to_string())
        );
        assert_eq!(
            headers.get("Content-Type"),
            Some(&"application/json".to_string())
        );
    }

    #[test]
    fn test_request_options_only_timeout() {
        let options = RequestOptions::new().timeout(Duration::from_secs(60));
        assert_eq!(options.timeout, Some(Duration::from_secs(60)));
        assert!(options.retry_count.is_none());
    }

    #[test]
    fn test_request_options_only_retry() {
        let options = RequestOptions::new().retry_count(5);
        assert_eq!(options.retry_count, Some(5));
        assert!(options.timeout.is_none());
    }

    #[test]
    fn test_api_response_data_clone() {
        let response = ApiResponseData {
            data: 42,
            success: true,
            message: Some("test".to_string()),
            request_id: "req".to_string(),
            timestamp: Some(1234567890),
            extra: std::collections::HashMap::new(),
        };
        let cloned = response.clone();
        assert_eq!(cloned.data, 42);
        assert!(cloned.success);
    }

    #[test]
    fn test_api_response_data_serialize() {
        let response: ApiResponseData<i32> = ApiResponseData::success(42);
        let json = serde_json::to_string(&response).unwrap();
        assert!(json.contains("42"));
        assert!(json.contains("true"));
    }

    #[test]
    fn test_api_response_data_deserialize() {
        let json = r#"{"data":42,"success":true,"message":null,"request_id":"req-123","timestamp":1234567890,"extra":{}}"#;
        let response: ApiResponseData<i32> = serde_json::from_str(json).expect("JSON 反序列化失败");
        assert_eq!(response.data, 42);
        assert!(response.success);
    }
}

//! OpenLark 传输层抽象
//!
//! 提供统一的HTTP通信层，支持请求执行、连接池管理和性能优化。

use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;

use async_trait::async_trait;
use reqwest::{Client, Method, RequestBuilder};
use serde::{Serialize, Deserialize};

use openlark_core::{SDKResult, config::Config as CoreConfig};

use super::{
    error::{UnifiedError, UnifiedResult},
    config::PerformanceConfig,
};

/// HTTP传输层
///
/// 负责所有HTTP请求的发送和响应处理。
#[derive(Debug, Clone)]
pub struct TransportLayer {
    /// HTTP客户端
    client: Client,
    /// 默认超时时间
    default_timeout: Duration,
    /// 重试配置
    retry_config: RetryConfig,
    /// 请求头
    default_headers: HashMap<String, String>,
}

/// HTTP请求
#[derive(Debug, Clone)]
pub struct HTTPRequest {
    /// HTTP方法
    pub method: Method,
    /// 请求URL
    pub url: String,
    /// 请求头
    pub headers: HashMap<String, String>,
    /// 查询参数
    pub query_params: HashMap<String, String>,
    /// 请求体
    pub body: Option<RequestData>,
    /// 超时时间
    pub timeout: Option<Duration>,
    /// 重试配置
    pub retry_config: Option<RetryConfig>,
    /// 元数据
    pub metadata: HashMap<String, String>,
}

/// 请求体数据
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RequestData {
    /// JSON数据
    JSON(serde_json::Value),
    /// 表单数据
    Form(HashMap<String, String>),
    /// 文本数据
    Text(String),
    /// 二进制数据
    Binary(Vec<u8>),
    /// 多部分表单
    Multipart(Vec<MultipartField>),
}

/// 多部分表单字段
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MultipartField {
    /// 字段名称
    pub name: String,
    /// 字段数据
    pub data: MultipartData,
    /// 文件名（如果是文件）
    pub filename: Option<String>,
    /// 内容类型
    pub content_type: Option<String>,
}

/// 多部分表单数据
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MultipartData {
    /// 文本数据
    Text(String),
    /// 文件数据
    File {
        data: Vec<u8>,
        filename: String,
        content_type: String,
    },
}

/// HTTP响应
#[derive(Debug, Clone)]
pub struct HTTPResponse {
    /// 状态码
    pub status_code: u16,
    /// 响应头
    pub headers: HashMap<String, String>,
    /// 响应体
    pub body: ResponseData,
    /// 请求元数据
    pub metadata: HashMap<String, String>,
}

/// 响应体数据
#[derive(Debug, Clone)]
pub struct ResponseData {
    /// 原始字节数据
    pub bytes: Vec<u8>,
    /// 解析后的JSON（如果适用）
    pub json: Option<serde_json::Value>,
    /// 文本内容
    pub text: Option<String>,
}

impl ResponseData {
    /// 创建新的响应数据
    pub fn new(bytes: Vec<u8>, content_type: Option<&str>) -> Self {
        let mut response = Self {
            bytes,
            json: None,
            text: None,
        };

        // 根据内容类型解析数据
        if let Some(content_type) = content_type {
            if content_type.contains("application/json") {
                if let Ok(json) = serde_json::from_slice::<serde_json::Value>(&response.bytes) {
                    response.json = Some(json);
                }
            } else if content_type.contains("text/") {
                if let Ok(text) = String::from_utf8(response.bytes.clone()) {
                    response.text = Some(text);
                }
            }
        }

        response
    }

    /// 获取JSON值
    pub fn json(&self) -> Option<&serde_json::Value> {
        self.json.as_ref()
    }

    /// 获取文本内容
    pub fn text(&self) -> Option<&str> {
        self.text.as_deref()
    }

    /// 获取原始字节
    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }
}

/// 重试配置
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetryConfig {
    /// 最大重试次数
    pub max_attempts: u32,
    /// 基础延迟时间（毫秒）
    pub base_delay_ms: u64,
    /// 最大延迟时间（毫秒）
    pub max_delay_ms: u64,
    /// 退避倍数
    pub backoff_multiplier: f64,
    /// 抖动因子
    pub jitter_factor: f64,
}

impl Default for RetryConfig {
    fn default() -> Self {
        Self {
            max_attempts: 3,
            base_delay_ms: 1000,
            max_delay_ms: 30000,
            backoff_multiplier: 2.0,
            jitter_factor: 0.1,
        }
    }
}

impl RetryConfig {
    /// 计算重试延迟
    pub fn calculate_delay(&self, attempt: u32) -> Duration {
        let base_delay = self.base_delay_ms * self.backoff_multiplier.powi(attempt as i32);
        let delay = base_delay.min(self.max_delay_ms as f64);

        // 添加抖动
        let jitter = delay * self.jitter_factor * (rand::random::<f64>() * 2.0 - 1.0);
        let final_delay = (delay + jitter).max(0.0) as u64;

        Duration::from_millis(final_delay)
    }
}

/// 请求执行器特征
#[async_trait]
pub trait RequestExecutor: Send + Sync {
    /// 执行HTTP请求
    async fn execute(&self, request: HTTPRequest) -> UnifiedResult<HTTPResponse>;

    /// 批量执行请求
    async fn execute_batch(&self, requests: Vec<HTTPRequest>) -> UnifiedResult<Vec<HTTPResponse>> {
        let mut responses = Vec::with_capacity(requests.len());

        for request in requests {
            responses.push(self.execute(request).await?);
        }

        Ok(responses)
    }

    /// 流式执行请求（支持并发）
    async fn execute_concurrent(
        &self,
        requests: Vec<HTTPRequest>,
        max_concurrency: usize,
    ) -> UnifiedResult<Vec<HTTPResponse>> {
        let semaphore = Arc::new(tokio::sync::Semaphore::new(max_concurrency));
        let mut tasks = Vec::new();

        for request in requests {
            let permit = semaphore.clone().acquire_owned().await?;
            let executor = self.clone();

            let task = tokio::spawn(async move {
                let _permit = permit;
                executor.execute(request).await
            });

            tasks.push(task);
        }

        let mut responses = Vec::with_capacity(tasks.len());
        for task in tasks {
            responses.push(task.await.map_err(|e| {
                UnifiedError::InternalError(format!("任务执行失败: {}", e))
            })??);
        }

        Ok(responses)
    }
}

impl TransportLayer {
    /// 从配置创建传输层
    pub fn from_config(config: &CoreConfig) -> UnifiedResult<Self> {
        let client = Self::build_http_client(config)?;

        Ok(Self {
            client,
            default_timeout: config.timeout,
            retry_config: RetryConfig::default(),
            default_headers: HashMap::new(),
        })
    }

    /// 构建HTTP客户端
    fn build_http_client(config: &CoreConfig) -> UnifiedResult<Client> {
        let mut builder = Client::builder()
            .timeout(config.timeout)
            .user_agent("OpenLark-Rust-SDK/1.0")
            .danger_accept_invalid_certs(!config.verify_ssl);

        // 设置连接池配置
        builder = builder
            .pool_max_idle_per_host(10)
            .pool_idle_timeout(Duration::from_secs(30));

        // 设置HTTP/2
        builder = builder.http2_prior_knowledge();

        builder
            .build()
            .map_err(|e| UnifiedError::ConfigurationError(format!("HTTP客户端构建失败: {}", e)))
    }

    /// 设置默认请求头
    pub fn with_default_header(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.default_headers.insert(key.into(), value.into());
        self
    }

    /// 设置重试配置
    pub fn with_retry_config(mut self, config: RetryConfig) -> Self {
        self.retry_config = config;
        self
    }

    /// 转换请求为reqwest请求
    fn convert_request(&self, request: HTTPRequest) -> UnifiedResult<reqwest::Request> {
        let mut req_builder = self.client.request(request.method, &request.url);

        // 添加默认请求头
        for (key, value) in &self.default_headers {
            req_builder = req_builder.header(key, value);
        }

        // 添加请求特定请求头
        for (key, value) in &request.headers {
            req_builder = req_builder.header(key, value);
        }

        // 添加查询参数
        for (key, value) in &request.query_params {
            req_builder = req_builder.query(&[(key, value)]);
        }

        // 设置超时
        let timeout = request.timeout.unwrap_or(self.default_timeout);
        req_builder = req_builder.timeout(timeout);

        // 设置请求体
        if let Some(body) = request.body {
            req_builder = match body {
                RequestData::JSON(json) => req_builder.json(&json),
                RequestData::Form(form) => req_builder.form(&form),
                RequestData::Text(text) => req_builder.body(text),
                RequestData::Binary(bytes) => req_builder.body(bytes),
                RequestData::Multipart(multipart) => {
                    let form = reqwest::multipart::Form::new();
                    let form = multipart.into_iter().fold(form, |form, field| {
                        match field.data {
                            MultipartData::Text(text) => form.text(field.name, text),
                            MultipartData::File { data, filename, content_type } => {
                                let part = reqwest::multipart::Part::bytes(data)
                                    .file_name(filename)
                                    .mime_str(&content_type);
                                form.part(field.name, part)
                            }
                        }
                    });
                    req_builder.multipart(form)
                }
            };
        }

        req_builder
            .build()
            .map_err(|e| UnifiedError::NetworkError {
                message: format!("请求构建失败: {}", e),
                code: None,
                retry_count: 0,
            })
    }

    /// 转换响应
    fn convert_response(response: reqwest::Response) -> UnifiedResult<HTTPResponse> {
        let status_code = response.status().as_u16();

        // 收集响应头
        let headers = response
            .headers()
            .iter()
            .map(|(name, value)| {
                (
                    name.as_str().to_string(),
                    value.to_str().unwrap_or("").to_string(),
                )
            })
            .collect();

        // 获取内容类型
        let content_type = response.headers().get(reqwest::header::CONTENT_TYPE)
            .and_then(|v| v.to_str().ok());

        // 读取响应体
        let bytes = response
            .bytes()
            .await
            .map_err(|e| UnifiedError::NetworkError {
                message: format!("响应读取失败: {}", e),
                code: Some(status_code as i32),
                retry_count: 0,
            })?
            .to_vec();

        let body = ResponseData::new(bytes, content_type);

        Ok(HTTPResponse {
            status_code,
            headers,
            body,
            metadata: HashMap::new(),
        })
    }

    /// 带重试的请求执行
    async fn execute_with_retry(&self, request: HTTPRequest) -> UnifiedResult<HTTPResponse> {
        let retry_config = request.retry_config.as_ref().unwrap_or(&self.retry_config);
        let mut last_error = None;

        for attempt in 0..retry_config.max_attempts {
            let req = self.convert_request(request.clone())?;

            match self.client.execute(req).await {
                Ok(response) => {
                    let unified_response = self.convert_response(response)?;

                    // 检查是否需要重试
                    if self.should_retry(&unified_response) && attempt < retry_config.max_attempts - 1 {
                        let delay = retry_config.calculate_delay(attempt);
                        tokio::time::sleep(delay).await;
                        continue;
                    }

                    return Ok(unified_response);
                }
                Err(e) if attempt < retry_config.max_attempts - 1 => {
                    last_error = Some(e.into());
                    let delay = retry_config.calculate_delay(attempt);
                    tokio::time::sleep(delay).await;
                }
                Err(e) => return Err(e.into()),
            }
        }

        Err(last_error.unwrap_or_else(|| {
            UnifiedError::NetworkError {
                message: "重试次数已用尽".to_string(),
                code: None,
                retry_count: retry_config.max_attempts,
            }
        }))
    }

    /// 检查响应是否需要重试
    fn should_retry(&self, response: &HTTPResponse) -> bool {
        matches!(response.status_code, 429 | 500 | 502 | 503 | 504 | 507 | 509)
    }
}

#[async_trait]
impl RequestExecutor for TransportLayer {
    async fn execute(&self, request: HTTPRequest) -> UnifiedResult<HTTPResponse> {
        self.execute_with_retry(request).await
    }
}

impl Clone for TransportLayer {
    fn clone(&self) -> Self {
        Self {
            client: self.client.clone(),
            default_timeout: self.default_timeout,
            retry_config: self.retry_config.clone(),
            default_headers: self.default_headers.clone(),
        }
    }
}

impl Default for TransportLayer {
    fn default() -> Self {
        let config = CoreConfig::default();
        Self::from_config(&config).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use reqwest::Method;

    #[test]
    fn test_retry_config() {
        let config = RetryConfig::default();
        let delay1 = config.calculate_delay(0);
        let delay2 = config.calculate_delay(1);
        let delay3 = config.calculate_delay(2);

        assert_eq!(delay1.as_millis(), 1000);
        assert_eq!(delay2.as_millis(), 2000);
        assert_eq!(delay3.as_millis(), 4000);
    }

    #[test]
    fn test_request_data() {
        let json_data = RequestData::JSON(serde_json::json!({"test": "value"}));

        if let RequestData::JSON(value) = json_data {
            assert_eq!(value["test"], "value");
        } else {
            panic!("期望JSON数据");
        }
    }

    #[test]
    fn test_response_data() {
        let json_bytes = br#"{"test": "value"}"#;
        let response_data = ResponseData::new(json_bytes.to_vec(), Some("application/json"));

        assert!(response_data.json().is_some());
        assert_eq!(response_data.json().unwrap()["test"], "value");
    }

    #[tokio::test]
    async fn test_transport_layer_creation() {
        let config = CoreConfig::default();
        let transport = TransportLayer::from_config(&config).unwrap();

        assert!(transport.default_timeout > Duration::from_secs(0));
    }
}
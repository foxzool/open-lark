//! OpenLark 中间件系统
//!
//! 提供可插拔的中间件支持，用于请求处理、日志记录、性能监控等。

use std::collections::HashMap;
use std::sync::Arc;

use async_trait::async_trait;

use super::{
    transport::{HTTPRequest, HTTPResponse, RequestExecutor},
    error::{UnifiedError, UnifiedResult},
};

/// 中间件特征
///
/// 所有中间件必须实现此特征。
#[async_trait]
pub trait Middleware: Send + Sync {
    /// 中间件名称
    fn name(&self) -> &'static str;

    /// 请求前处理
    async fn before_request(&self, request: &mut HTTPRequest) -> UnifiedResult<()> {
        let _ = request;
        Ok(())
    }

    /// 响应后处理
    async fn after_response(&self, request: &HTTPRequest, response: &mut HTTPResponse) -> UnifiedResult<()> {
        let _ = request;
        let _ = response;
        Ok(())
    }

    /// 错误处理
    async fn on_error(&self, request: &HTTPRequest, error: &UnifiedError) -> UnifiedError {
        let _ = request;
        error.clone()
    }
}

/// 中间件链
///
/// 管理多个中间件的执行顺序。
#[derive(Debug)]
pub struct MiddlewareChain {
    middlewares: Vec<Box<dyn Middleware>>,
}

impl MiddlewareChain {
    /// 创建新的中间件链
    pub fn new() -> Self {
        Self {
            middlewares: Vec::new(),
        }
    }

    /// 添加中间件
    pub fn add<M>(&mut self, middleware: M) -> &mut Self
    where
        M: Middleware + 'static,
    {
        self.middlewares.push(Box::new(middleware));
        self
    }

    /// 插入中间件到指定位置
    pub fn insert<M>(&mut self, index: usize, middleware: M) -> &mut Self
    where
        M: Middleware + 'static,
    {
        self.middlewares.insert(index, Box::new(middleware));
        self
    }

    /// 移除中间件
    pub fn remove(&mut self, index: usize) -> Option<Box<dyn Middleware>> {
        if index < self.middlewares.len() {
            Some(self.middlewares.remove(index))
        } else {
            None
        }
    }

    /// 获取中间件数量
    pub fn len(&self) -> usize {
        self.middlewares.len()
    }

    /// 检查是否为空
    pub fn is_empty(&self) -> bool {
        self.middlewares.is_empty()
    }

    /// 清空所有中间件
    pub fn clear(&mut self) {
        self.middlewares.clear();
    }

    /// 按名称查找中间件
    pub fn find_by_name(&self, name: &str) -> Option<usize> {
        self.middlewares
            .iter()
            .position(|m| m.name() == name)
    }

    /// 执行请求前中间件
    pub async fn execute_before(&self, request: &mut HTTPRequest) -> UnifiedResult<()> {
        for middleware in &self.middlewares {
            middleware.before_request(request).await?;
        }
        Ok(())
    }

    /// 执行响应后中间件
    pub async fn execute_after(
        &self,
        request: &HTTPRequest,
        response: &mut HTTPResponse,
    ) -> UnifiedResult<()> {
        // 反向执行中间件
        for middleware in self.middlewares.iter().rev() {
            middleware.after_response(request, response).await?;
        }
        Ok(())
    }

    /// 执行错误处理中间件
    pub async fn execute_error(
        &self,
        request: &HTTPRequest,
        error: &UnifiedError,
    ) -> UnifiedError {
        // 反向执行中间件
        let mut current_error = error.clone();
        for middleware in self.middlewares.iter().rev() {
            current_error = middleware.on_error(request, &current_error).await;
        }
        current_error
    }
}

impl Default for MiddlewareChain {
    fn default() -> Self {
        Self::new()
    }
}

/// 基础中间件实现
pub struct BaseMiddleware {
    name: &'static str,
}

impl BaseMiddleware {
    /// 创建新的基础中间件
    pub fn new(name: &'static str) -> Self {
        Self { name }
    }
}

#[async_trait]
impl Middleware for BaseMiddleware {
    fn name(&self) -> &'static str {
        self.name
    }
}

/// 日志中间件
///
/// 记录请求和响应的详细信息。
#[derive(Debug)]
pub struct LoggingMiddleware {
    name: &'static str,
    log_requests: bool,
    log_responses: bool,
    log_headers: bool,
    log_body: bool,
    max_body_size: usize,
}

impl LoggingMiddleware {
    /// 创建新的日志中间件
    pub fn new() -> Self {
        Self {
            name: "LoggingMiddleware",
            log_requests: true,
            log_responses: true,
            log_headers: false,
            log_body: false,
            max_body_size: 1024,
        }
    }

    /// 设置是否记录请求
    pub fn log_requests(mut self, enabled: bool) -> Self {
        self.log_requests = enabled;
        self
    }

    /// 设置是否记录响应
    pub fn log_responses(mut self, enabled: bool) -> Self {
        self.log_responses = enabled;
        self
    }

    /// 设置是否记录请求头
    pub fn log_headers(mut self, enabled: bool) -> Self {
        self.log_headers = enabled;
        self
    }

    /// 设置是否记录请求体
    pub fn log_body(mut self, enabled: bool) -> Self {
        self.log_body = enabled;
        self
    }

    /// 设置最大请求体记录大小
    pub fn max_body_size(mut self, size: usize) -> Self {
        self.max_body_size = size;
        self
    }

    /// 格式化请求体
    fn format_body(&self, body: &crate::unified::transport::RequestData) -> String {
        if !self.log_body {
            return "[已禁用]".to_string();
        }

        match body {
            crate::unified::transport::RequestData::JSON(json) => {
                let json_str = json.to_string();
                if json_str.len() > self.max_body_size {
                    format!("{}...[截断]", &json_str[..self.max_body_size])
                } else {
                    json_str
                }
            }
            crate::unified::transport::RequestData::Form(form) => {
                let form_str = format!("{:?}", form);
                if form_str.len() > self.max_body_size {
                    format!("{}...[截断]", &form_str[..self.max_body_size])
                } else {
                    form_str
                }
            }
            crate::unified::transport::RequestData::Text(text) => {
                if text.len() > self.max_body_size {
                    format!("{}...[截断]", &text[..self.max_body_size])
                } else {
                    text.clone()
                }
            }
            crate::unified::transport::RequestData::Binary(data) => {
                format!("<二进制数据，{}字节>", data.len())
            }
            crate::unified::transport::RequestData::Multipart(parts) => {
                format!("<多部分表单，{}个字段>", parts.len())
            }
        }
    }
}

impl Default for LoggingMiddleware {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl Middleware for LoggingMiddleware {
    fn name(&self) -> &'static str {
        self.name
    }

    async fn before_request(&self, request: &mut HTTPRequest) -> UnifiedResult<()> {
        if !self.log_requests {
            return Ok(());
        }

        let mut log_message = format!("HTTP Request: {} {}", request.method, request.url);

        if self.log_headers && !request.headers.is_empty() {
            log_message.push_str(" | Headers: ");
            for (key, value) in &request.headers {
                log_message.push_str(&format!("{}: {}; ", key, value));
            }
        }

        if let Some(body) = &request.body {
            let body_str = self.format_body(body);
            log_message.push_str(&format!(" | Body: {}", body_str));
        }

        tracing::info!("{}", log_message);
        Ok(())
    }

    async fn after_response(&self, request: &HTTPRequest, response: &mut HTTPResponse) -> UnifiedResult<()> {
        if !self.log_responses {
            return Ok(());
        }

        let mut log_message = format!(
            "HTTP Response: {} {} | Status: {}",
            request.method,
            request.url,
            response.status_code
        );

        if self.log_headers && !response.headers.is_empty() {
            log_message.push_str(" | Headers: ");
            for (key, value) in &response.headers {
                log_message.push_str(&format!("{}: {}; ", key, value));
            }
        }

        let body_size = response.body.bytes().len();
        log_message.push_str(&format!(" | Size: {} bytes", body_size));

        tracing::info!("{}", log_message);
        Ok(())
    }

    async fn on_error(&self, request: &HTTPRequest, error: &UnifiedError) -> UnifiedError {
        tracing::error!(
            "HTTP Error: {} {} | Error: {}",
            request.method,
            request.url,
            error
        );
        error.clone()
    }
}

/// 性能监控中间件
///
/// 记录请求的执行时间和性能指标。
#[derive(Debug)]
pub struct PerformanceMiddleware {
    name: &'static str,
    metrics: Arc<std::sync::RwLock<HashMap<String, MetricData>>>,
}

impl PerformanceMiddleware {
    /// 创建新的性能中间件
    pub fn new() -> Self {
        Self {
            name: "PerformanceMiddleware",
            metrics: Arc::new(std::sync::RwLock::new(HashMap::new())),
        }
    }

    /// 记录性能指标
    fn record_metric(&self, operation: &str, duration: std::time::Duration, success: bool) {
        let mut metrics = self.metrics.write().unwrap();
        let metric = metrics.entry(operation.to_string()).or_insert_with(|| MetricData {
            total_requests: 0,
            total_duration: std::time::Duration::from_nanos(0),
            successful_requests: 0,
            failed_requests: 0,
            min_duration: std::time::Duration::from_secs(999999),
            max_duration: std::time::Duration::from_nanos(0),
        });

        metric.total_requests += 1;
        metric.total_duration += duration;

        if success {
            metric.successful_requests += 1;
        } else {
            metric.failed_requests += 1;
        }

        metric.min_duration = metric.min_duration.min(duration);
        metric.max_duration = metric.max_duration.max(duration);
    }

    /// 获取性能指标
    pub fn get_metrics(&self) -> HashMap<String, MetricData> {
        self.metrics.read().unwrap().clone()
    }

    /// 重置性能指标
    pub fn reset_metrics(&self) {
        self.metrics.write().unwrap().clear();
    }
}

/// 性能指标数据
#[derive(Debug, Clone)]
pub struct MetricData {
    /// 总请求数
    pub total_requests: u64,
    /// 总耗时
    pub total_duration: std::time::Duration,
    /// 成功请求数
    pub successful_requests: u64,
    /// 失败请求数
    pub failed_requests: u64,
    /// 最小耗时
    pub min_duration: std::time::Duration,
    /// 最大耗时
    pub max_duration: std::time::Duration,
}

impl MetricData {
    /// 计算平均耗时
    pub fn average_duration(&self) -> std::time::Duration {
        if self.total_requests == 0 {
            std::time::Duration::from_nanos(0)
        } else {
            self.total_duration / self.total_requests as u32
        }
    }

    /// 计算成功率
    pub fn success_rate(&self) -> f64 {
        if self.total_requests == 0 {
            0.0
        } else {
            self.successful_requests as f64 / self.total_requests as f64
        }
    }
}

impl Default for PerformanceMiddleware {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl Middleware for PerformanceMiddleware {
    fn name(&self) -> &'static str {
        self.name
    }

    async fn before_request(&self, request: &mut HTTPRequest) -> UnifiedResult<()> {
        let start_time = std::time::Instant::now();

        // 将开始时间存储在请求元数据中
        request.metadata.insert(
            "start_time".to_string(),
            start_time.elapsed().as_nanos().to_string(),
        );

        Ok(())
    }

    async fn after_response(&self, request: &HTTPRequest, response: &mut HTTPResponse) -> UnifiedResult<()> {
        if let Some(start_time_str) = request.metadata.get("start_time") {
            if let Ok(start_time_nanos) = start_time_str.parse::<u128>() {
                let start_time = std::time::Instant::now()
                    - std::time::Duration::from_nanos(start_time_nanos as u64);
                let duration = start_time.elapsed();

                let operation = format!("{} {}", request.method, request.url);
                self.record_metric(&operation, duration, true);

                tracing::debug!("请求耗时: {:?}", duration);
            }
        }

        Ok(())
    }

    async fn on_error(&self, request: &HTTPRequest, _error: &UnifiedError) -> UnifiedError {
        if let Some(start_time_str) = request.metadata.get("start_time") {
            if let Ok(start_time_nanos) = start_time_str.parse::<u128>() {
                let start_time = std::time::Instant::now()
                    - std::time::Duration::from_nanos(start_time_nanos as u64);
                let duration = start_time.elapsed();

                let operation = format!("{} {}", request.method, request.url);
                self.record_metric(&operation, duration, false);

                tracing::debug!("请求失败，耗时: {:?}", duration);
            }
        }

        // 返回原始错误
        _error.clone()
    }
}

/// 重试中间件
///
/// 自动重试失败的请求。
#[derive(Debug)]
pub struct RetryMiddleware {
    name: &'static str,
    max_retries: u32,
    retryable_status_codes: Vec<u16>,
}

impl RetryMiddleware {
    /// 创建新的重试中间件
    pub fn new() -> Self {
        Self {
            name: "RetryMiddleware",
            max_retries: 3,
            retryable_status_codes: vec![429, 500, 502, 503, 504],
        }
    }

    /// 设置最大重试次数
    pub fn max_retries(mut self, max_retries: u32) -> Self {
        self.max_retries = max_retries;
        self
    }

    /// 设置可重试的状态码
    pub fn retryable_status_codes(mut self, codes: Vec<u16>) -> Self {
        self.retryable_status_codes = codes;
        self
    }

    /// 检查是否应该重试
    fn should_retry(&self, response: &HTTPResponse, attempt: u32) -> bool {
        attempt < self.max_retries && self.retryable_status_codes.contains(&response.status_code)
    }
}

impl Default for RetryMiddleware {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl Middleware for RetryMiddleware {
    fn name(&self) -> &'static str {
        self.name
    }

    async fn after_response(&self, request: &HTTPRequest, response: &mut HTTPResponse) -> UnifiedResult<()> {
        let attempt = request
            .metadata
            .get("retry_attempt")
            .and_then(|s| s.parse::<u32>().ok())
            .unwrap_or(0);

        if self.should_retry(response, attempt) {
            return Err(UnifiedError::NetworkError {
                message: format!("需要重试，状态码: {}", response.status_code),
                code: Some(response.status_code as i32),
                retry_count: attempt,
            });
        }

        Ok(())
    }
}

/// 带中间件支持的请求执行器
pub struct MiddlewareExecutor {
    inner: Box<dyn RequestExecutor>,
    middleware_chain: Arc<MiddlewareChain>,
}

impl MiddlewareExecutor {
    /// 创建新的中间件执行器
    pub fn new(executor: Box<dyn RequestExecutor>, middleware_chain: MiddlewareChain) -> Self {
        Self {
            inner: executor,
            middleware_chain: Arc::new(middleware_chain),
        }
    }

    /// 获取中间件链的引用
    pub fn middleware_chain(&self) -> &MiddlewareChain {
        &self.middleware_chain
    }
}

#[async_trait]
impl RequestExecutor for MiddlewareExecutor {
    async fn execute(&self, request: HTTPRequest) -> UnifiedResult<HTTPResponse> {
        // 克隆请求用于错误处理
        let request_clone = request.clone();

        // 执行实际请求
        let mut response = match self.inner.execute(request).await {
            Ok(response) => response,
            Err(error) => {
                // 执行错误处理中间件
                return Err(self.middleware_chain.execute_error(&request_clone, &error).await);
            }
        };

        // 执行响应后中间件
        self.middleware_chain.execute_after(&request_clone, &mut response).await?;

        Ok(response)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::unified::transport::{Method, RequestData};

    #[test]
    fn test_middleware_chain() {
        let mut chain = MiddlewareChain::new();
        chain.add(LoggingMiddleware::new());
        chain.add(PerformanceMiddleware::new());

        assert_eq!(chain.len(), 2);
        assert!(!chain.is_empty());
    }

    #[test]
    fn test_logging_middleware() {
        let middleware = LoggingMiddleware::new()
            .log_requests(true)
            .log_headers(true)
            .log_body(true);

        assert_eq!(middleware.name(), "LoggingMiddleware");
    }

    #[test]
    fn test_performance_middleware() {
        let middleware = PerformanceMiddleware::new();
        let metrics = middleware.get_metrics();
        assert!(metrics.is_empty());

        // 这里可以添加性能指标记录的测试
    }

    #[test]
    fn test_retry_middleware() {
        let middleware = RetryMiddleware::new()
            .max_retries(5)
            .retryable_status_codes(vec![500, 502]);

        assert_eq!(middleware.max_retries, 5);
        assert!(middleware.retryable_status_codes.contains(&500));
    }
}
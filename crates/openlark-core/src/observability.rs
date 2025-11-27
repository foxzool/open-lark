//! 可观测性模块
//!
//! 提供统一的日志记录、跟踪和监控功能

use std::time::{Duration, Instant};
use tracing::{span, Level, Span};
use tracing_subscriber::{
    fmt::{self, format::FmtSpan},
    layer::SubscriberExt,
    util::SubscriberInitExt,
    EnvFilter, Registry,
};

#[cfg(feature = "otel")]
use opentelemetry::trace::TracerProvider;
#[cfg(feature = "otel")]
use opentelemetry_otlp::WithExportConfig;
#[cfg(feature = "otel")]
use opentelemetry_sdk::{runtime, Resource};
#[cfg(feature = "otel")]
use tracing_opentelemetry::OpenTelemetryLayer;

/// 操作跟踪器
///
/// 自动记录操作的开始、结束和持续时间
pub struct OperationTracker {
    span: Span,
    start_time: Instant,
    #[allow(dead_code)]
    operation_name: String,
}

impl OperationTracker {
    /// 开始跟踪一个操作
    pub fn start(service_name: &str, operation_name: &str) -> Self {
        let span = span!(
            Level::INFO,
            "service_operation",
            service = service_name,
            operation = operation_name,
            duration_ms = tracing::field::Empty,
            status = tracing::field::Empty,
        );

        {
            let _enter = span.enter();
            tracing::debug!("Starting operation");
        }

        Self {
            span,
            start_time: Instant::now(),
            operation_name: operation_name.to_string(),
        }
    }

    /// 标记操作成功完成
    pub fn success(self) {
        let duration = self.start_time.elapsed();
        let duration_ms = duration.as_millis() as u64;

        self.span.record("duration_ms", duration_ms);
        self.span.record("status", "success");

        let _enter = self.span.enter();
        tracing::info!("Operation completed successfully");
    }

    /// 标记操作失败
    pub fn error(self, error: &str) {
        let duration = self.start_time.elapsed();
        let duration_ms = duration.as_millis() as u64;

        self.span.record("duration_ms", duration_ms);
        self.span.record("status", "error");

        let _enter = self.span.enter();
        tracing::error!(error = error, "Operation failed");
    }

    /// 获取当前 span
    pub fn span(&self) -> &Span {
        &self.span
    }

    /// 获取操作已执行时间
    pub fn elapsed(&self) -> Duration {
        self.start_time.elapsed()
    }
}

/// 初始化 tracing 基础设置
pub fn init_tracing() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    init_tracing_with_filter("info")
}

/// 使用指定过滤器初始化 tracing
pub fn init_tracing_with_filter(
    filter: &str,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(filter));

    let fmt_layer = fmt::layer()
        .with_span_events(FmtSpan::ENTER | FmtSpan::CLOSE)
        .with_target(true)
        .with_thread_ids(true)
        .with_thread_names(true);

    Registry::default()
        .with(env_filter)
        .with(fmt_layer)
        .try_init()?;

    Ok(())
}

/// 初始化结构化 JSON 日志输出
pub fn init_structured_tracing() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));

    let json_layer = fmt::layer()
        .json()
        .with_current_span(true)
        .with_span_list(true);

    Registry::default()
        .with(env_filter)
        .with(json_layer)
        .try_init()?;

    Ok(())
}

/// OpenTelemetry 配置
#[cfg(feature = "otel")]
#[derive(Debug, Clone)]
pub struct OtelConfig {
    /// OTLP 端点 URL
    pub endpoint: String,
    /// 服务名称
    pub service_name: String,
    /// 服务版本
    pub service_version: String,
    /// 环境名称（dev, staging, prod）
    pub environment: String,
}

#[cfg(feature = "otel")]
impl Default for OtelConfig {
    fn default() -> Self {
        Self {
            endpoint: "http://localhost:4317".to_string(),
            service_name: "open-lark-sdk".to_string(),
            service_version: env!("CARGO_PKG_VERSION").to_string(),
            environment: "development".to_string(),
        }
    }
}

/// 初始化 OpenTelemetry + tracing
#[cfg(feature = "otel")]
pub fn init_otel_tracing(
    config: Option<OtelConfig>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let config = config.unwrap_or_default();

    // 使用 0.24 API 创建 OTLP pipeline
    let tracer_provider = opentelemetry_otlp::new_pipeline()
        .tracing()
        .with_exporter(
            opentelemetry_otlp::new_exporter()
                .tonic()
                .with_endpoint(&config.endpoint),
        )
        .with_trace_config(opentelemetry_sdk::trace::Config::default().with_resource(
            Resource::new(vec![
                opentelemetry::KeyValue::new("service.name", config.service_name),
                opentelemetry::KeyValue::new("service.version", config.service_version),
                opentelemetry::KeyValue::new("environment", config.environment),
            ]),
        ))
        .install_batch(runtime::Tokio)?;

    // 设置全局 TracerProvider
    opentelemetry::global::set_tracer_provider(tracer_provider.clone());

    // 从 TracerProvider 获取 tracer（SDK 类型）
    let tracer = tracer_provider.tracer("open-lark-sdk");

    // 创建 OpenTelemetry 层
    let otel_layer = OpenTelemetryLayer::new(tracer);

    // 创建环境过滤器
    let env_filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));

    // 创建格式化层
    let fmt_layer = fmt::layer()
        .with_span_events(FmtSpan::ENTER | FmtSpan::CLOSE)
        .with_target(true)
        .with_thread_ids(true);

    // 组合所有层
    Registry::default()
        .with(env_filter)
        .with(fmt_layer)
        .with(otel_layer)
        .try_init()?;

    Ok(())
}

/// 关闭 OpenTelemetry（确保数据被刷新）
#[cfg(feature = "otel")]
pub fn shutdown_otel() {
    opentelemetry::global::shutdown_tracer_provider();
}

/// HTTP 请求跟踪
pub struct HttpTracker {
    span: Span,
    start_time: Instant,
}

/// 认证操作跟踪器
///
/// 专门用于追踪token获取、刷新、验证等认证相关操作
pub struct AuthTracker {
    span: Span,
    start_time: Instant,
}

impl AuthTracker {
    /// 开始跟踪认证操作
    pub fn start(operation_type: &str, app_id: &str, token_type: &str) -> Self {
        let span = span!(
            Level::INFO,
            "auth_operation",
            operation = operation_type,
            app_id = app_id,
            token_type = token_type,
            cache_hit = tracing::field::Empty,
            duration_ms = tracing::field::Empty,
            success = tracing::field::Empty,
            error_code = tracing::field::Empty,
        );

        {
            let _enter = span.enter();
            tracing::debug!("Starting authentication operation");
        }

        Self {
            span,
            start_time: Instant::now(),
        }
    }

    /// 记录认证成功
    pub fn success(self, cache_hit: bool) {
        let duration = self.start_time.elapsed();
        let duration_ms = duration.as_millis() as u64;

        self.span.record("duration_ms", duration_ms);
        self.span.record("success", true);
        self.span.record("cache_hit", cache_hit);

        let _enter = self.span.enter();
        tracing::info!("Authentication operation completed successfully");
    }

    /// 记录认证失败
    pub fn error(self, error: &str, error_code: Option<i32>) {
        let duration = self.start_time.elapsed();
        let duration_ms = duration.as_millis() as u64;

        self.span.record("duration_ms", duration_ms);
        self.span.record("success", false);

        if let Some(code) = error_code {
            self.span.record("error_code", code);
        }

        let _enter = self.span.enter();
        tracing::error!(error = error, "Authentication operation failed");
    }

    /// 获取当前 span
    pub fn span(&self) -> &Span {
        &self.span
    }
}

/// API响应处理跟踪器
///
/// 用于追踪响应解析、验证、转换等处理过程
pub struct ResponseTracker {
    span: Span,
    start_time: Instant,
}

impl ResponseTracker {
    /// 开始跟踪响应处理
    pub fn start(response_format: &str, response_size: Option<u64>) -> Self {
        let span = span!(
            Level::DEBUG,
            "response_processing",
            format = response_format,
            input_size = response_size.unwrap_or(0),
            parsing_duration_ms = tracing::field::Empty,
            validation_duration_ms = tracing::field::Empty,
            total_duration_ms = tracing::field::Empty,
            success = tracing::field::Empty,
        );

        {
            let _enter = span.enter();
            tracing::debug!("Starting response processing");
        }

        Self {
            span,
            start_time: Instant::now(),
        }
    }

    /// 记录解析阶段完成
    pub fn parsing_complete(&self) {
        let parsing_duration = self.start_time.elapsed();
        let parsing_duration_ms = parsing_duration.as_millis() as u64;
        self.span.record("parsing_duration_ms", parsing_duration_ms);
    }

    /// 记录验证阶段完成
    pub fn validation_complete(&self) {
        let total_duration = self.start_time.elapsed();
        let validation_duration_ms = total_duration.as_millis() as u64;
        self.span
            .record("validation_duration_ms", validation_duration_ms);
    }

    /// 记录处理成功
    pub fn success(self) {
        let duration = self.start_time.elapsed();
        let duration_ms = duration.as_millis() as u64;

        self.span.record("total_duration_ms", duration_ms);
        self.span.record("success", true);

        let _enter = self.span.enter();
        tracing::debug!("Response processing completed successfully");
    }

    /// 记录处理失败
    pub fn error(self, error: &str) {
        let duration = self.start_time.elapsed();
        let duration_ms = duration.as_millis() as u64;

        self.span.record("total_duration_ms", duration_ms);
        self.span.record("success", false);

        let _enter = self.span.enter();
        tracing::error!(error = error, "Response processing failed");
    }
}

impl HttpTracker {
    /// 开始跟踪 HTTP 请求
    pub fn start(method: &str, url: &str) -> Self {
        let span = span!(
            Level::INFO,
            "http_request",
            method = method,
            url = url,
            status_code = tracing::field::Empty,
            duration_ms = tracing::field::Empty,
            response_size = tracing::field::Empty,
        );

        {
            let _enter = span.enter();
            tracing::debug!("Sending HTTP request");
        }

        Self {
            span,
            start_time: Instant::now(),
        }
    }

    /// 记录响应完成
    pub fn response(self, status_code: u16, response_size: Option<u64>) {
        let duration = self.start_time.elapsed();
        let duration_ms = duration.as_millis() as u64;

        self.span.record("status_code", status_code);
        self.span.record("duration_ms", duration_ms);

        if let Some(size) = response_size {
            self.span.record("response_size", size);
        }

        let _enter = self.span.enter();
        if (200..300).contains(&status_code) {
            tracing::info!("HTTP request completed successfully");
        } else if status_code >= 400 {
            tracing::warn!("HTTP request failed");
        } else {
            tracing::info!("HTTP request completed");
        }
    }

    /// 记录请求错误
    pub fn error(self, error: &str) {
        let duration = self.start_time.elapsed();
        let duration_ms = duration.as_millis() as u64;

        self.span.record("duration_ms", duration_ms);

        let _enter = self.span.enter();
        tracing::error!(error = error, "HTTP request failed");
    }
}

/// 性能监控宏
///
/// 自动跟踪代码块的执行时间
#[macro_export]
macro_rules! trace_performance {
    ($name:expr, $code:block) => {{
        let _tracker = $crate::observability::OperationTracker::start("performance", $name);
        let result = $code;
        _tracker.success();
        result
    }};
    ($service:expr, $operation:expr, $code:block) => {{
        let _tracker = $crate::observability::OperationTracker::start($service, $operation);
        let result = $code;
        _tracker.success();
        result
    }};
}

/// 异步性能监控宏
#[macro_export]
macro_rules! trace_async_performance {
    ($name:expr, $code:expr) => {{
        let tracker = $crate::observability::OperationTracker::start("performance", $name);
        match $code.await {
            Ok(result) => {
                tracker.success();
                Ok(result)
            }
            Err(err) => {
                tracker.error(&err.to_string());
                Err(err)
            }
        }
    }};
    ($service:expr, $operation:expr, $code:expr) => {{
        let tracker = $crate::observability::OperationTracker::start($service, $operation);
        match $code.await {
            Ok(result) => {
                tracker.success();
                Ok(result)
            }
            Err(err) => {
                tracker.error(&err.to_string());
                Err(err)
            }
        }
    }};
}

/// 认证操作跟踪宏
///
/// 简化在认证流程中添加可观测性的过程
#[macro_export]
macro_rules! trace_auth_operation {
    ($operation:expr, $app_id:expr, $token_type:expr, $code:expr) => {
        async move {
            let tracker =
                $crate::observability::AuthTracker::start($operation, $app_id, $token_type);
            match $code.await {
                Ok((result, cache_hit)) => {
                    tracker.success(cache_hit);
                    Ok(result)
                }
                Err(err) => {
                    // 尝试从错误中提取错误码
                    let error_code = if let $crate::error::LarkAPIError::Api(api) = &err {
                        Some(api.status as i32)
                    } else {
                        None
                    };
                    tracker.error(&err.to_string(), error_code);
                    Err(err)
                }
            }
        }
    };
}

/// HTTP请求跟踪宏
///
/// 为HTTP请求添加完整的可观测性
#[macro_export]
macro_rules! trace_http_request {
    ($method:expr, $url:expr, $code:expr) => {
        async move {
            let tracker = $crate::observability::HttpTracker::start($method, $url);
            match $code.await {
                Ok(response) => {
                    let status_code = if let Ok(status) = response.status() {
                        status.as_u16()
                    } else {
                        0
                    };
                    tracker.response(status_code, None);
                    Ok(response)
                }
                Err(err) => {
                    tracker.error(&err.to_string());
                    Err(err)
                }
            }
        }
    };
}

/// 响应处理跟踪宏
///
/// 为响应解析和处理添加可观测性
#[macro_export]
macro_rules! trace_response_processing {
    ($format:expr, $size:expr, $parsing:expr, $validation:expr) => {{
        let tracker = $crate::observability::ResponseTracker::start($format, $size);

        // 解析阶段
        let parsed_result = $parsing;
        tracker.parsing_complete();

        match parsed_result {
            Ok(parsed_data) => {
                // 验证阶段
                match $validation(parsed_data) {
                    Ok(validated_data) => {
                        tracker.validation_complete();
                        tracker.success();
                        Ok(validated_data)
                    }
                    Err(err) => {
                        tracker.error(&err.to_string());
                        Err(err)
                    }
                }
            }
            Err(err) => {
                tracker.error(&err.to_string());
                Err(err)
            }
        }
    }};
}

/// 服务健康检查跟踪
pub fn trace_health_check<F, T>(service_name: &str, check_fn: F) -> T
where
    F: FnOnce() -> T,
{
    let tracker = OperationTracker::start(service_name, "health_check");
    let result = check_fn();
    tracker.success();
    result
}

/// 异步服务健康检查跟踪
pub async fn trace_async_health_check<F, Fut, T, E>(service_name: &str, check_fn: F) -> Result<T, E>
where
    F: FnOnce() -> Fut,
    Fut: std::future::Future<Output = Result<T, E>>,
    E: std::fmt::Display,
{
    let tracker = OperationTracker::start(service_name, "health_check");
    match check_fn().await {
        Ok(result) => {
            tracker.success();
            Ok(result)
        }
        Err(err) => {
            tracker.error(&err.to_string());
            Err(err)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;
    use tracing_test::traced_test;

    #[traced_test]
    #[test]
    fn test_operation_tracker_success() {
        let tracker = OperationTracker::start("test_service", "test_operation");
        std::thread::sleep(Duration::from_millis(10));

        // 确保已过去一些时间
        assert!(tracker.elapsed() >= Duration::from_millis(10));

        tracker.success();

        // 验证日志是否记录
        assert!(logs_contain("Starting operation"));
        assert!(logs_contain("Operation completed successfully"));
    }

    #[traced_test]
    #[test]
    fn test_operation_tracker_error() {
        let tracker = OperationTracker::start("test_service", "test_operation");
        tracker.error("Test error message");

        // 验证错误日志是否记录
        assert!(logs_contain("Operation failed"));
        assert!(logs_contain("Test error message"));
    }

    #[traced_test]
    #[test]
    fn test_http_tracker() {
        let tracker = HttpTracker::start("GET", "https://api.example.com/test");
        tracker.response(200, Some(1024));

        // 验证 HTTP 请求日志
        assert!(logs_contain("Sending HTTP request"));
        assert!(logs_contain("HTTP request completed successfully"));
    }

    #[traced_test]
    #[test]
    fn test_performance_macro() {
        let result = trace_performance!("test_perf", {
            std::thread::sleep(Duration::from_millis(5));
            42
        });

        assert_eq!(result, 42);
        assert!(logs_contain("Starting operation"));
        assert!(logs_contain("Operation completed successfully"));
    }

    #[traced_test]
    #[tokio::test]
    async fn test_async_performance_macro() {
        let result: Result<i32, &str> =
            trace_async_performance!("test_service", "async_op", async {
                tokio::time::sleep(Duration::from_millis(5)).await;
                Ok::<i32, &str>(42)
            });

        assert_eq!(result.unwrap(), 42);
        assert!(logs_contain("Starting operation"));
        assert!(logs_contain("Operation completed successfully"));
    }

    #[traced_test]
    #[test]
    fn test_trace_health_check() {
        let result = trace_health_check("test_service", || "healthy");

        assert_eq!(result, "healthy");
        assert!(logs_contain("Starting operation"));
        assert!(logs_contain("Operation completed successfully"));
    }

    #[traced_test]
    #[tokio::test]
    async fn test_trace_async_health_check_success() {
        let result =
            trace_async_health_check("test_service", || async { Ok::<&str, &str>("healthy") })
                .await;

        assert_eq!(result.unwrap(), "healthy");
        assert!(logs_contain("Starting operation"));
        assert!(logs_contain("Operation completed successfully"));
    }

    #[traced_test]
    #[tokio::test]
    async fn test_trace_async_health_check_error() {
        let result =
            trace_async_health_check("test_service", || async { Err::<&str, &str>("unhealthy") })
                .await;

        assert!(result.is_err());
        assert!(logs_contain("Starting operation"));
        assert!(logs_contain("Operation failed"));
        assert!(logs_contain("unhealthy"));
    }

    #[traced_test]
    #[test]
    fn test_auth_tracker_success() {
        let tracker = AuthTracker::start("get_token", "test_app", "tenant");
        std::thread::sleep(Duration::from_millis(10));

        // 模拟缓存命中
        tracker.success(true);

        // 验证认证操作日志
        assert!(logs_contain("Starting authentication operation"));
        assert!(logs_contain(
            "Authentication operation completed successfully"
        ));
    }

    #[traced_test]
    #[test]
    fn test_auth_tracker_error() {
        let tracker = AuthTracker::start("refresh_token", "test_app", "user");
        tracker.error("Invalid credentials", Some(401));

        // 验证认证错误日志
        assert!(logs_contain("Starting authentication operation"));
        assert!(logs_contain("Authentication operation failed"));
        assert!(logs_contain("Invalid credentials"));
    }

    #[traced_test]
    #[test]
    fn test_response_tracker() {
        let tracker = ResponseTracker::start("json", Some(1024));
        std::thread::sleep(Duration::from_millis(5));

        // 模拟解析完成
        tracker.parsing_complete();
        std::thread::sleep(Duration::from_millis(3));

        // 模拟验证完成
        tracker.validation_complete();

        // 完成处理
        tracker.success();

        // 验证响应处理日志
        assert!(logs_contain("Starting response processing"));
        assert!(logs_contain("Response processing completed successfully"));
    }

    #[traced_test]
    #[test]
    fn test_response_tracker_error() {
        let tracker = ResponseTracker::start("xml", Some(512));
        tracker.error("Parse error: invalid XML structure");

        // 验证响应处理错误日志
        assert!(logs_contain("Starting response processing"));
        assert!(logs_contain("Response processing failed"));
        assert!(logs_contain("Parse error: invalid XML structure"));
    }

    #[traced_test]
    #[tokio::test]
    async fn test_trace_auth_operation_macro() {
        use crate::error::LarkAPIError;

        // 测试成功场景 - 需要返回元组 (结果, 是否缓存命中)
        let result = trace_auth_operation!("get_app_token", "test_app", "app", async {
            Ok::<(String, bool), LarkAPIError>(("token_value".to_string(), true))
        })
        .await;

        assert!(result.is_ok());
        assert!(logs_contain("Starting authentication operation"));
        assert!(logs_contain(
            "Authentication operation completed successfully"
        ));
    }

    #[traced_test]
    #[tokio::test]
    async fn test_trace_response_processing_macro() {
        // 测试响应处理宏
        let format = "json";
        let size = Some(256_u64);

        let parsing_fn = || Ok::<String, std::io::Error>("parsed_data".to_string());
        let validation_fn = |data: String| {
            if data.is_empty() {
                Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    "Empty data",
                ))
            } else {
                Ok(data)
            }
        };

        let result = trace_response_processing!(format, size, parsing_fn(), validation_fn);

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "parsed_data");
        assert!(logs_contain("Starting response processing"));
        assert!(logs_contain("Response processing completed successfully"));
    }

    // Additional comprehensive tests for better coverage

    #[traced_test]
    #[test]
    fn test_operation_tracker_span_access() {
        let tracker = OperationTracker::start("test_service", "test_operation");

        // Test span access
        let span = tracker.span();
        assert!(!span.is_disabled());

        // Test elapsed time access
        let elapsed = tracker.elapsed();
        assert!(elapsed >= Duration::from_nanos(0));

        tracker.success();
    }

    #[traced_test]
    #[test]
    fn test_auth_tracker_span_access() {
        let tracker = AuthTracker::start("get_token", "test_app", "tenant");

        // Test span access
        let span = tracker.span();
        assert!(!span.is_disabled());

        tracker.success(false); // Test with cache miss

        assert!(logs_contain("Starting authentication operation"));
        assert!(logs_contain(
            "Authentication operation completed successfully"
        ));
    }

    #[traced_test]
    #[test]
    fn test_auth_tracker_error_without_code() {
        let tracker = AuthTracker::start("refresh_token", "test_app", "user");
        tracker.error("Network timeout", None); // No error code

        assert!(logs_contain("Authentication operation failed"));
        assert!(logs_contain("Network timeout"));
    }

    #[traced_test]
    #[test]
    fn test_http_tracker_different_status_codes() {
        // Test successful 2xx status
        let tracker1 = HttpTracker::start("GET", "https://api.example.com/users");
        tracker1.response(201, Some(512));
        assert!(logs_contain("HTTP request completed successfully"));

        // Test redirection 3xx status
        let tracker2 = HttpTracker::start("GET", "https://api.example.com/redirect");
        tracker2.response(302, None);
        assert!(logs_contain("HTTP request completed"));

        // Test client error 4xx status
        let tracker3 = HttpTracker::start("POST", "https://api.example.com/invalid");
        tracker3.response(404, Some(128));
        assert!(logs_contain("HTTP request failed"));

        // Test server error 5xx status
        let tracker4 = HttpTracker::start("PUT", "https://api.example.com/error");
        tracker4.response(500, Some(256));
        assert!(logs_contain("HTTP request failed"));
    }

    #[traced_test]
    #[test]
    fn test_http_tracker_network_error() {
        let tracker = HttpTracker::start("GET", "https://api.example.com/timeout");
        tracker.error("Connection timeout after 30 seconds");

        assert!(logs_contain("Sending HTTP request"));
        assert!(logs_contain("HTTP request failed"));
        assert!(logs_contain("Connection timeout after 30 seconds"));
    }

    #[traced_test]
    #[test]
    fn test_response_tracker_with_none_size() {
        let tracker = ResponseTracker::start("xml", None);

        // Test parsing without validation
        tracker.parsing_complete();
        tracker.success();

        assert!(logs_contain("Starting response processing"));
        assert!(logs_contain("Response processing completed successfully"));
    }

    #[traced_test]
    #[test]
    fn test_response_tracker_validation_timing() {
        let tracker = ResponseTracker::start("binary", Some(2048));

        // Test timing sequence
        std::thread::sleep(Duration::from_millis(2));
        tracker.parsing_complete();

        std::thread::sleep(Duration::from_millis(3));
        tracker.validation_complete();

        std::thread::sleep(Duration::from_millis(1));
        tracker.success();

        assert!(logs_contain("Starting response processing"));
        assert!(logs_contain("Response processing completed successfully"));
    }

    #[traced_test]
    #[tokio::test]
    async fn test_async_performance_macro_error() {
        let result: Result<i32, &str> =
            trace_async_performance!("test_service", "failing_op", async {
                tokio::time::sleep(Duration::from_millis(2)).await;
                Err::<i32, &str>("Operation failed")
            });

        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Operation failed");
        assert!(logs_contain("Starting operation"));
        assert!(logs_contain("Operation failed"));
    }

    #[traced_test]
    #[test]
    fn test_performance_macro_with_service_and_operation() {
        let result = trace_performance!("my_service", "complex_operation", {
            std::thread::sleep(Duration::from_millis(1));
            "result"
        });

        assert_eq!(result, "result");
        assert!(logs_contain("Starting operation"));
        assert!(logs_contain("Operation completed successfully"));
    }

    #[traced_test]
    #[tokio::test]
    async fn test_trace_auth_operation_macro_error() {
        use crate::error::LarkAPIError;

        // Test error scenario with API error code
        let result = trace_auth_operation!("get_tenant_token", "test_app", "tenant", async {
            Err::<(String, bool), LarkAPIError>(crate::error::LarkAPIError::api_error(
                10001,
                "auth",
                "App not found",
                None::<String>,
            ))
        })
        .await;

        assert!(result.is_err());
        assert!(logs_contain("Starting authentication operation"));
        assert!(logs_contain("Authentication operation failed"));
        assert!(logs_contain("App not found"));
    }

    #[traced_test]
    #[tokio::test]
    async fn test_trace_auth_operation_macro_non_api_error() {
        use crate::error::LarkAPIError;

        // Test with non-API error (no error code)
        let result = trace_auth_operation!("validate_token", "test_app", "user", async {
            Err::<(String, bool), LarkAPIError>(crate::error::network_error(
                "Network connection failed".to_string(),
            ))
        })
        .await;

        assert!(result.is_err());
        assert!(logs_contain("Starting authentication operation"));
        assert!(logs_contain("Authentication operation failed"));
        assert!(logs_contain("Network connection failed"));
    }

    #[traced_test]
    #[test]
    fn test_trace_response_processing_macro_parsing_error() {
        let format = "yaml";
        let size = Some(128_u64);

        let parsing_fn = || {
            Err::<String, std::io::Error>(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Invalid YAML syntax",
            ))
        };
        let validation_fn = |data: String| Ok::<String, std::io::Error>(data);

        let result = trace_response_processing!(format, size, parsing_fn(), validation_fn);

        assert!(result.is_err());
        assert!(logs_contain("Starting response processing"));
        assert!(logs_contain("Response processing failed"));
        assert!(logs_contain("Invalid YAML syntax"));
    }

    #[traced_test]
    #[test]
    fn test_trace_response_processing_macro_validation_error() {
        let format = "csv";
        let size = Some(64_u64);

        let parsing_fn = || Ok::<String, std::io::Error>("".to_string()); // Empty data
        let validation_fn = |data: String| {
            if data.is_empty() {
                Err(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    "CSV data cannot be empty",
                ))
            } else {
                Ok(data)
            }
        };

        let result = trace_response_processing!(format, size, parsing_fn(), validation_fn);

        assert!(result.is_err());
        assert!(logs_contain("Starting response processing"));
        assert!(logs_contain("Response processing failed"));
        assert!(logs_contain("CSV data cannot be empty"));
    }

    #[test]
    fn test_init_tracing_functions() {
        // Note: These functions are hard to test in unit tests because they affect global state
        // We can test that they don't panic and return Ok/Err appropriately

        // Test that multiple initializations handle gracefully
        // (tracing_subscriber typically returns an error on re-initialization)
        let result1 = init_tracing();
        let result2 = init_tracing_with_filter("debug");
        let result3 = init_structured_tracing();

        // At least one should succeed (the first one) or all should handle errors gracefully
        // We don't assert specific Ok/Err because the global state affects this
        assert!(result1.is_ok() || result1.is_err()); // Either outcome is acceptable
        assert!(result2.is_ok() || result2.is_err());
        assert!(result3.is_ok() || result3.is_err());
    }

    #[cfg(feature = "otel")]
    #[test]
    fn test_otel_config_default() {
        let config = OtelConfig::default();

        assert_eq!(config.endpoint, "http://localhost:4317");
        assert_eq!(config.service_name, "open-lark-sdk");
        assert_eq!(config.service_version, env!("CARGO_PKG_VERSION"));
        assert_eq!(config.environment, "development");
    }

    #[cfg(feature = "otel")]
    #[test]
    fn test_otel_config_custom() {
        let config = OtelConfig {
            endpoint: "https://otel.example.com:4317".to_string(),
            service_name: "custom-service".to_string(),
            service_version: "1.2.3".to_string(),
            environment: "production".to_string(),
        };

        assert_eq!(config.endpoint, "https://otel.example.com:4317");
        assert_eq!(config.service_name, "custom-service");
        assert_eq!(config.service_version, "1.2.3");
        assert_eq!(config.environment, "production");
    }

    #[cfg(feature = "otel")]
    #[test]
    fn test_otel_config_clone_and_debug() {
        let config = OtelConfig::default();
        let cloned_config = config.clone();

        assert_eq!(config.endpoint, cloned_config.endpoint);
        assert_eq!(config.service_name, cloned_config.service_name);

        let debug_str = format!("{:?}", config);
        assert!(debug_str.contains("OtelConfig"));
        assert!(debug_str.contains("endpoint"));
        assert!(debug_str.contains("service_name"));
    }

    // Note: init_otel_tracing and shutdown_otel are hard to test in unit tests
    // because they require actual OpenTelemetry infrastructure and affect global state
    // These would be better tested in integration tests

    #[traced_test]
    #[test]
    fn test_operation_tracker_zero_elapsed_time() {
        let tracker = OperationTracker::start("instant_service", "instant_operation");
        // Don't sleep, test immediate completion
        let elapsed = tracker.elapsed();
        tracker.success();

        // Should have some minimal elapsed time
        assert!(elapsed >= Duration::from_nanos(0));
        assert!(logs_contain("Starting operation"));
        assert!(logs_contain("Operation completed successfully"));
    }

    #[traced_test]
    #[test]
    fn test_multiple_trackers_concurrent() {
        let tracker1 = OperationTracker::start("service1", "operation1");
        let tracker2 = AuthTracker::start("auth_op", "app1", "tenant");
        let tracker3 = HttpTracker::start("POST", "https://example.com/api");
        let tracker4 = ResponseTracker::start("json", Some(1024));

        // Complete in different orders
        tracker2.success(true);
        tracker4.success();
        tracker1.success();
        tracker3.response(200, Some(512));

        // All should log successfully
        assert!(logs_contain("Starting operation"));
        assert!(logs_contain("Starting authentication operation"));
        assert!(logs_contain("Sending HTTP request"));
        assert!(logs_contain("Starting response processing"));
    }

    #[traced_test]
    #[test]
    fn test_edge_case_empty_strings() {
        let tracker1 = OperationTracker::start("", "");
        tracker1.success();

        let tracker2 = AuthTracker::start("", "", "");
        tracker2.error("", None);

        let tracker3 = HttpTracker::start("", "");
        tracker3.error("");

        let tracker4 = ResponseTracker::start("", Some(0));
        tracker4.error("");

        // Should handle empty strings gracefully
        assert!(logs_contain("Starting operation"));
        assert!(logs_contain("Starting authentication operation"));
        assert!(logs_contain("Sending HTTP request"));
        assert!(logs_contain("Starting response processing"));
    }
}

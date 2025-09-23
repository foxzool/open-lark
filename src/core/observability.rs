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
        let _tracker = $crate::core::observability::OperationTracker::start("performance", $name);
        let result = $code;
        _tracker.success();
        result
    }};
    ($service:expr, $operation:expr, $code:block) => {{
        let _tracker = $crate::core::observability::OperationTracker::start($service, $operation);
        let result = $code;
        _tracker.success();
        result
    }};
}

/// 异步性能监控宏
#[macro_export]
macro_rules! trace_async_performance {
    ($name:expr, $code:expr) => {{
        let tracker = $crate::core::observability::OperationTracker::start("performance", $name);
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
        let tracker = $crate::core::observability::OperationTracker::start($service, $operation);
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
}

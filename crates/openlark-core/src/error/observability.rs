//! Error Observability Module
//!
//! 提供错误观测性功能，包括结构化日志、指标收集、追踪和监控告警。

use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
    time::{Duration, SystemTime, UNIX_EPOCH},
};
use uuid::Uuid;

use super::{
    codes::{ErrorCode, ErrorCategory},
    core::ErrorSeverity,
    kinds::ErrorKind,
    ErrorRecord,
};

/// 错误观测性系统
///
/// 提供全面的错误监控和分析功能：
/// - 结构化日志记录
/// - 指标收集和统计
/// - 分布式追踪支持
/// - 告警和通知机制
#[derive(Debug)]
pub struct ErrorObservability {
    /// 错误日志记录器
    logger: Arc<dyn ErrorLogger>,
    /// 指标收集器
    metrics_collector: Arc<dyn MetricsCollector>,
    /// 追踪器
    tracer: Arc<dyn ErrorTracer>,
    /// 告警管理器
    alert_manager: Arc<dyn AlertManager>,
    /// 配置
    config: ObservabilityConfig,
}

impl ErrorObservability {
    /// 创建新的观测性系统
    pub fn new(config: ObservabilityConfig) -> Self {
        Self {
            logger: Arc::new(DefaultErrorLogger::new(config.clone())),
            metrics_collector: Arc::new(DefaultMetricsCollector::new()),
            tracer: Arc::new(DefaultErrorTracer::new()),
            alert_manager: Arc::new(DefaultAlertManager::new(config.clone())),
            config,
        }
    }

    /// 记录错误（兼容旧版接口；推荐改用 `record_error_record`）
    pub async fn record_error<E: ErrorObservable + Send + Sync>(&self, error: &E) {
        self.record_error_record(&error.as_error_record()).await;
    }

    /// 基于 ErrorRecord 记录错误（配合 CoreError 使用）
    pub async fn record_error_record(&self, record: &ErrorRecord) {
        let error_event = ErrorEvent::from_record(record);

        let logger = self.logger.clone();
        let metrics = self.metrics_collector.clone();
        let tracer = self.tracer.clone();
        let alert = self.alert_manager.clone();

        let event_clone = error_event.clone();

        tokio::spawn(async move {
            logger.log_error(&event_clone).await;
            metrics.record_metrics(&event_clone).await;
            tracer.trace_error(&event_clone).await;
            alert.check_alerts(&event_clone).await;
        });
    }

    /// 获取错误统计
    pub async fn get_error_stats(&self, time_range: Duration) -> ErrorStatistics {
        self.metrics_collector.get_statistics(time_range).await
    }

    /// 获取错误趋势
    pub async fn get_error_trend(&self, duration: Duration) -> ErrorTrend {
        self.metrics_collector.get_trend(duration).await
    }

    /// 配置告警规则
    pub fn configure_alert(&self, rule: AlertRule) {
        self.alert_manager.add_rule(rule);
    }

    /// 生成错误报告
    pub async fn generate_report(&self, time_range: Duration) -> ErrorReport {
        let stats = self.get_error_stats(time_range).await;
        let trend = self.get_error_trend(time_range).await;

        ErrorReport {
            time_range,
            statistics: stats,
            trend,
            generated_at: SystemTime::now(),
        }
    }

    /// 启用/禁用特定功能
    pub async fn toggle_feature(&mut self, feature: ObservabilityFeature, enabled: bool) {
        match feature {
            ObservabilityFeature::Logging => {
                if enabled {
                    self.logger = Arc::new(DefaultErrorLogger::new(self.config.clone()));
                } else {
                    self.logger = Arc::new(NoOpLogger);
                }
            }
            ObservabilityFeature::Metrics => {
                if enabled {
                    self.metrics_collector = Arc::new(DefaultMetricsCollector::new());
                } else {
                    self.metrics_collector = Arc::new(NoOpMetricsCollector);
                }
            }
            ObservabilityFeature::Tracing => {
                if enabled {
                    self.tracer = Arc::new(DefaultErrorTracer::new());
                } else {
                    self.tracer = Arc::new(NoOpTracer);
                }
            }
            ObservabilityFeature::Alerts => {
                if enabled {
                    self.alert_manager = Arc::new(DefaultAlertManager::new(self.config.clone()));
                } else {
                    self.alert_manager = Arc::new(NoOpAlertManager);
                }
            }
        }
    }
}

/// 观测性配置
#[derive(Debug, Clone)]
pub struct ObservabilityConfig {
    /// 是否启用结构化日志
    pub enable_structured_logging: bool,
    /// 日志级别
    pub log_level: LogLevel,
    /// 是否启用指标收集
    pub enable_metrics: bool,
    /// 是否启用追踪
    pub enable_tracing: bool,
    /// 是否启用告警
    pub enable_alerts: bool,
    /// 指标保留时间
    pub metrics_retention: Duration,
    /// 告警阈值配置
    pub alert_thresholds: AlertThresholds,
}

impl Default for ObservabilityConfig {
    fn default() -> Self {
        Self {
            enable_structured_logging: true,
            log_level: LogLevel::Error,
            enable_metrics: true,
            enable_tracing: true,
            enable_alerts: false, // 默认关闭告警
            metrics_retention: Duration::from_secs(3600), // 1小时
            alert_thresholds: AlertThresholds::default(),
        }
    }
}

/// 观测性功能
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ObservabilityFeature {
    Logging,
    Metrics,
    Tracing,
    Alerts,
}

/// 错误事件
#[derive(Debug, Clone)]
pub struct ErrorEvent {
    /// 错误ID
    pub error_id: Uuid,
    /// 发生时间
    pub timestamp: SystemTime,
    /// 错误种类
    pub kind: ErrorKind,
    /// 错误码
    pub code: Option<ErrorCode>,
    /// 错误消息
    pub message: String,
    /// 严重程度
    pub severity: ErrorSeverity,
    /// 是否可重试
    pub is_retryable: bool,
    /// 上下文信息
    pub context: HashMap<String, String>,
    /// 追踪ID
    pub trace_id: Option<Uuid>,
    /// 跨度ID
    pub span_id: Option<Uuid>,
    /// 相关请求ID
    pub request_id: Option<String>,
    /// 相关用户ID
    pub user_id: Option<String>,
    /// 相关租户ID
    pub tenant_id: Option<String>,
}

impl ErrorEvent {
    /// 从 ErrorRecord 创建事件（V3 首选路径）
    pub fn from_record(record: &ErrorRecord) -> Self {
        Self {
            error_id: Uuid::new_v4(),
            timestamp: SystemTime::now(),
            kind: kind_from_code(record.code),
            code: Some(record.code),
            message: record.message.clone(),
            severity: record.severity,
            is_retryable: record.retryable,
            context: record.context.clone(),
            trace_id: None,
            span_id: None,
            request_id: record.request_id.clone(),
            user_id: None,
            tenant_id: None,
        }
    }

    /// 从错误对象创建事件（旧接口）
    pub fn from_error<E: ErrorObservable>(error: &E) -> Self {
        Self {
            error_id: error.error_id(),
            timestamp: error.timestamp(),
            kind: error.kind(),
            code: error.code(),
            message: error.message().to_string(),
            severity: error.severity(),
            is_retryable: error.is_retryable(),
            context: error.context_data(),
            trace_id: error.trace_id(),
            span_id: error.span_id(),
            request_id: error.request_id().map(|s| s.to_string()),
            user_id: error.user_id().map(|s| s.to_string()),
            tenant_id: error.tenant_id().map(|s| s.to_string()),
        }
    }

    /// 转换为JSON格式
    pub fn to_json(&self) -> String {
        format!(
            r#"{{
  "error_id": "{}",
  "timestamp": "{:?}",
  "kind": "{:?}",
  "code": {:?},
  "message": "{}",
  "severity": "{:?}",
  "is_retryable": {},
  "context": {},
  "trace_id": {:?},
  "request_id": {:?},
  "user_id": {:?},
  "tenant_id": {:?}
}}"#,
            self.error_id,
            self.timestamp,
            self.kind,
            self.code,
            self.message,
            self.severity,
            self.is_retryable,
            serde_json::to_string(&self.context).unwrap_or_default(),
            self.trace_id,
            self.request_id,
            self.user_id,
            self.tenant_id
        )
    }
}

/// 基于 ErrorCode 推断 ErrorKind，保持向后兼容
fn kind_from_code(code: ErrorCode) -> ErrorKind {
    match code.category() {
        ErrorCategory::Network => ErrorKind::Network,
        ErrorCategory::Parameter => ErrorKind::Validation,
        ErrorCategory::Authentication | ErrorCategory::Permission => ErrorKind::Authentication,
        ErrorCategory::Business => ErrorKind::Business,
        ErrorCategory::System | ErrorCategory::Server => ErrorKind::Internal,
        ErrorCategory::RateLimit => ErrorKind::RateLimit,
        ErrorCategory::Resource => ErrorKind::Business,
        ErrorCategory::Success => ErrorKind::Business,
        ErrorCategory::Other => ErrorKind::Internal,
    }
}

/// 错误可观测特征
pub trait ErrorObservable {
    /// 兼容层：从旧错误对象生成 ErrorRecord 所需字段
    fn as_error_record(&self) -> ErrorRecord {
        ErrorRecord {
            code: self.code().unwrap_or(ErrorCode::Unknown),
            severity: self.severity(),
            retryable: self.is_retryable(),
            retry_delay_ms: None,
            message: self.message().to_string(),
            context: self.context_data(),
            request_id: self.request_id().map(|s| s.to_string()),
            operation: None,
            component: None,
            backtrace: None,
        }
    }
}

/// 日志级别
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum LogLevel {
    Debug = 0,
    Info = 1,
    Warn = 2,
    Error = 3,
    Critical = 4,
}

/// 错误日志记录器特征
#[async_trait::async_trait]
pub trait ErrorLogger: Send + Sync {
    /// 记录错误日志
    async fn log_error(&self, event: &ErrorEvent);

    /// 获取日志条目
    async fn get_logs(&self, limit: usize) -> Vec<LogEntry>;
}

/// 默认错误日志记录器
pub struct DefaultErrorLogger {
    config: ObservabilityConfig,
    logs: Arc<Mutex<Vec<LogEntry>>>,
}

impl DefaultErrorLogger {
    pub fn new(config: ObservabilityConfig) -> Self {
        Self {
            config,
            logs: Arc::new(Mutex::new(Vec::new())),
        }
    }
}

#[async_trait::async_trait]
impl ErrorLogger for DefaultErrorLogger {
    async fn log_error(&self, event: &ErrorEvent) {
        let entry = LogEntry {
            timestamp: event.timestamp,
            level: match event.severity {
                ErrorSeverity::Info => LogLevel::Info,
                ErrorSeverity::Warning => LogLevel::Warn,
                ErrorSeverity::Error => LogLevel::Error,
                ErrorSeverity::Critical => LogLevel::Critical,
            },
            message: event.message.clone(),
            error_id: Some(event.error_id),
            context: event.context.clone(),
            trace_id: event.trace_id,
        };

        // 控制台输出
        if event.severity >= self.config.log_level.into() {
            tracing::error!(
                error_id = %event.error_id,
                error_kind = ?event.kind,
                error_code = ?event.code,
                error_message = %event.message,
                error_severity = ?event.severity,
                trace_id = ?event.trace_id,
                "错误发生"
            );
        }

        // 存储日志
        {
            let mut logs = self.logs.lock().unwrap();
            logs.push(entry);

            // 限制日志数量
            if logs.len() > 10000 {
                logs.remove(0);
            }
        }
    }

    async fn get_logs(&self, limit: usize) -> Vec<LogEntry> {
        let logs = self.logs.lock().unwrap();
        logs.iter()
            .rev()
            .take(limit)
            .cloned()
            .collect()
    }
}

/// 指标收集器特征
#[async_trait::async_trait]
pub trait MetricsCollector: Send + Sync {
    /// 记录错误指标
    async fn record_metrics(&self, event: &ErrorEvent);

    /// 获取统计信息
    async fn get_statistics(&self, time_range: Duration) -> ErrorStatistics;

    /// 获取错误趋势
    async fn get_trend(&self, duration: Duration) -> ErrorTrend;
}

/// 默认指标收集器
pub struct DefaultMetricsCollector {
    metrics: Arc<Mutex<ErrorMetrics>>,
}

impl DefaultMetricsCollector {
    pub fn new() -> Self {
        Self {
            metrics: Arc::new(Mutex::new(ErrorMetrics::new())),
        }
    }
}

#[async_trait::async_trait]
impl MetricsCollector for DefaultMetricsCollector {
    async fn record_metrics(&self, event: &ErrorEvent) {
        let mut metrics = self.metrics.lock().unwrap();
        metrics.record_error(event);
    }

    async fn get_statistics(&self, time_range: Duration) -> ErrorStatistics {
        let metrics = self.metrics.lock().unwrap();
        metrics.get_statistics(time_range)
    }

    async fn get_trend(&self, duration: Duration) -> ErrorTrend {
        let metrics = self.metrics.lock().unwrap();
        metrics.get_trend(duration)
    }
}

/// 错误追踪器特征
#[async_trait::async_trait]
pub trait ErrorTracer: Send + Sync {
    /// 追踪错误
    async fn trace_error(&self, event: &ErrorEvent);

    /// 创建错误跨度
    async fn start_span(&self, operation: &str) -> Span;
}

/// 默认错误追踪器
pub struct DefaultErrorTracer;

impl DefaultErrorTracer {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait::async_trait]
impl ErrorTracer for DefaultErrorTracer {
    async fn trace_error(&self, event: &ErrorEvent) {
        if let Some(trace_id) = event.trace_id {
            tracing::error!(
                trace_id = %trace_id,
                span_id = ?event.span_id,
                error_id = %event.error_id,
                "错误追踪"
            );
        }
    }

    async fn start_span(&self, operation: &str) -> Span {
        Span {
            span_id: Uuid::new_v4(),
            trace_id: Uuid::new_v4(),
            operation: operation.to_string(),
            start_time: SystemTime::now(),
            end_time: None,
        }
    }
}

/// 告警管理器特征
#[async_trait::async_trait]
pub trait AlertManager: Send + Sync {
    /// 检查告警条件
    async fn check_alerts(&self, event: &ErrorEvent);

    /// 添加告警规则
    fn add_rule(&self, rule: AlertRule);
}

/// 默认告警管理器
pub struct DefaultAlertManager {
    config: ObservabilityConfig,
    rules: Arc<Mutex<Vec<AlertRule>>>,
}

impl DefaultAlertManager {
    pub fn new(config: ObservabilityConfig) -> Self {
        Self {
            config,
            rules: Arc::new(Mutex::new(Vec::new())),
        }
    }
}

#[async_trait::async_trait]
impl AlertManager for DefaultAlertManager {
    async fn check_alerts(&self, event: &ErrorEvent) {
        if !self.config.enable_alerts {
            return;
        }

        // 收集需要触发的规则，避免跨await持有锁
        let rules_to_trigger: Vec<AlertRule> = {
            let rules = self.rules.lock().unwrap();
            rules.iter().filter(|rule| rule.should_trigger(event)).cloned().collect()
        };

        for rule in rules_to_trigger.iter() {
            self.trigger_alert(rule, event).await;
        }
    }

    fn add_rule(&self, rule: AlertRule) {
        let mut rules = self.rules.lock().unwrap();
        rules.push(rule);
    }
}

impl DefaultAlertManager {
    async fn trigger_alert(&self, rule: &AlertRule, event: &ErrorEvent) {
        match rule.alert_type {
            AlertType::ErrorRate => {
                tracing::warn!(
                    "错误率告警触发: 规则={}, 错误类型={:?}, 错误消息={}",
                    rule.name,
                    event.kind,
                    event.message
                );
            }
            AlertType::CriticalError => {
                tracing::error!(
                    "严重错误告警触发: 规则={}, 错误ID={}, 错误消息={}",
                    rule.name,
                    event.error_id,
                    event.message
                );
            }
            AlertType::Custom => {
                tracing::warn!(
                    "自定义告警触发: 规则={}, 错误消息={}",
                    rule.name,
                    event.message
                );
            }
        }
    }
}

/// 告警规则
#[derive(Debug, Clone)]
pub struct AlertRule {
    /// 规则名称
    pub name: String,
    /// 告警类型
    pub alert_type: AlertType,
    /// 触发条件
    pub condition: AlertCondition,
    /// 是否启用
    pub enabled: bool,
}

impl AlertRule {
    /// 判断是否应该触发告警
    pub fn should_trigger(&self, event: &ErrorEvent) -> bool {
        if !self.enabled {
            return false;
        }

        match &self.condition {
            AlertCondition::ByKind { kinds } => kinds.contains(&event.kind),
            AlertCondition::BySeverity { min_severity } => event.severity >= *min_severity,
            AlertCondition::ByCode { codes } => {
                if let Some(code) = event.code {
                    codes.contains(&code)
                } else {
                    false
                }
            }
            AlertCondition::Custom { predicate } => predicate(event),
        }
    }
}

/// 告警类型
#[derive(Debug, Clone)]
pub enum AlertType {
    /// 错误率告警
    ErrorRate,
    /// 严重错误告警
    CriticalError,
    /// 自定义告警
    Custom,
}

/// 告警条件
#[derive(Debug, Clone)]
pub enum AlertCondition {
    /// 按错误种类
    ByKind { kinds: Vec<ErrorKind> },
    /// 按严重程度
    BySeverity { min_severity: ErrorSeverity },
    /// 按错误码
    ByCode { codes: Vec<ErrorCode> },
    /// 自定义条件
    Custom { predicate: fn(&ErrorEvent) -> bool },
}

/// 告警阈值
#[derive(Debug, Clone)]
pub struct AlertThresholds {
    /// 错误率阈值（百分比）
    pub error_rate_threshold: f64,
    /// 5分钟内错误数阈值
    pub error_count_threshold_5m: u64,
    /// 严重错误阈值
    pub critical_error_threshold: u64,
}

impl Default for AlertThresholds {
    fn default() -> Self {
        Self {
            error_rate_threshold: 5.0,         // 5%
            error_count_threshold_5m: 100,     // 5分钟内100个错误
            critical_error_threshold: 10,      // 10个严重错误
        }
    }
}

/// 错误指标
#[derive(Debug, Default)]
struct ErrorMetrics {
    /// 错误计数统计
    error_counts: HashMap<ErrorKind, u64>,
    /// 错误时间序列
    time_series: Vec<(SystemTime, ErrorKind)>,
    /// 总请求数
    total_requests: u64,
}

impl ErrorMetrics {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn record_error(&mut self, event: &ErrorEvent) {
        *self.error_counts.entry(event.kind).or_insert(0) += 1;
        self.time_series.push((event.timestamp, event.kind));

        // 限制时间序列大小
        if self.time_series.len() > 10000 {
            self.time_series.remove(0);
        }
    }

    pub fn get_statistics(&self, time_range: Duration) -> ErrorStatistics {
        let now = SystemTime::now();
        let cutoff = now - time_range;

        let recent_errors: u64 = self.time_series
            .iter()
            .filter(|(timestamp, _)| *timestamp >= cutoff)
            .count() as u64;

        let error_rate = if self.total_requests > 0 {
            (recent_errors as f64 / self.total_requests as f64) * 100.0
        } else {
            0.0
        };

        ErrorStatistics {
            total_errors: recent_errors,
            errors_by_kind: self.error_counts.clone(),
            error_rate,
            time_range,
        }
    }

    pub fn get_trend(&self, duration: Duration) -> ErrorTrend {
        // 简化的趋势分析
        let now = SystemTime::now();
        let recent_time = now - duration;
        let previous_time = now - duration * 2;

        let recent_count: u64 = self.time_series
            .iter()
            .filter(|(timestamp, _)| *timestamp >= recent_time)
            .count() as u64;

        let previous_count: u64 = self.time_series
            .iter()
            .filter(|(timestamp, _)| *timestamp >= previous_time && *timestamp < recent_time)
            .count() as u64;

        if recent_count > previous_count * 2 {
            ErrorTrend::Increasing
        } else if recent_count * 2 < previous_count {
            ErrorTrend::Decreasing
        } else {
            ErrorTrend::Stable
        }
    }
}

/// 错误统计信息
#[derive(Debug, Clone)]
pub struct ErrorStatistics {
    /// 总错误数
    pub total_errors: u64,
    /// 按种类统计的错误数
    pub errors_by_kind: HashMap<ErrorKind, u64>,
    /// 错误率（百分比）
    pub error_rate: f64,
    /// 统计时间范围
    pub time_range: Duration,
}

/// 错误趋势
#[derive(Debug, Clone)]
pub enum ErrorTrend {
    Increasing,
    Decreasing,
    Stable,
}

/// 错误报告
#[derive(Debug, Clone)]
pub struct ErrorReport {
    /// 报告时间范围
    pub time_range: Duration,
    /// 错误统计
    pub statistics: ErrorStatistics,
    /// 错误趋势
    pub trend: ErrorTrend,
    /// 生成时间
    pub generated_at: SystemTime,
}

/// 日志条目
#[derive(Debug, Clone)]
pub struct LogEntry {
    /// 时间戳
    pub timestamp: SystemTime,
    /// 日志级别
    pub level: LogLevel,
    /// 消息
    pub message: String,
    /// 错误ID
    pub error_id: Option<Uuid>,
    /// 上下文信息
    pub context: HashMap<String, String>,
    /// 追踪ID
    pub trace_id: Option<Uuid>,
}

/// 跨度信息
#[derive(Debug, Clone)]
pub struct Span {
    /// 跨度ID
    pub span_id: Uuid,
    /// 追踪ID
    pub trace_id: Uuid,
    /// 操作名称
    pub operation: String,
    /// 开始时间
    pub start_time: SystemTime,
    /// 结束时间
    pub end_time: Option<SystemTime>,
}

// No-op实现（用于禁用功能）
struct NoOpLogger;
struct NoOpMetricsCollector;
struct NoOpTracer;
struct NoOpAlertManager;

#[async_trait::async_trait]
impl ErrorLogger for NoOpLogger {
    async fn log_error(&self, _event: &ErrorEvent) {}
    async fn get_logs(&self, _limit: usize) -> Vec<LogEntry> {
        Vec::new()
    }
}

#[async_trait::async_trait]
impl MetricsCollector for NoOpMetricsCollector {
    async fn record_metrics(&self, _event: &ErrorEvent) {}
    async fn get_statistics(&self, _time_range: Duration) -> ErrorStatistics {
        ErrorStatistics {
            total_errors: 0,
            errors_by_kind: HashMap::new(),
            error_rate: 0.0,
            time_range: Duration::ZERO,
        }
    }
    async fn get_trend(&self, _duration: Duration) -> ErrorTrend {
        ErrorTrend::Stable
    }
}

#[async_trait::async_trait]
impl ErrorTracer for NoOpTracer {
    async fn trace_error(&self, _event: &ErrorEvent) {}
    async fn start_span(&self, operation: &str) -> Span {
        Span {
            span_id: Uuid::new_v4(),
            trace_id: Uuid::new_v4(),
            operation: operation.to_string(),
            start_time: SystemTime::now(),
            end_time: None,
        }
    }
}

#[async_trait::async_trait]
impl AlertManager for NoOpAlertManager {
    async fn check_alerts(&self, _event: &ErrorEvent) {}
    fn add_rule(&self, _rule: AlertRule) {}
}

impl From<ErrorSeverity> for LogLevel {
    fn from(severity: ErrorSeverity) -> Self {
        match severity {
            ErrorSeverity::Info => Self::Info,
            ErrorSeverity::Warning => Self::Warn,
            ErrorSeverity::Error => Self::Error,
            ErrorSeverity::Critical => Self::Critical,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    struct TestError {
        id: Uuid,
        timestamp: SystemTime,
        kind: ErrorKind,
        code: Option<ErrorCode>,
        message: String,
        severity: ErrorSeverity,
        retryable: bool,
        context: HashMap<String, String>,
    }

    impl ErrorObservable for TestError {
        fn error_id(&self) -> Uuid { self.id }
        fn timestamp(&self) -> SystemTime { self.timestamp }
        fn kind(&self) -> ErrorKind { self.kind }
        fn code(&self) -> Option<ErrorCode> { self.code }
        fn message(&self) -> &str { &self.message }
        fn severity(&self) -> ErrorSeverity { self.severity }
        fn is_retryable(&self) -> bool { self.retryable }
        fn context_data(&self) -> HashMap<String, String> { self.context.clone() }
        fn trace_id(&self) -> Option<Uuid> { None }
        fn span_id(&self) -> Option<Uuid> { None }
        fn request_id(&self) -> Option<&str> { None }
        fn user_id(&self) -> Option<&str> { None }
        fn tenant_id(&self) -> Option<&str> { None }
    }

    #[tokio::test]
    async fn test_error_observability() {
        let config = ObservabilityConfig::default();
        let observability = ErrorObservability::new(config);

        let error = TestError {
            id: Uuid::new_v4(),
            timestamp: SystemTime::now(),
            kind: ErrorKind::Network,
            code: Some(ErrorCode::NetworkConnectionFailed),
            message: "网络连接失败".to_string(),
            severity: ErrorSeverity::Error,
            retryable: true,
            context: HashMap::new(),
        };

        // 记录错误
        observability.record_error(&error).await;

        // 获取统计
        let stats = observability.get_error_stats(Duration::from_secs(60)).await;
        assert!(stats.total_errors >= 0);
    }

    #[test]
    fn test_error_event_creation() {
        let error = TestError {
            id: Uuid::new_v4(),
            timestamp: SystemTime::now(),
            kind: ErrorKind::Network,
            code: Some(ErrorCode::NetworkConnectionFailed),
            message: "网络连接失败".to_string(),
            severity: ErrorSeverity::Error,
            retryable: true,
            context: HashMap::new(),
        };

        let event = ErrorEvent::from_error(&error);
        assert_eq!(event.kind, ErrorKind::Network);
        assert_eq!(event.code, Some(ErrorCode::NetworkConnectionFailed));
        assert_eq!(event.message, "网络连接失败");
        assert!(event.is_retryable);
    }

    #[test]
    fn test_error_event_from_record() {
        let mut ctx = HashMap::new();
        ctx.insert("endpoint".to_string(), "/v1/ping".to_string());

        let record = ErrorRecord {
            code: ErrorCode::ServiceUnavailable,
            severity: ErrorSeverity::Critical,
            retryable: true,
            retry_delay_ms: Some(2000),
            message: "service down".to_string(),
            context: ctx.clone(),
            request_id: Some("req-123".to_string()),
            operation: Some("health_check".to_string()),
            component: None,
            backtrace: None,
        };

        let event = ErrorEvent::from_record(&record);
        assert_eq!(event.kind, ErrorKind::Internal);
        assert_eq!(event.code, Some(ErrorCode::ServiceUnavailable));
        assert_eq!(event.context.get("endpoint"), Some(&"/v1/ping".to_string()));
        assert_eq!(event.request_id.as_deref(), Some("req-123"));
        assert!(event.is_retryable);
    }

    #[test]
    fn test_alert_rule() {
        let rule = AlertRule {
            name: "网络错误告警".to_string(),
            alert_type: AlertType::ErrorRate,
            condition: AlertCondition::ByKind {
                kinds: vec![ErrorKind::Network],
            },
            enabled: true,
        };

        let error = TestError {
            id: Uuid::new_v4(),
            timestamp: SystemTime::now(),
            kind: ErrorKind::Network,
            code: None,
            message: "网络错误".to_string(),
            severity: ErrorSeverity::Error,
            retryable: true,
            context: HashMap::new(),
        };

        let event = ErrorEvent::from_error(&error);
        assert!(rule.should_trigger(&event));

        let non_network_error = TestError {
            kind: ErrorKind::Authentication,
            ..error
        };
        let non_network_event = ErrorEvent::from_error(&non_network_error);
        assert!(!rule.should_trigger(&non_network_event));
    }
}

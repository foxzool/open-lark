//! Error Observability Module
//!
//! 错误观测性模块，整合了日志记录和监控统计功能。
//! 提供结构化的错误日志记录、错误事件统计、性能分析和监控告警功能。
//!
//! # 主要功能
//!
//! - **结构化日志**: 支持多种日志格式和级别控制
//! - **错误统计**: 错误频率统计、类型分布、趋势分析
//! - **性能监控**: 错误处理耗时分析和性能影响评估
//! - **告警机制**: 自动错误告警和阈值监控
//! - **上下文追踪**: 完整的错误上下文信息记录
//!
//! # 使用示例
//!
//! ```rust
//! use openlark_core::error::prelude::*;
//!
//! // 记录错误
//! let error = LarkAPIError::MissingAccessToken;
//! log_error(&error, LogLevel::Error);
//!
//! // 记录错误事件
//! record_error(&error, Some("operation_context".to_string()));
//!
//! // 获取错误统计
//! let stats = get_error_stats();
//! println!("总错误数: {}", stats.total_errors);
//! ```

use std::collections::HashMap;
use std::{
    sync::{Arc, Mutex},
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use crate::{
    error::types::{ErrorHandlingCategory, ErrorSeverity, LarkAPIError, LarkErrorCode},
};

// ============================================================================
// 日志记录功能
// ============================================================================

/// 日志级别
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum LogLevel {
    /// 调试信息
    Debug = 1,
    /// 信息
    Info = 2,
    /// 警告
    Warn = 3,
    /// 错误
    Error = 4,
    /// 严重错误
    Critical = 5,
}

impl LogLevel {
    /// 从错误严重级别转换
    pub fn from_error_severity(severity: ErrorSeverity) -> Self {
        match severity {
            ErrorSeverity::Info => Self::Info,
            ErrorSeverity::Warning => Self::Warn,
            ErrorSeverity::Error => Self::Error,
            ErrorSeverity::Critical => Self::Critical,
        }
    }

    /// 获取颜色代码（用于控制台输出）
    pub fn color_code(&self) -> &'static str {
        match self {
            Self::Debug => "\x1b[36m",    // 青色
            Self::Info => "\x1b[32m",     // 绿色
            Self::Warn => "\x1b[33m",     // 黄色
            Self::Error => "\x1b[31m",    // 红色
            Self::Critical => "\x1b[35m", // 紫色
        }
    }

    /// 重置颜色
    pub fn reset_color() -> &'static str {
        "\x1b[0m"
    }

    /// 获取显示标签
    pub fn label(&self) -> &'static str {
        match self {
            Self::Debug => "DEBUG",
            Self::Info => "INFO",
            Self::Warn => "WARN",
            Self::Error => "ERROR",
            Self::Critical => "CRITICAL",
        }
    }
}

impl std::fmt::Display for LogLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.label())
    }
}

/// 日志条目
#[derive(Debug, Clone)]
pub struct LogEntry {
    /// 日志级别
    pub level: LogLevel,
    /// 时间戳
    pub timestamp: SystemTime,
    /// 消息
    pub message: String,
    /// 错误信息（如果有）
    pub error: Option<LarkAPIError>,
    /// 错误分类
    pub category: Option<ErrorHandlingCategory>,
    /// 错误码
    pub error_code: Option<LarkErrorCode>,
    /// 上下文信息
    pub context: HashMap<String, String>,
    /// 调用栈信息
    pub caller: Option<String>,
}

impl LogEntry {
    /// 创建新的日志条目
    pub fn new(level: LogLevel, message: impl Into<String>) -> Self {
        Self {
            level,
            timestamp: SystemTime::now(),
            message: message.into(),
            error: None,
            category: None,
            error_code: None,
            context: HashMap::new(),
            caller: None,
        }
    }

    /// 添加错误信息
    pub fn with_error(mut self, error: LarkAPIError) -> Self {
        self.error = Some(error.clone());
        self.category = Some(error.handling_category());

        if let LarkAPIError::ApiError { code, .. } = error {
            self.error_code = LarkErrorCode::from_code(code);
        }

        self
    }

    /// 添加上下文信息
    pub fn with_context(mut self, key: &str, value: &str) -> Self {
        self.context.insert(key.to_string(), value.to_string());
        self
    }

    /// 添加调用者信息
    pub fn with_caller(mut self, caller: &str) -> Self {
        self.caller = Some(caller.to_string());
        self
    }

    /// 转换为JSON格式
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        // 简单的JSON格式化
        let json = format!(
            r#"{{
  "level": "{:?}",
  "timestamp": "{:?}",
  "message": "{}",
  "category": {:?},
  "error_code": {:?}
}}"#,
            self.level,
            self.timestamp,
            self.message,
            self.category,
            self.error_code
        );
        Ok(json)
    }

    /// 格式化为控制台输出
    pub fn format_console(&self) -> String {
        let timestamp = self.timestamp
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        let time_str = chrono::DateTime::from_timestamp(timestamp as i64, 0)
            .map(|dt| dt.format("%Y-%m-%d %H:%M:%S").to_string())
            .unwrap_or_else(|| format!("timestamp:{}", timestamp));

        let color = self.level.color_code();
        let reset = LogLevel::reset_color();

        let mut output = format!(
            "{}[{}]{} {}{} {}{}",
            color,
            self.level,
            reset,
            time_str,
            color,
            self.message,
            reset
        );

        if let Some(error) = &self.error {
            output.push_str(&format!("{} - Error: {}{}",
                color,
                error.user_friendly_message(),
                reset
            ));
        }

        if !self.context.is_empty() {
            output.push_str(" {");
            for (key, value) in &self.context {
                output.push_str(&format!(" {}: {},", key, value));
            }
            output.push_str(" }");
        }

        output
    }
}

/// 错误日志记录器
#[derive(Debug, Clone)]
pub struct ErrorLogger {
    /// 最小日志级别
    pub min_level: LogLevel,
    /// 是否启用控制台输出
    pub console_output: bool,
    /// 是否启用结构化输出
    pub structured_output: bool,
    /// 日志缓存
    cache: Arc<Mutex<Vec<LogEntry>>>,
    /// 最大缓存条目数
    pub max_cache_entries: usize,
}

impl ErrorLogger {
    /// 创建新的错误日志记录器
    pub fn new(min_level: LogLevel) -> Self {
        Self {
            min_level,
            console_output: true,
            structured_output: false,
            cache: Arc::new(Mutex::new(Vec::new())),
            max_cache_entries: 1000,
        }
    }

    /// 创建默认配置的日志记录器
    pub fn default() -> Self {
        Self::new(LogLevel::Info)
    }

    /// 记录日志条目
    pub fn log(&self, entry: LogEntry) {
        if entry.level < self.min_level {
            return;
        }

        // 控制台输出
        if self.console_output {
            println!("{}", entry.format_console());
        }

        // 结构化输出
        if self.structured_output {
            if let Ok(json) = entry.to_json() {
                eprintln!("{}", json);
            }
        }

        // 添加到缓存
        {
            let mut cache = self.cache.lock().unwrap();
            cache.push(entry.clone());

            // 限制缓存大小
            if cache.len() > self.max_cache_entries {
                cache.remove(0);
            }
        }
    }

    /// 获取缓存的日志条目
    pub fn get_cached_entries(&self) -> Vec<LogEntry> {
        self.cache.lock().unwrap().clone()
    }

    /// 清空缓存
    pub fn clear_cache(&self) {
        self.cache.lock().unwrap().clear();
    }

    /// 获取按级别过滤的日志条目
    pub fn get_entries_by_level(&self, level: LogLevel) -> Vec<LogEntry> {
        let cache = self.cache.lock().unwrap();
        cache.iter()
            .filter(|entry| entry.level == level)
            .cloned()
            .collect()
    }

    /// 获取指定时间范围内的日志条目
    pub fn get_entries_by_time_range(&self, start: SystemTime, end: SystemTime) -> Vec<LogEntry> {
        let cache = self.cache.lock().unwrap();
        cache.iter()
            .filter(|entry| entry.timestamp >= start && entry.timestamp <= end)
            .cloned()
            .collect()
    }
}

// ============================================================================
// 错误统计和监控功能
// ============================================================================

/// 错误事件记录
#[derive(Debug, Clone)]
pub struct ErrorEvent {
    /// 错误实例
    pub error: LarkAPIError,
    /// 发生时间
    pub timestamp: SystemTime,
    /// 错误分类
    pub category: ErrorHandlingCategory,
    /// 错误码（如果是API错误）
    pub error_code: Option<LarkErrorCode>,
    /// 是否可重试
    pub is_retryable: bool,
    /// 处理耗时（如果有）
    pub processing_time: Option<Duration>,
    /// 上下文信息
    pub context: HashMap<String, String>,
}

impl ErrorEvent {
    /// 从LarkAPIError创建错误事件
    pub fn from_error(error: LarkAPIError) -> Self {
        let category = error.handling_category();
        let error_code = match &error {
            LarkAPIError::ApiError { code, .. } => LarkErrorCode::from_code(*code),
            _ => None,
        };

        Self {
            is_retryable: error.is_retryable(),
            error,
            timestamp: SystemTime::now(),
            category,
            error_code,
            processing_time: None,
            context: HashMap::new(),
        }
    }

    /// 添加上下文信息
    pub fn with_context(mut self, key: &str, value: &str) -> Self {
        self.context.insert(key.to_string(), value.to_string());
        self
    }

    /// 设置处理耗时
    pub fn with_processing_time(mut self, duration: Duration) -> Self {
        self.processing_time = Some(duration);
        self
    }

    /// 获取错误严重级别
    pub fn severity_level(&self) -> ErrorSeverity {
        match &self.category {
            ErrorHandlingCategory::Authentication => ErrorSeverity::Warning,
            ErrorHandlingCategory::Permission => ErrorSeverity::Error,
            ErrorHandlingCategory::Parameter => ErrorSeverity::Warning,
            ErrorHandlingCategory::Server => ErrorSeverity::Critical,
            ErrorHandlingCategory::Network => ErrorSeverity::Error,
            ErrorHandlingCategory::RateLimit => ErrorSeverity::Warning,
            ErrorHandlingCategory::System => ErrorSeverity::Critical,
            _ => ErrorSeverity::Error,
        }
    }
}

/// 错误统计信息
#[derive(Debug, Clone)]
pub struct ErrorStatistics {
    /// 总错误数
    pub total_errors: u64,
    /// 按类别统计的错误数
    pub errors_by_category: HashMap<ErrorHandlingCategory, u64>,
    /// 按严重级别统计的错误数
    pub errors_by_severity: HashMap<ErrorSeverity, u64>,
    /// 按错误码统计的错误数
    pub errors_by_code: HashMap<i32, u64>,
    /// 可重试错误数
    pub retryable_errors: u64,
    /// 不可重试错误数
    pub non_retryable_errors: u64,
    /// 平均处理时间
    pub average_processing_time: Option<Duration>,
    /// 错误率（错误数/总请求数）
    pub error_rate: f64,
    /// 最后更新时间
    pub last_updated: SystemTime,
    /// 时间范围内的错误趋势
    pub trend: ErrorTrend,
}

impl Default for ErrorStatistics {
    fn default() -> Self {
        Self {
            total_errors: 0,
            errors_by_category: HashMap::new(),
            errors_by_severity: HashMap::new(),
            errors_by_code: HashMap::new(),
            retryable_errors: 0,
            non_retryable_errors: 0,
            average_processing_time: None,
            error_rate: 0.0,
            last_updated: SystemTime::now(),
            trend: ErrorTrend::Unknown,
        }
    }
}

/// 错误趋势
#[derive(Debug, Clone)]
pub enum ErrorTrend {
    /// 上升趋势
    Increasing,
    /// 下降趋势
    Decreasing,
    /// 稳定
    Stable,
    /// 未知
    Unknown,
}

/// 错误监控器
#[derive(Debug)]
pub struct ErrorMonitor {
    /// 错误事件缓存
    events: Arc<Mutex<Vec<ErrorEvent>>>,
    /// 统计信息
    statistics: Arc<Mutex<ErrorStatistics>>,
    /// 最大缓存事件数
    max_events: usize,
    /// 告警阈值
    alert_thresholds: AlertThresholds,
}

/// 告警阈值配置
#[derive(Debug, Clone)]
pub struct AlertThresholds {
    /// 错误率阈值（百分比）
    pub error_rate_threshold: f64,
    /// 5分钟内错误数阈值
    pub error_count_threshold_5m: u64,
    /// 平均处理时间阈值
    pub processing_time_threshold: Duration,
}

impl Default for AlertThresholds {
    fn default() -> Self {
        Self {
            error_rate_threshold: 5.0, // 5%
            error_count_threshold_5m: 100, // 5分钟内100个错误
            processing_time_threshold: Duration::from_secs(30), // 30秒
        }
    }
}

impl ErrorMonitor {
    /// 创建新的错误监控器
    pub fn new(max_events: usize) -> Self {
        Self {
            events: Arc::new(Mutex::new(Vec::new())),
            statistics: Arc::new(Mutex::new(ErrorStatistics::default())),
            max_events,
            alert_thresholds: AlertThresholds::default(),
        }
    }

    /// 创建默认配置的监控器
    pub fn default() -> Self {
        Self::new(10000)
    }

    /// 记录错误事件
    pub fn record_error(&self, error: &LarkAPIError, context: Option<&str>) {
        let mut event = ErrorEvent::from_error(error.clone());

        if let Some(ctx) = context {
            event = event.with_context("context", ctx);
        }

        // 添加到事件缓存
        {
            let mut events = self.events.lock().unwrap();
            events.push(event.clone());

            // 限制缓存大小
            if events.len() > self.max_events {
                events.remove(0);
            }
        }

        // 更新统计信息
        self.update_statistics(&event);

        // 检查告警
        self.check_alerts();
    }

    /// 更新统计信息
    fn update_statistics(&self, event: &ErrorEvent) {
        let mut stats = self.statistics.lock().unwrap();

        stats.total_errors += 1;

        // 按类别统计
        *stats.errors_by_category.entry(event.category).or_insert(0) += 1;

        // 按严重级别统计
        *stats.errors_by_severity.entry(event.severity_level()).or_insert(0) += 1;

        // 按错误码统计
        if let Some(error_code) = event.error_code {
            *stats.errors_by_code.entry(error_code as i32).or_insert(0) += 1;
        }

        // 按可重试性统计
        if event.is_retryable {
            stats.retryable_errors += 1;
        } else {
            stats.non_retryable_errors += 1;
        }

        // 更新平均处理时间
        if let Some(processing_time) = event.processing_time {
            if let Some(current_avg) = stats.average_processing_time {
                let new_avg_nanos = (current_avg.as_nanos() * (stats.total_errors - 1) as u128 + processing_time.as_nanos())
                    / stats.total_errors as u128;
                // 安全转换为 u64，如果溢出则使用最大值
                let new_avg = Duration::from_nanos(new_avg_nanos.try_into().unwrap_or(u64::MAX));
                stats.average_processing_time = Some(new_avg);
            } else {
                stats.average_processing_time = Some(processing_time);
            }
        }

        stats.last_updated = SystemTime::now();
    }

    /// 检查告警条件
    fn check_alerts(&self) {
        let stats = self.statistics.lock().unwrap();

        // 检查错误率
        if stats.error_rate > self.alert_thresholds.error_rate_threshold {
            self.trigger_alert(ErrorAlert::HighErrorRate {
                current_rate: stats.error_rate,
                threshold: self.alert_thresholds.error_rate_threshold,
            });
        }

        // 检查5分钟内错误数
        let recent_errors = self.get_recent_errors(Duration::from_secs(300));
        if recent_errors.len() as u64 > self.alert_thresholds.error_count_threshold_5m {
            self.trigger_alert(ErrorAlert::HighErrorCount {
                count: recent_errors.len() as u64,
                threshold: self.alert_thresholds.error_count_threshold_5m,
                duration: Duration::from_secs(300),
            });
        }

        // 检查处理时间
        if let Some(avg_time) = stats.average_processing_time {
            if avg_time > self.alert_thresholds.processing_time_threshold {
                self.trigger_alert(ErrorAlert::HighProcessingTime {
                    current_avg: avg_time,
                    threshold: self.alert_thresholds.processing_time_threshold,
                });
            }
        }
    }

    /// 触发告警
    fn trigger_alert(&self, alert: ErrorAlert) {
        // 在实际应用中，这里可以发送到监控系统、日志系统等
        eprintln!("🚨 错误告警: {:?}", alert);
    }

    /// 获取最近的错误事件
    pub fn get_recent_errors(&self, duration: Duration) -> Vec<ErrorEvent> {
        let now = SystemTime::now();
        let events = self.events.lock().unwrap();

        events.iter()
            .filter(|event| {
                now.duration_since(event.timestamp).unwrap_or_default() <= duration
            })
            .cloned()
            .collect()
    }

    /// 获取统计信息
    pub fn get_statistics(&self) -> ErrorStatistics {
        self.statistics.lock().unwrap().clone()
    }

    /// 清空所有数据
    pub fn clear_all(&self) {
        self.events.lock().unwrap().clear();
        *self.statistics.lock().unwrap() = ErrorStatistics::default();
    }

    /// 计算错误率
    pub fn calculate_error_rate(&self, total_requests: u64) -> f64 {
        let stats = self.statistics.lock().unwrap();
        if total_requests == 0 {
            0.0
        } else {
            (stats.total_errors as f64 / total_requests as f64) * 100.0
        }
    }

    /// 获取错误趋势
    pub fn get_trend(&self) -> ErrorTrend {
        let events = self.events.lock().unwrap();
        if events.len() < 2 {
            return ErrorTrend::Unknown;
        }

        let recent_window = Duration::from_secs(300); // 5分钟窗口
        let now = SystemTime::now();

        let recent_count = events.iter()
            .filter(|event| {
                now.duration_since(event.timestamp).unwrap_or_default() <= recent_window
            })
            .count();

        let _previous_window_start = now - recent_window - recent_window;
        let previous_count = events.iter()
            .filter(|event| {
                let elapsed = now.duration_since(event.timestamp).unwrap_or_default();
                elapsed > recent_window && elapsed <= recent_window * 2
            })
            .count();

        if recent_count > previous_count * 2 {
            ErrorTrend::Increasing
        } else if recent_count * 2 < previous_count {
            ErrorTrend::Decreasing
        } else {
            ErrorTrend::Stable
        }
    }
}

/// 错误告警类型
#[derive(Debug, Clone)]
pub enum ErrorAlert {
    /// 高错误率告警
    HighErrorRate {
        current_rate: f64,
        threshold: f64,
    },
    /// 高错误数告警
    HighErrorCount {
        count: u64,
        threshold: u64,
        duration: Duration,
    },
    /// 高处理时间告警
    HighProcessingTime {
        current_avg: Duration,
        threshold: Duration,
    },
    /// 新错误类型告警
    NewErrorType {
        error_type: String,
        count: u64,
    },
}

// ============================================================================
// 全局实例和便利函数
// ============================================================================

/// 获取全局错误日志记录器
fn get_error_logger() -> &'static ErrorLogger {
    use std::sync::OnceLock;
    static LOGGER: OnceLock<ErrorLogger> = OnceLock::new();
    LOGGER.get_or_init(|| ErrorLogger::default())
}

/// 获取全局错误监控器
fn get_error_monitor() -> &'static ErrorMonitor {
    use std::sync::OnceLock;
    static MONITOR: OnceLock<ErrorMonitor> = OnceLock::new();
    MONITOR.get_or_init(|| ErrorMonitor::default())
}

/// 记录错误日志
///
/// # 参数
/// - `error`: 要记录的错误
/// - `level`: 日志级别
pub fn log_error(error: &LarkAPIError, level: LogLevel) {
    let entry = LogEntry::new(level, "Error occurred")
        .with_error(error.clone());

    get_error_logger().log(entry);
}

/// 记录错误事件
///
/// # 参数
/// - `error`: 要记录的错误
/// - `context`: 可选的上下文信息
pub fn record_error(error: &LarkAPIError, context: Option<String>) {
    get_error_monitor().record_error(error, context.as_deref());
}

/// 获取错误统计信息
///
/// # 返回值
/// 当前的错误统计信息
pub fn get_error_stats() -> ErrorStatistics {
    get_error_monitor().get_statistics()
}

/// 获取最近的错误事件
///
/// # 参数
/// - `duration`: 时间范围
///
/// # 返回值
/// 指定时间范围内的错误事件
pub fn get_recent_errors(duration: Duration) -> Vec<ErrorEvent> {
    get_error_monitor().get_recent_errors(duration)
}

/// 获取错误趋势
///
/// # 返回值
/// 当前的错误趋势
pub fn get_error_trend() -> ErrorTrend {
    get_error_monitor().get_trend()
}

// ============================================================================
// 测试
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_log_levels() {
        assert!(LogLevel::Error > LogLevel::Info);
        assert_eq!(LogLevel::from_error_severity(ErrorSeverity::Critical), LogLevel::Critical);
        assert_eq!(LogLevel::Debug.label(), "DEBUG");
    }

    #[test]
    fn test_log_entry() {
        let error = LarkAPIError::MissingAccessToken;
        let entry = LogEntry::new(LogLevel::Error, "Test error")
            .with_error(error)
            .with_context("operation", "test")
            .with_caller("test_function");

        assert_eq!(entry.level, LogLevel::Error);
        assert!(entry.error.is_some());
        assert!(entry.context.contains_key("operation"));
        assert_eq!(entry.caller.as_ref().unwrap(), "test_function");
    }

    #[test]
    fn test_error_logger() {
        let logger = ErrorLogger::new(LogLevel::Warn);
        let error = LarkAPIError::MissingAccessToken;

        let entry = LogEntry::new(LogLevel::Error, "Test error")
            .with_error(error.clone());

        logger.log(entry);

        // 测试过滤（Debug级别应该被过滤）
        let debug_entry = LogEntry::new(LogLevel::Debug, "Debug message");
        logger.log(debug_entry);

        let entries = logger.get_cached_entries();
        assert_eq!(entries.len(), 1); // 只有Error级别的被记录
    }

    #[test]
    fn test_error_event() {
        let error = LarkAPIError::MissingAccessToken;
        let event = ErrorEvent::from_error(error)
            .with_context("test", "value")
            .with_processing_time(Duration::from_millis(100));

        assert!(event.context.contains_key("test"));
        assert_eq!(event.processing_time, Some(Duration::from_millis(100)));
        assert!(!event.is_retryable);
    }

    #[test]
    fn test_error_monitor() {
        let monitor = ErrorMonitor::new(100);
        let error = LarkAPIError::MissingAccessToken;

        monitor.record_error(&error, Some("test context"));

        let stats = monitor.get_statistics();
        assert_eq!(stats.total_errors, 1);
        assert_eq!(stats.retryable_errors, 0);
        assert_eq!(stats.non_retryable_errors, 1);
    }

    #[test]
    fn test_convenience_functions() {
        let error = LarkAPIError::MissingAccessToken;

        // 测试日志记录
        log_error(&error, LogLevel::Error);

        // 测试错误事件记录
        record_error(&error, Some("test".to_string()));

        // 测试统计获取
        let stats = get_error_stats();
        assert!(stats.total_errors > 0);

        // 测试趋势获取
        let trend = get_error_trend();
        matches!(trend, ErrorTrend::Unknown | ErrorTrend::Stable);
    }

    #[test]
    fn test_alert_thresholds() {
        let thresholds = AlertThresholds::default();
        assert_eq!(thresholds.error_rate_threshold, 5.0);
        assert_eq!(thresholds.error_count_threshold_5m, 100);
        assert_eq!(thresholds.processing_time_threshold, Duration::from_secs(30));
    }

    #[test]
    fn test_error_severity_mapping() {
        let auth_error = LarkAPIError::MissingAccessToken;
        let event = ErrorEvent::from_error(auth_error);
        assert_eq!(event.severity_level(), ErrorSeverity::Warning);

        let server_error = LarkAPIError::api_error(500, "Server Error", None);
        let event = ErrorEvent::from_error(server_error);
        assert_eq!(event.severity_level(), ErrorSeverity::Critical);
    }

    #[test]
    fn test_recent_errors() {
        let monitor = ErrorMonitor::new(10);
        let error = LarkAPIError::MissingAccessToken;

        // 记录几个错误
        for i in 0..3 {
            monitor.record_error(&error, Some(&format!("test {}", i)));
        }

        let recent = monitor.get_recent_errors(Duration::from_secs(1));
        assert_eq!(recent.len(), 3);

        let older = monitor.get_recent_errors(Duration::from_nanos(1));
        assert_eq!(older.len(), 0);
    }
}
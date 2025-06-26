/// 错误统计和监控模块
///
/// 提供错误的统计分析和监控功能：
/// - 错误频率统计
/// - 错误类型分布
/// - 性能影响分析
/// - 趋势分析
/// - 自动告警
use std::collections::HashMap;
use std::{
    sync::{Arc, Mutex},
    time::{Duration, SystemTime},
};

use crate::core::{
    error::LarkAPIError,
    error_codes::{ErrorCategory, LarkErrorCode},
    error_helper::ErrorHandlingCategory,
};

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
        let category = match &error {
            LarkAPIError::ApiError { code, .. } => {
                if let Some(error_code) = LarkErrorCode::from_code(*code) {
                    match error_code.category() {
                        ErrorCategory::Authentication => ErrorHandlingCategory::Authentication,
                        ErrorCategory::Permission => ErrorHandlingCategory::Permission,
                        ErrorCategory::Parameter => ErrorHandlingCategory::ClientError,
                        ErrorCategory::Resource => ErrorHandlingCategory::ClientError,
                        ErrorCategory::Server => ErrorHandlingCategory::ServerError,
                        ErrorCategory::Network => ErrorHandlingCategory::NetworkError,
                        ErrorCategory::RateLimit => ErrorHandlingCategory::RateLimit,
                        ErrorCategory::Other => ErrorHandlingCategory::Unknown,
                    }
                } else {
                    ErrorHandlingCategory::Unknown
                }
            }
            LarkAPIError::RequestError(_) => ErrorHandlingCategory::NetworkError,
            LarkAPIError::MissingAccessToken => ErrorHandlingCategory::Authentication,
            LarkAPIError::IllegalParamError(_) => ErrorHandlingCategory::ClientError,
            _ => ErrorHandlingCategory::SystemError,
        };

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
            ErrorHandlingCategory::ClientError => ErrorSeverity::Warning,
            ErrorHandlingCategory::ServerError => ErrorSeverity::Critical,
            ErrorHandlingCategory::NetworkError => ErrorSeverity::Warning,
            ErrorHandlingCategory::RateLimit => ErrorSeverity::Warning,
            ErrorHandlingCategory::SystemError => ErrorSeverity::Critical,
            ErrorHandlingCategory::Unknown => ErrorSeverity::Error,
        }
    }
}

/// 错误严重级别
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ErrorSeverity {
    /// 信息
    Info,
    /// 警告
    Warning,
    /// 错误
    Error,
    /// 严重
    Critical,
}

impl ErrorSeverity {
    /// 获取数值权重（用于排序）
    pub fn weight(&self) -> u8 {
        match self {
            Self::Info => 1,
            Self::Warning => 2,
            Self::Error => 3,
            Self::Critical => 4,
        }
    }

    /// 获取显示符号
    pub fn symbol(&self) -> &'static str {
        match self {
            Self::Info => "ℹ️",
            Self::Warning => "⚠️",
            Self::Error => "❌",
            Self::Critical => "🚨",
        }
    }
}

/// 错误统计数据
#[derive(Debug, Clone, Default)]
pub struct ErrorStatistics {
    /// 总错误数
    pub total_errors: u64,
    /// 按类别分组的错误数
    pub errors_by_category: HashMap<ErrorHandlingCategory, u64>,
    /// 按错误码分组的错误数
    pub errors_by_code: HashMap<LarkErrorCode, u64>,
    /// 按严重级别分组的错误数
    pub errors_by_severity: HashMap<ErrorSeverity, u64>,
    /// 可重试错误数
    pub retryable_errors: u64,
    /// 平均处理时间
    pub average_processing_time: Option<Duration>,
    /// 第一个错误时间
    pub first_error_time: Option<SystemTime>,
    /// 最后一个错误时间
    pub last_error_time: Option<SystemTime>,
}

impl ErrorStatistics {
    /// 计算错误率（每分钟）
    pub fn error_rate_per_minute(&self) -> f64 {
        if let (Some(first), Some(last)) = (self.first_error_time, self.last_error_time) {
            if let Ok(duration) = last.duration_since(first) {
                let minutes = duration.as_secs_f64() / 60.0;
                if minutes > 0.0 {
                    return self.total_errors as f64 / minutes;
                }
            }
        }
        0.0
    }

    /// 获取最常见的错误类别
    pub fn most_common_category(&self) -> Option<ErrorHandlingCategory> {
        self.errors_by_category
            .iter()
            .max_by_key(|(_, count)| *count)
            .map(|(category, _)| *category)
    }

    /// 获取最严重的错误级别
    pub fn highest_severity(&self) -> Option<ErrorSeverity> {
        self.errors_by_severity
            .keys()
            .max_by_key(|severity| severity.weight())
            .copied()
    }

    /// 计算可重试错误百分比
    pub fn retryable_percentage(&self) -> f64 {
        if self.total_errors == 0 {
            0.0
        } else {
            (self.retryable_errors as f64 / self.total_errors as f64) * 100.0
        }
    }

    /// 打印统计摘要
    pub fn print_summary(&self) {
        println!("📊 错误统计摘要:");
        println!("   总错误数: {}", self.total_errors);
        println!("   错误率: {:.2} 错误/分钟", self.error_rate_per_minute());
        println!(
            "   可重试错误: {} ({:.1}%)",
            self.retryable_errors,
            self.retryable_percentage()
        );

        if let Some(category) = self.most_common_category() {
            println!("   最常见类别: {:?}", category);
        }

        if let Some(severity) = self.highest_severity() {
            println!("   最高严重级别: {} {:?}", severity.symbol(), severity);
        }

        if let Some(avg_time) = self.average_processing_time {
            println!("   平均处理时间: {:?}", avg_time);
        }
    }

    /// 打印详细统计
    pub fn print_detailed(&self) {
        self.print_summary();

        println!("\n📈 错误分类统计:");
        for (category, count) in &self.errors_by_category {
            let percentage = (*count as f64 / self.total_errors as f64) * 100.0;
            println!("   {:?}: {} ({:.1}%)", category, count, percentage);
        }

        println!("\n🔢 错误码统计:");
        let mut sorted_codes: Vec<_> = self.errors_by_code.iter().collect();
        sorted_codes.sort_by(|a, b| b.1.cmp(a.1));
        for (code, count) in sorted_codes.iter().take(10) {
            let percentage = (**count as f64 / self.total_errors as f64) * 100.0;
            println!("   {}: {} ({:.1}%)", code, count, percentage);
        }

        println!("\n⚠️ 严重级别统计:");
        for severity in [
            ErrorSeverity::Critical,
            ErrorSeverity::Error,
            ErrorSeverity::Warning,
            ErrorSeverity::Info,
        ] {
            if let Some(count) = self.errors_by_severity.get(&severity) {
                let percentage = (*count as f64 / self.total_errors as f64) * 100.0;
                println!(
                    "   {} {:?}: {} ({:.1}%)",
                    severity.symbol(),
                    severity,
                    count,
                    percentage
                );
            }
        }
    }
}

/// 错误监控器
pub struct ErrorMonitor {
    /// 错误事件历史
    events: Arc<Mutex<Vec<ErrorEvent>>>,
    /// 统计数据
    statistics: Arc<Mutex<ErrorStatistics>>,
    /// 配置
    config: MonitorConfig,
}

/// 监控配置
#[derive(Debug, Clone)]
pub struct MonitorConfig {
    /// 最大保存事件数量
    pub max_events: usize,
    /// 统计时间窗口
    pub time_window: Duration,
    /// 是否启用自动清理
    pub auto_cleanup: bool,
    /// 告警阈值
    pub alert_thresholds: AlertThresholds,
}

impl Default for MonitorConfig {
    fn default() -> Self {
        Self {
            max_events: 1000,
            time_window: Duration::from_secs(24 * 60 * 60), // 24小时
            auto_cleanup: true,
            alert_thresholds: AlertThresholds::default(),
        }
    }
}

/// 告警阈值配置
#[derive(Debug, Clone)]
pub struct AlertThresholds {
    /// 错误率阈值（每分钟）
    pub error_rate_per_minute: f64,
    /// 严重错误阈值
    pub critical_errors_count: u64,
    /// 连续失败阈值
    pub consecutive_failures: u32,
}

impl Default for AlertThresholds {
    fn default() -> Self {
        Self {
            error_rate_per_minute: 10.0,
            critical_errors_count: 5,
            consecutive_failures: 3,
        }
    }
}

impl ErrorMonitor {
    /// 创建新的错误监控器
    pub fn new(config: MonitorConfig) -> Self {
        Self {
            events: Arc::new(Mutex::new(Vec::new())),
            statistics: Arc::new(Mutex::new(ErrorStatistics::default())),
            config,
        }
    }

    /// 使用默认配置创建
    pub fn default() -> Self {
        Self::new(MonitorConfig::default())
    }

    /// 记录错误事件
    pub fn record_error(&self, error: LarkAPIError) {
        let event = ErrorEvent::from_error(error);
        self.record_event(event);
    }

    /// 记录带上下文的错误事件
    pub fn record_error_with_context(&self, error: LarkAPIError, context: HashMap<String, String>) {
        let mut event = ErrorEvent::from_error(error);
        event.context = context;
        self.record_event(event);
    }

    /// 记录错误事件
    pub fn record_event(&self, event: ErrorEvent) {
        // 更新统计数据
        if let Ok(mut stats) = self.statistics.lock() {
            stats.total_errors += 1;

            // 更新分类统计
            *stats.errors_by_category.entry(event.category).or_insert(0) += 1;

            // 更新错误码统计
            if let Some(code) = event.error_code {
                *stats.errors_by_code.entry(code).or_insert(0) += 1;
            }

            // 更新严重级别统计
            let severity = event.severity_level();
            *stats.errors_by_severity.entry(severity).or_insert(0) += 1;

            // 更新可重试统计
            if event.is_retryable {
                stats.retryable_errors += 1;
            }

            // 更新时间范围
            if stats.first_error_time.is_none() {
                stats.first_error_time = Some(event.timestamp);
            }
            stats.last_error_time = Some(event.timestamp);
        }

        // 添加到事件历史
        if let Ok(mut events) = self.events.lock() {
            events.push(event);

            // 自动清理旧事件
            if self.config.auto_cleanup && events.len() > self.config.max_events {
                let len = events.len();
                let max_events = self.config.max_events;
                events.drain(0..(len - max_events));
            }
        }

        // 检查告警条件
        self.check_alerts();
    }

    /// 获取统计数据
    pub fn get_statistics(&self) -> ErrorStatistics {
        self.statistics.lock().unwrap().clone()
    }

    /// 获取最近的错误事件
    pub fn get_recent_events(&self, limit: usize) -> Vec<ErrorEvent> {
        if let Ok(events) = self.events.lock() {
            events.iter().rev().take(limit).cloned().collect()
        } else {
            Vec::new()
        }
    }

    /// 清理旧事件
    pub fn cleanup_old_events(&self) {
        if let Ok(mut events) = self.events.lock() {
            let cutoff_time = SystemTime::now() - self.config.time_window;
            events.retain(|event| event.timestamp >= cutoff_time);
        }
    }

    /// 重置统计数据
    pub fn reset_statistics(&self) {
        if let Ok(mut stats) = self.statistics.lock() {
            *stats = ErrorStatistics::default();
        }
        if let Ok(mut events) = self.events.lock() {
            events.clear();
        }
    }

    /// 检查告警条件
    fn check_alerts(&self) {
        let stats = self.get_statistics();

        // 检查错误率
        if stats.error_rate_per_minute() > self.config.alert_thresholds.error_rate_per_minute {
            self.trigger_alert(
                AlertType::HighErrorRate,
                format!("错误率过高: {:.2} 错误/分钟", stats.error_rate_per_minute()),
            );
        }

        // 检查严重错误
        if let Some(critical_count) = stats.errors_by_severity.get(&ErrorSeverity::Critical) {
            if *critical_count >= self.config.alert_thresholds.critical_errors_count {
                self.trigger_alert(
                    AlertType::CriticalErrors,
                    format!("严重错误过多: {} 个", critical_count),
                );
            }
        }
    }

    /// 触发告警
    fn trigger_alert(&self, alert_type: AlertType, message: String) {
        println!("🚨 告警 [{:?}]: {}", alert_type, message);
        // 这里可以集成外部告警系统
    }

    /// 生成错误报告
    pub fn generate_report(&self) -> ErrorReport {
        let stats = self.get_statistics();
        let recent_events = self.get_recent_events(10);

        ErrorReport {
            statistics: stats,
            recent_events,
            generated_at: SystemTime::now(),
            time_window: self.config.time_window,
        }
    }
}

/// 告警类型
#[derive(Debug)]
enum AlertType {
    HighErrorRate,
    CriticalErrors,
    ConsecutiveFailures,
}

/// 错误报告
#[derive(Debug)]
pub struct ErrorReport {
    /// 统计数据
    pub statistics: ErrorStatistics,
    /// 最近事件
    pub recent_events: Vec<ErrorEvent>,
    /// 报告生成时间
    pub generated_at: SystemTime,
    /// 统计时间窗口
    pub time_window: Duration,
}

impl ErrorReport {
    /// 打印报告
    pub fn print(&self) {
        println!("📋 错误监控报告");
        println!("生成时间: {:?}", self.generated_at);
        println!("统计窗口: {:?}", self.time_window);
        println!("{}", "=".repeat(50));

        self.statistics.print_detailed();

        println!("\n🕒 最近错误事件:");
        for (i, event) in self.recent_events.iter().enumerate() {
            println!(
                "   {}. [{:?}] {} {:?}",
                i + 1,
                event.timestamp,
                event.severity_level().symbol(),
                event.category
            );
        }
    }

    /// 保存到文件
    pub fn save_to_file(&self, path: &str) -> Result<(), std::io::Error> {
        use std::{fs::File, io::Write};

        let mut file = File::create(path)?;

        writeln!(file, "错误监控报告")?;
        writeln!(file, "生成时间: {:?}", self.generated_at)?;
        writeln!(file, "统计窗口: {:?}", self.time_window)?;
        writeln!(file, "{}", "=".repeat(50))?;

        writeln!(file, "\n统计摘要:")?;
        writeln!(file, "总错误数: {}", self.statistics.total_errors)?;
        writeln!(
            file,
            "错误率: {:.2} 错误/分钟",
            self.statistics.error_rate_per_minute()
        )?;
        writeln!(
            file,
            "可重试错误: {:.1}%",
            self.statistics.retryable_percentage()
        )?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_event_creation() {
        let error = LarkAPIError::api_error(403, "Forbidden", None);
        let event = ErrorEvent::from_error(error);

        assert_eq!(event.category, ErrorHandlingCategory::Permission);
        assert_eq!(event.error_code, Some(LarkErrorCode::Forbidden));
        assert!(!event.is_retryable);
    }

    #[test]
    fn test_error_statistics() {
        let mut stats = ErrorStatistics::default();
        stats.total_errors = 100;
        stats.retryable_errors = 60;

        assert_eq!(stats.retryable_percentage(), 60.0);
    }

    #[test]
    fn test_error_monitor() {
        let monitor = ErrorMonitor::default();

        // 记录一些错误
        monitor.record_error(LarkAPIError::api_error(403, "Forbidden", None));
        monitor.record_error(LarkAPIError::api_error(500, "Server Error", None));

        let stats = monitor.get_statistics();
        assert_eq!(stats.total_errors, 2);
        assert_eq!(stats.errors_by_category.len(), 2);
    }

    #[test]
    fn test_error_severity() {
        assert_eq!(ErrorSeverity::Critical.weight(), 4);
        assert_eq!(ErrorSeverity::Warning.weight(), 2);
        assert_eq!(ErrorSeverity::Critical.symbol(), "🚨");
    }
}

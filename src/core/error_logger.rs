/// 错误日志记录模块
///
/// 提供结构化的错误日志记录功能：
/// - 结构化日志输出
/// - 多种日志格式
/// - 日志级别控制
/// - 上下文信息记录
/// - 外部日志系统集成
use std::collections::HashMap;
use std::{fmt, time::SystemTime};

use crate::core::{
    error::LarkAPIError,
    error_codes::LarkErrorCode,
    error_helper::ErrorHandlingCategory,
    error_metrics::{ErrorEvent, ErrorSeverity},
};

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

impl fmt::Display for LogLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
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
    pub fn new(level: LogLevel, message: String) -> Self {
        Self {
            level,
            timestamp: SystemTime::now(),
            message,
            error: None,
            category: None,
            error_code: None,
            context: HashMap::new(),
            caller: None,
        }
    }

    /// 从错误事件创建日志条目
    pub fn from_error_event(event: &ErrorEvent) -> Self {
        let level = LogLevel::from_error_severity(event.severity_level());
        let message = format!("API调用错误: {}", event.error);

        Self {
            level,
            timestamp: event.timestamp,
            message,
            error: Some(event.error.clone()),
            category: Some(event.category),
            error_code: event.error_code,
            context: event.context.clone(),
            caller: None,
        }
    }

    /// 添加上下文信息
    pub fn with_context(mut self, key: &str, value: &str) -> Self {
        self.context.insert(key.to_string(), value.to_string());
        self
    }

    /// 添加调用者信息
    pub fn with_caller(mut self, caller: String) -> Self {
        self.caller = Some(caller);
        self
    }

    /// 设置错误信息
    pub fn with_error(mut self, error: LarkAPIError) -> Self {
        self.error = Some(error);
        self
    }
}

/// 日志格式器
pub trait LogFormatter {
    /// 格式化日志条目
    fn format(&self, entry: &LogEntry) -> String;
}

/// 简单文本格式器
#[derive(Debug, Clone)]
pub struct SimpleFormatter {
    /// 是否包含时间戳
    pub include_timestamp: bool,
    /// 是否使用颜色
    pub use_colors: bool,
}

impl Default for SimpleFormatter {
    fn default() -> Self {
        Self {
            include_timestamp: true,
            use_colors: true,
        }
    }
}

impl LogFormatter for SimpleFormatter {
    fn format(&self, entry: &LogEntry) -> String {
        let mut output = String::new();

        // 添加颜色前缀
        if self.use_colors {
            output.push_str(entry.level.color_code());
        }

        // 添加时间戳
        if self.include_timestamp {
            if let Ok(duration) = entry.timestamp.duration_since(SystemTime::UNIX_EPOCH) {
                let seconds = duration.as_secs();
                let millis = duration.subsec_millis();
                output.push_str(&format!("[{seconds}.{millis:03}] "));
            }
        }

        // 添加日志级别
        output.push_str(&format!("[{}] ", entry.level.label()));

        // 添加消息
        output.push_str(&entry.message);

        // 添加错误信息
        if let Some(ref error) = entry.error {
            output.push_str(&format!(" | 错误: {error}"));
        }

        // 添加错误码
        if let Some(code) = entry.error_code {
            output.push_str(&format!(" | 错误码: {code}"));
        }

        // 添加上下文
        if !entry.context.is_empty() {
            output.push_str(" | 上下文: {");
            for (i, (key, value)) in entry.context.iter().enumerate() {
                if i > 0 {
                    output.push_str(", ");
                }
                output.push_str(&format!("{key}={value}"));
            }
            output.push('}');
        }

        // 添加调用者信息
        if let Some(ref caller) = entry.caller {
            output.push_str(&format!(" | 调用者: {caller}"));
        }

        // 重置颜色
        if self.use_colors {
            output.push_str(LogLevel::reset_color());
        }

        output
    }
}

/// JSON格式器
#[derive(Debug, Clone)]
pub struct JsonFormatter;

impl LogFormatter for JsonFormatter {
    fn format(&self, entry: &LogEntry) -> String {
        use serde_json::{Map, Value};

        let mut json_entry = Map::new();

        // 基础字段
        json_entry.insert(
            "level".to_string(),
            Value::String(entry.level.label().to_string()),
        );
        json_entry.insert(
            "timestamp".to_string(),
            Value::Number(serde_json::Number::from(
                entry
                    .timestamp
                    .duration_since(SystemTime::UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_millis() as u64,
            )),
        );
        json_entry.insert("message".to_string(), Value::String(entry.message.clone()));

        // 错误信息
        if let Some(ref error) = entry.error {
            json_entry.insert("error".to_string(), Value::String(error.to_string()));
        }

        // 错误分类
        if let Some(category) = entry.category {
            json_entry.insert(
                "category".to_string(),
                Value::String(format!("{category:?}")),
            );
        }

        // 错误码
        if let Some(code) = entry.error_code {
            json_entry.insert(
                "error_code".to_string(),
                Value::Number(serde_json::Number::from(code as i32)),
            );
        }

        // 上下文
        if !entry.context.is_empty() {
            let context_value = entry
                .context
                .iter()
                .map(|(k, v)| (k.clone(), Value::String(v.clone())))
                .collect::<Map<String, Value>>();
            json_entry.insert("context".to_string(), Value::Object(context_value));
        }

        // 调用者
        if let Some(ref caller) = entry.caller {
            json_entry.insert("caller".to_string(), Value::String(caller.clone()));
        }

        serde_json::to_string(&Value::Object(json_entry)).unwrap_or_default()
    }
}

/// 结构化格式器
#[derive(Debug, Clone)]
pub struct StructuredFormatter {
    /// 字段分隔符
    pub separator: String,
    /// 键值分隔符
    pub kv_separator: String,
}

impl Default for StructuredFormatter {
    fn default() -> Self {
        Self {
            separator: " | ".to_string(),
            kv_separator: "=".to_string(),
        }
    }
}

impl LogFormatter for StructuredFormatter {
    fn format(&self, entry: &LogEntry) -> String {
        let mut fields = Vec::new();

        // 时间戳
        if let Ok(duration) = entry.timestamp.duration_since(SystemTime::UNIX_EPOCH) {
            fields.push(format!("time{}{}", self.kv_separator, duration.as_millis()));
        }

        // 级别
        fields.push(format!("level{}{}", self.kv_separator, entry.level.label()));

        // 消息
        fields.push(format!("msg{}{}", self.kv_separator, entry.message));

        // 错误信息
        if let Some(ref error) = entry.error {
            fields.push(format!("error{}{}", self.kv_separator, error));
        }

        // 错误分类
        if let Some(category) = entry.category {
            fields.push(format!("category{}{:?}", self.kv_separator, category));
        }

        // 错误码
        if let Some(code) = entry.error_code {
            fields.push(format!("error_code{}{}", self.kv_separator, code as i32));
        }

        // 上下文
        for (key, value) in &entry.context {
            fields.push(format!("{}{}{}", key, self.kv_separator, value));
        }

        // 调用者
        if let Some(ref caller) = entry.caller {
            fields.push(format!("caller{}{}", self.kv_separator, caller));
        }

        fields.join(&self.separator)
    }
}

/// 日志记录器配置
#[derive(Debug, Clone)]
pub struct LoggerConfig {
    /// 最小日志级别
    pub min_level: LogLevel,
    /// 格式器类型
    pub formatter: FormatterType,
    /// 输出目标
    pub output: OutputTarget,
    /// 是否记录上下文信息
    pub include_context: bool,
    /// 是否记录调用栈
    pub include_caller: bool,
}

/// 格式器类型
#[derive(Debug, Clone)]
pub enum FormatterType {
    Simple(SimpleFormatter),
    Json(JsonFormatter),
    Structured(StructuredFormatter),
}

/// 输出目标
#[derive(Debug, Clone)]
pub enum OutputTarget {
    /// 标准输出
    Stdout,
    /// 标准错误
    Stderr,
    /// 文件
    File(String),
    /// 多个目标
    Multiple(Vec<OutputTarget>),
}

impl Default for LoggerConfig {
    fn default() -> Self {
        Self {
            min_level: LogLevel::Info,
            formatter: FormatterType::Simple(SimpleFormatter::default()),
            output: OutputTarget::Stdout,
            include_context: true,
            include_caller: false,
        }
    }
}

/// 错误日志记录器
pub struct ErrorLogger {
    config: LoggerConfig,
}

impl Default for ErrorLogger {
    fn default() -> Self {
        Self::new(LoggerConfig::default())
    }
}

impl ErrorLogger {
    /// 创建新的错误日志记录器
    pub fn new(config: LoggerConfig) -> Self {
        Self { config }
    }

    /// 记录日志条目
    pub fn log(&self, entry: LogEntry) {
        // 检查日志级别
        if entry.level < self.config.min_level {
            return;
        }

        // 格式化日志
        let formatted = match &self.config.formatter {
            FormatterType::Simple(formatter) => formatter.format(&entry),
            FormatterType::Json(formatter) => formatter.format(&entry),
            FormatterType::Structured(formatter) => formatter.format(&entry),
        };

        // 输出日志
        self.output_log(&formatted);
    }

    /// 记录错误
    pub fn error(&self, message: &str) {
        let entry = LogEntry::new(LogLevel::Error, message.to_string());
        self.log(entry);
    }

    /// 记录错误和上下文
    pub fn error_with_context(&self, message: &str, context: HashMap<String, String>) {
        let mut entry = LogEntry::new(LogLevel::Error, message.to_string());
        entry.context = context;
        self.log(entry);
    }

    /// 记录API错误
    pub fn log_api_error(&self, error: &LarkAPIError) {
        let event = ErrorEvent::from_error(error.clone());
        let entry = LogEntry::from_error_event(&event);
        self.log(entry);
    }

    /// 记录错误事件
    pub fn log_error_event(&self, event: &ErrorEvent) {
        let entry = LogEntry::from_error_event(event);
        self.log(entry);
    }

    /// 记录警告
    pub fn warn(&self, message: &str) {
        let entry = LogEntry::new(LogLevel::Warn, message.to_string());
        self.log(entry);
    }

    /// 记录信息
    pub fn info(&self, message: &str) {
        let entry = LogEntry::new(LogLevel::Info, message.to_string());
        self.log(entry);
    }

    /// 记录调试信息
    pub fn debug(&self, message: &str) {
        let entry = LogEntry::new(LogLevel::Debug, message.to_string());
        self.log(entry);
    }

    /// 输出日志到目标
    fn output_log(&self, formatted: &str) {
        match &self.config.output {
            OutputTarget::Stdout => {
                println!("{formatted}");
            }
            OutputTarget::Stderr => {
                eprintln!("{formatted}");
            }
            OutputTarget::File(path) => {
                self.write_to_file(path, formatted);
            }
            OutputTarget::Multiple(targets) => {
                for target in targets {
                    let temp_config = LoggerConfig {
                        output: target.clone(),
                        ..self.config.clone()
                    };
                    let temp_logger = ErrorLogger::new(temp_config);
                    temp_logger.output_log(formatted);
                }
            }
        }
    }

    /// 写入文件
    fn write_to_file(&self, path: &str, content: &str) {
        use std::{fs::OpenOptions, io::Write};

        if let Ok(mut file) = OpenOptions::new().create(true).append(true).open(path) {
            let _ = writeln!(file, "{content}");
        }
    }
}

/// 日志记录器构建器
pub struct LoggerBuilder {
    config: LoggerConfig,
}

impl LoggerBuilder {
    /// 创建新的构建器
    pub fn new() -> Self {
        Self {
            config: LoggerConfig::default(),
        }
    }

    /// 设置最小日志级别
    pub fn min_level(mut self, level: LogLevel) -> Self {
        self.config.min_level = level;
        self
    }

    /// 使用简单格式器
    pub fn simple_format(mut self) -> Self {
        self.config.formatter = FormatterType::Simple(SimpleFormatter::default());
        self
    }

    /// 使用JSON格式器
    pub fn json_format(mut self) -> Self {
        self.config.formatter = FormatterType::Json(JsonFormatter);
        self
    }

    /// 使用结构化格式器
    pub fn structured_format(mut self) -> Self {
        self.config.formatter = FormatterType::Structured(StructuredFormatter::default());
        self
    }

    /// 输出到文件
    pub fn output_to_file(mut self, path: &str) -> Self {
        self.config.output = OutputTarget::File(path.to_string());
        self
    }

    /// 输出到标准错误
    pub fn output_to_stderr(mut self) -> Self {
        self.config.output = OutputTarget::Stderr;
        self
    }

    /// 包含上下文信息
    pub fn include_context(mut self, include: bool) -> Self {
        self.config.include_context = include;
        self
    }

    /// 构建日志记录器
    pub fn build(self) -> ErrorLogger {
        ErrorLogger::new(self.config)
    }
}

impl Default for LoggerBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_log_level_ordering() {
        assert!(LogLevel::Debug < LogLevel::Info);
        assert!(LogLevel::Error > LogLevel::Warn);
        assert!(LogLevel::Critical > LogLevel::Error);
    }

    #[test]
    fn test_log_entry_creation() {
        let entry = LogEntry::new(LogLevel::Error, "Test message".to_string());
        assert_eq!(entry.level, LogLevel::Error);
        assert_eq!(entry.message, "Test message");
        assert!(entry.context.is_empty());
    }

    #[test]
    fn test_simple_formatter() {
        let formatter = SimpleFormatter::default();
        let entry = LogEntry::new(LogLevel::Info, "Test message".to_string());
        let formatted = formatter.format(&entry);

        assert!(formatted.contains("[INFO]"));
        assert!(formatted.contains("Test message"));
    }

    #[test]
    fn test_json_formatter() {
        let formatter = JsonFormatter;
        let entry = LogEntry::new(LogLevel::Error, "Error message".to_string());
        let formatted = formatter.format(&entry);

        assert!(formatted.contains("\"level\":\"ERROR\""));
        assert!(formatted.contains("\"message\":\"Error message\""));
    }

    #[test]
    fn test_structured_formatter() {
        let formatter = StructuredFormatter::default();
        let entry = LogEntry::new(LogLevel::Warn, "Warning message".to_string());
        let formatted = formatter.format(&entry);

        assert!(formatted.contains("level=WARN"));
        assert!(formatted.contains("msg=Warning message"));
    }

    #[test]
    fn test_logger_builder() {
        let logger = LoggerBuilder::new()
            .min_level(LogLevel::Debug)
            .json_format()
            .output_to_stderr()
            .build();

        assert_eq!(logger.config.min_level, LogLevel::Debug);
        matches!(logger.config.formatter, FormatterType::Json(_));
        matches!(logger.config.output, OutputTarget::Stderr);
    }
}

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
    use rstest::rstest;
    use std::collections::HashMap;

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

    // New comprehensive tests

    #[rstest]
    #[case(ErrorSeverity::Info, LogLevel::Info)]
    #[case(ErrorSeverity::Warning, LogLevel::Warn)]
    #[case(ErrorSeverity::Error, LogLevel::Error)]
    #[case(ErrorSeverity::Critical, LogLevel::Critical)]
    fn test_log_level_from_error_severity(
        #[case] severity: ErrorSeverity,
        #[case] expected_level: LogLevel,
    ) {
        assert_eq!(LogLevel::from_error_severity(severity), expected_level);
    }

    #[test]
    fn test_log_level_color_codes() {
        assert_eq!(LogLevel::Debug.color_code(), "\x1b[36m");
        assert_eq!(LogLevel::Info.color_code(), "\x1b[32m");
        assert_eq!(LogLevel::Warn.color_code(), "\x1b[33m");
        assert_eq!(LogLevel::Error.color_code(), "\x1b[31m");
        assert_eq!(LogLevel::Critical.color_code(), "\x1b[35m");
        assert_eq!(LogLevel::reset_color(), "\x1b[0m");
    }

    #[test]
    fn test_log_level_labels() {
        assert_eq!(LogLevel::Debug.label(), "DEBUG");
        assert_eq!(LogLevel::Info.label(), "INFO");
        assert_eq!(LogLevel::Warn.label(), "WARN");
        assert_eq!(LogLevel::Error.label(), "ERROR");
        assert_eq!(LogLevel::Critical.label(), "CRITICAL");
    }

    #[test]
    fn test_log_level_display() {
        assert_eq!(format!("{}", LogLevel::Debug), "DEBUG");
        assert_eq!(format!("{}", LogLevel::Info), "INFO");
    }

    #[test]
    fn test_log_entry_with_context() {
        let entry = LogEntry::new(LogLevel::Error, "Test message".to_string())
            .with_context("key1", "value1")
            .with_context("key2", "value2");

        assert_eq!(entry.context.len(), 2);
        assert_eq!(entry.context.get("key1"), Some(&"value1".to_string()));
        assert_eq!(entry.context.get("key2"), Some(&"value2".to_string()));
    }

    #[test]
    fn test_log_entry_with_caller() {
        let entry = LogEntry::new(LogLevel::Info, "Test message".to_string())
            .with_caller("test_function".to_string());

        assert_eq!(entry.caller, Some("test_function".to_string()));
    }

    #[test]
    fn test_log_entry_with_error() {
        let error = LarkAPIError::MissingAccessToken;
        let entry =
            LogEntry::new(LogLevel::Error, "Test message".to_string()).with_error(error.clone());

        assert!(entry.error.is_some());
        match entry.error.unwrap() {
            LarkAPIError::MissingAccessToken => (),
            _ => panic!("Wrong error type"),
        }
    }

    #[test]
    fn test_log_entry_from_error_event() {
        let error = LarkAPIError::api_error(403, "Forbidden", None);
        let event = ErrorEvent::from_error(error.clone());
        let entry = LogEntry::from_error_event(&event);

        assert_eq!(entry.level, LogLevel::Error);
        assert!(entry.message.contains("API调用错误"));
        assert!(entry.error.is_some());
        assert!(entry.category.is_some());
    }

    #[test]
    fn test_simple_formatter_with_options() {
        // Test without timestamp
        let formatter = SimpleFormatter {
            include_timestamp: false,
            ..Default::default()
        };
        let entry = LogEntry::new(LogLevel::Info, "Test".to_string());
        let formatted = formatter.format(&entry);
        // Should still contain [INFO] but not timestamp
        assert!(formatted.contains("[INFO]"));
        assert!(!formatted.contains(".")); // No milliseconds timestamp

        // Test without colors
        let formatter = SimpleFormatter {
            use_colors: false,
            ..Default::default()
        };
        let formatted = formatter.format(&entry);
        assert!(!formatted.contains("\x1b["));
    }

    #[test]
    fn test_simple_formatter_with_complete_entry() {
        let formatter = SimpleFormatter::default();
        let error = LarkAPIError::api_error(400, "Bad Request", None);
        let mut entry = LogEntry::new(LogLevel::Error, "API call failed".to_string())
            .with_error(error)
            .with_context("endpoint", "/api/test")
            .with_caller("test_function".to_string());
        entry.error_code = Some(LarkErrorCode::BadRequest);

        let formatted = formatter.format(&entry);

        assert!(formatted.contains("[ERROR]"));
        assert!(formatted.contains("API call failed"));
        assert!(formatted.contains("错误:"));
        assert!(formatted.contains("错误码:"));
        assert!(formatted.contains("上下文:"));
        assert!(formatted.contains("调用者:"));
    }

    #[test]
    fn test_json_formatter_complete() {
        let formatter = JsonFormatter;
        let error = LarkAPIError::IllegalParamError("invalid param".to_string());
        let mut entry = LogEntry::new(LogLevel::Error, "Validation failed".to_string())
            .with_error(error)
            .with_context("field", "username")
            .with_caller("validate_user".to_string());
        entry.category = Some(ErrorHandlingCategory::ClientError);
        entry.error_code = Some(LarkErrorCode::BadRequest);

        let formatted = formatter.format(&entry);

        assert!(formatted.contains("\"level\":\"ERROR\""));
        assert!(formatted.contains("\"message\":\"Validation failed\""));
        assert!(formatted.contains("\"error\":"));
        assert!(formatted.contains("\"category\":\"ClientError\""));
        assert!(formatted.contains("\"error_code\":400"));
        assert!(formatted.contains("\"context\":{"));
        assert!(formatted.contains("\"caller\":\"validate_user\""));
    }

    #[test]
    fn test_structured_formatter_custom() {
        let formatter = StructuredFormatter {
            separator: " || ".to_string(),
            kv_separator: ":".to_string(),
        };

        let entry = LogEntry::new(LogLevel::Warn, "Warning message".to_string())
            .with_context("module", "auth");

        let formatted = formatter.format(&entry);

        assert!(formatted.contains("level:WARN"));
        assert!(formatted.contains("msg:Warning message"));
        assert!(formatted.contains("module:auth"));
        assert!(formatted.contains(" || "));
    }

    #[test]
    fn test_logger_config_default() {
        let config = LoggerConfig::default();

        assert_eq!(config.min_level, LogLevel::Info);
        assert!(matches!(config.formatter, FormatterType::Simple(_)));
        assert!(matches!(config.output, OutputTarget::Stdout));
        assert!(config.include_context);
        assert!(!config.include_caller);
    }

    #[test]
    fn test_error_logger_default() {
        let logger = ErrorLogger::default();
        assert_eq!(logger.config.min_level, LogLevel::Info);
    }

    #[test]
    fn test_error_logger_new() {
        let config = LoggerConfig {
            min_level: LogLevel::Debug,
            formatter: FormatterType::Json(JsonFormatter),
            output: OutputTarget::Stderr,
            include_context: false,
            include_caller: true,
        };
        let logger = ErrorLogger::new(config);
        assert_eq!(logger.config.min_level, LogLevel::Debug);
    }

    #[test]
    fn test_error_logger_log_level_filtering() {
        let config = LoggerConfig {
            min_level: LogLevel::Warn,
            ..LoggerConfig::default()
        };
        let logger = ErrorLogger::new(config);

        // This would normally print, but we can't easily test stdout
        // Instead we test the logic by checking the min_level
        assert_eq!(logger.config.min_level, LogLevel::Warn);

        // Debug and Info should be filtered out
        assert!(LogLevel::Debug < logger.config.min_level);
        assert!(LogLevel::Info < logger.config.min_level);

        // Warn, Error, Critical should pass
        assert!(LogLevel::Warn >= logger.config.min_level);
        assert!(LogLevel::Error >= logger.config.min_level);
        assert!(LogLevel::Critical >= logger.config.min_level);
    }

    #[test]
    fn test_error_logger_convenience_methods() {
        let logger = ErrorLogger::default();

        // These methods should not panic
        logger.error("Test error");
        logger.warn("Test warning");
        logger.info("Test info");
        logger.debug("Test debug");
    }

    #[test]
    fn test_error_logger_error_with_context() {
        let logger = ErrorLogger::default();
        let mut context = HashMap::new();
        context.insert("user_id".to_string(), "123".to_string());
        context.insert("action".to_string(), "login".to_string());

        // Should not panic
        logger.error_with_context("Login failed", context);
    }

    #[test]
    fn test_error_logger_log_api_error() {
        let logger = ErrorLogger::default();
        let error = LarkAPIError::api_error(429, "Too Many Requests", None);

        // Should not panic
        logger.log_api_error(&error);
    }

    #[test]
    fn test_error_logger_log_error_event() {
        let logger = ErrorLogger::default();
        let error = LarkAPIError::RequestError("Network timeout".to_string());
        let event = ErrorEvent::from_error(error);

        // Should not panic
        logger.log_error_event(&event);
    }

    #[test]
    fn test_logger_builder_complete() {
        let logger = LoggerBuilder::new()
            .min_level(LogLevel::Critical)
            .simple_format()
            .output_to_file("/tmp/test.log")
            .include_context(false)
            .build();

        assert_eq!(logger.config.min_level, LogLevel::Critical);
        assert!(matches!(logger.config.formatter, FormatterType::Simple(_)));
        assert!(matches!(logger.config.output, OutputTarget::File(_)));
        assert!(!logger.config.include_context);
    }

    #[test]
    fn test_logger_builder_structured_format() {
        let logger = LoggerBuilder::new().structured_format().build();

        assert!(matches!(
            logger.config.formatter,
            FormatterType::Structured(_)
        ));
    }

    #[test]
    fn test_logger_builder_default() {
        let builder = LoggerBuilder::default();
        let logger = builder.build();
        assert_eq!(logger.config.min_level, LogLevel::Info);
    }

    #[test]
    fn test_output_target_multiple() {
        let targets = vec![
            OutputTarget::Stdout,
            OutputTarget::Stderr,
            OutputTarget::File("/tmp/test.log".to_string()),
        ];
        let config = LoggerConfig {
            output: OutputTarget::Multiple(targets),
            ..LoggerConfig::default()
        };
        let logger = ErrorLogger::new(config);

        // Should not panic when logging to multiple targets
        logger.info("Test message for multiple targets");
    }

    #[test]
    fn test_formatter_type_variants() {
        let simple = FormatterType::Simple(SimpleFormatter::default());
        let json = FormatterType::Json(JsonFormatter);
        let structured = FormatterType::Structured(StructuredFormatter::default());

        // Test that the variants exist and can be matched
        match simple {
            FormatterType::Simple(_) => (),
            _ => panic!("Wrong formatter type"),
        }

        match json {
            FormatterType::Json(_) => (),
            _ => panic!("Wrong formatter type"),
        }

        match structured {
            FormatterType::Structured(_) => (),
            _ => panic!("Wrong formatter type"),
        }
    }

    #[test]
    fn test_log_entry_timestamp() {
        let entry = LogEntry::new(LogLevel::Info, "Test".to_string());
        let now = SystemTime::now();

        // Timestamp should be recent (within 1 second)
        let diff = now.duration_since(entry.timestamp).unwrap_or_default();
        assert!(diff.as_secs() < 1);
    }
}

# 飞书SDK错误处理最佳实践指南

## 📖 概述

本指南提供了使用 open-lark SDK 进行错误处理的最佳实践，帮助开发者构建健壮、用户友好的应用程序。

## 🏗️ 错误处理架构

### 分层错误处理模型

```
┌─────────────────────────────────────┐
│           业务逻辑层                  │  ← 处理业务特定错误
├─────────────────────────────────────┤
│           错误管理层                  │  ← 统计、监控、日志
├─────────────────────────────────────┤
│           中间件层                    │  ← 重试、恢复策略
├─────────────────────────────────────┤
│           SDK核心层                   │  ← 错误定义、分类
├─────────────────────────────────────┤
│           传输层                      │  ← HTTP错误、网络异常
└─────────────────────────────────────┘
```

## 🚀 快速开始

### 1. 基础错误处理

```rust
use open_lark::prelude::*;
use open_lark::core::error_helper::ErrorHelper;

async fn basic_error_handling() -> Result<(), Box<dyn std::error::Error>> {
    let client = LarkClient::builder("app_id", "app_secret").build();
    
    match client.im.v1.message.create(&request).await {
        Ok(response) => {
            if response.success() {
                println!("消息发送成功: {:?}", response.data());
            } else {
                // 处理API响应错误
                if let Some(error_code) = response.error_code() {
                    let advice = ErrorHelper::handle_error(
                        &LarkAPIError::api_error(response.code(), response.msg(), None)
                    );
                    println!("错误建议: {}", advice.message);
                    for action in &advice.actions {
                        println!("  - {}", action);
                    }
                }
            }
        }
        Err(error) => {
            // 处理SDK错误
            println!("用户友好消息: {}", error.user_friendly_message());
            
            // 获取详细错误分析
            let context = ErrorHelper::create_error_context(&error);
            context.print_details();
        }
    }
    
    Ok(())
}
```

### 2. 集成错误监控

```rust
use open_lark::core::{
    error_metrics::{ErrorMonitor, MonitorConfig},
    error_logger::{ErrorLogger, LoggerBuilder, LogLevel},
};

async fn setup_error_monitoring() -> (ErrorMonitor, ErrorLogger) {
    // 设置错误监控
    let monitor_config = MonitorConfig {
        max_events: 1000,
        time_window: Duration::from_secs(3600), // 1小时
        auto_cleanup: true,
        ..Default::default()
    };
    let monitor = ErrorMonitor::new(monitor_config);
    
    // 设置错误日志
    let logger = LoggerBuilder::new()
        .min_level(LogLevel::Info)
        .json_format()  // 生产环境推荐JSON格式
        .output_to_file("logs/errors.log")
        .include_context(true)
        .build();
    
    (monitor, logger)
}

async fn api_call_with_monitoring(
    client: &LarkClient,
    monitor: &ErrorMonitor,
    logger: &ErrorLogger,
) -> SDKResult<MessageResponse> {
    let result = client.im.v1.message.create(&request).await;
    
    match &result {
        Ok(response) if !response.success() => {
            // 记录API响应错误
            let error = LarkAPIError::api_error(response.code(), response.msg(), None);
            monitor.record_error(error.clone());
            logger.log_api_error(&error);
        }
        Err(error) => {
            // 记录SDK错误
            monitor.record_error(error.clone());
            logger.log_api_error(error);
        }
        _ => {}
    }
    
    result
}
```

### 3. 智能重试策略

```rust
use open_lark::core::retry_middleware::{
    RetryMiddleware, RetryConfig, RetryStrategyBuilder
};

async fn setup_intelligent_retry() -> RetryMiddleware {
    // 配置重试策略
    let retry_strategy = RetryStrategyBuilder::exponential(
        5,                          // 最大重试5次
        Duration::from_millis(500), // 基础延迟500ms
        Duration::from_secs(30),    // 最大延迟30秒
    );
    
    let retry_config = RetryConfig::new()
        .enabled(true)
        .default_strategy(retry_strategy)
        .server_errors_only()  // 只重试服务器错误
        .on_retry(|attempt| {
            println!("重试第{}次，延迟{:?}", attempt.attempt, attempt.delay);
        });
    
    RetryMiddleware::new(retry_config)
}

async fn api_call_with_retry(
    client: &LarkClient,
    retry_middleware: &RetryMiddleware,
) -> SDKResult<MessageResponse> {
    retry_middleware.execute(|| async {
        client.im.v1.message.create(&request).await
    }).await
}
```

## 📋 错误处理模式

### 1. 错误分类处理模式

```rust
use open_lark::core::error_helper::ErrorHandlingCategory;

async fn categorized_error_handling(error: &LarkAPIError) {
    let advice = ErrorHelper::handle_error(error);
    
    match advice.category {
        ErrorHandlingCategory::Authentication => {
            // 认证错误 - 刷新token
            println!("🔐 认证失败，请检查访问令牌");
            // 实现token刷新逻辑
        }
        
        ErrorHandlingCategory::Permission => {
            // 权限错误 - 提示用户联系管理员
            println!("🚫 权限不足，请联系管理员添加权限");
            // 记录权限请求日志
        }
        
        ErrorHandlingCategory::RateLimit => {
            // 限流错误 - 实现退避策略
            println!("⏰ 请求频率过高，请稍后重试");
            if let Some(delay) = advice.retry_delay {
                tokio::time::sleep(Duration::from_secs(delay)).await;
            }
        }
        
        ErrorHandlingCategory::ServerError => {
            // 服务器错误 - 自动重试
            println!("🔧 服务器错误，正在重试...");
            // 使用重试中间件处理
        }
        
        ErrorHandlingCategory::NetworkError => {
            // 网络错误 - 检查连接
            println!("🌐 网络连接异常，请检查网络设置");
            // 实现网络诊断逻辑
        }
        
        _ => {
            println!("❓ 未知错误类型: {}", advice.message);
        }
    }
}
```

### 2. 渐进式错误恢复模式

```rust
async fn progressive_error_recovery(
    client: &LarkClient,
    request: &CreateMessageRequest,
    max_attempts: u32,
) -> Result<MessageResponse, String> {
    let mut attempt = 0;
    let mut last_error = None;
    
    while attempt < max_attempts {
        attempt += 1;
        
        match client.im.v1.message.create(request).await {
            Ok(response) => {
                if response.success() {
                    return Ok(response);
                } else {
                    // API响应错误 - 分析是否可恢复
                    let error = LarkAPIError::api_error(response.code(), response.msg(), None);
                    if !error.is_retryable() {
                        return Err(error.user_friendly_message());
                    }
                    last_error = Some(error.user_friendly_message());
                }
            }
            Err(error) => {
                // SDK错误 - 检查是否可重试
                if !error.is_retryable() || attempt >= max_attempts {
                    return Err(error.user_friendly_message());
                }
                last_error = Some(error.user_friendly_message());
            }
        }
        
        // 指数退避
        let delay = Duration::from_millis(500 * 2_u64.pow(attempt - 1));
        tokio::time::sleep(delay).await;
    }
    
    Err(last_error.unwrap_or_else(|| "未知错误".to_string()))
}
```

### 3. 上下文感知错误处理模式

```rust
#[derive(Debug)]
struct ApiContext {
    user_id: String,
    operation: String,
    request_id: String,
    retry_count: u32,
}

async fn context_aware_error_handling(
    error: &LarkAPIError,
    context: &ApiContext,
    logger: &ErrorLogger,
) -> bool {  // 返回是否应该重试
    // 记录带上下文的错误
    let mut error_context = HashMap::new();
    error_context.insert("user_id".to_string(), context.user_id.clone());
    error_context.insert("operation".to_string(), context.operation.clone());
    error_context.insert("request_id".to_string(), context.request_id.clone());
    error_context.insert("retry_count".to_string(), context.retry_count.to_string());
    
    logger.error_with_context(&format!("操作失败: {}", error), error_context);
    
    // 根据上下文决定重试策略
    match error {
        LarkAPIError::ApiError { code, .. } => {
            match *code {
                429 => {
                    // 限流错误 - 检查用户重试次数
                    if context.retry_count < 3 {
                        println!("⏰ 用户 {} 遇到限流，第{}次重试", context.user_id, context.retry_count + 1);
                        return true;
                    }
                }
                403 => {
                    // 权限错误 - 记录用户权限问题
                    println!("🚫 用户 {} 权限不足，操作: {}", context.user_id, context.operation);
                    return false;
                }
                500..=599 => {
                    // 服务器错误 - 根据操作类型决定重试
                    match context.operation.as_str() {
                        "send_message" => return context.retry_count < 5,
                        "upload_file" => return context.retry_count < 3,
                        _ => return context.retry_count < 2,
                    }
                }
                _ => {}
            }
        }
        _ => {}
    }
    
    false
}
```

## 🔧 高级配置

### 1. 自定义错误监控配置

```rust
async fn advanced_monitoring_setup() -> ErrorMonitor {
    let config = MonitorConfig {
        max_events: 5000,
        time_window: Duration::from_secs(86400), // 24小时
        auto_cleanup: true,
        alert_thresholds: AlertThresholds {
            error_rate_per_minute: 20.0,     // 每分钟20个错误触发告警
            critical_errors_count: 3,        // 3个严重错误触发告警
            consecutive_failures: 5,         // 连续5次失败触发告警
        },
    };
    
    let monitor = ErrorMonitor::new(config);
    
    // 定期生成报告
    let monitor_clone = monitor.clone();
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(Duration::from_secs(3600)); // 每小时
        loop {
            interval.tick().await;
            let report = monitor_clone.generate_report();
            let _ = report.save_to_file(&format!(
                "reports/hourly_error_report_{}.txt",
                chrono::Utc::now().format("%Y%m%d_%H%M%S")
            ));
        }
    });
    
    monitor
}
```

### 2. 多级日志配置

```rust
async fn multi_level_logging_setup() -> Vec<ErrorLogger> {
    let mut loggers = Vec::new();
    
    // 控制台日志 - 开发环境
    let console_logger = LoggerBuilder::new()
        .min_level(LogLevel::Debug)
        .simple_format()
        .include_context(true)
        .build();
    loggers.push(console_logger);
    
    // 文件日志 - 生产环境
    let file_logger = LoggerBuilder::new()
        .min_level(LogLevel::Info)
        .json_format()
        .output_to_file("logs/app.log")
        .include_context(true)
        .build();
    loggers.push(file_logger);
    
    // 错误专用日志
    let error_logger = LoggerBuilder::new()
        .min_level(LogLevel::Error)
        .structured_format()
        .output_to_file("logs/errors.log")
        .include_context(true)
        .build();
    loggers.push(error_logger);
    
    loggers
}
```

### 3. 动态重试策略

```rust
use std::sync::Arc;
use std::sync::atomic::{AtomicU32, Ordering};

struct DynamicRetryConfig {
    current_error_rate: Arc<AtomicU32>,
    base_strategy: RetryStrategy,
}

impl DynamicRetryConfig {
    fn new() -> Self {
        Self {
            current_error_rate: Arc::new(AtomicU32::new(0)),
            base_strategy: RetryStrategy::default(),
        }
    }
    
    fn get_strategy(&self) -> RetryStrategy {
        let error_rate = self.current_error_rate.load(Ordering::Relaxed);
        
        match error_rate {
            0..=5 => {
                // 低错误率 - 激进重试
                RetryStrategy {
                    max_attempts: 5,
                    base_delay: Duration::from_millis(200),
                    ..self.base_strategy
                }
            }
            6..=20 => {
                // 中等错误率 - 标准重试
                self.base_strategy
            }
            _ => {
                // 高错误率 - 保守重试
                RetryStrategy {
                    max_attempts: 2,
                    base_delay: Duration::from_secs(2),
                    ..self.base_strategy
                }
            }
        }
    }
    
    fn update_error_rate(&self, rate: u32) {
        self.current_error_rate.store(rate, Ordering::Relaxed);
    }
}
```

## 📊 性能优化

### 1. 错误处理性能最佳实践

```rust
// ✅ 推荐：使用引用避免克隆
fn analyze_error(error: &LarkAPIError) -> ErrorHandlingAdvice {
    ErrorHelper::handle_error(error)
}

// ❌ 不推荐：不必要的克隆
fn analyze_error_bad(error: LarkAPIError) -> ErrorHandlingAdvice {
    ErrorHelper::handle_error(&error)
}

// ✅ 推荐：延迟初始化错误上下文
fn handle_error_lazy(error: &LarkAPIError) {
    if error.is_retryable() {
        // 只在需要时创建上下文
        let context = ErrorHelper::create_error_context(error);
        context.print_details();
    }
}

// ✅ 推荐：批量错误处理
async fn batch_error_processing(errors: Vec<LarkAPIError>, monitor: &ErrorMonitor) {
    for error in errors {
        monitor.record_error(error);
    }
    
    // 批量生成报告
    let report = monitor.generate_report();
    report.print();
}
```

### 2. 内存使用优化

```rust
// ✅ 推荐：使用Arc共享错误监控器
type SharedMonitor = Arc<ErrorMonitor>;

async fn shared_monitoring(monitor: SharedMonitor) {
    // 多个任务共享同一个监控器实例
    let monitor1 = Arc::clone(&monitor);
    let monitor2 = Arc::clone(&monitor);
    
    tokio::join!(
        async { monitor1.record_error(error1) },
        async { monitor2.record_error(error2) }
    );
}

// ✅ 推荐：限制错误历史大小
let config = MonitorConfig {
    max_events: 1000,        // 限制内存使用
    auto_cleanup: true,      // 自动清理旧事件
    ..Default::default()
};
```

## 🧪 测试策略

### 1. 错误处理单元测试

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_error_categorization() {
        let error = LarkAPIError::api_error(403, "Forbidden", None);
        let advice = ErrorHelper::handle_error(&error);
        
        assert_eq!(advice.category, ErrorHandlingCategory::Permission);
        assert!(advice.is_recoverable);
        assert!(!advice.is_retryable);
    }
    
    #[tokio::test]
    async fn test_retry_mechanism() {
        let retry_middleware = RetryMiddleware::new(
            RetryConfig::new()
                .default_strategy(RetryStrategy {
                    max_attempts: 3,
                    base_delay: Duration::from_millis(100),
                    ..Default::default()
                })
        );
        
        let mut call_count = 0;
        let result = retry_middleware.execute(|| {
            call_count += 1;
            async {
                if call_count < 3 {
                    Err(LarkAPIError::api_error(500, "Server Error", None))
                } else {
                    Ok("Success")
                }
            }
        }).await;
        
        assert!(result.is_ok());
        assert_eq!(call_count, 3);
    }
    
    #[test]
    fn test_error_monitoring() {
        let monitor = ErrorMonitor::default();
        
        // 模拟不同类型的错误
        monitor.record_error(LarkAPIError::api_error(403, "Forbidden", None));
        monitor.record_error(LarkAPIError::api_error(500, "Server Error", None));
        
        let stats = monitor.get_statistics();
        assert_eq!(stats.total_errors, 2);
        assert_eq!(stats.errors_by_category.len(), 2);
    }
}
```

### 2. 集成测试

```rust
#[tokio::test]
async fn test_integrated_error_handling() {
    let monitor = Arc::new(ErrorMonitor::default());
    let logger = LoggerBuilder::new()
        .min_level(LogLevel::Debug)
        .simple_format()
        .build();
    
    let retry_middleware = RetryMiddleware::new(
        RetryConfig::new()
            .enabled(true)
            .server_errors_only()
    );
    
    // 模拟API调用失败场景
    let result = simulate_api_failure(&retry_middleware, &monitor, &logger).await;
    
    // 验证错误被正确记录和处理
    let stats = monitor.get_statistics();
    assert!(stats.total_errors > 0);
    
    // 验证重试机制
    let retry_stats = retry_middleware.get_stats();
    assert!(retry_stats.retry_count > 0);
}
```

## 🎯 最佳实践总结

### ✅ 推荐做法

1. **分层错误处理** - 在不同层次处理不同类型的错误
2. **用户友好消息** - 始终提供可理解的错误信息
3. **错误分类** - 根据错误类型采取不同的处理策略
4. **智能重试** - 只重试可恢复的错误，使用指数退避
5. **详细日志** - 记录足够的上下文信息便于调试
6. **性能监控** - 监控错误率和处理时间
7. **渐进式恢复** - 从轻微恢复策略到完全重试
8. **上下文感知** - 根据业务上下文调整错误处理

### ❌ 避免的做法

1. **忽略错误** - 永远不要静默忽略错误
2. **盲目重试** - 不要重试不可恢复的错误
3. **泄露技术细节** - 避免向用户显示技术错误信息
4. **同步阻塞** - 避免在错误处理中使用阻塞操作
5. **内存泄漏** - 限制错误历史记录的大小
6. **无限重试** - 始终设置重试次数上限
7. **缺乏监控** - 错误处理必须包含监控和日志
8. **硬编码策略** - 使用可配置的错误处理策略

## 📚 扩展阅读

- [飞书开放平台错误码文档](https://open.feishu.cn/document/home/error-code-description)
- [Rust错误处理最佳实践](https://doc.rust-lang.org/book/ch09-00-error-handling.html)
- [异步错误处理模式](https://rust-lang.github.io/async-book/07_workarounds/02_err_in_async_blocks.html)
- [日志记录最佳实践](https://docs.rs/log/latest/log/)

---

*本文档会随着SDK的发展持续更新，建议定期查看最新版本。*
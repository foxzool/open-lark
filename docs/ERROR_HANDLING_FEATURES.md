# 🛡️ 错误处理系统功能介绍

open-lark SDK 现在具备**企业级错误处理系统**，提供智能错误分析、自动重试、实时监控和用户友好的错误体验。

## ✨ 核心特性

### 🧠 智能错误分析
```rust
use open_lark::core::error_helper::ErrorHelper;

let advice = ErrorHelper::handle_error(&error);
println!("建议: {}", advice.message);
for action in &advice.actions {
    println!("  - {}", action);
}
```

### 🔄 自动重试机制
```rust
use open_lark::core::retry_middleware::{RetryMiddleware, RetryConfig};

let retry_middleware = RetryMiddleware::new(
    RetryConfig::new()
        .enabled(true)
        .server_errors_only()
        .aggressive()
);

let result = retry_middleware.execute(|| async {
    client.im.v1.message.create(&request).await
}).await;
```

### 📊 实时错误监控
```rust
use open_lark::core::error_metrics::ErrorMonitor;

let monitor = ErrorMonitor::default();
monitor.record_error(error);

let stats = monitor.get_statistics();
stats.print_detailed();
```

### 📝 结构化日志
```rust
use open_lark::core::error_logger::{ErrorLogger, LoggerBuilder, LogLevel};

let logger = LoggerBuilder::new()
    .min_level(LogLevel::Info)
    .json_format()
    .output_to_file("logs/errors.log")
    .build();

logger.log_api_error(&error);
```

### 🏷️ 语义化错误码
```rust
use open_lark::core::error_codes::LarkErrorCode;

let error_code = LarkErrorCode::from_code(403).unwrap();
println!("错误: {}", error_code.description());
println!("建议: {}", error_code.detailed_description());

if error_code.is_retryable() {
    println!("可重试，建议延迟: {:?}秒", error_code.suggested_retry_delay());
}
```

## 🚀 快速开始

### 基础错误处理
```rust
use open_lark::core::error_helper::ErrorHelper;

match client.im.v1.message.create(&request).await {
    Ok(response) => {
        if response.success() {
            println!("✅ 消息发送成功");
        } else {
            // 处理API响应错误
            response.print_error_details();
        }
    }
    Err(error) => {
        // 用户友好的错误消息
        println!("❌ {}", error.user_friendly_message());
        
        // 获取详细错误分析
        let context = ErrorHelper::create_error_context(&error);
        context.print_details();
    }
}
```

### 完整错误管理
```rust
use open_lark::core::{
    error_metrics::ErrorMonitor,
    error_logger::{ErrorLogger, LoggerBuilder, LogLevel},
    retry_middleware::{RetryMiddleware, RetryConfig},
};

// 设置监控和日志
let monitor = ErrorMonitor::default();
let logger = LoggerBuilder::new()
    .min_level(LogLevel::Info)
    .simple_format()
    .build();

// 设置重试策略
let retry_middleware = RetryMiddleware::new(
    RetryConfig::new()
        .enabled(true)
        .server_errors_only()
);

// 执行API调用
let result = retry_middleware.execute(|| async {
    client.im.v1.message.create(&request).await
}).await;

// 处理结果
match result {
    Ok(response) => println!("✅ 成功"),
    Err(error) => {
        monitor.record_error(error.clone());
        logger.log_api_error(&error);
        println!("❌ {}", error.user_friendly_message());
    }
}

// 查看统计报告
let report = monitor.generate_report();
report.print();
```

## 📚 完整文档

- **[错误处理最佳实践](ERROR_HANDLING_BEST_PRACTICES.md)** - 完整的使用指南
- **[API文档](https://docs.rs/open-lark)** - 详细的API参考
- **[示例代码](examples/api/)** - 5个完整的演示程序

## 🎯 支持的错误码

支持 30+ 个业务特定错误码，包括：

- **认证类** - 访问令牌、应用票据相关
- **权限类** - 应用权限、文档权限相关  
- **参数类** - 请求参数、文件类型相关
- **资源类** - 用户、群组、文档不存在相关
- **限流类** - API调用频率限制相关
- **服务器类** - 内部错误、服务不可用相关
- **网络类** - 连接超时、DNS解析相关

## 🔧 系统要求

- Rust 1.70+
- tokio (异步运行时)
- 可选: websocket feature for real-time monitoring

## 📈 性能特点

- **零开销抽象** - 编译时优化，运行时高效
- **内存安全** - Rust保证的内存安全
- **并发安全** - Arc/Mutex保证线程安全
- **模块化设计** - 按需使用，最小化依赖

---

通过这个错误处理系统，open-lark SDK 为开发者提供了**世界级的错误处理体验**！🚀
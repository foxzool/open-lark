/// 简化的错误处理演示
///
/// 展示新的错误处理和监控功能：
/// - 错误监控和统计
/// - 结构化错误日志
/// - 用户友好的错误消息
///
/// 使用方法：
/// ```bash
/// cargo run --example simple_error_demo
/// ```
use std::collections::HashMap;
use std::time::Duration;
use tokio::time::sleep;

use open_lark::core::{
    error::LarkAPIError,
    error_codes::LarkErrorCode,
    error_helper::{ErrorContext, ErrorHelper},
    error_logger::{ErrorLogger, LogLevel, LoggerBuilder},
    error_metrics::{ErrorMonitor, MonitorConfig},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 开始错误处理系统演示\n");

    // 1. 创建错误监控器
    println!("📊 初始化错误监控器...");
    let monitor = ErrorMonitor::new(MonitorConfig::default());

    // 2. 创建错误日志记录器
    println!("📝 初始化错误日志记录器...");
    let logger = LoggerBuilder::new()
        .min_level(LogLevel::Info)
        .simple_format()
        .build();

    // 3. 演示不同类型的错误
    println!("\n=== 错误处理演示 ===");

    let test_scenarios = vec![
        ("权限不足", 403, "用户无权限访问此资源"),
        ("访问令牌无效", 99991671, "访问令牌已过期"),
        ("请求过于频繁", 429, "超出API调用频率限制"),
        ("服务器内部错误", 500, "服务器处理请求时发生错误"),
        ("网络连接超时", 0, "connection timeout"),
    ];

    for (scenario_name, error_code, error_msg) in test_scenarios {
        println!("\n🔍 场景: {}", scenario_name);

        let error = if error_code > 0 {
            LarkAPIError::api_error(error_code, error_msg, Some("req_123".to_string()))
        } else {
            LarkAPIError::RequestError(error_msg.to_string())
        };

        // 记录错误到监控系统
        let mut context = HashMap::new();
        context.insert("scenario".to_string(), scenario_name.to_string());
        context.insert("timestamp".to_string(), chrono::Utc::now().to_rfc3339());
        monitor.record_error_with_context(error.clone(), context);

        // 记录错误到日志系统
        logger.log_api_error(&error);

        // 获取错误处理建议
        let advice = ErrorHelper::handle_error(&error);
        println!("   💡 建议: {}", advice.message);
        println!(
            "   🔄 可重试: {}",
            if advice.is_retryable { "是" } else { "否" }
        );

        if !advice.actions.is_empty() {
            println!("   📋 推荐操作:");
            for (i, action) in advice.actions.iter().enumerate() {
                println!("      {}. {}", i + 1, action);
            }
        }

        // 获取用户友好的错误消息
        let user_message = error.user_friendly_message();
        println!("   👤 用户提示: {}", user_message);

        // 创建完整的错误上下文
        let error_context = ErrorHelper::create_error_context(&error);
        if let Some(retry_strategy) = &error_context.retry_strategy {
            println!(
                "   ⏱️ 重试策略: 最多{}次，基础延迟{:?}",
                retry_strategy.max_attempts, retry_strategy.base_delay
            );
        }

        // 模拟处理延迟
        sleep(Duration::from_millis(200)).await;
    }

    // 4. 显示统计摘要
    println!("\n=== 错误统计报告 ===");
    let stats = monitor.get_statistics();
    stats.print_summary();

    // 5. 生成详细报告
    println!("\n=== 详细错误分析 ===");
    let report = monitor.generate_report();
    report.print();

    // 6. 演示错误上下文的详细打印
    println!("\n=== 错误上下文演示 ===");
    let demo_error = LarkAPIError::api_error(403, "权限不足", Some("demo_req".to_string()));
    let context = ErrorHelper::create_error_context(&demo_error);
    context.print_details();

    println!("\n✅ 错误处理系统演示完成！");
    println!("   总错误数: {}", stats.total_errors);
    println!("   错误率: {:.2} 错误/分钟", stats.error_rate_per_minute());

    if let Some(category) = stats.most_common_category() {
        println!("   最常见错误类型: {:?}", category);
    }

    Ok(())
}

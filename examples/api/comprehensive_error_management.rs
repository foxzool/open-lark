/// 综合错误管理示例
///
/// 这个示例展示了open-lark SDK的完整错误管理生态系统：
/// - 智能重试中间件
/// - 错误统计和监控
/// - 结构化日志记录
/// - 错误恢复策略
/// - 性能监控
/// - 告警系统
///
/// 使用方法：
/// cargo run --example comprehensive_error_management
///
/// 环境变量：
/// APP_ID=your_app_id (可选，用于真实API测试)
/// APP_SECRET=your_app_secret (可选，用于真实API测试)
/// LOG_LEVEL=debug|info|warn|error (可选，默认info)
use open_lark::core::{
    error_helper::ErrorHelper,
    error_logger::{ErrorLogger, LogLevel, LoggerBuilder},
    error_metrics::{ErrorMonitor, MonitorConfig},
    retry_middleware::{RetryConfig, RetryMiddleware, RetryStrategyBuilder},
    trait_system::ExecutableBuilder,
};
use open_lark::prelude::*;
use serde_json::json;
use std::{
    collections::HashMap,
    sync::Arc,
    time::{Duration, Instant},
};
use tokio::time::sleep;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 加载环境变量
    dotenvy::dotenv().ok();

    println!("🛡️ 飞书SDK综合错误管理示例");
    println!("{}", "=".repeat(60));

    // 初始化错误管理组件
    let (logger, monitor, retry_middleware) = setup_error_management().await;

    // 演示各种错误管理场景
    demonstrate_retry_strategies(&retry_middleware, &logger, &monitor).await;
    demonstrate_error_monitoring(&monitor, &logger).await;
    demonstrate_structured_logging(&logger).await;
    demonstrate_integrated_workflow(&retry_middleware, &monitor, &logger).await;

    // 生成最终报告
    generate_final_report(&monitor, &logger).await;

    Ok(())
}

/// 设置错误管理组件
async fn setup_error_management() -> (ErrorLogger, Arc<ErrorMonitor>, RetryMiddleware) {
    println!("⚙️ 初始化错误管理组件...");

    // 1. 设置结构化日志记录器
    let log_level = std::env::var("LOG_LEVEL")
        .unwrap_or_else(|_| "info".to_string())
        .to_lowercase();

    let min_level = match log_level.as_str() {
        "debug" => LogLevel::Debug,
        "info" => LogLevel::Info,
        "warn" => LogLevel::Warn,
        "error" => LogLevel::Error,
        "critical" => LogLevel::Critical,
        _ => LogLevel::Info,
    };

    let logger = LoggerBuilder::new()
        .min_level(min_level)
        .structured_format()
        .include_context(true)
        .build();

    logger.info("错误日志记录器初始化完成");

    // 2. 设置错误监控器
    let monitor_config = MonitorConfig {
        max_events: 500,
        time_window: Duration::from_secs(3600), // 1小时
        auto_cleanup: true,
        ..Default::default()
    };

    let monitor = Arc::new(ErrorMonitor::new(monitor_config));
    logger.info("错误监控器初始化完成");

    // 3. 设置重试中间件
    let retry_strategy = RetryStrategyBuilder::exponential(
        5,                          // 最大重试5次
        Duration::from_millis(500), // 基础延迟500ms
        Duration::from_secs(30),    // 最大延迟30秒
    );

    let monitor_clone = Arc::clone(&monitor);
    let retry_config = RetryConfig::new()
        .enabled(true)
        .default_strategy(retry_strategy)
        .server_errors_only()
        .on_retry(move |attempt| {
            // 记录重试事件到监控器
            let mut context = HashMap::new();
            context.insert("retry_attempt".to_string(), attempt.attempt.to_string());
            context.insert("max_attempts".to_string(), attempt.max_attempts.to_string());
            context.insert(
                "delay_ms".to_string(),
                attempt.delay.as_millis().to_string(),
            );

            monitor_clone.record_error_with_context(attempt.error.clone(), context);
        });

    let retry_middleware = RetryMiddleware::new(retry_config);
    logger.info("重试中间件初始化完成");

    println!("✅ 错误管理组件初始化成功\n");

    (logger, monitor, retry_middleware)
}

/// 演示重试策略
async fn demonstrate_retry_strategies(
    retry_middleware: &RetryMiddleware,
    logger: &ErrorLogger,
    monitor: &Arc<ErrorMonitor>,
) {
    println!("📋 场景1: 智能重试策略演示");
    println!("{}", "-".repeat(40));

    logger.info("开始重试策略演示");

    // 模拟不同的错误场景
    let scenarios = vec![
        ("服务器临时错误", 500, 2), // 2次后成功
        ("限流错误", 429, 3),       // 3次后成功
        ("网关超时", 504, 1),       // 1次后成功
        ("权限错误", 403, 0),       // 不可重试，立即失败
    ];

    for (description, error_code, success_after) in scenarios {
        println!("\n🔄 测试场景: {}", description);
        let mut attempt_count = 0;

        let result = retry_middleware
            .execute(|| {
                attempt_count += 1;
                let error =
                    LarkAPIError::api_error(error_code, description, Some("req_123".to_string()));

                async move {
                    if success_after > 0 && attempt_count > success_after {
                        Ok(format!("成功 - 第{}次尝试", attempt_count))
                    } else {
                        Err(error)
                    }
                }
            })
            .await;

        match result {
            Ok(message) => {
                println!("   ✅ {}", message);
                logger.info(&format!("重试成功: {}", description));
            }
            Err(error) => {
                println!("   ❌ 最终失败: {}", error);
                logger.error(&format!("重试失败: {}", description));
                monitor.record_error(error);
            }
        }
    }

    logger.info("重试策略演示完成");
}

/// 演示错误监控
async fn demonstrate_error_monitoring(monitor: &Arc<ErrorMonitor>, logger: &ErrorLogger) {
    println!("\n📋 场景2: 错误监控和统计");
    println!("{}", "-".repeat(40));

    logger.info("开始错误监控演示");

    // 模拟各种错误类型
    let error_scenarios = vec![
        (403, "权限不足", 3),
        (500, "服务器错误", 5),
        (429, "请求过频", 2),
        (404, "资源不存在", 1),
        (99991671, "令牌无效", 4),
    ];

    for (code, message, count) in error_scenarios {
        for i in 1..=count {
            let mut context = HashMap::new();
            context.insert("scenario".to_string(), "monitoring_demo".to_string());
            context.insert("iteration".to_string(), i.to_string());

            let error = LarkAPIError::api_error(code, message, Some(format!("req_{}", i)));
            monitor.record_error_with_context(error.clone(), context);

            // 短暂延迟模拟真实场景
            sleep(Duration::from_millis(100)).await;
        }
    }

    // 展示统计结果
    println!("\n📊 监控统计结果:");
    let stats = monitor.get_statistics();
    stats.print_detailed();

    // 展示最近的错误事件
    println!("\n🕒 最近的错误事件:");
    let recent_events = monitor.get_recent_events(5);
    for (i, event) in recent_events.iter().enumerate() {
        println!(
            "   {}. [{:?}] {} {:?} - {}",
            i + 1,
            event.timestamp,
            event.severity_level().symbol(),
            event.category,
            event.error
        );
    }

    logger.info("错误监控演示完成");
}

/// 演示结构化日志记录
async fn demonstrate_structured_logging(logger: &ErrorLogger) {
    println!("\n📋 场景3: 结构化日志记录");
    println!("{}", "-".repeat(40));

    logger.info("开始结构化日志演示");

    // 演示不同级别的日志
    logger.debug("调试信息: 开始API调用准备");
    logger.info("信息: 正在发送API请求");
    logger.warn("警告: API响应时间较长");

    // 带上下文的错误日志
    let mut error_context = HashMap::new();
    error_context.insert("api_endpoint".to_string(), "/v1/messages".to_string());
    error_context.insert("user_id".to_string(), "ou_example_user".to_string());
    error_context.insert("retry_count".to_string(), "3".to_string());

    logger.error_with_context("API调用失败，已达到最大重试次数", error_context);

    // 记录API错误
    let api_error = LarkAPIError::api_error(429, "Too Many Requests", Some("req_456".to_string()));
    logger.log_api_error(&api_error);

    logger.info("结构化日志演示完成");
}

/// 演示集成工作流
async fn demonstrate_integrated_workflow(
    retry_middleware: &RetryMiddleware,
    monitor: &Arc<ErrorMonitor>,
    logger: &ErrorLogger,
) {
    println!("\n📋 场景4: 集成错误管理工作流");
    println!("{}", "-".repeat(40));

    logger.info("开始集成工作流演示");

    // 模拟真实的API调用场景
    let app_id = std::env::var("APP_ID").unwrap_or_else(|_| "demo_app_id".to_string());
    let app_secret = std::env::var("APP_SECRET").unwrap_or_else(|_| "demo_app_secret".to_string());

    let client = LarkClient::builder(&app_id, &app_secret)
        .with_enable_token_cache(true)
        .build();

    // 使用集成的错误管理执行API调用
    let start_time = Instant::now();
    let mut success_count = 0;
    let mut failure_count = 0;

    for i in 1..=5 {
        logger.debug(&format!("开始第{}次API调用", i));

        let message_body = open_lark::service::im::v1::message::CreateMessageRequestBody::builder()
            .receive_id("demo_user")
            .msg_type("text")
            .content(json!({"text": format!("测试消息 #{}", i)}).to_string())
            .build();

        // 使用重试中间件执行API调用
        let result = retry_middleware
            .execute(|| {
                let client = &client;
                let message_body = message_body.clone();

                async move {
                    open_lark::service::im::v1::message::CreateMessageRequest::builder()
                        .receive_id_type("open_id")
                        .request_body(message_body)
                        .execute(&client.im.v1.message)
                        .await
                }
            })
            .await;

        match result {
            Ok(response) => {
                if response.success() {
                    success_count += 1;
                    logger.info(&format!("API调用 #{} 成功", i));
                } else {
                    failure_count += 1;
                    let error_msg = format!("API响应失败: {}", response.msg());
                    logger.warn(&error_msg);

                    // 记录响应错误到监控系统
                    let api_error = LarkAPIError::api_error(response.code(), response.msg(), None);
                    monitor.record_error(api_error);
                }
            }
            Err(error) => {
                failure_count += 1;
                logger.error(&format!("API调用 #{} 失败: {}", i, error));

                // 记录错误并分析
                let context = ErrorHelper::create_error_context(&error);
                monitor.record_error(error);

                // 根据错误类型采取不同策略
                if context.is_recoverable {
                    logger.info("错误可恢复，继续下一次调用");
                } else {
                    logger.warn("错误不可恢复，可能需要人工干预");
                }
            }
        }

        // 短暂间隔
        sleep(Duration::from_millis(200)).await;
    }

    let total_time = start_time.elapsed();

    // 记录工作流摘要
    let summary = format!(
        "集成工作流完成: 成功{}, 失败{}, 总耗时{:?}",
        success_count, failure_count, total_time
    );

    logger.info(&summary);
    println!("📈 {}", summary);
}

/// 生成最终报告
async fn generate_final_report(monitor: &Arc<ErrorMonitor>, logger: &ErrorLogger) {
    println!("\n📋 场景5: 生成综合错误报告");
    println!("{}", "-".repeat(40));

    logger.info("开始生成最终报告");

    // 生成详细报告
    let report = monitor.generate_report();

    println!("\n📄 错误管理综合报告");
    println!("{}", "=".repeat(50));
    report.print();

    // 保存报告到文件
    if let Err(e) = report.save_to_file("reports/error_management_report.txt") {
        logger.warn(&format!("无法保存报告到文件: {}", e));
    } else {
        logger.info("报告已保存到 reports/error_management_report.txt");
    }

    // 性能分析
    println!("\n⚡ 性能分析:");
    let stats = monitor.get_statistics();
    if let Some(avg_time) = stats.average_processing_time {
        println!("   平均错误处理时间: {:?}", avg_time);
    }
    println!("   错误恢复率: {:.1}%", stats.retryable_percentage());

    // 建议和优化
    println!("\n💡 优化建议:");
    if stats.error_rate_per_minute() > 5.0 {
        println!("   - 错误率较高，建议检查API调用频率");
        logger.warn("错误率过高，需要优化");
    }

    if let Some(category) = stats.most_common_category() {
        println!("   - 最常见错误类型: {:?}，建议重点优化", category);
        logger.info(&format!("需要关注的错误类型: {:?}", category));
    }

    if stats.retryable_percentage() > 70.0 {
        println!("   - 可重试错误比例较高，建议优化重试策略");
        logger.info("建议调整重试策略配置");
    }

    logger.info("最终报告生成完成");

    println!("\n🎉 综合错误管理演示完成！");
    println!("   查看生成的报告文件了解详细信息");
}

/// 模拟网络不稳定的API调用
#[allow(dead_code)]
async fn simulate_unstable_api_call(
    success_rate: f32,
    call_id: u32,
) -> Result<String, LarkAPIError> {
    use rand::Rng;

    let mut rng = rand::thread_rng();
    let random_value: f32 = rng.gen();

    // 模拟网络延迟
    let delay_ms = rng.gen_range(100..500);
    sleep(Duration::from_millis(delay_ms)).await;

    if random_value < success_rate {
        Ok(format!("API调用 #{} 成功", call_id))
    } else {
        // 随机选择错误类型
        let error_codes = [403, 429, 500, 502, 503, 504];
        let error_messages = [
            "权限不足",
            "请求频率过高",
            "内部服务器错误",
            "网关错误",
            "服务不可用",
            "网关超时",
        ];

        let index = rng.gen_range(0..error_codes.len());
        Err(LarkAPIError::api_error(
            error_codes[index],
            error_messages[index],
            Some(format!("req_{}", call_id)),
        ))
    }
}

/// 压力测试场景
#[allow(dead_code)]
async fn stress_test_scenario(
    retry_middleware: &RetryMiddleware,
    monitor: &Arc<ErrorMonitor>,
    logger: &ErrorLogger,
) {
    logger.info("开始压力测试场景");

    let concurrent_calls = 10;
    let calls_per_routine = 5;

    let mut handles = Vec::new();

    for routine_id in 0..concurrent_calls {
        let retry_middleware = retry_middleware.clone();
        let monitor = Arc::clone(monitor);
        let logger = logger.clone();

        let handle = tokio::spawn(async move {
            for call_id in 0..calls_per_routine {
                let global_call_id = routine_id * calls_per_routine + call_id;

                let result = retry_middleware
                    .execute(|| {
                        simulate_unstable_api_call(0.3, global_call_id) // 30% 成功率
                    })
                    .await;

                match result {
                    Ok(msg) => logger.info(&msg),
                    Err(error) => {
                        logger.error(&format!("调用失败: {}", error));
                        monitor.record_error(error);
                    }
                }
            }
        });

        handles.push(handle);
    }

    // 等待所有并发任务完成
    for handle in handles {
        let _ = handle.await;
    }

    logger.info("压力测试场景完成");
}

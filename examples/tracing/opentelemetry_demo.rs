//! OpenTelemetry 集成演示
//!
//! 展示如何使用 OpenTelemetry 进行分布式跟踪

use open_lark::core::observability::{
    init_otel_tracing, shutdown_otel, HttpTracker, OperationTracker, OtelConfig,
};
use std::time::Duration;
use tokio::time::sleep;
use tracing::{error, info, warn};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    println!("=== OpenTelemetry 演示 ===");
    println!("注意：此演示需要运行 OTLP 收集器（如 Jaeger）");
    println!("你可以使用以下命令启动 Jaeger:");
    println!("docker run -d --name jaeger \\");
    println!("  -p 16686:16686 \\");
    println!("  -p 14250:14250 \\");
    println!("  -p 4317:4317 \\");
    println!("  jaegertracing/all-in-one:latest");
    println!("\\n然后访问 http://localhost:16686 查看追踪数据\\n");

    // 配置 OpenTelemetry
    let otel_config = OtelConfig {
        endpoint: "http://localhost:4317".to_string(),
        service_name: "open-lark-demo".to_string(),
        service_version: "1.0.0".to_string(),
        environment: "demo".to_string(),
    };

    // 初始化 OpenTelemetry
    match init_otel_tracing(Some(otel_config)) {
        Ok(()) => {
            info!("OpenTelemetry 初始化成功");

            // 运行演示
            run_demo().await?;

            // 等待数据刷新
            sleep(Duration::from_secs(2)).await;

            // 关闭 OpenTelemetry
            shutdown_otel();
            info!("OpenTelemetry 已关闭");
        }
        Err(e) => {
            eprintln!("OpenTelemetry 初始化失败: {}", e);
            eprintln!("提示：确保 OTLP 收集器（如 Jaeger）正在运行");
            eprintln!("或者运行不带 --features otel 的普通版本");
        }
    }

    println!("=== 演示完成 ===");
    Ok(())
}

/// 运行各种演示场景
async fn run_demo() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // 演示复杂的操作链
    demonstrate_service_operations().await;

    // 演示并发操作
    demonstrate_concurrent_operations().await;

    // 演示错误处理
    demonstrate_error_scenarios().await;

    Ok(())
}

/// 演示服务操作链
async fn demonstrate_service_operations() {
    info!("开始演示服务操作链");

    let main_tracker = OperationTracker::start("demo", "complete_workflow");

    // 第一步：用户认证
    {
        let auth_tracker = OperationTracker::start("auth_service", "authenticate_user");
        sleep(Duration::from_millis(100)).await;
        auth_tracker.success();
    }

    // 第二步：获取用户数据
    {
        let user_tracker = OperationTracker::start("user_service", "get_user_profile");
        sleep(Duration::from_millis(150)).await;
        user_tracker.success();
    }

    // 第三步：调用 Lingo API
    {
        let lingo_tracker = OperationTracker::start("lingo_service", "create_draft");

        // 模拟 HTTP 请求
        let http_tracker =
            HttpTracker::start("POST", "https://open.feishu.cn/open-apis/lingo/v1/drafts");
        sleep(Duration::from_millis(300)).await;
        http_tracker.response(201, Some(512));

        lingo_tracker.success();
    }

    main_tracker.success();
    info!("服务操作链演示完成");
}

/// 演示并发操作
async fn demonstrate_concurrent_operations() {
    info!("开始演示并发操作");

    let main_tracker = OperationTracker::start("demo", "concurrent_requests");

    // 启动多个并发任务
    let tasks = (1..=3).map(|i| {
        tokio::spawn(async move {
            let tracker = OperationTracker::start("worker_service", &format!("task_{}", i));

            // 模拟不同的处理时间
            let delay = Duration::from_millis(50 * i);
            sleep(delay).await;

            info!("任务 {} 完成", i);
            tracker.success();
        })
    });

    // 等待所有任务完成
    for task in tasks {
        task.await.unwrap();
    }

    main_tracker.success();
    info!("并发操作演示完成");
}

/// 演示错误场景
async fn demonstrate_error_scenarios() {
    info!("开始演示错误场景");

    // 演示网络错误
    {
        let tracker = OperationTracker::start("network_service", "failed_request");
        sleep(Duration::from_millis(100)).await;
        warn!("模拟网络超时");
        tracker.error("网络请求超时");
    }

    // 演示 HTTP 错误
    {
        let http_tracker = HttpTracker::start("GET", "https://api.example.com/nonexistent");
        sleep(Duration::from_millis(50)).await;
        http_tracker.response(404, None);
    }

    // 演示应用错误
    {
        let tracker = OperationTracker::start("business_service", "process_data");
        sleep(Duration::from_millis(75)).await;
        error!("模拟业务逻辑错误");
        tracker.error("数据验证失败：缺少必填字段");
    }

    info!("错误场景演示完成");
}

/// 演示分布式跟踪的上下文传播
#[allow(dead_code)]
async fn demonstrate_context_propagation() {
    use tracing::{instrument, Instrument};

    #[instrument]
    async fn service_a() {
        info!("Service A 开始处理");
        service_b().await;
        info!("Service A 处理完成");
    }

    #[instrument]
    async fn service_b() {
        info!("Service B 开始处理");
        sleep(Duration::from_millis(100)).await;
        service_c().await;
        info!("Service B 处理完成");
    }

    #[instrument]
    async fn service_c() {
        info!("Service C 开始处理");
        sleep(Duration::from_millis(50)).await;
        info!("Service C 处理完成");
    }

    // 创建根 span 并执行服务链
    async {
        service_a().await;
    }
    .instrument(tracing::info_span!(
        "distributed_request",
        trace_id = "demo-trace-123"
    ))
    .await;
}

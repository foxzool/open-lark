//! Service 可观测性演示
//!
//! 展示如何使用 Service trait 的可观测性功能

use open_lark::{
    core::{
        config::ConfigBuilder,
        observability::{init_tracing, HttpTracker, OperationTracker},
        trait_system::Service,
    },
    service::lingo::{
        classification::{ClassificationListRequest, ClassificationService},
        draft::{DraftCreateRequest, DraftService},
    },
};
use std::time::Duration;
use tokio::time::sleep;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    // 初始化 tracing
    init_tracing()?;

    println!("=== Service 可观测性演示 ===");

    // 创建配置
    let config = ConfigBuilder::default()
        .app_id("demo_app_id")
        .app_secret("demo_app_secret")
        .build();

    // 创建服务实例
    let draft_service = DraftService::new(config.clone());
    let classification_service = ClassificationService::new(config);

    // 演示基础服务信息
    println!("Draft Service Name: {}", DraftService::service_name());
    println!("Draft Service Version: {}", DraftService::service_version());
    println!(
        "Classification Service Name: {}",
        ClassificationService::service_name()
    );
    println!(
        "Classification Service Version: {}",
        ClassificationService::service_version()
    );

    // 演示手动操作跟踪
    demonstrate_manual_tracking().await;

    // 演示 HTTP 请求跟踪
    demonstrate_http_tracking().await;

    // 演示服务操作跟踪（模拟）
    demonstrate_service_operations(&draft_service, &classification_service).await;

    println!("=== 演示完成 ===");
    Ok(())
}

/// 演示手动操作跟踪
async fn demonstrate_manual_tracking() {
    println!("\\n--- 手动操作跟踪演示 ---");

    // 成功操作
    let tracker = OperationTracker::start("demo_service", "successful_operation");
    sleep(Duration::from_millis(100)).await;
    tracker.success();

    // 失败操作
    let tracker = OperationTracker::start("demo_service", "failed_operation");
    sleep(Duration::from_millis(50)).await;
    tracker.error("模拟错误：操作超时");
}

/// 演示 HTTP 请求跟踪
async fn demonstrate_http_tracking() {
    println!("\\n--- HTTP 请求跟踪演示 ---");

    // 成功的 HTTP 请求模拟
    let tracker = HttpTracker::start("GET", "https://open.feishu.cn/open-apis/lingo/v1/drafts");
    sleep(Duration::from_millis(200)).await;
    tracker.response(200, Some(1024));

    // 失败的 HTTP 请求模拟
    let tracker = HttpTracker::start("POST", "https://open.feishu.cn/open-apis/lingo/v1/drafts");
    sleep(Duration::from_millis(150)).await;
    tracker.response(429, None);

    // 错误的 HTTP 请求模拟
    let tracker = HttpTracker::start(
        "PUT",
        "https://open.feishu.cn/open-apis/lingo/v1/drafts/123",
    );
    sleep(Duration::from_millis(300)).await;
    tracker.error("连接超时");
}

/// 演示服务操作跟踪
async fn demonstrate_service_operations(
    draft_service: &DraftService,
    classification_service: &ClassificationService,
) {
    println!("\\n--- 服务操作跟踪演示 ---");

    // 模拟草稿创建（这里不会真实调用API，因为我们没有有效的token）
    println!("模拟 DraftService::create_draft 操作...");
    let request = DraftCreateRequest {
        entity_id: None,
        main_keys: vec!["演示词条".to_string()],
        aliases: None,
        description: "这是一个演示用的词条描述".to_string(),
        classification_id: None,
        outer_info: None,
        related_meta: None,
    };

    // 由于我们没有有效的token，这里只演示跟踪功能
    // 实际调用会失败，但跟踪功能仍然工作
    let result = draft_service.create_draft(request, None).await;
    match result {
        Ok(_) => println!("草稿创建成功（这在演示中不应该发生）"),
        Err(e) => println!("草稿创建失败（预期结果）: {}", e),
    }

    // 模拟分类查询
    println!("\\n模拟 ClassificationService::list_classifications 操作...");
    let request = ClassificationListRequest {
        page_token: None,
        page_size: Some(10),
        repo_id: None,
    };

    let result = classification_service
        .list_classifications(request, None)
        .await;
    match result {
        Ok(_) => println!("分类查询成功（这在演示中不应该发生）"),
        Err(e) => println!("分类查询失败（预期结果）: {}", e),
    }
}

/// 演示宏的使用
#[allow(dead_code)]
async fn demonstrate_macros() {
    use open_lark::{trace_async_performance, trace_performance};

    println!("\\n--- 性能跟踪宏演示 ---");

    // 同步性能跟踪
    let result = trace_performance!("sync_operation", {
        std::thread::sleep(Duration::from_millis(50));
        42
    });
    println!("同步操作结果: {}", result);

    // 异步性能跟踪
    let result: Result<i32, &str> =
        trace_async_performance!("demo_service", "async_operation", async {
            sleep(Duration::from_millis(100)).await;
            Ok::<i32, &str>(84)
        });
    println!("异步操作结果: {:?}", result);
}

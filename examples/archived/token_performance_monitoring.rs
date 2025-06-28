/// TokenManager性能监控示例
///
/// 此示例展示如何使用TokenManager的性能监控功能
/// 包括实时指标收集和定期性能报告
use open_lark::prelude::*;
use std::time::Duration;
use tokio::time::{interval, sleep};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 初始化日志系统
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("debug")).init();

    println!("🚀 TokenManager性能监控示例");
    println!("================================");

    // 创建客户端配置
    let client = LarkClient::builder("test_app_id", "test_app_secret")
        .with_app_type(AppType::SelfBuild)
        .with_enable_token_cache(true)
        .build();

    // 获取TokenManager的引用
    let token_manager = client.config.token_manager.clone();

    // 启动后台性能监控任务
    let monitoring_task = {
        let token_manager = token_manager.clone();
        tokio::spawn(async move {
            let mut interval = interval(Duration::from_secs(30)); // 每30秒报告一次

            loop {
                interval.tick().await;

                // 输出性能报告
                let manager = token_manager.lock().await;
                manager.log_performance_metrics();

                // 检查是否需要优化警告
                let metrics = manager.metrics();
                if metrics.app_cache_hit_rate() < 0.8 {
                    log::warn!(
                        "⚠️  App token缓存命中率较低: {:.1}%",
                        metrics.app_cache_hit_rate() * 100.0
                    );
                }

                if metrics.tenant_cache_hit_rate() < 0.8 {
                    log::warn!(
                        "⚠️  Tenant token缓存命中率较低: {:.1}%",
                        metrics.tenant_cache_hit_rate() * 100.0
                    );
                }

                if metrics.refresh_success_rate() < 0.95 {
                    log::warn!(
                        "⚠️  Token刷新成功率较低: {:.1}%",
                        metrics.refresh_success_rate() * 100.0
                    );
                }
            }
        })
    };

    // 模拟并发API调用场景
    println!("🔄 开始模拟并发API调用...");

    let mut handles = vec![];

    // 创建10个并发任务，每个任务执行5次token获取
    for task_id in 0..10 {
        let token_manager = token_manager.clone();
        let config = client.config.clone();
        let handle = tokio::spawn(async move {
            for i in 0..5 {
                // 模拟获取app access token
                let manager = token_manager.lock().await;

                // 注意：这里会因为无效的认证信息而失败，但我们主要关注性能指标
                let _ = manager
                    .get_app_access_token(&config, "", &config.app_ticket_manager)
                    .await;

                println!("📊 Task {} - Call {} completed", task_id, i + 1);

                // 模拟一些延迟
                sleep(Duration::from_millis(100)).await;
            }
        });
        handles.push(handle);
    }

    // 等待所有任务完成
    for handle in handles {
        handle.await?;
    }

    // 等待一段时间，让监控输出更多报告
    println!("⏳ 等待性能报告生成...");
    sleep(Duration::from_secs(35)).await;

    // 输出最终性能总结
    println!("\n📈 最终性能总结");
    println!("================");

    let manager = token_manager.lock().await;
    let metrics = manager.metrics();

    println!("📊 综合缓存命中率:");
    println!(
        "  - App Token: {:.2}%",
        metrics.app_cache_hit_rate() * 100.0
    );
    println!(
        "  - Tenant Token: {:.2}%",
        metrics.tenant_cache_hit_rate() * 100.0
    );

    println!("\n🔄 刷新统计:");
    println!("  - 成功率: {:.2}%", metrics.refresh_success_rate() * 100.0);
    println!(
        "  - 成功次数: {}",
        metrics
            .refresh_success
            .load(std::sync::atomic::Ordering::Relaxed)
    );
    println!(
        "  - 失败次数: {}",
        metrics
            .refresh_failures
            .load(std::sync::atomic::Ordering::Relaxed)
    );

    println!("\n🔒 锁使用统计:");
    println!(
        "  - 读锁获取: {} 次",
        metrics
            .read_lock_acquisitions
            .load(std::sync::atomic::Ordering::Relaxed)
    );
    println!(
        "  - 写锁获取: {} 次",
        metrics
            .write_lock_acquisitions
            .load(std::sync::atomic::Ordering::Relaxed)
    );

    let total_locks = metrics
        .read_lock_acquisitions
        .load(std::sync::atomic::Ordering::Relaxed)
        + metrics
            .write_lock_acquisitions
            .load(std::sync::atomic::Ordering::Relaxed);
    if total_locks > 0 {
        let read_ratio = metrics
            .read_lock_acquisitions
            .load(std::sync::atomic::Ordering::Relaxed) as f64
            / total_locks as f64;
        println!("  - 读锁比例: {:.1}% (越高越好)", read_ratio * 100.0);
    }

    // 停止监控任务
    monitoring_task.abort();

    println!("\n✅ 性能监控示例完成");
    println!("\n💡 优化建议:");
    println!("  - 缓存命中率 > 80% 为良好");
    println!("  - 刷新成功率 > 95% 为健康");
    println!("  - 读锁比例 > 70% 表示并发优化有效");

    Ok(())
}

/// 扩展功能：性能基准测试
#[allow(dead_code)]
async fn performance_benchmark() -> Result<(), Box<dyn std::error::Error>> {
    use std::time::Instant;

    println!("🏃‍♂️ 开始性能基准测试...");

    // 创建测试配置
    let client = LarkClient::builder("benchmark_app", "benchmark_secret")
        .with_enable_token_cache(true)
        .build();

    let token_manager = client.config.token_manager.clone();

    // 热身阶段
    println!("🔥 热身阶段...");
    for _ in 0..10 {
        let manager = token_manager.lock().await;
        // 模拟token获取
        let _ = manager.metrics(); // 简单的操作来热身
    }

    // 基准测试：并发读取性能
    println!("📏 测试并发读取性能...");
    let start = Instant::now();
    let concurrent_tasks = 100;
    let mut handles = vec![];

    for _ in 0..concurrent_tasks {
        let token_manager = token_manager.clone();
        handles.push(tokio::spawn(async move {
            let manager = token_manager.lock().await;
            // 模拟缓存读取操作
            let _ = manager.metrics().app_cache_hit_rate();
        }));
    }

    for handle in handles {
        handle.await?;
    }

    let duration = start.elapsed();
    println!(
        "✅ {} 个并发任务完成，耗时: {:?}",
        concurrent_tasks, duration
    );
    println!("📊 平均每个任务: {:?}", duration / concurrent_tasks);

    // 输出性能指标
    let manager = token_manager.lock().await;
    println!("\n{}", manager.metrics().performance_report());

    Ok(())
}

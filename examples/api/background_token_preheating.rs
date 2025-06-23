/// 后台Token预热机制示例
/// 
/// 此示例展示如何启用和使用TokenManager的后台预热功能
/// 包括自动token刷新、故障处理和监控集成

use std::time::Duration;
use tokio::time::sleep;
use open_lark::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 初始化日志系统，启用INFO级别以查看预热日志
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    println!("🚀 Token后台预热机制示例");
    println!("==========================");

    // 创建客户端配置
    let client = LarkClient::builder("demo_app_id", "demo_app_secret")
        .with_app_type(AppType::SelfBuild)
        .with_enable_token_cache(true)
        .build();

    println!("✅ 客户端初始化完成");

    // 获取TokenManager相关组件的引用
    let token_manager = client.config.token_manager.clone();
    let config = client.config.clone();

    // 从token_manager中提取缓存和指标
    let (cache, metrics) = {
        let manager = token_manager.lock().await;
        (manager.get_cache(), manager.get_metrics())
    };

    // 启动后台预热任务（使用自定义配置）
    println!("🔄 启动后台Token预热机制...");
    
    // 为演示创建自定义预热配置
    let preheat_config = PreheatingConfig {
        check_interval_seconds: 120,  // 每2分钟检查一次（演示用）
        preheat_threshold_seconds: 300, // 5分钟阈值（演示用）
        enable_tenant_preheating: true,
        max_concurrent_preheat: 2,
    };
    
    let preheating_handle = open_lark::core::token_manager::TokenManager::start_background_preheating_with_config(
        cache.clone(),
        metrics.clone(),
        config,
        client.config.app_ticket_manager.clone(),
        preheat_config,
    );

    println!("✅ 后台预热任务已启动");
    println!("ℹ️  预热任务会每2分钟检查一次token状态（演示配置）");
    println!("ℹ️  如果token不存在或即将在5分钟内过期，会自动刷新");

    // 模拟应用运行
    println!("\n🏃‍♂️ 模拟应用运行...");
    
    // 显示初始状态
    {
        let manager = token_manager.lock().await;
        let metrics = manager.metrics();
        println!("📊 初始性能指标:");
        println!("  - App Token缓存命中率: {:.2}%", metrics.app_cache_hit_rate() * 100.0);
        println!("  - Token刷新成功次数: {}", metrics.refresh_success.load(std::sync::atomic::Ordering::Relaxed));
        println!("  - Token刷新失败次数: {}", metrics.refresh_failures.load(std::sync::atomic::Ordering::Relaxed));
    }

    // 模拟一些API调用来触发token使用
    println!("\n🔄 模拟API调用以测试token机制...");
    
    for i in 1..=5 {
        println!("📡 模拟API调用 #{}", i);
        
        // 模拟获取token的操作
        let manager = token_manager.lock().await;
        let result = manager.get_app_access_token(
            &client.config,
            "",
            &client.config.app_ticket_manager,
        ).await;

        match result {
            Ok(_) => println!("✅ Token获取成功"),
            Err(e) => println!("❌ Token获取失败: {:?} (这是预期的，因为使用了示例凭据)", e),
        }

        // 显示当前性能指标
        let metrics = manager.metrics();
        println!("📊 当前指标 - 命中率: {:.1}%, 成功: {}, 失败: {}",
                metrics.app_cache_hit_rate() * 100.0,
                metrics.refresh_success.load(std::sync::atomic::Ordering::Relaxed),
                metrics.refresh_failures.load(std::sync::atomic::Ordering::Relaxed));

        drop(manager); // 释放锁
        
        // 短暂等待
        sleep(Duration::from_secs(2)).await;
    }

    // 等待并观察预热机制
    println!("\n⏳ 等待预热检查周期...");
    println!("ℹ️  在生产环境中，预热检查每30分钟运行一次");
    println!("ℹ️  为了演示，我们等待10秒来观察系统状态");

    for countdown in (1..=10).rev() {
        print!("\r⏱️  等待 {} 秒...", countdown);
        sleep(Duration::from_secs(1)).await;
    }
    println!("\r✅ 等待完成        ");

    // 显示最终性能报告
    println!("\n📈 最终性能报告");
    println!("================");
    
    let manager = token_manager.lock().await;
    manager.log_performance_metrics();
    
    let metrics = manager.metrics();
    println!("\n🎯 关键指标总结:");
    println!("  📊 缓存命中率: {:.2}%", metrics.app_cache_hit_rate() * 100.0);
    println!("  ✅ 刷新成功: {} 次", metrics.refresh_success.load(std::sync::atomic::Ordering::Relaxed));
    println!("  ❌ 刷新失败: {} 次", metrics.refresh_failures.load(std::sync::atomic::Ordering::Relaxed));
    println!("  🔒 读锁获取: {} 次", metrics.read_lock_acquisitions.load(std::sync::atomic::Ordering::Relaxed));
    println!("  ✏️  写锁获取: {} 次", metrics.write_lock_acquisitions.load(std::sync::atomic::Ordering::Relaxed));

    // 计算读锁使用比例
    let total_locks = metrics.read_lock_acquisitions.load(std::sync::atomic::Ordering::Relaxed) + 
                      metrics.write_lock_acquisitions.load(std::sync::atomic::Ordering::Relaxed);
    if total_locks > 0 {
        let read_ratio = metrics.read_lock_acquisitions.load(std::sync::atomic::Ordering::Relaxed) as f64 / total_locks as f64;
        println!("  📈 读锁占比: {:.1}% (越高表示并发性能越好)", read_ratio * 100.0);
    }

    drop(manager); // 释放锁

    // 演示手动停止预热任务
    println!("\n🛑 停止后台预热任务...");
    preheating_handle.abort();
    println!("✅ 后台预热任务已停止");

    println!("\n💡 预热机制的优势:");
    println!("  🚀 减少用户等待时间: 提前刷新即将过期的token");
    println!("  🔧 故障隔离: 预热失败不影响主业务逻辑");
    println!("  📊 智能监控: 基于使用模式优化预热策略");
    println!("  ⚡ 性能提升: 避免关键时刻的同步token获取");

    println!("\n✨ 生产环境配置建议:");
    println!("  1. 预热间隔设置:");
    println!("     - 轻量应用: 30-60分钟");
    println!("     - 中等负载: 15-30分钟"); 
    println!("     - 高负载应用: 5-15分钟");
    println!("  2. 预热阈值设置:");
    println!("     - 标准配置: 15分钟（900秒）");
    println!("     - 保守配置: 30分钟（1800秒）");
    println!("     - 激进配置: 5分钟（300秒）");
    println!("  3. 并发控制:");
    println!("     - 单租户应用: max_concurrent_preheat = 1");
    println!("     - 多租户应用: max_concurrent_preheat = 3-5");
    println!("  4. 监控和优化:");
    println!("     - 监控预热成功率 > 95%");
    println!("     - 定期检查预热日志");
    println!("     - 在应用关闭时优雅停止预热任务");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_background_preheating_setup() {
        // 测试预热机制的基本设置
        let client = LarkClient::builder("test_app", "test_secret")
            .with_enable_token_cache(true)
            .build();

        let token_manager = client.config.token_manager.clone();
        let (cache, metrics) = {
            let manager = token_manager.lock().await;
            (manager.get_cache(), manager.get_metrics())
        };

        // 启动预热任务
        let handle = open_lark::core::token_manager::TokenManager::start_background_preheating(
            cache,
            metrics,
            client.config.clone(),
            client.config.app_ticket_manager.clone(),
        );

        // 验证任务已启动
        assert!(!handle.is_finished());

        // 停止任务
        handle.abort();
        
        // 短暂等待确保任务停止
        tokio::time::sleep(Duration::from_millis(100)).await;
        assert!(handle.is_finished());
    }

    #[tokio::test]
    async fn test_preheating_metrics_integration() {
        // 测试预热与监控系统的集成
        let client = LarkClient::builder("test_app", "test_secret").build();
        let token_manager = client.config.token_manager.lock().await;
        
        // 验证指标初始状态
        let metrics = token_manager.metrics();
        assert_eq!(metrics.refresh_success.load(std::sync::atomic::Ordering::Relaxed), 0);
        assert_eq!(metrics.refresh_failures.load(std::sync::atomic::Ordering::Relaxed), 0);
        
        // 预热机制会在实际使用中更新这些指标
    }
}
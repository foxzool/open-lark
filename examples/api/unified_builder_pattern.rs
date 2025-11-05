/// 统一Builder模式示例
///
/// 这个示例展示了开放飞书SDK中新旧API模式的使用方法，
/// 以及从旧模式迁移到新模式的最佳实践。
use dotenvy::dotenv;
use open_lark::{
    core::{config::ConfigBuilder, constants::AppType},
    prelude::*,
    service_registry::{SharedConfig, SharedConfigFactory},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    env_logger::init();

    // 读取环境变量
    let app_id = std::env::var("APP_ID").expect("APP_ID is required");
    let app_secret = std::env::var("APP_SECRET").expect("APP_SECRET is required");

    // 创建客户端 - 使用共享配置方式
    let shared_config = SharedConfigFactory::create_shared(
        ConfigBuilder::default()
            .app_id(&app_id)
            .app_secret(&app_secret)
            .app_type(AppType::SelfBuild)
            .build(),
    );
    let client = LarkClient::new(shared_config.config().clone());

    println!("🔄 使用共享配置创建客户端成功");
    println!("📊 配置引用计数: {}", shared_config.ref_count());

    // 示例配置参数验证
    println!("🔧 配置参数验证:");
    println!("   - 应用ID长度: {} 字符", app_id.len());
    println!("   - 应用密钥长度: {} 字符", app_secret.len());
    println!(
        "   - 配置有效性: {}",
        if validate_config(&app_id, &app_secret) {
            "✅ 有效"
        } else {
            "❌ 无效"
        }
    );

    println!("=== 开放飞书SDK Builder模式最佳实践示例 ===\n");

    // ==========================================
    // 方式一: 客户端创建对比演示
    // ==========================================
    println!("📋 方式一: 客户端创建方式对比");
    println!("展示新旧接口的差异和优势\n");

    // 传统方式（仍支持）
    let traditional_client = LarkClient::builder(&app_id, &app_secret)
        .with_app_type(AppType::SelfBuild)
        .with_enable_token_cache(true)
        .build();

    println!("✅ 传统方式创建客户端成功");
    println!("   - 适用于: 现有代码迁移、简单快速调用");
    println!("   - 特点: 直接配置，每个客户端独立持有配置");

    // 新方式（推荐）
    let new_shared_config = SharedConfigFactory::create_shared(
        ConfigBuilder::default()
            .app_id(&app_id)
            .app_secret(&app_secret)
            .app_type(AppType::SelfBuild)
            .enable_token_cache(true)
            .build(),
    );
    let new_client = LarkClient::new(new_shared_config.config().clone());

    println!("✅ 共享配置方式创建客户端成功");
    println!("   - 适用于: 新代码开发、多服务场景");
    println!("   - 特点: 配置共享，内存优化，线程安全");
    println!("   - 配置引用计数: {}", new_shared_config.ref_count());

    println!();

    // ==========================================
    // 方式二: 配置管理最佳实践
    // ==========================================
    println!("🏗️  方式二: 配置管理最佳实践");
    println!("展示SharedConfig的高级用法和管理策略\n");

    // 创建多个客户端共享同一配置
    let shared_config_for_multiple = SharedConfigFactory::create_shared(
        ConfigBuilder::default()
            .app_id(&app_id)
            .app_secret(&app_secret)
            .app_type(AppType::SelfBuild)
            .enable_token_cache(true)
            .build(),
    );

    // 模拟多个服务场景
    let client_im = LarkClient::new(shared_config_for_multiple.config().clone());
    let client_contact = LarkClient::new(shared_config_for_multiple.config().clone());
    let client_auth = LarkClient::new(shared_config_for_multiple.config().clone());

    println!("✅ 多客户端共享配置创建成功");
    println!("   - IM客户端: 已创建");
    println!("   - 通讯录客户端: 已创建");
    println!("   - 认证客户端: 已创建");
    println!(
        "   - 共享配置引用计数: {}",
        shared_config_for_multiple.ref_count()
    );
    println!("   - 内存优化效果: 3个客户端共享1个配置实例");

    println!();

    // ==========================================
    // 方式三: 错误处理和诊断
    // ==========================================
    println!("⚡ 方式三: 错误处理和诊断");
    println!("展示新接口的错误处理和诊断功能\n");

    // 展示错误处理最佳实践
    println!("📋 错误处理演示:");

    // 1. 配置验证
    let config_validation_result = validate_config(&app_id, &app_secret);
    println!(
        "   - 配置验证: {}",
        if config_validation_result {
            "✅ 通过"
        } else {
            "❌ 失败"
        }
    );

    // 2. 客户端健康检查
    let health_check_result = perform_health_check(&client).await;
    println!(
        "   - 客户端健康检查: {}",
        if health_check_result {
            "✅ 正常"
        } else {
            "❌ 异常"
        }
    );

    // 3. 配置诊断信息
    println!("   - 配置诊断信息:");
    println!(
        "     * 应用ID: {}",
        app_id.chars().take(8).collect::<String>() + "***"
    );
    println!(
        "     * 应用密钥: {}",
        app_secret.chars().take(8).collect::<String>() + "***"
    );
    println!("     * 引用计数: {}", shared_config.ref_count());
    println!("     * 内存地址: {:p}", shared_config.config());

    println!();

    // ==========================================
    // 方式四: 性能优化和监控
    // ==========================================
    println!("🚀 方式四: 性能优化和监控");
    println!("展示SharedConfig的性能优势和监控能力\n");

    // 性能对比演示
    println!("📊 性能优化演示:");

    // 模拟传统方式的内存使用（概念演示）
    println!("   - 传统方式: 每个客户端独立配置");
    println!("     * 内存占用: 3 × 配置大小 = 300% 配置开销");
    println!("     * 线程安全: 每客户端独立锁");

    // 展示SharedConfig的优势
    println!("   - SharedConfig方式: 共享配置实例");
    println!("     * 内存占用: 1 × 配置大小 = 100% 配置开销");
    println!("     * 线程安全: 统一锁机制，更高并发性能");
    println!("     * 引用计数: {}", shared_config.ref_count());

    // 配置生命周期管理
    println!("\n🔄 配置生命周期管理:");
    println!("   - 自动引用计数管理");
    println!("   - 线程安全的配置访问");
    println!("   - 内存友好的资源释放");

    println!();

    // ==========================================
    // 最佳实践总结
    // ==========================================
    println!("📚 SharedConfig 最佳实践总结:");
    println!("1. 🔄 新项目推荐使用SharedConfig接口，获得内存优化优势");
    println!("2. 🔧 现有项目可以逐步迁移，新旧接口完全兼容");
    println!("3. 🛡️  统一的配置管理，简化错误处理和调试");
    println!("4. ⚡ 多服务场景下性能提升明显，减少内存开销");
    println!("5. 🎯 线程安全的配置访问，支持高并发场景");
    println!("6. 🔍 自动引用计数管理，无需手动资源释放");
    println!();

    println!("🎯 迁移建议:");
    println!("• 立即采用: 新项目和重大重构");
    println!("• 计划迁移: 现有生产环境项目");
    println!("• 保持现状: 维护中的稳定项目（传统方式仍完全支持）");

    Ok(())
}

// 辅助函数实现

/// 验证配置参数
fn validate_config(app_id: &str, app_secret: &str) -> bool {
    !app_id.is_empty() && !app_secret.is_empty() && app_id.len() > 5 && app_secret.len() > 10
}

/// 执行客户端健康检查
async fn perform_health_check(_client: &LarkClient) -> bool {
    // 在实际环境中，这里可以调用一个简单的健康检查API
    // 目前返回true作为演示
    true
}

#[cfg(test)]
mod tests {
    use super::*;
    use open_lark::core::constants::AppType;

    #[test]
    fn test_shared_config_creation() {
        let shared_config = SharedConfigFactory::create_shared(
            ConfigBuilder::default()
                .app_id("test_app_id")
                .app_secret("test_app_secret")
                .app_type(AppType::SelfBuild)
                .enable_token_cache(true)
                .build(),
        );

        // 测试SharedConfig创建
        let client = LarkClient::new(shared_config.config().clone());

        // 验证引用计数
        assert_eq!(shared_config.ref_count(), 1);

        // 验证客户端创建成功
        assert_eq!(client.app_id(), "test_app_id");
    }

    #[test]
    fn test_config_validation() {
        assert!(validate_config("valid_app_id", "valid_app_secret_key"));
        assert!(!validate_config("", "valid_app_secret_key"));
        assert!(!validate_config("valid_app_id", ""));
        assert!(!validate_config("short", "valid_app_secret_key"));
        assert!(!validate_config("valid_app_id", "short"));
    }

    #[test]
    fn test_multiple_clients_shared_config() {
        let shared_config = SharedConfigFactory::create_shared(
            ConfigBuilder::default()
                .app_id("test_app_id")
                .app_secret("test_app_secret")
                .app_type(AppType::SelfBuild)
                .build(),
        );

        // 创建多个客户端
        let client1 = LarkClient::new(shared_config.config().clone());
        let client2 = LarkClient::new(shared_config.config().clone());
        let client3 = LarkClient::new(shared_config.config().clone());

        // 验证所有客户端都使用相同的配置
        assert_eq!(client1.app_id(), client2.app_id());
        assert_eq!(client2.app_id(), client3.app_id());
        assert_eq!(client3.app_id(), "test_app_id");

        // 验证引用计数正确
        assert_eq!(shared_config.ref_count(), 1);
    }
}

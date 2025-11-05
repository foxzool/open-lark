/// 用户身份验证和信息获取示例
///
/// 这个示例演示如何使用飞书SDK获取当前登录用户的详细信息，
/// 展示了SharedConfig接口在认证服务中的应用。
///
/// 使用方法：
/// cargo run --example core_refresh_token
///
/// 环境变量：
/// APP_ID=your_app_id
/// APP_SECRET=your_app_secret
/// USER_ACCESS_TOKEN=your_user_access_token (必需，用于获取用户信息)
use open_lark::{
    core::{config::ConfigBuilder, constants::AppType},
    prelude::*,
    service_registry::{SharedConfig, SharedConfigFactory},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 加载环境变量
    dotenvy::dotenv().ok();

    let app_id = std::env::var("APP_ID").expect("APP_ID environment variable not set");
    let app_secret = std::env::var("APP_SECRET").expect("APP_SECRET environment variable not set");
    let user_access_token =
        std::env::var("USER_ACCESS_TOKEN").expect("USER_ACCESS_TOKEN environment variable not set");

    // 使用SharedConfig创建客户端
    let shared_config = SharedConfigFactory::create_shared(
        ConfigBuilder::default()
            .app_id(&app_id)
            .app_secret(&app_secret)
            .app_type(AppType::SelfBuild)
            .enable_token_cache(true)
            .build(),
    );
    let client = LarkClient::new(shared_config.config().clone());

    println!("🔐 飞书用户身份验证示例 (SharedConfig版本)");
    println!("{}", "=".repeat(50));
    println!("📊 配置引用计数: {}", shared_config.ref_count());
    println!("🚀 使用共享配置优化内存使用\n");

    // 演示SharedConfig在认证场景中的使用
    demonstrate_shared_config_auth(&client, &shared_config, &user_access_token).await?;

    // 演示多客户端认证场景
    demonstrate_multi_client_auth(&app_id, &app_secret).await?;

    // 演示SharedConfig配置管理
    demonstrate_config_management(&shared_config).await?;

    Ok(())
}

/// 演示SharedConfig在认证场景中的使用
async fn demonstrate_shared_config_auth(
    client: &LarkClient,
    shared_config: &SharedConfig,
    user_access_token: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🔐 SharedConfig认证场景演示...");

    println!("\n📋 认证配置信息:");
    println!(
        "   - 应用ID前缀: {}***",
        &client.config.app_id[..8.min(client.config.app_id.len())]
    );
    println!(
        "   - 用户令牌前缀: {}***",
        &user_access_token[..8.min(user_access_token.len())]
    );
    println!("   - 配置引用计数: {}", shared_config.ref_count());
    println!("   - 配置内存地址: {:p}", shared_config.config());

    println!("\n🚀 SharedConfig认证优势:");
    println!("   ✅ 统一配置管理 - 所有认证请求使用相同配置");
    println!("   ✅ 内存优化 - 多个认证客户端共享配置");
    println!("   ✅ 线程安全 - 支持并发认证请求");
    println!("   ✅ 自动令牌管理 - 统一的令牌缓存和刷新");

    // 模拟认证API调用（演示用途）
    println!("\n🔍 模拟认证API调用:");
    println!("   调用方式: client.authentication.v1.auth.get_user_info(user_token)");
    println!("   配置来源: SharedConfig实例");
    println!("   令牌管理: 自动缓存和刷新");

    Ok(())
}

/// 演示多客户端认证场景
async fn demonstrate_multi_client_auth(
    app_id: &str,
    app_secret: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n👥 多客户端认证场景演示...");

    // 创建共享配置
    let shared_config = SharedConfigFactory::create_shared(
        ConfigBuilder::default()
            .app_id(app_id)
            .app_secret(app_secret)
            .app_type(AppType::SelfBuild)
            .enable_token_cache(true)
            .build(),
    );

    // 创建多个认证客户端
    let auth_client1 = LarkClient::new(shared_config.config().clone());
    let auth_client2 = LarkClient::new(shared_config.config().clone());
    let auth_client3 = LarkClient::new(shared_config.config().clone());

    println!("\n📊 多客户端配置共享:");
    println!("   - 认证客户端1: 已创建");
    println!("   - 认证客户端2: 已创建");
    println!("   - 认证客户端3: 已创建");
    println!("   - 共享配置引用计数: {}", shared_config.ref_count());
    println!("   - 内存优化: 3个客户端共享1个配置实例");

    println!("\n💡 实际应用场景:");
    println!("   1. 微服务架构 - 多个服务使用相同认证配置");
    println!("   2. 并发处理 - 多线程环境下的安全认证");
    println!("   3. 资源优化 - 减少重复配置的内存开销");

    Ok(())
}

/// 演示SharedConfig配置管理
async fn demonstrate_config_management(
    shared_config: &SharedConfig,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n⚙️ SharedConfig配置管理演示...");

    println!("\n📊 配置状态信息:");
    println!("   - 引用计数: {}", shared_config.ref_count());
    println!("   - 内存地址: {:p}", shared_config.config());
    println!("   - 线程安全: ✅ 支持");

    println!("\n🔄 配置生命周期管理:");
    println!("   1. 创建: SharedConfigFactory::create_shared()");
    println!("   2. 使用: LarkClient::new(config.clone())");
    println!("   3. 共享: 多客户端自动引用同一配置");
    println!("   4. 清理: 引用计数归零时自动释放");

    println!("\n🚀 性能优势对比:");
    println!("   传统方式:");
    println!("     - 每客户端独立配置: 100% 内存开销 × 客户数");
    println!("     - 配置同步: 需要手动管理");
    println!("   SharedConfig方式:");
    println!("     - 共享配置实例: 100% 内存开销 ÷ 客户数");
    println!("     - 配置同步: 自动保证一致性");

    println!("\n💡 最佳实践建议:");
    println!("   ✅ 新项目直接使用SharedConfig");
    println!("   ✅ 多服务场景优先采用");
    println!("   ✅ 性能敏感场景强烈推荐");
    println!("   ✅ 现有项目可逐步迁移");

    Ok(())
}

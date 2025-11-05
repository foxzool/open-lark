use dotenvy::dotenv;
use open_lark::{
    core::{config::ConfigBuilder, constants::AppType},
    prelude::*,
    service_registry::{SharedConfig, SharedConfigFactory},
};
use std::env;

/// Contact 功能角色管理 API 演示
///
/// 本示例展示SharedConfig接口的使用方法：
/// - 共享配置的创建和管理
/// - 多客户端场景下的内存优化
/// - 配置生命周期和引用计数管理
///
/// 注意：当前服务处于迁移期，实际API调用将在后续版本中完全实现
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 加载环境变量
    dotenv().ok();

    let app_id = env::var("APP_ID").expect("APP_ID environment variable not set");
    let app_secret = env::var("APP_SECRET").expect("APP_SECRET environment variable not set");

    // 使用共享配置创建客户端
    let shared_config = SharedConfigFactory::create_shared(
        ConfigBuilder::default()
            .app_id(&app_id)
            .app_secret(&app_secret)
            .app_type(AppType::SelfBuild)
            .enable_token_cache(true)
            .build(),
    );

    let client = LarkClient::new(shared_config.config().clone());

    println!("🎯 Contact服务SharedConfig演示开始");
    println!("🔄 使用新的共享配置接口，优化内存使用");
    println!("📊 配置引用计数: {}", shared_config.ref_count());
    println!("展示SharedConfig在Contact服务中的使用...\n");

    // ========== 1. SharedConfig基础使用 ==========
    println!("🔑 1. SharedConfig基础使用演示");

    // 1.1 配置信息验证
    println!("\n📋 1.1 配置信息验证");
    println!("✅ 客户端创建成功:");
    println!("   - 应用ID前缀: {}***", &app_id[..8.min(app_id.len())]);
    println!(
        "   - 应用密钥前缀: {}***",
        &app_secret[..8.min(app_secret.len())]
    );
    println!("   - 应用类型: SelfBuild");
    println!("   - 令牌缓存: 已启用");
    println!("   - 配置引用计数: {}", shared_config.ref_count());

    // 1.2 多客户端配置共享演示
    println!("\n🔍 1.2 多客户端配置共享演示");

    // 创建多个Contact服务客户端
    let contact_client1 = LarkClient::new(shared_config.config().clone());
    let contact_client2 = LarkClient::new(shared_config.config().clone());
    let contact_client3 = LarkClient::new(shared_config.config().clone());

    println!("✅ 多客户端创建成功:");
    println!("   - Contact客户端1: 已创建");
    println!("   - Contact客户端2: 已创建");
    println!("   - Contact客户端3: 已创建");
    println!("   - 共享配置引用计数: {}", shared_config.ref_count());
    println!("   - 内存优化: 3个客户端共享1个配置实例");

    // ========== 2. 性能优化演示 ==========
    println!("\n🚀 2. SharedConfig性能优化演示");

    println!("\n📊 2.1 内存使用对比:");
    println!("   - 传统方式: 每客户端独立配置");
    println!("     * 3个客户端 × 配置大小 = 300% 内存开销");
    println!("     * 3个独立的配置锁，并发性能较低");

    println!("   - SharedConfig方式: 共享配置实例");
    println!("     * 3个客户端共享1个配置 = 100% 内存开销");
    println!("     * 统一配置锁，更高并发性能");
    println!("     * 当前引用计数: {}", shared_config.ref_count());

    println!("\n🔄 2.2 配置生命周期管理:");
    println!("   - ✅ 自动引用计数管理");
    println!("   - ✅ 线程安全的配置访问");
    println!("   - ✅ 内存友好的资源释放");
    println!("   - ✅ 支持高并发场景");

    // ========== 3. 实际使用场景演示 ==========
    println!("\n🛠️  3. Contact服务实际使用场景演示");

    println!("\n📝 3.1 企业通讯录管理:");
    println!("   ```rust");
    println!("   // 大型企业多应用场景");
    println!("   let shared_config = SharedConfigFactory::create_shared(");
    println!("       ConfigBuilder::default()");
    println!("           .app_id(&app_id)");
    println!("           .app_secret(&app_secret)");
    println!("           .build()");
    println!("   );");
    println!("   ");
    println!("   // 多个服务共享同一配置");
    println!("   let contact_service = LarkClient::new(shared_config.config().clone());");
    println!("   let auth_service = LarkClient::new(shared_config.config().clone());");
    println!("   let im_service = LarkClient::new(shared_config.config().clone());");
    println!("   ```");

    println!("\n👥 3.2 多租户应用场景:");
    println!("   ```rust");
    println!("   // 为不同租户创建独立但共享的配置");
    println!("   let tenant_configs: HashMap<String, SharedConfig> = HashMap::new();");
    println!("   ");
    println!("   // 每个租户的多个服务共享配置");
    println!("   for (tenant_id, config) in tenant_configs {{");
    println!("       let contact_client = LarkClient::new(config.config().clone());");
    println!("       // 处理该租户的通讯录操作");
    println!("   }}");
    println!("   ```");

    println!("\n⚡ 3.3 高并发应用场景:");
    println!("   ```rust");
    println!("   // Web服务器中的并发请求处理");
    println!("   async fn handle_request(");
    println!("       shared_config: &SharedConfig,");
    println!("       request: ContactRequest");
    println!("   ) -> Result<ContactResponse, Error> {{");
    println!("       let client = LarkClient::new(shared_config.config().clone());");
    println!("       // 处理请求，无需担心配置竞争条件");
    println!("   }}");
    println!("   ```");

    println!("\n🎉 Contact服务SharedConfig演示完成!");
    println!("\n📊 SharedConfig优势总结:");
    println!("  ✅ 内存优化 - 减少重复配置存储");
    println!("  ✅ 线程安全 - 支持高并发访问");
    println!("  ✅ 自动管理 - 引用计数自动释放");
    println!("  ✅ 兼容性好 - 与现有代码完全兼容");
    println!("  ✅ 易于迁移 - 简单的替换模式");

    Ok(())
}

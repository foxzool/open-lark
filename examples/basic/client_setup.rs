use open_lark::core::config::ConfigBuilder;
use open_lark::prelude::*;
use open_lark::service_registry::{SharedConfig, SharedConfigFactory};
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 加载环境变量
    dotenvy::dotenv().ok();

    let app_id = std::env::var("APP_ID").expect("APP_ID environment variable not set");
    let app_secret = std::env::var("APP_SECRET").expect("APP_SECRET environment variable not set");

    println!("🚀 OpenLark SDK 客户端设置演示");
    println!("==============================");
    println!();

    // === 方式1: 传统客户端创建方式 ===
    println!("📋 方式1: 传统客户端创建");
    println!("--------------------");

    let traditional_client = LarkClient::builder(&app_id, &app_secret)
        .with_app_type(AppType::SelfBuild)
        .with_enable_token_cache(true)
        .build();

    println!("✅ 传统方式创建客户端成功");
    println!("📱 App ID: {}", app_id);
    println!("🔧 每个服务独立持有配置");
    println!("💾 内存使用: 配置被复制到各个服务");
    println!();

    // === 方式2: 新的共享配置方式 ===
    println!("📋 方式2: 共享配置客户端创建");
    println!("----------------------------");

    // 使用工厂方法创建共享配置
    let shared_config = SharedConfigFactory::create_shared(
        ConfigBuilder::default()
            .app_id(&app_id)
            .app_secret(&app_secret)
            .app_type(AppType::SelfBuild)
            .enable_token_cache(true)
            .build(),
    );

    let shared_client = LarkClient::new(shared_config.config().clone());

    println!("✅ 共享配置方式创建客户端成功");
    println!("📱 App ID: {}", app_id);
    println!("🔗 所有服务共享同一个配置实例");
    println!("💾 内存使用: 配置通过 Arc 共享，节省内存");
    println!("📊 引用计数: {}", shared_config.ref_count());
    println!();

    // === 两种方式对比 ===
    println!("📋 两种方式对比");
    println!("================");
    println!("传统方式:");
    println!("  ✅ 简单直接，容易理解");
    println!("  ✅ 向后兼容性好");
    println!("  ❌ 每个服务独立持有配置，内存开销较大");
    println!("  ❌ 配置更新需要同步到所有服务实例");
    println!();
    println!("共享配置方式:");
    println!("  ✅ 内存使用优化，特别是在多服务场景");
    println!("  ✅ 配置一致性保证");
    println!("  ✅ 线程安全的并发访问");
    println!("  ✅ 为未来服务优化奠定基础");
    println!("  ❌ 需要理解 Arc 和共享概念");
    println!();

    // === 推荐使用场景 ===
    println!("📋 推荐使用场景");
    println!("================");
    println!("🎯 新项目/学习: 建议使用共享配置方式");
    println!("🏢 企业应用: 强烈推荐共享配置方式（内存优化）");
    println!("🔄 现有项目迁移: 可以逐步迁移到共享配置");
    println!("📚 学习目的: 建议先了解传统方式，再学习共享配置");
    println!();

    // === 可用服务列表 ===
    println!("🚀 可用服务列表");
    println!("================");
    let services = vec![
        ("IM", "即时通讯"),
        ("Drive", "文件管理"),
        ("Docs", "文档操作"),
        ("Sheets", "电子表格"),
        ("Wiki", "知识库"),
        ("Bitable", "多维表格"),
        ("Comments", "文档评论"),
        ("Permission", "权限控制"),
        ("Search", "内容搜索"),
        ("Contact", "联系人管理"),
        ("Group", "群组管理"),
        ("Calendar", "日程管理"),
        ("Approval", "审批流程"),
        ("Task", "任务管理"),
        ("Board", "看板"),
        ("Minutes", "会议纪要"),
        ("VC", "视频会议"),
    ];

    for (service_en, service_zh) in services {
        println!("- {} ({})", service_en, service_zh);
    }

    println!();
    println!("💡 提示: 根据启用的功能标志，某些服务可能不可用");
    println!("📖 更多信息请查看项目文档和示例");

    Ok(())
}

/// 创建传统方式客户端的辅助函数
fn create_traditional_client(app_id: &str, app_secret: &str) -> LarkClient {
    LarkClient::builder(app_id, app_secret)
        .with_app_type(AppType::SelfBuild)
        .with_enable_token_cache(true)
        .build()
}

/// 创建共享配置客户端的辅助函数
fn create_shared_config_client(app_id: &str, app_secret: &str) -> (LarkClient, SharedConfig) {
    let shared_config = SharedConfigFactory::create_shared(
        ConfigBuilder::default()
            .app_id(app_id)
            .app_secret(app_secret)
            .app_type(AppType::SelfBuild)
            .enable_token_cache(true)
            .build(),
    );

    let client = LarkClient::new(shared_config.config().clone());
    (client, shared_config)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_traditional_client_creation() {
        let app_id = "test_app_id";
        let app_secret = "test_app_secret";

        let client = create_traditional_client(app_id, app_secret);

        // 验证客户端创建成功
        assert_eq!(client.config.app_id, app_id);
        assert_eq!(client.config.app_secret, app_secret);
    }

    #[test]
    fn test_shared_config_client_creation() {
        let app_id = "test_app_id";
        let app_secret = "test_app_secret";

        let (client, shared_config) = create_shared_config_client(app_id, app_secret);

        // 验证客户端创建成功
        assert_eq!(client.config.app_id, app_id);
        assert_eq!(client.config.app_secret, app_secret);

        // 验证共享配置引用计数
        assert_eq!(shared_config.ref_count(), 1);
    }

    #[test]
    fn test_shared_config_cloning() {
        let app_id = "test_app_id";
        let app_secret = "test_app_secret";

        let (_, shared_config) = create_shared_config_client(app_id, app_secret);
        let cloned_config = shared_config.clone_shared();

        // 验证克隆后引用计数增加
        assert_eq!(shared_config.ref_count(), 2);
        assert_eq!(cloned_config.ref_count(), 2);
        assert!(shared_config.is_shared());
        assert!(cloned_config.is_shared());
    }
}

//! OpenLark SDK 新接口迁移模板
//!
//! 本文件提供了从传统接口迁移到新接口（SharedConfig）的标准模板。
//! 可以作为其他示例文件迁移的参考。

use open_lark::prelude::*;
use open_lark::service_registry::{SharedConfig, SharedConfigFactory};
use open_lark::core::config::{Config, ConfigBuilder};
use open_lark::constants::AppType;

/// ========================================
/// 标准导入模板
/// ========================================
// 传统接口导入
use open_lark::prelude::*;

// 新接口导入
use open_lark::service_registry::{SharedConfig, SharedConfigFactory};
use open_lark::core::config::{Config, ConfigBuilder};

/// ========================================
/// 标准配置创建函数模板
/// ========================================

/// 使用传统方式创建客户端
///
/// # Arguments
/// * `app_id` - 应用ID
/// * `app_secret` - 应用密钥
///
/// # Returns
/// 返回 LarkClient 实例
///
/// # Examples
/// ```
/// let client = create_traditional_client("app_id", "app_secret");
/// ```
pub fn create_traditional_client(app_id: &str, app_secret: &str) -> LarkClient {
    LarkClient::builder(app_id, app_secret)
        .with_app_type(AppType::SelfBuild)
        .with_enable_token_cache(true)
        .build()
}

/// 使用共享配置方式创建客户端
///
/// # Arguments
/// * `app_id` - 应用ID
/// * `app_secret` - 应用密钥
///
/// # Returns
/// 返回 (LarkClient, SharedConfig) 元组
///
/// # Examples
/// ```
/// let (client, shared_config) = create_shared_config_client("app_id", "app_secret");
/// ```
pub fn create_shared_config_client(app_id: &str, app_secret: &str) -> (LarkClient, SharedConfig) {
    let shared_config = SharedConfigFactory::create_shared(
        ConfigBuilder::default()
            .app_id(app_id)
            .app_secret(app_secret)
            .app_type(AppType::SelfBuild)
            .enable_token_cache(true)
            .build()
    );

    let client = LarkClient::new(shared_config.config().clone());
    (client, shared_config)
}

/// 带错误处理的客户端创建函数
///
/// # Arguments
/// * `app_id` - 应用ID
/// * `app_secret` - 应用密钥
/// * `use_shared_config` - 是否使用共享配置
///
/// # Returns
/// 返回 Result<LarkClient, Box<dyn std::error::Error>>
pub fn create_client_with_fallback(
    app_id: &str,
    app_secret: &str,
    use_shared_config: bool
) -> Result<LarkClient, Box<dyn std::error::Error>> {
    if use_shared_config {
        // 尝试使用新接口
        match create_shared_config_client(app_id, app_secret) {
            (client, shared_config) => {
                println!("[INFO] 使用共享配置创建客户端成功");
                println!("[INFO] 配置引用计数: {}", shared_config.ref_count());
                Ok(client)
            }
        }
    } else {
        // 使用传统接口
        let client = create_traditional_client(app_id, app_secret);
        println!("[INFO] 使用传统方式创建客户端成功");
        Ok(client)
    }
}

/// ========================================
/// 标准示例函数模板
/// ========================================

/// 演示新旧接口对比的函数
pub fn demo_interface_comparison() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 OpenLark SDK 接口对比演示");
    println!("==============================");

    let app_id = "demo_app_id";
    let app_secret = "demo_app_secret";

    // 传统方式
    println!("\n📋 传统方式:");
    let traditional_client = create_traditional_client(app_id, app_secret);
    println!("  ✅ 客户端创建成功");
    println!("  📱 App ID: {}", traditional_client.config.app_id);

    // 新接口方式
    println!("\n📋 共享配置方式:");
    let (shared_client, shared_config) = create_shared_config_client(app_id, app_secret);
    println!("  ✅ 客户端创建成功");
    println!("  📱 App ID: {}", shared_client.config.app_id);
    println!("  📊 引用计数: {}", shared_config.ref_count());

    // 对比说明
    println!("\n📋 主要差异:");
    println!("  🔄 内存使用: 共享配置使用 Arc<Config> 减少内存开销");
    println!("  🔗 配置共享: 所有服务共享同一个配置实例");
    println!("  🧵 线程安全: 支持安全的并发访问");
    println!("  📈 性能优化: 特别适合多服务场景");

    Ok(())
}

/// 标准服务使用模板
///
/// 这个函数展示了如何在新接口下使用服务
pub async fn demo_service_usage(client: &LarkClient) -> Result<(), Box<dyn std::error::Error>> {
    println!("🔧 服务使用演示");
    println!("================");

    // 根据启用的功能标志展示可用服务
    #[cfg(feature = "im")]
    {
        println!("✅ IM 服务可用");
        // client.im.v1.message.send_text(...).await?;
    }

    #[cfg(feature = "contact")]
    {
        println!("✅ Contact 服务可用");
        // client.contact.v3.user.create(...).await?;
    }

    #[cfg(feature = "cloud-docs")]
    {
        println!("✅ CloudDocs 服务可用");
        // client.cloud_docs.v2.file.upload(...).await?;
    }

    println!("💡 使用方式与原来完全相同，只是客户端创建方式有所改变");
    Ok(())
}

/// ========================================
/// 错误处理模板
/// ========================================

/// 标准错误处理示例
pub fn demo_error_handling() -> Result<(), Box<dyn std::error::Error>> {
    println!("⚠️  错误处理演示");
    println!("================");

    let app_id = "invalid_app_id";
    let app_secret = "invalid_app_secret";

    // 尝试创建客户端
    match create_client_with_fallback(app_id, app_secret, true) {
        Ok(client) => {
            println!("✅ 客户端创建成功");
            // 使用客户端...
            let _ = client;
        }
        Err(e) => {
            println!("❌ 客户端创建失败: {}", e);
            println!("💡 请检查应用ID和密钥是否正确");
        }
    }

    Ok(())
}

/// ========================================
/// 测试模板
/// ========================================

#[cfg(test)]
mod template_tests {
    use super::*;

    #[test]
    fn test_traditional_creation() {
        let client = create_traditional_client("test_id", "test_secret");
        assert_eq!(client.config.app_id, "test_id");
    }

    #[test]
    fn test_shared_config_creation() {
        let (client, shared_config) = create_shared_config_client("test_id", "test_secret");
        assert_eq!(client.config.app_id, "test_id");
        assert_eq!(shared_config.ref_count(), 1);
    }

    #[test]
    fn test_config_sharing() {
        let (_, shared_config) = create_shared_config_client("test_id", "test_secret");
        let cloned = shared_config.clone_shared();

        assert_eq!(shared_config.ref_count(), 2);
        assert_eq!(cloned.ref_count(), 2);
        assert!(shared_config.is_shared());
    }

    #[test]
    fn test_fallback_creation() {
        let result = create_client_with_fallback("test_id", "test_secret", false);
        assert!(result.is_ok());
    }
}

/// ========================================
/// 迁移检查清单
/// ========================================

/// 这个函数提供了一个迁移检查清单
pub fn migration_checklist() {
    println!("📋 迁移检查清单");
    println!("================");
    println!("✅ 1. 添加新接口导入语句");
    println!("✅ 2. 替换客户端创建方式");
    println!("✅ 3. 更新错误处理逻辑");
    println!("✅ 4. 验证服务使用方式");
    println!("✅ 5. 添加必要的测试");
    println!("✅ 6. 更新文档和注释");
    println!("✅ 7. 运行编译测试");
    println!("✅ 8. 验证功能完整性");
    println!();
    println!("🎯 迁移完成后:");
    println!("  • 内存使用优化");
    println!("  • 代码现代化");
    println!("  • 更好的扩展性");
    println!("  • 保持功能一致性");
}

/// ========================================
/// 使用说明
/// ========================================

/*
使用本模板迁移现有示例的步骤:

1. 复制必要的导入语句
2. 根据需要选择客户端创建函数
3. 替换现有的客户端创建代码
4. 更新错误处理逻辑
5. 添加必要的测试
6. 更新文档说明

迁移示例:

// 原来的代码
let client = LarkClient::builder(&app_id, &app_secret).build();

// 迁移后的代码
let (client, shared_config) = create_shared_config_client(&app_id, &app_secret);

// 或使用带错误处理的版本
let client = create_client_with_fallback(&app_id, &app_secret, true)?;

注意事项:
- 保持服务使用方式不变
- 确保功能标志正确配置
- 添加适当的测试用例
- 更新相关文档
*/
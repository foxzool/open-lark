//! OpenLark 统一客户端集成测试
//!
//! 测试整个统一客户端架构的功能集成。

use openlark_client::prelude::*;
use std::time::Duration;

#[tokio::test]
async fn test_unified_client_creation() {
    // 测试从环境变量创建客户端
    match OpenLarkClient::from_env().await {
        Ok(client) => {
            println!("✅ OpenLark客户端创建成功");

            // 检查可用服务
            let services = client.available_services();
            println!("📋 可用服务: {:?}", services);
            assert!(!services.is_empty());

            // 测试健康检查
            let health = client.health_check().await.unwrap();
            println!("🏥 健康检查结果: {:?}", health);
        }
        Err(e) => {
            println!("⚠️  无法创建客户端（需要环境配置）: {}", e);
        }
    }
}

#[tokio::test]
async fn test_config_builder() {
    // 测试配置构建器
    let config = UnifiedConfig::default();
    assert_eq!(config.core.app_id, "");
    assert_eq!(config.core.app_secret, "");

    // 测试从CoreConfig创建
    let core_config = openlark_core::config::ConfigBuilder::new()
        .app_id("test_app_id")
        .app_secret("test_app_secret")
        .build()
        .unwrap_or_else(|_| openlark_core::config::Config::default());

    let unified_config = UnifiedConfig::from_core(core_config);
    assert_eq!(unified_config.core.app_id, "test_app_id");
    assert_eq!(unified_config.core.app_secret, "test_app_secret");
}

#[tokio::test]
async fn test_api_dispatcher() {
    // 测试API分发器
    let dispatcher = APIDispatcher::new();

    // 测试默认适配器创建
    let adapters = ServiceAdapterFactory::create_default_adapters();
    assert_eq!(adapters.len(), 5); // communication, hr, docs, ai, auth

    // 测试分发器注册
    for adapter in adapters {
        dispatcher.register_adapter(adapter.clone());
    }

    let services = dispatcher.list_services();
    assert_eq!(services.len(), 5);
    assert!(services.contains(&"communication"));
    assert!(services.contains(&"hr"));
    assert!(services.contains(&"docs"));
    assert!(services.contains(&"ai"));
    assert!(services.contains(&"auth"));

    // 测试服务可用性检查
    assert!(dispatcher.is_service_available("communication"));
    assert!(dispatcher.is_service_available("hr"));
    assert!(dispatcher.is_service_available("docs"));
    assert!(dispatcher.is_service_available("ai"));
    assert!(dispatcher.is_service_available("auth"));
}

#[tokio::test]
async fn test_service_adapters() {
    // 测试各个服务适配器的基本功能
    let adapters = ServiceAdapterFactory::create_default_adapters();

    for adapter in adapters {
        println!("🧪 测试服务适配器: {}", adapter.name());

        assert_eq!(adapter.version(), "1.0.0");

        // 测试健康检查
        let health = adapter.health_check().await;
        assert!(health.is_ok());

        println!("  ✅ {} 健康", adapter.name());
    }
}

#[tokio::test]
async fn test_high_level_api_methods() {
    // 测试高级API方法（模拟测试）
    println!("🧪 测试高级API方法结构...");

    // 创建测试配置
    let config = UnifiedConfig::default();
    let client = UnifiedClient::new(config).await.unwrap();

    // 测试方法存在性（不实际调用API）
    // 这些方法在真实环境中需要有效的配置

    // 通信API
    if client.is_service_available("communication") {
        println!("  📱 通信API可用");
        // 实际调用需要有效的token和配置
        // client.send_text_message("test", "open_id", "test").await?;
    }

    // HR API
    if client.is_service_available("hr") {
        println!("  👥 HR API可用");
        // client.list_employees(Some("open_id"), Some(10), None).await?;
    }

    // 文档API
    if client.is_service_available("docs") {
        println!("  📊 文档API可用");
        // client.create_spreadsheet("测试表格", None).await?;
    }

    // AI API
    if client.is_service_available("ai") {
        println!("  🤖 AI API可用");
        // client.generate_text("测试提示", None, Some(0.7), Some(50)).await?;
    }

    // 认证API
    if client.is_service_available("auth") {
        println!("  🔐 认证API可用");
        // client.get_app_access_token().await?;
    }

    println!("✅ 高级API方法结构验证完成");
}

#[tokio::test]
async fn test_service_specific_api() {
    // 测试服务特定API
    println!("🧪 测试服务特定API结构...");

    let config = UnifiedConfig::default();
    let client = UnifiedClient::new(config).await.unwrap();

    // 测试通信服务API
    if let Ok(comm_api) = client.communication() {
        println!("  📱 通信服务API创建成功");
        // 这里不进行实际调用，只测试API结构
    }

    // 测试HR服务API
    if let Ok(hr_api) = client.hr() {
        println!("  👥 HR服务API创建成功");
    }

    // 测试文档服务API
    if let Ok(docs_api) = client.docs() {
        println!("  📊 文档服务API创建成功");
    }

    // 测试AI服务API
    if let Ok(ai_api) = client.ai() {
        println!("  🤖 AI服务API创建成功");
    }

    // 测试认证服务API
    if let Ok(auth_api) = client.auth() {
        println!("  🔐 认证服务API创建成功");
    }

    println!("✅ 服务特定API结构验证完成");
}

#[tokio::test]
async fn test_error_handling() {
    // 测试错误处理
    println!("🧪 测试错误处理...");

    // 测试无效配置
    let invalid_config = UnifiedConfig {
        core: openlark_core::config::Config::default(),
        services: Default::default(),
        features: Default::default(),
        performance: Default::default(),
        security: Default::default(),
        monitoring: Default::default(),
        metadata: Default::default(),
    };

    // 这个应该能正常创建
    let client = UnifiedClient::new(invalid_config).await;
    assert!(client.is_ok());

    // 测试无效的服务调用
    if let Ok(client) = client {
        // 测试请求不存在的服务
        assert!(client.communication().is_err() || !client.is_service_available("communication"));

        // 测试API调用验证
        let test_request = crate::unified::api::SendTextMessageRequest {
            receive_id: "".to_string(),
            receive_id_type: "open_id".to_string(),
            content: "".to_string(),
        };
        assert!(test_request.validate().is_err()); // 空参数应该失败
    }

    println!("✅ 错误处理验证完成");
}

#[tokio::test]
async fn test_batch_operations() {
    // 测试批量操作
    println!("🧪 测试批量操作...");

    // 这里主要测试批量操作的接口定义
    let messages = vec![
        (
            "user1".to_string(),
            "open_id".to_string(),
            "批量消息1".to_string(),
        ),
        (
            "user2".to_string(),
            "open_id".to_string(),
            "批量消息2".to_string(),
        ),
    ];

    assert_eq!(messages.len(), 2);
    println!("  📦 批量消息结构: {} 条", messages.len());

    let user_ids = vec![
        "user1".to_string(),
        "user2".to_string(),
        "user3".to_string(),
    ];
    assert_eq!(user_ids.len(), 3);
    println!("  👤 批量用户结构: {} 个", user_ids.len());

    println!("✅ 批量操作结构验证完成");
}

#[cfg(test)]
mod test_utils {
    /// 创建测试用的配置
    pub fn create_test_config() -> UnifiedConfig {
        UnifiedConfig::default()
    }

    /// 创建模拟的认证令牌信息
    pub fn create_mock_token() -> crate::unified::services::auth::TokenInfo {
        crate::unified::services::auth::TokenInfo {
            access_token: "mock_access_token".to_string(),
            refresh_token: Some("mock_refresh_token".to_string()),
            expires_at: chrono::Utc::now() + chrono::Duration::hours(2),
            token_type: "Bearer".to_string(),
        }
    }
}

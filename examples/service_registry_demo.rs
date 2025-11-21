//! 服务注册和发现机制演示
//!
//! 展示如何使用新的服务注册表、功能标志和依赖解析功能

use std::collections::HashMap;
use openlark_client::prelude::*;
use openlark_client::registry::*;

#[tokio::main]
async fn main() -> Result<()> {
    println!("🚀 OpenLark 服务注册和发现机制演示\n");

    // 1. 创建配置
    let config = Config {
        app_id: "demo_app_id".to_string(),
        app_secret: "demo_app_secret".to_string(),
        base_url: "https://open.feishu.cn".to_string(),
        timeout: std::time::Duration::from_secs(30),
        ..Default::default()
    };

    // 2. 创建服务注册表
    let mut registry = DefaultServiceRegistry::from_config(RegistryConfig::default());

    println!("📋 1. 注册服务元数据\n");

    // 3. 注册核心层服务
    register_core_services(&mut registry)?;

    // 4. 注册专业层服务
    register_professional_services(&mut registry)?;

    // 5. 注册企业层服务
    register_enterprise_services(&mut registry)?;

    println!("\n🔍 2. 服务依赖分析\n");

    // 6. 分析依赖关系
    let dependency_graph = registry.get_dependency_graph();
    let resolver = DependencyResolver::new();

    match resolver.resolve_dependencies(dependency_graph) {
        Ok(sorted_services) => {
            println!("✅ 服务启动顺序:");
            for (index, service) in sorted_services.iter().enumerate() {
                let entry = registry.get_service(service)?;
                println!("   {}. {} (优先级: {})",
                    index + 1,
                    entry.metadata.name,
                    entry.metadata.priority
                );
            }
        },
        Err(e) => {
            println!("❌ 依赖解析失败: {}", e);
            return Err(e.into());
        }
    }

    println!("\n🎛️  3. 功能标志管理\n");

    // 7. 功能标志演示
    demonstrate_feature_flags(&registry)?;

    println!("\n📊 4. 生成依赖报告\n");

    // 8. 生成详细报告
    let report = resolver.generate_dependency_report(&registry.get_dependency_graph())?;
    println!("{}", report.to_text());

    println!("\n🔄 5. 服务生命周期管理\n");

    // 9. 服务生命周期演示
    demonstrate_service_lifecycle(&mut registry).await?;

    println!("\n✨ 演示完成！");

    Ok(())
}

/// 注册核心层服务
fn register_core_services(registry: &mut DefaultServiceRegistry) -> Result<()> {
    let core_services = vec![
        ServiceMetadata {
            name: "auth".to_string(),
            version: "1.0.0".to_string(),
            description: Some("认证和授权服务".to_string()),
            dependencies: vec![],
            provides: vec!["token-management".to_string(), "permission-control".to_string()],
            status: ServiceStatus::Uninitialized,
            priority: 1,
        },
        ServiceMetadata {
            name: "communication".to_string(),
            version: "1.0.0".to_string(),
            description: Some("通讯和消息服务".to_string()),
            dependencies: vec!["auth".to_string()],
            provides: vec!["im".to_string(), "contacts".to_string(), "groups".to_string()],
            status: ServiceStatus::Uninitialized,
            priority: 2,
        },
        ServiceMetadata {
            name: "docs".to_string(),
            version: "1.0.0".to_string(),
            description: Some("文档和知识库服务".to_string()),
            dependencies: vec!["auth".to_string()],
            provides: vec!["cloud-docs".to_string(), "sheets".to_string(), "wiki".to_string()],
            status: ServiceStatus::Uninitialized,
            priority: 2,
        },
    ];

    for metadata in core_services {
        let name = metadata.name.clone();
        registry.register_service(metadata)?;
        println!("   ✅ 注册核心服务: {}", name);
    }

    Ok(())
}

/// 注册专业层服务
fn register_professional_services(registry: &mut DefaultServiceRegistry) -> Result<()> {
    let professional_services = vec![
        ServiceMetadata {
            name: "hr".to_string(),
            version: "1.0.0".to_string(),
            description: Some("人力资源服务".to_string()),
            dependencies: vec!["auth".to_string()],
            provides: vec!["attendance".to_string(), "corehr".to_string(), "ehr".to_string()],
            status: ServiceStatus::Uninitialized,
            priority: 3,
        },
        ServiceMetadata {
            name: "ai".to_string(),
            version: "1.0.0".to_string(),
            description: Some("AI和智能服务".to_string()),
            dependencies: vec!["auth".to_string(), "communication".to_string()],
            provides: vec!["chatbot".to_string(), "smart-analysis".to_string()],
            status: ServiceStatus::Uninitialized,
            priority: 4,
        },
        ServiceMetadata {
            name: "calendar".to_string(),
            version: "1.0.0".to_string(),
            description: Some("日历和会议服务".to_string()),
            dependencies: vec!["auth".to_string(), "communication".to_string()],
            provides: vec!["schedule".to_string(), "meetings".to_string()],
            status: ServiceStatus::Uninitialized,
            priority: 4,
        },
    ];

    for metadata in professional_services {
        let name = metadata.name.clone();
        registry.register_service(metadata)?;
        println!("   ✅ 注册专业服务: {}", name);
    }

    Ok(())
}

/// 注册企业层服务
fn register_enterprise_services(registry: &mut DefaultServiceRegistry) -> Result<()> {
    let enterprise_services = vec![
        ServiceMetadata {
            name: "admin".to_string(),
            version: "1.0.0".to_string(),
            description: Some("管理服务".to_string()),
            dependencies: vec!["auth".to_string(), "hr".to_string()],
            provides: vec!["user-management".to_string(), "system-config".to_string()],
            status: ServiceStatus::Uninitialized,
            priority: 5,
        },
        ServiceMetadata {
            name: "approval".to_string(),
            version: "1.0.0".to_string(),
            description: Some("审批流程服务".to_string()),
            dependencies: vec!["auth".to_string(), "communication".to_string()],
            provides: vec!["workflow".to_string(), "template".to_string()],
            status: ServiceStatus::Uninitialized,
            priority: 5,
        },
        ServiceMetadata {
            name: "helpdesk".to_string(),
            version: "1.0.0".to_string(),
            description: Some("帮助台服务".to_string()),
            dependencies: vec!["auth".to_string(), "communication".to_string(), "ai".to_string()],
            provides: vec!["ticket".to_string(), "knowledge-base".to_string()],
            status: ServiceStatus::Uninitialized,
            priority: 6,
        },
    ];

    for metadata in enterprise_services {
        let name = metadata.name.clone();
        registry.register_service(metadata)?;
        println!("   ✅ 注册企业服务: {}", name);
    }

    Ok(())
}

/// 演示功能标志管理
fn demonstrate_feature_flags(registry: &DefaultServiceRegistry) -> Result<()> {
    // 这里我们创建一个功能标志管理器来演示
    let flag_manager = FeatureFlagManager::default();

    println!("   📋 默认功能标志状态:");
    let flags = flag_manager.list_flags();
    for flag in &flags {
        let status = if flag.current_value.as_bool() { "✅ 启用" } else { "❌ 禁用" };
        println!("      - {}: {} ({})",
            flag.name,
            status,
            flag.description.as_deref().unwrap_or("无描述")
        );
    }

    // 演示功能切换
    println!("\n   🔄 演示功能切换:");
    println!("      启用 communication 功能...");
    flag_manager.set_bool_flag("communication", true)?;
    println!("      启用 docs 功能...");
    flag_manager.set_bool_flag("docs", true)?;

    // 演示用户特定功能
    println!("\n   👥 用户特定功能:");
    let test_users = vec!["user_001", "user_002", "user_003"];
    for user in test_users {
        let enabled = flag_manager.is_enabled_for_user("ai", user);
        println!("      - AI功能对用户 {}: {}", user, if enabled { "启用" } else { "禁用" });
    }

    Ok(())
}

/// 演示服务生命周期
async fn demonstrate_service_lifecycle(registry: &mut DefaultServiceRegistry) -> Result<()> {
    println!("   🔄 开始服务初始化...");
    registry.initialize_services()?;

    println!("   ✅ 服务初始化完成，当前状态:");
    let services = registry.list_services();
    for service in services {
        println!("      - {}: {:?}",
            service.metadata.name,
            service.metadata.status
        );
    }

    println!("\n   🚀 启动就绪的服务...");
    registry.start_services()?;

    println!("   ✅ 服务启动完成，当前状态:");
    let services = registry.list_services();
    for service in services {
        println!("      - {}: {:?}",
            service.metadata.name,
            service.metadata.status
        );
    }

    Ok(())
}
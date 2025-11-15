//! 服务管理演示
//!
//! 展示 openlark-client 的条件编译服务管理功能

#[cfg(feature = "docs")]
use openlark_client::{DefaultLarkClient, LarkClient, ServiceRegistry};
#[cfg(feature = "docs")]
use openlark_core::{config::Config, constants::AppType};

#[cfg(feature = "docs")]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 OpenLark Client 服务管理演示");

    // 创建客户端配置
    let config = Config::builder()
        .app_id("demo_app_id")
        .app_secret("demo_app_secret")
        .app_type(AppType::SelfBuild)
        .build();

    // 创建客户端实例（这将根据启用的功能自动初始化相应服务）
    let client = DefaultLarkClient::new(config);

    println!("✅ 客户端创建成功");
    println!("📋 已注册的服务: {:?}", client.services().list_services());

    // 检查是否有 docs 服务
    if let Some(_docs_service) = client
        .services()
        .get_service::<openlark_docs::BaseService>("docs")
    {
        println!("📄 Docs 服务已启用并可用");

        // 可以使用服务进行实际操作（这里仅作演示）
        println!("   - 文档创建和管理功能可用");
        println!("   - 支持文档版本控制和协作");
        println!("   - 提供文档搜索和导出功能");
    } else {
        println!("❌ Docs 服务未启用");
    }

    // 演示服务列表功能
    let enabled_services = openlark_client::services::ServiceManager::get_enabled_services();
    println!("🎯 当前启用的功能标志对应的服务:");
    for service in enabled_services {
        println!("   - {}", service);
    }

    Ok(())
}

#[cfg(not(feature = "docs"))]
fn main() {
    println!("请启用 docs 功能来运行此演示:");
    println!("cargo run --example service_management_demo --features docs");
}

//! 迁移演示
//!
//! 展示如何从原有 LarkClient 迁移到新的 openlark-client

#[cfg(feature = "docs")]
use openlark_client::{
    accessors::{CompatibleClientBuilder, ServiceAccessorsExt},
    DefaultLarkClient, ServiceRegistry,
};

#[cfg(feature = "docs")]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔄 OpenLark Client 迁移演示");
    println!("====================================");

    // 方式 1: 使用新的构建器 API（推荐）
    println!("\n1️⃣  新的构建器 API:");
    let config = openlark_core::config::Config::builder()
        .app_id("demo_app_id")
        .app_secret("demo_app_secret")
        .app_type(openlark_core::constants::AppType::SelfBuild)
        .build();

    let client = DefaultLarkClient::new(config);
    println!("✅ 使用 Config::builder() 创建客户端");

    // 方式 2: 使用兼容的构建器（向后兼容）
    println!("\n2️⃣  兼容的构建器 API:");
    let client_compatible = DefaultLarkClient::builder("demo_app_id", "demo_app_secret")
        .with_app_type(openlark_core::constants::AppType::SelfBuild)
        .build();
    println!("✅ 使用 DefaultLarkClient::builder() 创建客户端");

    // 方式 3: 扩展服务访问器（类型安全）
    println!("\n3️⃣  扩展服务访问器:");
    if let Some(_docs_service) = client.service_accessors().docs_ext() {
        println!("✅ 通过 client.service_accessors().docs_ext() 访问文档服务");
    }

    // 方式 4: 通用服务访问
    println!("\n4️⃣  通用服务访问:");
    if let Some(_docs_service) = client
        .services()
        .get_service::<openlark_docs::docs::DocxService>("docs")
    {
        println!("✅ 通过 client.services().get_service<T>() 访问服务");
    }

    // 列出所有可用服务
    println!("\n📋 已启用的服务:");
    for service in client.services().list_services() {
        println!("   - {}", service);
    }

    println!("\n🎯 迁移完成！所有功能正常工作。");

    println!("\n📖 迁移指南:");
    println!("   • 新代码: 使用 Config::builder() -> DefaultLarkClient::new()");
    println!("   • 兼容代码: 使用 DefaultLarkClient::builder()");
    println!("   • 服务访问: 使用 client.service_accessors().service_name_ext()");
    println!("   • 高级访问: 使用 client.services().get_service<T>()");

    Ok(())
}

#[cfg(not(feature = "docs"))]
fn main() {
    println!("请启用 docs 功能来运行此演示:");
    println!("cargo run --example migration_demo --features docs");
}

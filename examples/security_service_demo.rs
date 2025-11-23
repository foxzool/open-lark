//! 安全服务演示示例
//!
//! 展示如何使用 openlark-client 的 security 功能来访问飞书安全认证服务。

use openlark_client::{DefaultLarkClient, Result};
use openlark_core::config::Config;

#[tokio::main]
async fn main() -> Result<()> {
    println!("🔒 OpenLark 安全服务演示");

    // 创建客户端
    let config = Config::builder()
        .app_id("your_app_id")
        .app_secret("your_app_secret")
        .build()?;

    let client = DefaultLarkClient::new(config)?;

    // 检查安全服务是否可用
    if client.has_service("security") {
        println!("✅ 安全服务已启用");

        // 获取安全服务
        let security_service = client.security();

        if let Some(security) = security_service {
            println!("🛡️ 安全服务初始化成功");

            // 获取配置信息
            let config_ref = security.config();
            println!("🔧 App ID: {}", config_ref.app_id);

            // 这里可以添加具体的安全操作代码
            println!("🔒 安全服务已准备就绪，可以开始操作");
        } else {
            println!("❌ 安全服务初始化失败");
        }
    } else {
        println!("⚠️ 安全服务未启用");
        println!("请确保在构建时启用 security 功能");
    }

    println!("✨ 演示完成");

    Ok(())
}

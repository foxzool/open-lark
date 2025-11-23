//! 文档服务演示示例
//!
//! 展示如何使用 openlark-client 的 docs 功能来访问飞书云文档服务。

use openlark_client::{DefaultLarkClient, Result};
use openlark_core::config::Config;

#[tokio::main]
async fn main() -> Result<()> {
    println!("🚀 OpenLark 文档服务演示");

    // 创建客户端
    let config = Config::builder()
        .app_id("your_app_id")
        .app_secret("your_app_secret")
        .build()?;

    let client = DefaultLarkClient::new(config)?;

    // 检查文档服务是否可用
    if client.has_service("docs") {
        println!("✅ 文档服务已启用");

        // 获取文档服务
        let docs_service = client.docs();

        if let Some(docs) = docs_service {
            println!("📝 文档服务初始化成功");

            // 获取配置信息
            let config_ref = docs.config();
            println!("🔧 App ID: {}", config_ref.app_id);

            // 这里可以添加具体的文档操作代码
            println!("💡 文档服务已准备就绪，可以开始操作");
        } else {
            println!("❌ 文档服务初始化失败");
        }
    } else {
        println!("⚠️ 文档服务未启用");
        println!("请确保在构建时启用 docs 功能");
    }

    println!("✨ 演示完成");

    Ok(())
}

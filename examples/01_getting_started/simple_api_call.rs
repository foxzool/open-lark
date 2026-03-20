//! 第一个API调用示例（简化版）
//!
//! 演示如何使用 openlark SDK 进行飞书API调用。
//!
//! 运行方式：
//! ```bash
//! export OPENLARK_APP_ID="your_app_id"
//! export OPENLARK_APP_SECRET="your_app_secret"
//! cargo run --example simple_api_call --features "auth,communication"
//! ```

use open_lark::communication::endpoints::IM_V1_MESSAGES;
use open_lark::prelude::*;
use serde_json::json;

#[tokio::main]
async fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    println!("openlark SDK 简化示例\n");

    // 从环境变量加载配置
    dotenvy::dotenv().ok();

    let app_id = std::env::var("OPENLARK_APP_ID").unwrap_or_else(|_| "cli_demo".to_string());
    let app_secret =
        std::env::var("OPENLARK_APP_SECRET").unwrap_or_else(|_| "demo_secret".to_string());

    println!("应用ID: {}", &app_id[..8.min(app_id.len())]);

    let client = Client::builder()
        .app_id(app_id)
        .app_secret(app_secret)
        .build()?;

    println!("✅ Client 创建成功");
    println!("认证入口已启用: {}", client.auth.app.config().app_id());

    // 访问 communication 模块的常量
    let endpoint = IM_V1_MESSAGES;
    println!("消息API端点: {}", endpoint);

    // 构建消息请求
    let message = json!({
        "text": "Hello from openlark SDK!"
    });

    println!("\n📤 消息内容: {}", message);
    println!("\n✅ 示例完成 - 这是一个简化演示");
    println!("   实际使用时需要处理认证令牌和HTTP请求");

    Ok(())
}

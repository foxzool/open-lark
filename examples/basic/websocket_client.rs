use log::info;
use open_lark::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 初始化环境变量和日志
    dotenvy::dotenv().ok();
    env_logger::init();

    let app_id = std::env::var("APP_ID").expect("APP_ID environment variable not set");
    let app_secret = std::env::var("APP_SECRET").expect("APP_SECRET environment variable not set");

    println!("🔌 WebSocket Client Example");
    println!("Note: WebSocket features require the 'websocket' feature flag");

    #[cfg(not(feature = "websocket"))]
    {
        println!("❌ WebSocket feature is not enabled");
        println!("💡 Enable with: cargo run --example websocket_client --features websocket");
        println!("\n🔧 To enable WebSocket support:");
        println!("1. Add 'websocket' to your Cargo.toml features");
        println!("2. Use EventDispatcherHandler for event handling");
        println!("3. Register event handlers for different event types");
        return Ok(());
    }

    #[cfg(feature = "websocket")]
    {
        use open_lark::client::ws_client::LarkWsClient;
        use std::sync::Arc;

        println!("✅ WebSocket feature is enabled");
        println!("🎯 Connecting to Lark WebSocket...");

        // 创建客户端并获取配置
        let client = LarkClient::builder(&app_id, &app_secret)
            .with_app_type(AppType::SelfBuild)
            .with_enable_token_cache(true)
            .build();

        let config = Arc::new(client.config);

        // 创建事件处理器
        let event_handler = match EventDispatcherHandler::builder()
            .register_p2_im_message_receive_v1(|event| {
                info!("📩 收到消息事件: {event:?}");
                println!("📩 收到新消息:");
                println!("  - 消息ID: {:?}", event.header.event_id);
                println!("  - 消息类型: {:?}", event.event.message.message_type);
                if !event.event.message.content.is_empty() {
                    println!("  - 消息内容: {}", &event.event.message.content);
                }
                println!("  - 发送者: {:?}", event.event.sender);
            }) {
            Ok(builder) => builder.build(),
            Err(e) => {
                eprintln!("❌ Failed to register event handler: {e}");
                return Err(Box::new(std::io::Error::other(e)) as Box<dyn std::error::Error>);
            }
        };

        println!("📡 事件处理器已注册，支持的事件类型:");
        println!("  - 消息接收事件 (im.message.receive_v1)");
        println!("  - 消息已读事件 (im.message.message_read_v1)");

        println!("\n🚀 启动 WebSocket 连接...");

        // 启动 WebSocket 客户端
        if let Err(e) = LarkWsClient::open(config, event_handler).await {
            eprintln!("❌ WebSocket 连接失败: {e:?}");
            return Err(format!("WebSocket connection failed: {e:?}").into());
        }

        println!("✅ WebSocket 连接已建立并正常运行");
    }

    Ok(())
}

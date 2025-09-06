use log::info;
use open_lark::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 初始化环境变量和日志
    dotenvy::dotenv().ok();

    // 设置日志级别以获取更详细的调试信息
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "debug");
    }

    env_logger::init();

    let app_id = std::env::var("APP_ID").expect("APP_ID environment variable not set");
    let app_secret = std::env::var("APP_SECRET").expect("APP_SECRET environment variable not set");

    println!("🔌 WebSocket Echo Server Example");
    println!("该示例演示一个简单的 Echo Server，将收到的消息原样返回给发送者");
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

        let client_for_echo = Arc::new(client);
        let config = Arc::new(client_for_echo.config.clone());

        // 创建事件处理器，实现 echo server 功能
        let echo_client = client_for_echo.clone();
        let event_handler = match EventDispatcherHandler::builder()
            .register_p2_im_message_receive_v1(move |event| {
                let client = echo_client.clone();
                tokio::spawn(async move {
                    println!("📩 收到消息接收事件:");
                    println!("  - 事件ID: {:?}", event.header.event_id);
                    println!("  - 消息ID: {:?}", event.event.message.message_id);
                    println!("  - 消息类型: {:?}", event.event.message.message_type);
                    println!("  - 发送者: {:?}", event.event.sender.sender_id.open_id);

                    if !event.event.message.content.is_empty() {
                        println!("  - 消息内容: {}", &event.event.message.content);

                        // Echo server 逻辑：解析消息内容并回显
                        if let Ok(content_json) =
                            serde_json::from_str::<serde_json::Value>(&event.event.message.content)
                        {
                            if let Some(text) = content_json.get("text").and_then(|v| v.as_str()) {
                                let echo_content = format!("Echo: {text}");
                                println!("🔄 正在发送回显消息: {echo_content}");

                                // 构建回显消息
                                let echo_request = CreateMessageRequest::builder()
                                    .receive_id_type("open_id")
                                    .request_body(
                                        CreateMessageRequestBody::builder()
                                            .receive_id(&event.event.sender.sender_id.open_id)
                                            .msg_type("text")
                                            .content(format!("{{\"text\":\"{echo_content}\"}}"))
                                            .build(),
                                    )
                                    .build();

                                // 发送回显消息
                                match client.im.v1.message.create(echo_request, None).await {
                                    Ok(_) => {
                                        println!("✅ Echo 消息发送成功: {echo_content}");
                                    }
                                    Err(e) => {
                                        eprintln!("❌ Echo 消息发送失败: {e:?}");
                                    }
                                }
                            } else {
                                println!("  - 非文本消息，跳过回显");
                            }
                        } else {
                            println!("  - 无法解析消息内容");
                        }
                    } else {
                        println!("  - 空消息内容");
                    }

                    info!("📩 消息接收事件处理完成");
                });
            })
            .and_then(|builder| {
                builder.register_p2_im_message_read_v1(|event| {
                    tokio::spawn(async move {
                        println!("👁️ 收到消息已读事件:");
                        println!("  - 事件ID: {:?}", event.header.event_id);
                        println!("  - 阅读者: {:?}", event.event.reader.reader_id.open_id);
                        println!("  - 阅读时间: {}", event.event.reader.read_time);
                        println!("  - 已读消息ID列表: {:?}", event.event.message_id_list);
                        println!("  - 租户: {}", event.event.reader.tenant_key);

                        info!("👁️ 消息已读事件处理完成");
                    });
                })
            }) {
            Ok(builder) => builder.build(),
            Err(e) => {
                eprintln!("❌ Failed to register event handler: {e}");
                return Err(Box::new(std::io::Error::other(e)) as Box<dyn std::error::Error>);
            }
        };

        println!("📡 事件处理器已注册，支持的事件类型:");
        println!("  - 消息接收事件 (im.message.receive_v1) - 支持文本消息回显");
        println!("  - 消息已读事件 (im.message.message_read_v1) - 显示消息阅读状态");

        println!("\n💡 使用提示:");
        println!("  - 向机器人发送文本消息将收到 'Echo: [消息内容]' 的回复");
        println!("  - 消息阅读状态会实时显示在控制台");
        println!("  - 如果遇到解析错误，请检查事件数据结构是否匹配");

        println!("\n🚀 启动 WebSocket 连接...");

        // 启动 WebSocket 客户端
        if let Err(e) = LarkWsClient::open(config, event_handler).await {
            eprintln!("❌ WebSocket 连接失败: {e:?}");
            return Err(format!("WebSocket connection failed: {e:?}").into());
        }

        println!("✅ Echo Server 已启动并正常运行，等待接收消息...");
    }

    Ok(())
}

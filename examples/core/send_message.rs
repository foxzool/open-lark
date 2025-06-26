use open_lark::core::trait_system::ExecutableBuilder;
/// 发送文本消息示例
///
/// 这个示例演示如何使用飞书SDK发送基础的文本消息。
///
/// 使用方法：
/// cargo run --example send_message
///
/// 环境变量：
/// APP_ID=your_app_id
/// APP_SECRET=your_app_secret
/// RECEIVE_ID=target_user_open_id_or_chat_id
use open_lark::prelude::*;
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 加载环境变量
    dotenvy::dotenv().ok();

    let app_id = std::env::var("APP_ID").expect("APP_ID environment variable not set");
    let app_secret = std::env::var("APP_SECRET").expect("APP_SECRET environment variable not set");
    let receive_id =
        std::env::var("RECEIVE_ID").unwrap_or_else(|_| "ou_example_user_id".to_string());

    // 创建客户端
    let client = LarkClient::builder(&app_id, &app_secret)
        .with_enable_token_cache(true)
        .build();

    println!("📨 飞书消息发送示例");
    println!("目标接收者: {}", receive_id);
    println!("{}", "=".repeat(50));

    // 发送文本消息
    send_text_message(&client, &receive_id).await?;

    Ok(())
}

/// 发送文本消息
async fn send_text_message(
    client: &LarkClient,
    receive_id: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n📤 发送文本消息...");

    // 构建消息体
    let message_body = open_lark::service::im::v1::message::CreateMessageRequestBody::builder()
        .receive_id(receive_id)
        .msg_type("text")
        .content(json!({"text": "Hello from 飞书SDK! 🚀"}).to_string())
        .build();

    // 使用增强Builder模式发送消息
    match open_lark::service::im::v1::message::CreateMessageRequest::builder()
        .receive_id_type("open_id") // 可以是 open_id, user_id, union_id, email, chat_id
        .request_body(message_body)
        .execute(&client.im.v1.message)
        .await
    {
        Ok(response) => {
            if let Some(data) = &response.data {
                println!("✅ 消息发送成功!");
                println!("   消息ID: {}", data.message_id);
                println!("   消息类型: {}", data.msg_type);
                println!("   发送时间: {}", data.create_time);
                println!("   所属群组: {}", data.chat_id);
            } else {
                println!("⚠️ 消息发送请求成功，但未返回消息数据");
            }
        }
        Err(e) => {
            println!("❌ 消息发送失败");

            // 使用新的错误处理功能
            let context = open_lark::core::error_helper::ErrorHelper::create_error_context(&e);

            // 打印用户友好的错误信息
            println!("错误原因: {}", context.user_friendly_message);

            // 显示是否可重试
            if context.is_retryable {
                println!("🔄 此错误可以重试");
                if let Some(strategy) = &context.retry_strategy {
                    println!("   建议延迟 {:?} 后重试", strategy.base_delay);
                }
            } else {
                println!("🚫 此错误需要手动处理");
            }

            // 显示建议的解决方案
            if !context.suggested_actions.is_empty() {
                println!("\n💡 建议解决方案:");
                for (i, action) in context.suggested_actions.iter().enumerate() {
                    println!("   {}. {}", i + 1, action);
                }
            }

            // 显示帮助链接
            if let Some(help_url) = &context.help_url {
                println!("\n📚 参考文档: {}", help_url);
            }

            return Err(e.into());
        }
    }

    Ok(())
}

/// 发送富文本消息示例
#[allow(dead_code)]
async fn send_rich_text_message(
    client: &LarkClient,
    receive_id: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n📝 发送富文本消息...");

    // 富文本消息格式
    let rich_content = json!({
        "post": {
            "zh_cn": {
                "title": "项目更新通知",
                "content": [
                    [
                        {
                            "tag": "text",
                            "text": "项目进展: "
                        },
                        {
                            "tag": "text",
                            "text": "已完成",
                            "style": ["bold"]
                        }
                    ],
                    [
                        {
                            "tag": "text",
                            "text": "详细信息请查看: "
                        },
                        {
                            "tag": "a",
                            "text": "项目文档",
                            "href": "https://example.com/project-docs"
                        }
                    ]
                ]
            }
        }
    });

    let message_body = open_lark::service::im::v1::message::CreateMessageRequestBody::builder()
        .receive_id(receive_id)
        .msg_type("post")
        .content(rich_content.to_string())
        .build();

    match open_lark::service::im::v1::message::CreateMessageRequest::builder()
        .receive_id_type("open_id")
        .request_body(message_body)
        .execute(&client.im.v1.message)
        .await
    {
        Ok(response) => {
            if let Some(data) = &response.data {
                println!("✅ 富文本消息发送成功!");
                println!("   消息ID: {}", data.message_id);
            }
        }
        Err(e) => {
            println!("❌ 富文本消息发送失败");

            // 使用增强的错误处理
            let user_message = open_lark::core::error_helper::ErrorHelper::format_user_error(&e);
            println!("错误详情: {}", user_message);

            // 创建并显示错误上下文
            let context = open_lark::core::error_helper::ErrorHelper::create_error_context(&e);
            context.print_details();

            return Err(e.into());
        }
    }

    Ok(())
}

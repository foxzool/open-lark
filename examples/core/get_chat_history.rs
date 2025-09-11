use open_lark::core::trait_system::ExecutableBuilder;
/// 获取聊天历史消息示例
///
/// 这个示例演示如何使用飞书SDK获取会话的历史消息记录。
///
/// 使用方法：
/// cargo run --example get_chat_history
///
/// 环境变量：
/// APP_ID=your_app_id
/// APP_SECRET=your_app_secret
/// CHAT_ID=target_chat_id
use open_lark::prelude::*;
use std::time::{SystemTime, UNIX_EPOCH};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 加载环境变量
    dotenvy::dotenv().ok();

    let app_id = std::env::var("APP_ID").expect("APP_ID environment variable not set");
    let app_secret = std::env::var("APP_SECRET").expect("APP_SECRET environment variable not set");
    let chat_id = std::env::var("CHAT_ID").unwrap_or_else(|_| "oc_example_chat_id".to_string());

    // 创建客户端
    let client = LarkClient::builder(&app_id, &app_secret)
        .with_enable_token_cache(true)
        .build();

    println!("📋 飞书聊天历史获取示例");
    println!("目标会话: {chat_id}");
    println!("{}", "=".repeat(50));

    // 获取最近的聊天记录
    get_recent_messages(&client, &chat_id).await?;

    // 获取指定时间段的聊天记录
    get_messages_by_timerange(&client, &chat_id).await?;

    Ok(())
}

/// 获取最近的聊天消息
async fn get_recent_messages(
    client: &LarkClient,
    chat_id: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n📥 获取最近20条聊天消息...");

    // 使用增强Builder模式获取消息历史
    match open_lark::service::im::v1::message::ListMessageRequest::builder()
        .container_id_type("chat")
        .container_id(chat_id)
        .sort_type("ByCreateTimeDesc") // 按创建时间倒序
        .page_size(20)
        .execute(&client.im.v1.message)
        .await
    {
        Ok(response) => {
            println!("✅ 消息获取成功!");
            println!("   消息总数: {}", response.items.len());
            println!("   是否有更多: {}", response.has_more);

            if !response.items.is_empty() {
                println!("\n📝 消息列表:");
                for (index, message) in response.items.iter().enumerate() {
                    println!(
                        "   {}. [{}] {} - {}",
                        index + 1,
                        format_timestamp(&message.create_time),
                        message.msg_type,
                        format_sender(&message.sender)
                    );

                    // 显示消息内容预览（前50个字符）
                    if let Some(content) = extract_text_content(&message.body.content) {
                        let preview = if content.len() > 50 {
                            format!("{}...", &content[..50])
                        } else {
                            content
                        };
                        println!("      内容: {preview}");
                    }
                }

                if response.has_more {
                    println!("\n💡 提示: 还有更多消息可以通过分页获取");
                    if let Some(page_token) = &response.page_token {
                        println!("   下一页token: {page_token}");
                    }
                }
            } else {
                println!("⚠️ 请求成功，但未返回消息数据");
            }
        }
        Err(e) => {
            println!("❌ 获取聊天历史失败: {e:?}");
            println!("\n💡 常见错误解决方案:");
            println!("   1. 检查APP_ID和APP_SECRET是否正确");
            println!("   2. 确认应用有读取消息的权限");
            println!("   3. 验证CHAT_ID是否为有效的群组ID");
            println!("   4. 确保机器人在目标群组中");
            return Err(e.into());
        }
    }

    Ok(())
}

/// 获取指定时间段的聊天消息
async fn get_messages_by_timerange(
    client: &LarkClient,
    chat_id: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🕐 获取最近24小时的聊天消息...");

    // 计算时间范围（最近24小时）
    let now = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs() as i64;
    let yesterday = now - 24 * 60 * 60; // 24小时前

    match open_lark::service::im::v1::message::ListMessageRequest::builder()
        .container_id_type("chat")
        .container_id(chat_id)
        .start_time(yesterday)
        .end_time(now)
        .sort_type("ByCreateTimeAsc") // 按时间正序
        .page_size(50)
        .execute(&client.im.v1.message)
        .await
    {
        Ok(response) => {
            println!("✅ 时间段消息获取成功!");
            println!(
                "   时间范围: {} - {}",
                format_timestamp(&yesterday.to_string()),
                format_timestamp(&now.to_string())
            );
            println!("   消息数量: {}", response.items.len());

            if !response.items.is_empty() {
                // 按消息类型统计
                let mut type_stats = std::collections::HashMap::new();
                for message in &response.items {
                    *type_stats.entry(&message.msg_type).or_insert(0) += 1;
                }

                println!("\n📊 消息类型统计:");
                for (msg_type, count) in type_stats {
                    println!("   {msg_type}: {count} 条");
                }

                // 显示最近几条消息
                println!("\n📝 最近的消息:");
                for message in response.items.iter().take(5) {
                    println!(
                        "   [{}] {} - {}",
                        format_timestamp(&message.create_time),
                        message.msg_type,
                        format_sender(&message.sender)
                    );
                }
            } else {
                println!("⚠️ 请求成功，但未返回消息数据");
            }
        }
        Err(e) => {
            println!("❌ 获取时间段消息失败: {e:?}");
            return Err(e.into());
        }
    }

    Ok(())
}

/// 格式化时间戳显示
fn format_timestamp(timestamp_str: &str) -> String {
    if let Ok(timestamp) = timestamp_str.parse::<i64>() {
        let timestamp_secs = if timestamp > 1_000_000_000_000 {
            // 毫秒时间戳
            timestamp / 1000
        } else {
            // 秒时间戳
            timestamp
        };

        if let Some(_datetime) = SystemTime::UNIX_EPOCH
            .checked_add(std::time::Duration::from_secs(timestamp_secs as u64))
        {
            // 这里简化处理，实际项目中可以使用chrono库进行更好的时间格式化
            format!("时间戳: {timestamp_secs}")
        } else {
            timestamp_str.to_string()
        }
    } else {
        timestamp_str.to_string()
    }
}

/// 格式化发送者信息
fn format_sender(_sender: &open_lark::service::im::v1::message::types::Sender) -> String {
    // 由于Sender字段是私有的，我们只能使用可访问的信息
    // 在实际使用中，需要根据API响应的实际结构来调整
    "发送者: [ID信息]".to_string()
}

/// 提取文本消息内容
fn extract_text_content(content: &str) -> Option<String> {
    // 尝试解析JSON内容
    if let Ok(json_value) = serde_json::from_str::<serde_json::Value>(content) {
        if let Some(text) = json_value.get("text").and_then(|t| t.as_str()) {
            return Some(text.to_string());
        }

        // 处理富文本消息
        if let Some(post) = json_value.get("post") {
            if let Some(zh_cn) = post.get("zh_cn") {
                if let Some(title) = zh_cn.get("title").and_then(|t| t.as_str()) {
                    return Some(format!("[富文本] {title}"));
                }
            }
        }
    }

    None
}

/**
 * OpenLark SDK IM消息服务示例
 *
 * 本示例展示了如何使用 OpenLark SDK 进行即时消息操作，包括：
 * - 发送各种类型的消息（文本、图片、文件、卡片等）
 * - 接收和回复消息
 * - 消息格式化和富文本
 * - 群聊和私聊消息处理
 * - 消息状态和回执处理
 *
 * 运行方法：
 * cargo run --example im_messaging --features communication
 */

use openlark_core::config::ConfigBuilder;
use openlark_core::constants::AppType;
use openlark_core::prelude::*;
use openlark_client::LarkClient;
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 加载环境变量
    dotenvy::dotenv().ok();

    println!("💬 OpenLark SDK IM消息服务示例");
    println!("=================================");
    println!();

    // 创建客户端
    let app_id = std::env::var("OPENLARK_APP_ID")
        .expect("请设置环境变量 OPENLARK_APP_ID");
    let app_secret = std::env::var("OPENLARK_APP_SECRET")
        .expect("请设置环境变量 OPENLARK_APP_SECRET");

    let client = create_client(&app_id, &app_secret)?;

    // 从环境变量获取测试参数
    let test_user_id = std::env::var("OPENLARK_TEST_USER_ID").ok();
    let test_group_id = std::env::var("OPENLARK_TEST_GROUP_ID").ok();

    if test_user_id.is_none() && test_group_id.is_none() {
        println!("⚠️  需要设置测试目标");
        println!("请设置以下环境变量之一:");
        println!("   • OPENLARK_TEST_USER_ID - 测试用户ID");
        println!("   • OPENLARK_TEST_GROUP_ID - 测试群聊ID");
        return Ok(());
    }

    println!("📋 消息类型演示");
    println!("==================");

    // === 文本消息 ===
    println!("1️⃣ 发送文本消息");
    if let Some(ref user_id) = test_user_id {
        demo_send_text_message(&client, user_id).await?;
    }
    println!();

    // === 富文本消息 ===
    println!("2️⃣ 发送富文本消息");
    if let Some(ref user_id) = test_user_id {
        demo_send_rich_text_message(&client, user_id).await?;
    }
    println!();

    // === 图片消息 ===
    println!("3️⃣ 发送图片消息");
    if let Some(ref user_id) = test_user_id {
        demo_send_image_message(&client, user_id).await?;
    }
    println!();

    // === 文件消息 ===
    println!("4️⃣ 发送文件消息");
    if let Some(ref user_id) = test_user_id {
        demo_send_file_message(&client, user_id).await?;
    }
    println!();

    // === 卡片消息 ===
    println!("5️⃣ 发送卡片消息");
    if let Some(ref group_id) = test_group_id {
        demo_send_card_message(&client, group_id).await?;
    }
    println!();

    // === 群聊消息 ===
    println!("6️⃣ 群聊消息处理");
    if let Some(ref group_id) = test_group_id {
        demo_group_messaging(&client, group_id).await?;
    }
    println!();

    // === 消息接收和回复 ===
    println!("7️⃣ 消息接收和回复");
    demo_message_reply(&client).await?;
    println!();

    // === 批量消息处理 ===
    println!("8️⃣ 批量消息处理");
    if let Some(ref user_id) = test_user_id {
        demo_batch_messaging(&client, user_id).await?;
    }
    println!();

    // === 消息状态查询 ===
    println!("9️⃣ 消息状态查询");
    demo_message_status(&client).await?;
    println!();

    // === 最佳实践总结 ===
    println!("💡 IM消息最佳实践");
    println!("==================");
    println!("1. 📝 消息内容:");
    println!("   • 文本消息: 简单文本，适合快速通知");
    println!("   • 富文本: 支持格式化，适合重要信息");
    println!("   • 卡片消息: 结构化展示，适合复杂信息");
    println!("   • 媒体文件: 图片、视频、文档共享");
    println!();
    println!("2. 🎯 发送策略:");
    println!("   • 批量发送: 提升效率，但注意限流");
    println!("   • 异步处理: 避免阻塞主线程");
    println!("   • 错误重试: 处理网络和临时错误");
    println!("   • 消息去重: 避免重复发送");
    println!();
    println!("3. 🛡️ 安全考虑:");
    println!("   • 内容过滤: 检查敏感词汇");
    println!("   • 权限验证: 确认发送权限");
    println!("   • 频率控制: 避免触发限流");
    println!("   • 数据保护: 不发送敏感信息");

    Ok(())
}

/**
 * 创建客户端
 */
fn create_client(app_id: &str, app_secret: &str) -> Result<LarkClient, Box<dyn std::error::Error>> {
    let config = ConfigBuilder::default()
        .app_id(app_id)
        .app_secret(app_secret)
        .app_type(AppType::SelfBuild)
        .enable_token_cache(true)
        .http_timeout(30000)
        .retry_times(3)
        .build();

    Ok(LarkClient::new(config))
}

/**
 * 发送文本消息
 */
async fn demo_send_text_message(client: &LarkClient, user_id: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("📝 发送文本消息给用户: {}", user_id);

    // 构建文本消息请求
    let request = open_lark::service::im::v1::models::MessageSendRequest {
        receive_id_type: Some("user_id".to_string()),
        receive_id: user_id.to_string(),
        msg_type: "text".to_string(),
        content: json!({
            "text": "Hello from OpenLark SDK! 🚀\n\n这是一条通过SDK发送的测试消息。"
        }).to_string(),
        ..Default::default()
    };

    match client.im.v1.message.send(&request).await {
        Ok(response) => {
            if response.success() {
                if let Some(data) = response.data {
                    println!("   ✅ 消息发送成功");
                    println!("   📧 消息ID: {:?}", data.message_id);
                    println!("   🕐 发送时间: {:?}", data.create_time);
                } else {
                    println!("   ✅ 消息发送成功（无返回数据）");
                }
            } else {
                println!("   ❌ 消息发送失败");
                if let Some(error) = response.error {
                    println!("   错误: {:?}", error.msg);
                }
            }
        }
        Err(e) => {
            println!("   ❌ 消息发送异常: {}", e);
        }
    }

    Ok(())
}

/**
 * 发送富文本消息
 */
async fn demo_send_rich_text_message(client: &LarkClient, user_id: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("🎨 发送富文本消息给用户: {}", user_id);

    // 构建富文本内容
    let rich_text_content = json!({
        "post": {
            "zh_cn": {
                "title": "📋 重要通知",
                "content": [
                    [{
                        "tag": "text",
                        "text": "这是一条富文本消息，支持多种格式化选项："
                    }],
                    [{
                        "tag": "text",
                        "text": "\n• "
                    }, {
                        "tag": "text",
                        "text": "粗体文字",
                        "style": {
                            "bold": true
                        }
                    }],
                    [{
                        "tag": "text",
                        "text": "\n• "
                    }, {
                        "tag": "text",
                        "text": "斜体文字",
                        "style": {
                            "italic": true
                        }
                    }],
                    [{
                        "tag": "text",
                        "text": "\n• "
                    }, {
                        "tag": "text",
                        "text": "删除线文字",
                        "style": {
                            "strikethrough": true
                        }
                    }],
                    [{
                        "tag": "text",
                        "text": "\n• "
                    }, {
                        "tag": "a",
                        "text": "链接文字",
                        "href": "https://open.feishu.cn"
                    }],
                    [{
                        "tag": "text",
                        "text": "\n• "
                    }, {
                        "tag": "at",
                        "text": "@用户",
                        "user_id": user_id
                    }]
                ]
            }
        }
    });

    let request = open_lark::service::im::v1::models::MessageSendRequest {
        receive_id_type: Some("user_id".to_string()),
        receive_id: user_id.to_string(),
        msg_type: "post".to_string(),
        content: rich_text_content.to_string(),
        ..Default::default()
    };

    match client.im.v1.message.send(&request).await {
        Ok(response) => {
            if response.success() {
                println!("   ✅ 富文本消息发送成功");
                if let Some(data) = response.data {
                    println!("   📧 消息ID: {:?}", data.message_id);
                }
            } else {
                println!("   ❌ 富文本消息发送失败");
            }
        }
        Err(e) => {
            println!("   ❌ 富文本消息发送异常: {}", e);
        }
    }

    Ok(())
}

/**
 * 发送图片消息
 */
async fn demo_send_image_message(client: &LarkClient, user_id: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("🖼️ 发送图片消息给用户: {}", user_id);

    // 注意：实际使用时需要先上传图片获取image_key
    // 这里使用模拟的image_key
    let image_key = "img_v2_example_key";

    let image_content = json!({
        "image_key": image_key
    });

    let request = open_lark::service::im::v1::models::MessageSendRequest {
        receive_id_type: Some("user_id".to_string()),
        receive_id: user_id.to_string(),
        msg_type: "image".to_string(),
        content: image_content.to_string(),
        ..Default::default()
    };

    println!("💡 提示: 实际使用需要:");
    println!("   1. 先上传图片文件获取 image_key");
    println!("   2. 使用 image_key 构建图片消息");
    println!("   3. 发送图片消息");

    // 这里不实际发送，因为我们没有真实的image_key
    println!("   ℹ️  跳过实际发送（需要真实image_key）");

    Ok(())
}

/**
 * 发送文件消息
 */
async fn demo_send_file_message(client: &LarkClient, user_id: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("📎 发送文件消息给用户: {}", user_id);

    // 注意：实际使用时需要先上传文件获取file_key
    let file_key = "file_v2_example_key";

    let file_content = json!({
        "file_key": file_key,
        "file_name": "example_document.pdf"
    });

    let request = open_lark::service::im::v1::models::MessageSendRequest {
        receive_id_type: Some("user_id".to_string()),
        receive_id: user_id.to_string(),
        msg_type: "file".to_string(),
        content: file_content.to_string(),
        ..Default::default()
    };

    println!("💡 提示: 实际使用需要:");
    println!("   1. 先上传文件获取 file_key");
    println!("   2. 使用 file_key 和文件名构建文件消息");
    println!("   3. 发送文件消息");

    println!("   ℹ️  跳过实际发送（需要真实file_key）");

    Ok(())
}

/**
 * 发送卡片消息
 */
async fn demo_send_card_message(client: &LarkClient, group_id: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("🎴 发送卡片消息到群聊: {}", group_id);

    // 构建交互式卡片
    let card_content = json!({
        "config": {
            "wide_screen_mode": true
        },
        "elements": [
            {
                "tag": "div",
                "text": {
                    "content": "📊 OpenLark SDK 演示卡片",
                    "tag": "lark_md"
                }
            },
            {
                "tag": "hr"
            },
            {
                "tag": "div",
                "fields": [
                    {
                        "is_short": true,
                        "text": {
                            "content": "**功能特性**\n• 多种消息类型\n• 异步处理\n• 错误重试",
                            "tag": "lark_md"
                        }
                    },
                    {
                        "is_short": true,
                        "text": {
                            "content": "**支持服务**\n• IM消息\n• 联系人管理\n• 文档协作",
                            "tag": "lark_md"
                        }
                    }
                ]
            },
            {
                "tag": "action",
                "actions": [
                    {
                        "tag": "button",
                        "text": {
                            "content": "查看文档",
                            "tag": "plain_text"
                        },
                        "type": "primary",
                        "url": "https://docs.rs/open-lark"
                    },
                    {
                        "tag": "button",
                        "text": {
                            "content": "GitHub",
                            "tag": "plain_text"
                        },
                        "url": "https://github.com/foxzool/open-lark"
                    }
                ]
            }
        ]
    });

    let request = open_lark::service::im::v1::models::MessageSendRequest {
        receive_id_type: Some("chat_id".to_string()),
        receive_id: group_id.to_string(),
        msg_type: "interactive".to_string(),
        content: card_content.to_string(),
        ..Default::default()
    };

    match client.im.v1.message.send(&request).await {
        Ok(response) => {
            if response.success() {
                println!("   ✅ 卡片消息发送成功");
                if let Some(data) = response.data {
                    println!("   📧 消息ID: {:?}", data.message_id);
                }
            } else {
                println!("   ❌ 卡片消息发送失败");
            }
        }
        Err(e) => {
            println!("   ❌ 卡片消息发送异常: {}", e);
        }
    }

    Ok(())
}

/**
 * 群聊消息处理
 */
async fn demo_group_messaging(client: &LarkClient, group_id: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("👥 群聊消息处理演示");

    // 获取群聊信息
    let chat_request = open_lark::service::im::v1::models::ChatGetRequest {
        chat_id: group_id.to_string(),
        ..Default::default()
    };

    match client.im.v1.chat.get(&chat_request).await {
        Ok(response) => {
            if response.success() {
                if let Some(chat_info) = response.data {
                    println!("   📋 群聊信息:");
                    println!("   • 名称: {:?}", chat_info.name);
                    println!("   • 描述: {:?}", chat_info.description);
                    println!("   • 成员数: {:?}", chat_info.member_count);
                }
            }
        }
        Err(e) => {
            println!("   ⚠️  获取群聊信息失败: {}", e);
        }
    }

    // @全体成员的消息
    let all_mention_content = json!({
        "text": "📢 @all OpenLark SDK 群聊演示！"
    });

    let request = open_lark::service::im::v1::models::MessageSendRequest {
        receive_id_type: Some("chat_id".to_string()),
        receive_id: group_id.to_string(),
        msg_type: "text".to_string(),
        content: all_mention_content.to_string(),
        ..Default::default()
    };

    println!("💡 群聊特性:");
    println!("   • 支持@全体成员 (@all)");
    println!("   • 支持@特定用户");
    println!("   • 支持富文本和卡片");
    println!("   • 支持消息撤回和编辑");

    Ok(())
}

/**
 * 消息回复功能
 */
async fn demo_message_reply(client: &LarkClient) -> Result<(), Box<dyn std::error::Error>> {
    println!("💬 消息回复演示");

    println!("💡 回复功能:");
    println!("   • 引用原消息进行回复");
    println!("   • 保持消息上下文关联");
    println!("   • 支持各种消息类型的回复");
    println!("   • 提供良好的对话体验");

    // 构建回复消息（需要原消息ID）
    println!("ℹ️  实际回复需要:");
    println!("   1. 获取原消息ID");
    println!("   2. 构建包含reply消息的请求");
    println!("   3. 发送回复消息");

    Ok(())
}

/**
 * 批量消息处理
 */
async fn demo_batch_messaging(client: &LarkClient, user_id: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("📦 批量消息处理演示");

    let messages = vec![
        "📋 批量消息 1/3",
        "📋 批量消息 2/3",
        "📋 批量消息 3/3",
    ];

    println!("🔄 并发发送消息...");
    let start_time = std::time::Instant::now();

    // 使用 futures 进行并发发送
    use futures::stream::{self, StreamExt};

    let futures: Vec<_> = messages.into_iter().enumerate().map(|(i, msg)| {
        let client = client.clone();
        let user_id = user_id.to_string();

        async move {
            let request = open_lark::service::im::v1::models::MessageSendRequest {
                receive_id_type: Some("user_id".to_string()),
                receive_id: user_id,
                msg_type: "text".to_string(),
                content: json!({
                    "text": format!("{} (消息 {})", msg, i + 1)
                }).to_string(),
                ..Default::default()
            };

            client.im.v1.message.send(&request).await
        }
    }).collect();

    let results: Vec<_> = stream::iter(futures)
        .buffer_unordered(3) // 最多3个并发
        .collect()
        .await;

    let elapsed = start_time.elapsed();
    let success_count = results.iter().filter(|r: &&Result<_, _>| {
        r.as_ref().map_or(false, |resp| resp.success())
    }).count();

    println!("📊 批量发送结果:");
    println!("   • 成功: {}/{}", success_count, results.len());
    println!("   • 耗时: {:?}", elapsed);
    println!("   • 平均: {:?}/消息", elapsed / results.len() as u32);

    Ok(())
}

/**
 * 消息状态查询
 */
async fn demo_message_status(client: &LarkClient) -> Result<(), Box<dyn std::error::Error>> {
    println!("📊 消息状态查询演示");

    println!("💡 状态类型:");
    println!("   • 发送状态: 已发送、发送失败");
    println!("   • 接收状态: 已接收、已读");
    println!("   • 更新状态: 已撤回、已编辑");

    println!("ℹ️  查询需要:");
    println!("   1. 消息ID");
    println!("   2. 相应的查询权限");
    println!("   3. 合理的查询频率");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_creation() {
        let app_id = "test_app_id";
        let app_secret = "test_app_secret";

        let result = create_client(app_id, app_secret);
        assert!(result.is_ok(), "客户端创建应该成功");
    }

    #[test]
    fn test_message_request_creation() {
        let request = open_lark::service::im::v1::models::MessageSendRequest {
            receive_id_type: Some("user_id".to_string()),
            receive_id: "test_user".to_string(),
            msg_type: "text".to_string(),
            content: json!({"text": "test"}).to_string(),
            ..Default::default()
        };

        assert_eq!(request.receive_id, "test_user");
        assert_eq!(request.msg_type, "text");
    }
}
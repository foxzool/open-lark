use dotenvy::dotenv;
use open_lark::{
    prelude::*,
    service::im::v1::{
        batch_message::BatchSendMessageRequest,
        buzz_messages::UrgentAppRequest,
        message_card::PatchMessageCardRequest,
        models::{ReceiveIdType, UserIdType},
        url_preview::{BatchUpdateUrlPreviewRequest, UrlPreviewInfo},
    },
};
use serde_json::json;

/// IM v1模块功能演示
///
/// 展示消息表情回复、Pin消息、图片文件处理、批量消息等功能
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let app_id = std::env::var("APP_ID").expect("APP_ID environment variable not set");
    let app_secret = std::env::var("APP_SECRET").expect("APP_SECRET environment variable not set");

    // 创建客户端
    let client = LarkClient::builder(&app_id, &app_secret)
        .with_app_type(AppType::SelfBuild)
        .with_enable_token_cache(true)
        .build();

    println!("🚀 开始演示 IM v1 模块功能...\n");

    // 演示消息表情回复功能
    demo_message_reactions(&client).await?;

    // 演示Pin消息功能
    demo_pin_messages(&client).await?;

    // 演示图片上传功能
    demo_image_upload(&client).await?;

    // 演示批量消息功能
    demo_batch_messages(&client).await?;

    // 演示消息卡片功能
    demo_message_cards(&client).await?;

    // 演示消息加急功能
    demo_urgent_messages(&client).await?;

    // 演示URL预览功能
    demo_url_preview(&client).await?;

    println!("✅ IM v1 模块功能演示完成！");
    Ok(())
}

/// 演示消息表情回复功能
async fn demo_message_reactions(client: &LarkClient) -> Result<(), Box<dyn std::error::Error>> {
    println!("📝 演示消息表情回复功能...");

    let message_id = "om_example_message_id";
    let emoji_type = "THUMBSUP"; // 👍 表情

    // 添加表情回复
    match client
        .im
        .v1
        .message_reaction
        .create(message_id, emoji_type, Some(UserIdType::OpenId), None)
        .await
    {
        Ok(_) => {
            println!("  ✅ 添加表情回复成功");
        }
        Err(e) => {
            println!("  ❌ 添加表情回复失败: {e:?}");
        }
    }

    // 获取表情回复列表
    match client
        .im
        .v1
        .message_reaction
        .list(message_id, Some(UserIdType::OpenId), Some(20), None, None)
        .await
    {
        Ok(response) => {
            println!("  📋 获取到 {} 个表情回复", response.reactions.len());
        }
        Err(e) => {
            println!("  ❌ 获取表情回复失败: {e:?}");
        }
    }

    println!();
    Ok(())
}

/// 演示Pin消息功能
async fn demo_pin_messages(client: &LarkClient) -> Result<(), Box<dyn std::error::Error>> {
    println!("📌 演示Pin消息功能...");

    let message_id = "om_example_message_id";
    let chat_id = "oc_example_chat_id";

    // Pin消息
    match client
        .im
        .v1
        .pin
        .create(message_id, Some(UserIdType::OpenId), None)
        .await
    {
        Ok(response) => {
            println!("  ✅ Pin消息成功: {}", response.pin.pin_id);
        }
        Err(e) => {
            println!("  ❌ Pin消息失败: {e:?}");
        }
    }

    // 获取群内Pin消息列表
    match client
        .im
        .v1
        .pin
        .list(chat_id, Some(UserIdType::OpenId), Some(10), None, None)
        .await
    {
        Ok(response) => {
            println!("  📋 获取到 {} 个Pin消息", response.pins.len());
        }
        Err(e) => {
            println!("  ❌ 获取Pin消息失败: {e:?}");
        }
    }

    println!();
    Ok(())
}

/// 演示图片上传功能
async fn demo_image_upload(client: &LarkClient) -> Result<(), Box<dyn std::error::Error>> {
    println!("🖼️ 演示图片上传功能...");

    // 模拟图片数据
    let image_data = vec![0x89, 0x50, 0x4E, 0x47]; // PNG文件头的一部分

    // 上传图片
    match client.im.v1.image.create("png", image_data, None).await {
        Ok(response) => {
            println!("  ✅ 图片上传成功: {}", response.image_key);

            // 下载图片
            match client.im.v1.image.get(&response.image_key, None).await {
                Ok(download_response) => {
                    println!(
                        "  📥 图片下载成功，大小: {} bytes",
                        download_response.data.len()
                    );
                }
                Err(e) => {
                    println!("  ❌ 图片下载失败: {e:?}");
                }
            }
        }
        Err(e) => {
            println!("  ❌ 图片上传失败: {e:?}");
        }
    }

    println!();
    Ok(())
}

/// 演示批量消息功能
async fn demo_batch_messages(client: &LarkClient) -> Result<(), Box<dyn std::error::Error>> {
    println!("📤 演示批量消息功能...");

    let request = BatchSendMessageRequest {
        receive_id_list: vec![
            "ou_example_user1".to_string(),
            "ou_example_user2".to_string(),
        ],
        msg_type: "text".to_string(),
        content: json!({"text": "这是一条批量消息"}).to_string(),
        uuid: Some("unique_batch_id".to_string()),
    };

    // 批量发送消息
    match client
        .im
        .v1
        .batch_message
        .send(ReceiveIdType::OpenId, request, None)
        .await
    {
        Ok(response) => {
            println!("  ✅ 批量消息发送成功: {}", response.batch_message_id);

            // 查询批量消息进度
            match client
                .im
                .v1
                .batch_message
                .get_progress(&response.batch_message_id, None)
                .await
            {
                Ok(progress_response) => {
                    println!(
                        "  📊 批量消息进度: 总数{}, 成功{}, 失败{}",
                        progress_response.batch_message_progress.total_count,
                        progress_response.batch_message_progress.success_count,
                        progress_response.batch_message_progress.fail_count
                    );
                }
                Err(e) => {
                    println!("  ❌ 查询进度失败: {e:?}");
                }
            }
        }
        Err(e) => {
            println!("  ❌ 批量消息发送失败: {e:?}");
        }
    }

    println!();
    Ok(())
}

/// 演示消息卡片功能
async fn demo_message_cards(client: &LarkClient) -> Result<(), Box<dyn std::error::Error>> {
    println!("🎴 演示消息卡片功能...");

    let message_id = "om_example_message_id";
    let card_content = json!({
        "config": {
            "wide_screen_mode": true
        },
        "elements": [{
            "tag": "div",
            "text": {
                "content": "更新后的卡片内容",
                "tag": "lark_md"
            }
        }]
    });

    let request = PatchMessageCardRequest {
        card: card_content,
        token: Some("update_token".to_string()),
    };

    // 更新消息卡片
    match client
        .im
        .v1
        .message_card
        .patch(message_id, request, None)
        .await
    {
        Ok(_) => {
            println!("  ✅ 消息卡片更新成功");
        }
        Err(e) => {
            println!("  ❌ 消息卡片更新失败: {e:?}");
        }
    }

    println!();
    Ok(())
}

/// 演示消息加急功能
async fn demo_urgent_messages(client: &LarkClient) -> Result<(), Box<dyn std::error::Error>> {
    println!("🚨 演示消息加急功能...");

    let message_id = "om_example_message_id";
    let request = UrgentAppRequest {
        user_id_list: vec!["ou_example_user".to_string()],
    };

    // 发送应用内加急
    match client
        .im
        .v1
        .buzz_messages
        .urgent_app(message_id, UserIdType::OpenId, request, None)
        .await
    {
        Ok(response) => {
            println!(
                "  ✅ 应用内加急发送成功，无效用户数: {}",
                response.invalid_user_id_list.len()
            );
        }
        Err(e) => {
            println!("  ❌ 应用内加急发送失败: {e:?}");
        }
    }

    println!();
    Ok(())
}

/// 演示URL预览功能
async fn demo_url_preview(client: &LarkClient) -> Result<(), Box<dyn std::error::Error>> {
    println!("🔗 演示URL预览功能...");

    let message_id = "om_example_message_id";
    let request = BatchUpdateUrlPreviewRequest {
        previews: vec![UrlPreviewInfo {
            url: "https://example.com".to_string(),
            title: Some("示例网站".to_string()),
            description: Some("这是一个示例网站的描述".to_string()),
            image_url: Some("https://example.com/image.png".to_string()),
            extra: None,
        }],
    };

    // 批量更新URL预览
    match client
        .im
        .v1
        .url_preview
        .batch_update(message_id, request, None)
        .await
    {
        Ok(_) => {
            println!("  ✅ URL预览更新成功");
        }
        Err(e) => {
            println!("  ❌ URL预览更新失败: {e:?}");
        }
    }

    println!();
    Ok(())
}

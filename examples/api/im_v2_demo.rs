use dotenvy::dotenv;
use open_lark::{
    prelude::*,
    service::im::v2::{
        app_feed_card::{CreateAppFeedCardRequest, UpdateAppFeedCardRequest},
        groups_bots::{BotTimeSentiveRequest, TimelyReminderRequest, UpdateFeedCardButtonRequest},
        models::{ButtonInfo, UserIdType},
    },
};
use serde_json::json;

/// IM v2模块功能演示
///
/// 展示消息流卡片管理、机器人即时提醒、消息流按钮更新等功能
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

    println!("🚀 开始演示 IM v2 模块功能...\\n");

    // 演示应用消息流卡片功能
    demo_app_feed_card(&client).await?;

    // 演示群聊或机器人消息功能
    demo_groups_bots(&client).await?;

    println!("✅ IM v2 模块功能演示完成！");
    Ok(())
}

/// 演示应用消息流卡片功能
async fn demo_app_feed_card(client: &LarkClient) -> Result<(), Box<dyn std::error::Error>> {
    println!("📋 演示应用消息流卡片功能...");

    // 创建消息流卡片
    let card_content = json!({
        "config": {
            "wide_screen_mode": true
        },
        "elements": [{
            "tag": "div",
            "text": {
                "content": "这是一个消息流卡片示例",
                "tag": "lark_md"
            }
        }]
    });

    let create_request = CreateAppFeedCardRequest {
        card_content,
        target_users: vec![
            "ou_example_user1".to_string(),
            "ou_example_user2".to_string(),
        ],
        title: Some("示例消息流卡片".to_string()),
        description: Some("这是一个演示用的消息流卡片".to_string()),
    };

    match client
        .im
        .v2
        .app_feed_card
        .create(create_request, None)
        .await
    {
        Ok(response) => {
            if let Some(data) = response.data {
                println!("  ✅ 消息流卡片创建成功: {}", data.card_id);
                let card_id = data.card_id;

                // 更新消息流卡片
                let update_content = json!({
                    "config": {
                        "wide_screen_mode": true
                    },
                    "elements": [{
                        "tag": "div",
                        "text": {
                            "content": "这是更新后的消息流卡片内容",
                            "tag": "lark_md"
                        }
                    }]
                });

                let update_request = UpdateAppFeedCardRequest {
                    card_content: update_content,
                    title: Some("更新后的标题".to_string()),
                    description: Some("更新后的描述".to_string()),
                };

                match client
                    .im
                    .v2
                    .app_feed_card
                    .update(&card_id, update_request, None)
                    .await
                {
                    Ok(update_response) => {
                        if let Some(update_data) = update_response.data {
                            println!("  ✅ 消息流卡片更新成功: {}", update_data.update_time);
                        }
                    }
                    Err(e) => {
                        println!("  ❌ 消息流卡片更新失败: {e:?}");
                    }
                }

                // 删除消息流卡片
                match client.im.v2.app_feed_card.delete(&card_id, None).await {
                    Ok(_) => {
                        println!("  ✅ 消息流卡片删除成功");
                    }
                    Err(e) => {
                        println!("  ❌ 消息流卡片删除失败: {e:?}");
                    }
                }
            }
        }
        Err(e) => {
            println!("  ❌ 消息流卡片创建失败: {e:?}");
        }
    }

    println!();
    Ok(())
}

/// 演示群聊或机器人消息功能
async fn demo_groups_bots(client: &LarkClient) -> Result<(), Box<dyn std::error::Error>> {
    println!("🤖 演示群聊或机器人消息功能...");

    // 机器人单聊即时提醒
    let bot_request = BotTimeSentiveRequest {
        receive_id: "ou_example_user".to_string(),
        content: json!({
            "text": "这是一条机器人即时提醒消息"
        }),
        msg_type: "text".to_string(),
    };

    match client
        .im
        .v2
        .groups_bots
        .bot_time_sentive(UserIdType::OpenId, bot_request, None)
        .await
    {
        Ok(response) => {
            if let Some(data) = response.data {
                println!("  ✅ 机器人即时提醒发送成功: {}", data.message_id);
                let message_id = data.message_id;

                // 更新消息流卡片按钮
                let buttons = vec![
                    ButtonInfo {
                        button_id: "btn_1".to_string(),
                        text: "确认".to_string(),
                        button_type: Some("primary".to_string()),
                        action: Some("confirm".to_string()),
                    },
                    ButtonInfo {
                        button_id: "btn_2".to_string(),
                        text: "取消".to_string(),
                        button_type: Some("default".to_string()),
                        action: Some("cancel".to_string()),
                    },
                ];

                let button_request = UpdateFeedCardButtonRequest {
                    buttons,
                    reason: Some("更新按钮状态".to_string()),
                };

                match client
                    .im
                    .v2
                    .groups_bots
                    .update(&message_id, button_request, None)
                    .await
                {
                    Ok(button_response) => {
                        if let Some(button_data) = button_response.data {
                            println!(
                                "  ✅ 消息流按钮更新成功，更新了 {} 个按钮",
                                button_data.updated_button_count
                            );
                        }
                    }
                    Err(e) => {
                        println!("  ❌ 消息流按钮更新失败: {e:?}");
                    }
                }
            }
        }
        Err(e) => {
            println!("  ❌ 机器人即时提醒发送失败: {e:?}");
        }
    }

    // 即时提醒
    let reminder_request = TimelyReminderRequest {
        content: json!({
            "text": "这是一条即时提醒消息"
        }),
        target_users: vec![
            "ou_example_user1".to_string(),
            "ou_example_user2".to_string(),
        ],
        reminder_type: Some("urgent".to_string()),
    };

    match client
        .im
        .v2
        .groups_bots
        .patch(UserIdType::OpenId, reminder_request, None)
        .await
    {
        Ok(response) => {
            if let Some(data) = response.data {
                println!(
                    "  ✅ 即时提醒发送成功: 成功发送 {} 人，失败 {} 人",
                    data.success_count,
                    data.failed_users.len()
                );
            }
        }
        Err(e) => {
            println!("  ❌ 即时提醒发送失败: {e:?}");
        }
    }

    println!();
    Ok(())
}

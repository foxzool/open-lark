use dotenvy::dotenv;
use open_lark::prelude::*;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 加载环境变量
    dotenv().ok();

    let app_id = env::var("APP_ID").expect("APP_ID 环境变量未设置");
    let app_secret = env::var("APP_SECRET").expect("APP_SECRET 环境变量未设置");

    let client = LarkClient::builder(&app_id, &app_secret)
        .with_app_type(AppType::SelfBuild)
        .with_enable_token_cache(true)
        .build();

    println!("🤖 开始机器人模块演示...");

    // 获取机器人信息
    println!("\n📋 获取机器人信息");
    demo_get_bot_info(&client).await?;

    println!("\n✅ 机器人模块演示完成!");
    Ok(())
}

/// 获取机器人信息示例
async fn demo_get_bot_info(client: &LarkClient) -> Result<(), Box<dyn std::error::Error>> {
    match client.bot.v3.info.get(None).await {
        Ok(response) => {
            if let Some(data) = response.data {
                println!("  ✅ 机器人信息获取成功:");

                if let Some(app_name) = &data.bot.app_name {
                    println!("    📱 机器人名称: {app_name}");
                }

                if let Some(avatar_url) = &data.bot.avatar_url {
                    println!("    🖼️  头像URL: {avatar_url}");
                }

                if let Some(open_id) = &data.bot.open_id {
                    println!("    🆔 Open ID: {open_id}");
                }

                if let Some(app_status) = &data.bot.app_status {
                    println!("    📊 应用状态: {app_status:?}");
                }

                if let Some(ip_white_list) = &data.bot.ip_white_list {
                    if !ip_white_list.is_empty() {
                        println!("    🔒 IP白名单:");
                        for ip in ip_white_list {
                            println!("      - {ip}");
                        }
                    } else {
                        println!("    🔒 IP白名单: 未设置");
                    }
                }
            } else {
                println!("  ⚠️  未获取到机器人信息数据");
            }
        }
        Err(e) => {
            println!("  ❌ 机器人信息获取失败: {e:?}");
        }
    }

    Ok(())
}

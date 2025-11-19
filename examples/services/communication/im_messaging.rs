//! OpenLark IM 消息服务示例
//!
//! 本示例演示如何使用OpenLark SDK进行即时消息操作
//!

use dotenvy::dotenv;
use log::{info, warn};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 初始化日志系统
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();
    info!("🚀 启动 OpenLark IM 消息服务示例");

    // 加载环境变量
    if let Err(e) = dotenv() {
        warn!("⚠️ 未找到 .env 文件: {}", e);
        warn!("⚠️ 请确保设置 OPENLARK_APP_ID 和 OPENLARK_APP_SECRET 环境变量");
    }

    // 读取环境变量配置
    let app_id = match std::env::var("OPENLARK_APP_ID") {
        Ok(id) => id,
        Err(_) => return Err("请设置 OPENLARK_APP_ID 环境变量".into()),
    };
    let app_secret = match std::env::var("OPENLARK_APP_SECRET") {
        Ok(secret) => secret,
        Err(_) => return Err("请设置 OPENLARK_APP_SECRET 环境变量".into()),
    };

    info!("📱 App ID: {}", &app_id[..std::cmp::min(8, app_id.len())]);
    info!(
        "🔑 App Secret: {}***",
        &app_secret[..std::cmp::min(6, app_secret.len())]
    );

    // 注意：此示例当前为基础框架
    // IM 消息服务功能正在适配新的客户端架构中
    println!("📝 IM 消息服务示例");
    println!("⚠️  注意：此功能正在适配新的客户端架构");
    println!("🔧 当前展示基础的环境配置和连接准备");

    display_messaging_features();

    info!("👋 示例程序结束");
    Ok(())
}

/// 显示消息服务功能
fn display_messaging_features() {
    println!("\n📋 IM 消息服务功能:");
    println!("💬 文本消息 - 发送和接收文本消息");
    println!("🖼️  富媒体消息 - 图片、文件、视频等");
    println!("📋 消息卡片 - 交互式卡片消息");
    println!("🔄 消息回复 - 引用回复功能");
    println!("📊 消息状态 - 已读回执和发送状态");
    println!();

    println!("🎯 API 功能列表:");
    println!("  • send_message() - 发送消息");
    println!("  • reply_message() - 回复消息");
    println!("  • get_message() - 获取消息详情");
    println!("  • list_messages() - 获取消息列表");
    println!("  • delete_message() - 删除消息");
    println!("  • mark_read() - 标记已读");
    println!();

    println!("🔧 使用示例:");
    println!();
    println!("```rust");
    println!("use openlark_client::prelude::*;");
    println!();
    println!("// 创建客户端");
    println!("let client = Client::from_env()?;");
    println!();
    println!("// 发送文本消息");
    println!("let result = client.communication()");
    println!("    .im.v1.message.send_text()");
    println!("    .receive_id(\"user_id\")");
    println!("    .content(\"Hello, World!\")");
    println!("    .await?;");
    println!();
    println!("println!(\"消息ID: {{}}\", result.message_id);");
    println!("```");
}

/// 显示消息类型
fn display_message_types() {
    println!("\n📨 支持的消息类型:");
    println!("text - 文本消息");
    println!("image - 图片消息");
    println!("file - 文件消息");
    println!("audio - 音频消息");
    println!("video - 视频消息");
    println!("sticker - 表情包消息");
    println!("interactive - 交互式卡片");
    println!("share_chat - 分享聊天卡片");
    println!("share_user - 分享用户卡片");
}

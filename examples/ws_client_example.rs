#!/usr/bin/env rust-script
//! OpenLark WebSocket 客户端基础示例
//!
//! 本示例演示如何使用OpenLark SDK建立WebSocket连接，接收实时事件
//!
//! 环境变量配置:
//! ```env
//! OPENLARK_APP_ID=your_app_id
//! OPENLARK_APP_SECRET=your_app_secret
//! ```

use dotenvy::dotenv;
use log::{info, warn};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 初始化日志系统
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();
    info!("🚀 启动 OpenLark WebSocket 客户端示例");

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
    // WebSocket客户端功能正在适配新的架构中
    println!("📝 WebSocket 客户端示例");
    println!("⚠️  注意：此功能正在适配新的客户端架构");
    println!("🔧 当前展示基础的环境配置和连接准备");

    display_connection_info();

    info!("👋 示例程序结束");
    Ok(())
}

/// 显示连接状态信息
fn display_connection_info() {
    println!("\n📋 连接状态信息:");
    println!("🔗 WebSocket 端点: wss://open.feishu.cn/callback/ws/endpoint");
    println!("💓 心跳间隔: 30秒 (可动态调整)");
    println!("🔄 重连机制: 指数退避，最大重试5次");
    println!("📊 事件类型: 消息接收、用户状态变更、群组变更等");
    println!();

    println!("🎯 下一步开发计划:");
    println!("  • 完成新客户端架构适配");
    println!("  • 实现自动事件分发");
    println!("  • 添加连接状态监控");
    println!("  • 集成错误恢复机制");
    println!();
}

/// 显示使用示例
fn show_usage_examples() {
    println!("📚 使用示例:");
    println!();
    println!("```rust");
    println!("use openlark_client::prelude::*;");
    println!();
    println!("// 创建客户端");
    println!("let client = Client::builder()");
    println!("    .app_id(\"your_app_id\")");
    println!("    .app_secret(\"your_app_secret\")");
    println!("    .build()?;");
    println!();
    println!("// 启用WebSocket连接");
    println!("let ws_client = client.websocket().connect().await?;");
    println!();
    println!("// 注册事件处理器");
    println!("ws_client.on_message(|event| {{");
    println!("    println!(\"收到事件: {{:?}}\", event);");
    println!("}});");
    println!("```");
}
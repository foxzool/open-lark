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

use std::sync::Arc;
use dotenvy::dotenv;
use log::{debug, error, info, warn};
use openlark_client::{
    ws_client::LarkWsClient,
    ws_client::WsClientError,
};
use openlark_core::{
    config::Config,
    event::dispatcher::EventDispatcherHandler,
};
use tokio::time::Duration;

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
    let app_id = std::env::var("OPENLARK_APP_ID")
        .map_err(|_| "请设置 OPENLARK_APP_ID 环境变量")?;
    let app_secret = std::env::var("OPENLARK_APP_SECRET")
        .map_err(|_| "请设置 OPENLARK_APP_SECRET 环境变量")?;

    info!("📱 App ID: {}", &app_id[..std::cmp::min(8, app_id.len())]);
    info!("🔑 App Secret: {}***", &app_secret[..std::cmp::min(6, app_secret.len())]);

    // 创建配置
    let config_builder = Config::builder()
        .app_id(app_id)
        .app_secret(app_secret)
        .base_url("https://open.feishu.cn")
        .req_timeout(Duration::from_secs(30));

    let config = Arc::new(config_builder.build());
    info!("⚙️ 配置创建完成");

    // 创建事件处理器
    let event_handler = EventDispatcherHandler::builder().build();
    info!("📡 事件处理器创建完成");

    // 显示连接提示
    println!("\n🔌 正在连接到飞书 WebSocket 服务...");
    println!("📊 连接成功后将显示实时事件统计");
    println!("⏹️  按 Ctrl+C 停止连接");
    println!();

    // 建立WebSocket连接
    match openlark_client::ws_client::LarkWsClient::open(config.clone(), event_handler).await {
        Ok(_) => {
            info!("✅ WebSocket 连接已正常关闭");
        }
        Err(e) => {
            error!("❌ WebSocket 连接失败: {}", e);

            // 提供详细的错误信息和建议
            match &e {
                openlark_client::ws_client::WsClientError::ServerError { code, message } => {
                    error!("📋 服务器错误 - Code: {}, Message: {}", code, message);

                    match code {
                        1 => {
                            error!("💡 可能的原因: App ID 或 App Secret 不正确");
                            error!("💡 解决方案: 请检查 .env 文件中的凭据是否正确");
                        }
                        1000040343 => {
                            error!("💡 可能的原因: 应用未启用 WebSocket 权限");
                            error!("💡 解决方案: 请在飞书开发者后台启用 WebSocket 回调权限");
                        }
                        _ => {
                            error!("💡 请检查网络连接和凭据配置");
                        }
                    }
                }
                openlark_client::ws_client::WsClientError::RequestError(_) => {
                    error!("💡 可能的原因: 网络连接问题或服务器不可达");
                    error!("💡 解决方案: 请检查网络连接和防火墙设置");
                }
                openlark_client::ws_client::WsClientError::UnexpectedResponse => {
                    error!("💡 可能的原因: 服务器返回了意外的响应格式");
                    error!("💡 解决方案: 请检查 API 端点是否正确");
                }
                _ => {
                    error!("💡 请检查配置和网络连接");
                }
            }

            return Err(e.into());
        }
    }

    info!("👋 示例程序结束");
    Ok(())
}


/// 显示连接状态信息
fn display_connection_info() {
    println!("\n📋 连接状态信息:");
    println!("🔗 WebSocket 端点: wss://open.feishu.cn/callback/ws/endpoint");
    println!("💓 心跳间隔: 30秒 (可动态调整)");
    println!("🔄 重连机制: 自动重连 (可配置次数和间隔)");
    println!("📦 数据协议: Protobuf (lark-websocket-protobuf)");
    println!();
}

/// 显示使用提示
fn display_usage_tips() {
    println!("💡 使用提示:");
    println!("   1. 确保在飞书开发者后台启用 WebSocket 权限");
    println!("   2. 配置正确的回调地址");
    println!("   3. 应用需要发布到生产环境或设置测试环境");
    println!("   4. 网络需要能够访问飞书服务器");
    println!();
}
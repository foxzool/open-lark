//! 获取事件出口IP示例
//!
//! 本示例展示如何使用飞书开放平台SDK获取事件推送时使用的出口IP地址。
//! 这些IP地址可以用于配置防火墙规则，允许飞书服务器向您的回调地址推送事件。

#[cfg(feature = "event")]
use open_lark::prelude::*;

#[cfg(feature = "event")]
use open_lark::service::event::v1::GetOutboundIpRequest;

#[cfg(feature = "event")]
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 初始化日志
    env_logger::init();

    // 从环境变量获取配置
    let app_id = std::env::var("APP_ID").unwrap_or_else(|_| "your_app_id".to_string());
    let app_secret = std::env::var("APP_SECRET").unwrap_or_else(|_| "your_app_secret".to_string());

    // 创建客户端
    let client = LarkClient::builder(&app_id, &app_secret).build();

    println!("🚀 开始获取事件出口IP地址...");

    // 创建获取事件出口IP的请求
    let request = GetOutboundIpRequest::default();

    // 调用API
    match client.event.v1.get_outbound_ip(&request).await {
        Ok(response) => {
            println!("✅ 成功获取事件出口IP地址！");
            println!("状态码: {}", response.code);
            println!("消息: {}", response.msg);

            if let Some(data) = response.data {
                println!("\n📋 事件出口IP地址列表:");
                println!("总计 {} 个IP地址:", data.ip_list.len());

                for (index, ip) in data.ip_list.iter().enumerate() {
                    println!("  {}. {}", index + 1, ip);
                }

                println!("\n💡 使用说明:");
                println!("1. 将这些IP地址添加到您的防火墙白名单");
                println!("2. 确保您的回调地址可以从这些IP访问");
                println!("3. 定期检查IP地址列表的变化");
            } else {
                println!("⚠️  响应中没有IP地址数据");
            }
        }
        Err(e) => {
            eprintln!("❌ 获取事件出口IP失败: {}", e);
            eprintln!("请检查:");
            eprintln!("1. 应用ID和应用密钥是否正确");
            eprintln!("2. 应用是否有权限访问事件API");
            eprintln!("3. 网络连接是否正常");
        }
    }

    Ok(())
}

#[cfg(not(feature = "event"))]
fn main() {
    println!("❌ 本示例需要启用 'event' 功能标志");
    println!("请使用以下命令运行:");
    println!("cargo run --example get_outbound_ip --features event");
}

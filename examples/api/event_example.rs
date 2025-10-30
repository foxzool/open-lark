//! 事件模块使用示例
//!
//! 演示如何使用event模块进行事件管理操作，包括：
//! - 获取事件出口IP地址

use open_lark::prelude::*;
use open_lark_extensions::event::v1::{
    GetOutboundIpRequest, OutboundIpResponse
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 初始化日志
    env_logger::init();

    println!("🚀 事件模块示例演示");

    // 创建客户端（这里使用测试配置）
    let client = LarkClient::builder("test_app_id", "test_app_secret")
        .with_app_type(AppType::SelfBuild)
        .build();

    println!("✅ 客户端创建成功");

    // 演示获取事件出口IP地址
    println!("\n📋 获取事件出口IP地址");
    let ip_request = GetOutboundIpRequest {};

    match client.event.v1.get_outbound_ip(&ip_request).await {
        Ok(response) => {
            println!("✅ 事件出口IP地址获取成功");
            if let Some(data) = response.data {
                println!("   找到 {} 个IP地址:", data.ips.len());
                for (i, ip) in data.ips.iter().enumerate() {
                    println!("   {}. {}", i + 1, ip);
                }
                println!("   更新时间: {}", data.updated_time);
            }
        }
        Err(e) => {
            println!("❌ 事件出口IP地址获取失败: {}", e);
        }
    }

    println!("\n🎉 事件模块示例演示完成！");
    Ok(())
}
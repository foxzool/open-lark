//! 发送消息示例
//!
//! 演示如何使用Open-Lark SDK发送各种类型的消息。
//!
//! 运行方式：
//! ```bash
//! cargo run --example send_message --features communication
//! ```

use colored::*;
use openlark_client::LarkClient;
use serde_json::json;

fn print_success(message: &str) {
    println!("{} {}", "✅".green(), message.green());
}

fn print_error(message: &str) {
    println!("{} {}", "❌".red(), message.red());
}

fn print_step(step: usize, description: &str) {
    println!("{} {}: {}", "📍".blue(), step, description.bright_white());
}

fn print_json(data: &serde_json::Value, title: &str) -> Result<(), serde_json::Error> {
    println!("{}", title.bright_cyan().underline());
    println!("{}", serde_json::to_string_pretty(data)?);
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("{}", "🚀 Open-Lark 发送消息示例".bright_green().bold());
    println!(
        "{}",
        "演示如何使用Open-Lark SDK发送各种类型的飞书消息".bright_black()
    );
    println!("{}", "═".repeat(50).bright_black());
    println!();

    print_step(1, "初始化客户端");

    // 从环境变量获取配置
    let app_id = std::env::var("OPENLARK_APP_ID").unwrap_or_else(|_| "demo_app_id".to_string());
    let app_secret =
        std::env::var("OPENLARK_APP_SECRET").unwrap_or_else(|_| "demo_app_secret".to_string());

    let config = openlark_client::Config::builder()
        .app_id(&app_id)
        .app_secret(&app_secret)
        .build()?;

    let client = LarkClient::new(config)?;
    print_success("客户端初始化成功");

    print_step(2, "消息类型演示");

    // 文本消息示例
    let text_message = json!({
        "receive_id": "ou_xxx", // 替换为实际用户ID
        "content": json!({
            "text": "Hello from Open-Lark SDK! 🚀\n这是一条通过SDK发送的测试消息。"
        }),
        "msg_type": "text"
    });

    println!("📝 文本消息示例:");
    print_json(&text_message, "文本消息结构")?;

    // 富文本消息示例
    let rich_text_message = json!({
        "receive_id": "ou_xxx",
        "content": json!({
            "post": {
                "zh_cn": {
                    "title": "🎯 Open-Lark SDK 富文本消息",
                    "content": [
                        [
                            {
                                "tag": "text",
                                "text": "这是一条富文本消息，包含"
                            },
                            {
                                "tag": "text",
                                "text": "多种格式",
                                "style": ["bold", "color", "underline"]
                            },
                            {
                                "tag": "text",
                                "text": "的文本内容。"
                            }
                        ]
                    ]
                }
            }
        }),
        "msg_type": "post"
    });

    println!("📄 富文本消息示例:");
    print_json(&rich_text_message, "富文本消息结构")?;

    // 卡片消息示例
    let card_message = json!({
        "receive_id": "ou_xxx",
        "content": json!({
            "card": {
                "header": {
                    "title": {
                        "tag": "plain_text",
                        "content": "🚀 Open-Lark SDK 交互卡片"
                    },
                    "template": "blue"
                },
                "elements": [
                    {
                        "tag": "div",
                        "text": {
                            "tag": "plain_text",
                            "content": "这是一个交互卡片示例，展示了SDK的强大功能。"
                        }
                    }
                ]
            }
        }),
        "msg_type": "interactive"
    });

    println!("🃏 卡片消息示例:");
    print_json(&card_message, "卡片消息结构")?;

    print_step(3, "发送最佳实践");

    let best_practices = vec![
        "📝 确保接收者ID格式正确",
        "🔗 遵守飞书API的速率限制",
        "🔄 实现重试机制处理网络错误",
        "📊 记录消息发送结果",
        "🛡️ 验证消息内容和格式",
        "⚡ 使用批量发送提高效率",
    ];

    println!("消息发送最佳实践:");
    for (i, practice) in best_practices.iter().enumerate() {
        println!("  {}. {}", i + 1, practice);
    }

    print_step(4, "错误处理");

    let error_handling = json!({
        "常见错误": {
            "接收者不存在": "检查receive_id是否正确",
            "权限不足": "确保应用有发送消息权限",
            "消息格式错误": "验证content字段格式",
            "网络超时": "实现重试机制",
            "速率限制": "控制发送频率"
        },
        "处理策略": {
            "临时错误": "使用指数退避重试",
            "永久错误": "记录日志并通知用户",
            "格式错误": "在发送前验证消息格式",
            "权限错误": "检查应用权限设置"
        }
    });

    println!("🚨 错误处理指南:");
    print_json(&error_handling, "错误处理策略")?;

    println!();
    println!("{}", "═".repeat(50).bright_black());
    print_success("示例执行完成！");
    println!("💡 下一步: 学习用户管理 -> cargo run --example user_management");
    println!();

    Ok(())
}

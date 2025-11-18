//! CardKit API 使用示例
//!
//! 展示如何使用 CardKit API 创建和发送消息卡片

use openlark_docs::prelude::*;
use serde_json::json;

#[tokio::main]
async fn main() -> Result<()> {
    // 初始化日志
    env_logger::init();

    // 从环境变量加载配置
    let config = Config::from_env()?;

    // 创建 CardKit 服务
    let cardkit_service = CardKitService::new(&config);

    // 示例1: 创建简单文本卡片
    let text_card = create_text_card();
    println!(
        "创建文本卡片: {}",
        serde_json::to_string_pretty(&text_card)?
    );

    // 示例2: 创建富媒体卡片
    let rich_card = create_rich_card();
    println!(
        "创建富媒体卡片: {}",
        serde_json::to_string_pretty(&rich_card)?
    );

    // 示例3: 发送卡片（需要配置真实的环境变量）
    // let result = send_card(&cardkit_service, "user_id", text_card).await?;
    // println!("发送卡片结果: {:?}", result);

    Ok(())
}

/// 创建简单的文本卡片
fn create_text_card() -> serde_json::Value {
    json!({
        "config": {
            "wide_screen_mode": true
        },
        "header": {
            "title": {
                "content": "文本卡片示例",
                "tag": "plain_text"
            }
        },
        "elements": [
            {
                "tag": "div",
                "text": {
                    "content": "这是一个简单的文本卡片示例",
                    "tag": "plain_text"
                }
            },
            {
                "tag": "action",
                "actions": [
                    {
                        "tag": "button",
                        "text": {
                            "content": "确定",
                            "tag": "plain_text"
                        },
                        "type": "default",
                        "value": {
                            "action": "confirm"
                        }
                    }
                ]
            }
        ]
    })
}

/// 创建富媒体卡片
fn create_rich_card() -> serde_json::Value {
    json!({
        "config": {
            "wide_screen_mode": true
        },
        "header": {
            "title": {
                "content": "富媒体卡片示例",
                "tag": "plain_text"
            },
            "template": "turquoise"
        },
        "elements": [
            {
                "tag": "div",
                "text": {
                    "content": "**这是一个富媒体卡片示例**\n\n支持以下特性：\n• Markdown 格式\n• 图片展示\n• 交互按钮",
                    "tag": "lark_md"
                }
            },
            {
                "tag": "img",
                "img_key": "img_v2_041b8e4c-8d5f-4b8e-a1e1-3d8e5c5f5a5g",
                "alt": {
                    "content": "示例图片",
                    "tag": "plain_text"
                }
            },
            {
                "tag": "hr"
            },
            {
                "tag": "note",
                "elements": [
                    {
                        "tag": "lark_md",
                        "content": "**注意**: 这是一个示例卡片，仅用于演示功能"
                    }
                ]
            },
            {
                "tag": "action",
                "actions": [
                    {
                        "tag": "button",
                        "text": {
                            "content": "主要操作",
                            "tag": "plain_text"
                        },
                        "type": "primary",
                        "value": {
                            "action": "primary"
                        }
                    },
                    {
                        "tag": "button",
                        "text": {
                            "content": "次要操作",
                            "tag": "plain_text"
                        },
                        "type": "default",
                        "value": {
                            "action": "secondary"
                        }
                    }
                ]
            }
        ]
    })
}

/// 发送卡片到指定用户
async fn send_card(
    service: &CardKitService,
    user_id: &str,
    card: serde_json::Value,
) -> Result<serde_json::Value> {
    let request = CardSendRequest {
        receive_id_type: Some("user_id".to_string()),
        receive_id: user_id.to_string(),
        card_content: serde_json::to_string(&card)?,
        ..Default::default()
    };

    let response = service.send_card(&request).await?;
    Ok(serde_json::to_value(response)?)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_text_card() {
        let card = create_text_card();
        assert!(card.get("config").is_some());
        assert!(card.get("header").is_some());
        assert!(card.get("elements").is_some());
    }

    #[test]
    fn test_create_rich_card() {
        let card = create_rich_card();
        assert!(card.get("config").is_some());
        assert!(card.get("header").is_some());
        assert!(card.get("elements").is_some());

        let elements = card["elements"].as_array().unwrap();
        assert!(!elements.is_empty());

        // 检查是否包含图片元素
        let has_img = elements
            .iter()
            .any(|e| e.get("tag").and_then(|t| t.as_str()) == Some("img"));
        assert!(has_img);
    }

    #[test]
    fn test_card_serialization() {
        let card = create_text_card();
        let json_str = serde_json::to_string(&card);
        assert!(json_str.is_ok());

        let parsed: serde_json::Value = serde_json::from_str(&json_str.unwrap());
        assert!(parsed.is_ok());
    }
}

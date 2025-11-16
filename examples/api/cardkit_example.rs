//! CardKit 卡片组件使用示例
//!
//! 演示如何使用飞书卡片组件API创建交互式卡片

use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 注意：这是一个示例，实际使用时需要配置正确的认证信息
    // let config = Config::builder()
    //     .app_id("your_app_id")
    //     .app_secret("your_app_secret")
    //     .build()?;

    // let service = CardKitService::new(config);

    // 示例1: 创建交互式卡片
    create_interactive_card_example()?;

    // 示例2: 创建卡片组件
    create_card_element_example()?;

    // 示例3: 更新组件内容
    update_element_content_example()?;

    println!("所有 CardKit 示例演示完成！");
    Ok(())
}

/// 示例1: 创建交互式卡片
fn create_interactive_card_example() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n=== 创建交互式卡片示例 ===");

    // 构建卡片内容
    let card_content = json!({
        "config": {
            "wide_screen_mode": true
        },
        "elements": [
            {
                "tag": "div",
                "text": {
                    "tag": "plain_text",
                    "content": "欢迎使用飞书卡片！"
                }
            },
            {
                "tag": "action",
                "actions": [
                    {
                        "tag": "button",
                        "text": {
                            "tag": "plain_text",
                            "content": "确认"
                        },
                        "type": "primary",
                        "value": {
                            "action": "confirm"
                        }
                    }
                ]
            }
        ]
    });

    println!("卡片内容: {}", serde_json::to_string_pretty(&card_content)?);

    // 在实际使用中，你会这样创建卡片：
    /*
    let response = service
        .card()
        .create_card_builder()
        .card_content(card_content)
        .card_type("interactive")
        .execute(&service.card())
        .await?;

    if let Some(card_id) = response.card_id {
        println!("创建的卡片ID: {}", card_id);
    }
    */

    Ok(())
}

/// 示例2: 创建卡片组件
fn create_card_element_example() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n=== 创建卡片组件示例 ===");

    // 构建文本组件
    let text_element = json!({
        "tag": "div",
        "text": {
            "tag": "plain_text",
            "content": "这是一个文本组件"
        }
    });

    println!("组件内容: {}", serde_json::to_string_pretty(&text_element)?);

    // 在实际使用中，你会这样添加组件：
    /*
    let response = service
        .card_element()
        .create_card_element_builder()
        .card_id("your_card_id")
        .element(text_element)
        .element_type("text")
        .execute(&service.card_element())
        .await?;

    if let Some(element_id) = response.element_id {
        println!("创建的组件ID: {}", element_id);
    }
    */

    Ok(())
}

/// 示例3: 更新组件内容
fn update_element_content_example() -> Result<(), Box<dyn std::error::Error>> {
    println!("\n=== 更新组件内容示例 ===");

    // 构建新的文本内容
    let new_content = json!("更新后的文本内容");

    println!("新内容: {}", serde_json::to_string_pretty(&new_content)?);

    // 在实际使用中，你会这样更新内容：
    /*
    let response = service
        .card_element()
        .update_card_element_content_builder()
        .card_id("your_card_id")
        .element_id("your_element_id")
        .content(new_content)
        .append(false)
        .stream(false)
        .execute(&service.card_element())
        .await?;

    if response.success.unwrap_or(false) {
        println!("内容更新成功！");
    }
    */

    Ok(())
}

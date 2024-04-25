use std::env;

use dotenvy::dotenv;
use tokio::time::sleep;

use open_lark::message::{
    CustomRichTextMessage, InteractiveMessage, MessageCard, RichTextParagraph, TextMessage,
};
use open_lark::prelude::*;

#[tokio::main]
async fn main() {
    dotenv().expect(".env file not found");

    // 创建 CustomBot 实例
    let bot = CustomBot::new(env::var("URL").unwrap(), None);
    // bot.send_raw_message(json!({
    //     "msg_type": "text",
    //     "content": {
    //         "text": "新更新提醒"
    //     }
    // }));

    // 发送文本消息
    let message = TextMessage::new("新更新提醒");

    bot.send_message(message).await;
    sleep(std::time::Duration::from_secs(1)).await;

    // 发送富文本消息
    let content = vec![
        vec![RichTextParagraph::Text {
            text: "项目有更新: ".to_string(),
            un_escape: None,
        }],
        vec![RichTextParagraph::A {
            text: "请查看".to_string(),
            href: "http://www.example.com/".to_string(),
        }],
    ];
    let message = CustomRichTextMessage::new("富文本标题", content);
    bot.send_message(message).await;

    // 发送消息卡片
    let message = InteractiveMessage::JsonCard(MessageCard {
        header: Default::default(),
        elements: Default::default(),
        ..Default::default()
    });
    bot.send_message(message).await;
}

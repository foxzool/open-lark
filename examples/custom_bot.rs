use std::thread::sleep;

use open_lark::message::{RichTextMessage, RichTextParagraph, TextMessage};
use open_lark::prelude::*;

fn main() {
    // 获取命令行参数
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <webhook_url> [secret]", args[0]);
        std::process::exit(1);
    }

    // 创建 CustomBot 实例
    let bot = CustomBot::new(&args[1], args.get(2).map(|s| s.as_str()));
    // bot.send_raw_message(json!({
    //     "msg_type": "text",
    //     "content": {
    //         "text": "新更新提醒"
    //     }
    // }));

    // 发送文本消息
    let message = TextMessage::new("新更新提醒");

    bot.send_message(message);
    sleep(std::time::Duration::from_secs(1));

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
    let message = RichTextMessage::new("富文本标题", content);
    bot.send_message(message);
}

use std::thread::sleep;

use open_lark::message::MessageBuilder;
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
    // let message = MessageBuilder::new_text().add_text("纯文本消息").build();
    // println!("{:?}", serde_json::to_string(&message).unwrap());
    // bot.send_message(&message);

    sleep(std::time::Duration::from_secs(1));
    // 发送富文本消息
    let message = MessageBuilder::new_rich_text().build();

    println!("{:?}", serde_json::to_string(&message).unwrap());
}

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
    bot.send_text_message("纯文本")
}

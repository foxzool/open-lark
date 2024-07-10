use std::env;

use dotenvy::dotenv;
use serde_json::json;
use tokio::time::sleep;

use open_lark::{
    custom_bot::CustomBot,
    service::im::v1::message::{
        ANode, AtNode, MessageCardTemplate, MessagePost, MessagePostNode, MessageText, TextNode,
    },
};

#[tokio::main]
async fn main() {
    dotenv().expect(".env file not found");
    let hook_url = &(env::var("HOOK_URL").unwrap());
    let secret = env::var("HOOK_SECRET").ok();
    // 创建 CustomBot 实例
    let bot = CustomBot::new(hook_url, secret.as_deref());

    // 发送文本消息
    let message = MessageText::new("新更新提醒");

    bot.send_message(message).await.unwrap();
    sleep(std::time::Duration::from_secs(1)).await;

    // 发送富文本消息
    let message = MessagePost::new("zh_cn")
        .title("我是一个标题")
        .append_content(vec![
            MessagePostNode::Text(TextNode::new("新版本已发布，快来体验吧！")),
            MessagePostNode::A(ANode::new("查看详情", "https://www.feishu.cn")),
            MessagePostNode::At(AtNode::new("ou_1avnmsbv3k45jnk34j5")),
        ]);
    bot.send_message(message).await.unwrap();

    // 发送消息卡片
    let card_template = MessageCardTemplate::new(
        "AAqk4PdEIBaSV",
        json!({"project_name": "project", "address": "address", "money": 100, "zlrq": "zlrq", "comment": "comment", "search_url": "search_url"}),
    );
    let _resp = bot.send_card(card_template).await.unwrap();
}

use std::env;

use dotenvy::dotenv;
use serde_json::json;
use uuid::Uuid;

use open_lark::{
    client::LarkClientBuilder,
    service::im::v1::message::{
        ANode, AtNode, CreateMessageReqBody, CreateMessageReqBuilder, EmotionNode, ImgNode,
        MessageCardTemplate, MessagePost, MessagePostNode, MessageText, SendMessageTrait, TextNode,
    },
};

// POST /open-apis/im/v1/messages

fn main() {
    dotenv().expect(".env file not found");
    env_logger::init();
    let app_id = env::var("APP_ID").unwrap();
    let app_secret = env::var("APP_SECRET").unwrap();
    // 创建 Client
    let client = LarkClientBuilder::new(&app_id, &app_secret).build();
    let uuid = Uuid::new_v4();
    // 文本
    let _text_message = MessageText::new("hello world")
        .text_line("next line")
        .add_text("你好!")
        .build();

    // 富文本
    let _rich_text_message = MessagePost::new("zh_cn")
        .title("我是一个标题")
        .append_content(vec![
            MessagePostNode::Text(TextNode::new("第一行:").style(vec!["bold", "underline"])),
            MessagePostNode::A(
                ANode::new("超链接", "https://www.feishu.cn").style(vec!["bold", "underline"]),
            ),
            MessagePostNode::At(AtNode::new("ou_1avnmsbv3k45jnk34j5").style(vec!["lineThrough"])),
        ])
        .append_content(vec![MessagePostNode::Img(ImgNode::new(
            "img_7ea74629-9191-4176-998c-2e603c9c5e8g",
        ))])
        .append_content(vec![MessagePostNode::Emotion(EmotionNode::new("SMILE"))]);

    // 卡片模板
    let card_template = MessageCardTemplate::new(
        "AAqk4PdEIBaSV",
        json!({"project_name": "project", "address": "address", "money": "money", "zlrq": "zlrq", "comment": "comment", "search_url": "search_url"}),
    );

    let req = CreateMessageReqBuilder::new()
        .receive_id_type("chat_id")
        .body(CreateMessageReqBody {
            receive_id: "oc_84d53efe245072c16ba4b4ff597f52f3".to_string(),
            msg_type: card_template.msg_type(),
            content: card_template.content(),
            uuid: Some(uuid.to_string()),
        })
        .build();

    // 发起请求
    match client.im.v1.message.create(req, None) {
        Ok(resp) => {
            if resp.success() {
                // 业务处理
                println!("response: {:?}", resp.data);
            } else {
                println!("send message failed: {} ", resp.error_msg());
            }
        }
        Err(err) => {
            println!("send message http error: {} ", err.to_string());
        }
    }
}

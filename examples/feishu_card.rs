use std::env;

use dotenvy::dotenv;
use uuid::Uuid;

use open_lark::client::LarkClientBuilder;
use open_lark::feishu_card::{FeishuCardBuilder, FeishuCardConfig, FeishuCardConfigBuilder};
use open_lark::service::im::v1::message::{
    CreateMessageReqBody, CreateMessageReqBuilder, MessagePost, MessagePostNode,
    MessageTextBuilder, SendMessageTrait,
};



fn main() {
    dotenv().expect(".env file not found");
    env_logger::init();
    let app_id = env::var("APP_ID").unwrap();
    let app_secret = env::var("APP_SECRET").unwrap();
    // 创建 Client
    let client = LarkClientBuilder::new(&app_id, &app_secret).build();
    let uuid = Uuid::new_v4();
    // 飞书卡片
    let feishu_card = FeishuCardBuilder::new().build();



    // 富文本
    let rich_text_message = MessagePost::zh_cn()
        .title("我是一个标题")
        .append_content(vec![
            MessagePostNode::Text {
                text: "我是一个文本".to_string(),
                un_escape: None,
                style: Some(vec!["bold".to_string(), "underline".to_string()]),
            },
            MessagePostNode::A {
                text: "超链接".to_string(),
                href: "https://www.feishu.cn".to_string(),
                style: Some(vec!["bold".to_string(), "italic".to_string()]),
            },
            MessagePostNode::At {
                user_id: "ou_1avnmsbv3k45jnk34j5".to_string(),
                style: Some(vec!["lineThrough".to_string()]),
            },
        ])
        .append_content(vec![MessagePostNode::Img {
            image_key: "img_7ea74629-9191-4176-998c-2e603c9c5e8g".to_string(),
        }])
        // .append_content(vec![MessagePostElement::Media {
        //     file_key: "75235e0c-4f92-430a-a99b-8446610223cg".to_string(),
        //     image_key: Some("img_7ea74629-9191-4176-998c-2e603c9c5e8g".to_string()),
        // }])
        .append_content(vec![MessagePostNode::Emotion {
            emoji_type: "SMILE".to_string(),
        }]);

    let req = CreateMessageReqBuilder::new()
        .receive_id_type("chat_id")
        .body(CreateMessageReqBody {
            receive_id: "oc_84d53efe245072c16ba4b4ff597f52f3".to_string(),
            msg_type: rich_text_message.msg_type(),
            content: rich_text_message.content(),
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

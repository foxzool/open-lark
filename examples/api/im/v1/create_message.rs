use std::env;

use dotenvy::dotenv;
use serde_json::json;
use uuid::Uuid;

use open_lark::client::LarkClientBuilder;
use open_lark::service::im::v1::message::{CreateMessageReqBody, CreateMessageReqBuilder};

// POST /open-apis/im/v1/messages

fn main() {
    dotenv().expect(".env file not found");
    env_logger::init();
    let app_id = env::var("APP_ID").unwrap();
    let app_secret = env::var("APP_SECRET").unwrap();
    // 创建 Client
    let client = LarkClientBuilder::new(&app_id, &app_secret).build();
    let uuid = Uuid::new_v4();
    let req = CreateMessageReqBuilder::new()
        .receive_id_type("chat_id")
        .body(CreateMessageReqBody {
            receive_id: "oc_84d53efe245072c16ba4b4ff597f52f3".to_string(),
            msg_type: "text".to_string(),
            content: json!("{\"text\":\"test content!\"}"),
            uuid: Some(uuid.to_string()),
        })
        .build();

    // 发起请求
    let resp = client.im.v1.message.create(req, None).unwrap();

    if resp.success() {
        // 业务处理
        println!("response: {:?}", resp.data);
    } else {
        println!("send message failed: {} ", resp.error_msg());
    }
}

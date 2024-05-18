use std::env;

use dotenvy::dotenv;

use open_lark::{client::LarkClientBuilder, service::im::v1::message::ListMessageReqBuilder};

/// 获取会话历史消息
/// GET /open-apis/im/v1/messages
fn main() {
    dotenv().expect(".env file not found");
    env_logger::init();
    let app_id = env::var("APP_ID").unwrap();
    let app_secret = env::var("APP_SECRET").unwrap();
    // 创建 Client
    let client = LarkClientBuilder::new(&app_id, &app_secret).build();
    let req = ListMessageReqBuilder::new()
        .container_id_type("chat")
        .container_id("oc_84d53efe245072c16ba4b4ff597f52f3")
        .build();

    // 发起请求
    let resp = client.im.v1.message.list(&req, None).unwrap();
    println!("response: {:?}", resp);

    // 使用迭代器
    client
        .im
        .v1
        .message
        .list_iter(req, None)
        .for_each(|messages| {
            for message in messages {
                println!("message {:?}", message);
            }
        })
}

use std::env;

use dotenvy::dotenv;

use open_lark::{client::LarkClientBuilder, service::im::v1::chats::ListChatReq};

/// 获取用户或机器人所在的群列表
/// GET /open-apis/im/v1/messages
fn main() {
    dotenv().expect(".env file not found");
    env_logger::init();
    let app_id = env::var("APP_ID").unwrap();
    let app_secret = env::var("APP_SECRET").unwrap();
    // 创建 Client
    let client = LarkClientBuilder::new(&app_id, &app_secret).build();
    let req = ListChatReq::new().build();

    // 发起请求
    let resp = client.im.v1.chats.list(&req, None).unwrap();
    println!("response: {:?}", resp);

    // 使用迭代器
    client.im.v1.chats.list_iter(req, None).for_each(|chats| {
        for chat in chats {
            println!("chat {:?}", chat);
        }
    })
}

use std::env;

use dotenvy::dotenv;
use log::{error, info};

use open_lark::client::LarkClient;
use open_lark::service::im::v1::chats::ListChatReqBuilder;

/// 获取用户或机器人所在的群列表
/// GET /open-apis/im/v1/messages
fn main() {
    dotenv().expect(".env file not found");
    env_logger::init();
    let app_id = env::var("APP_ID").unwrap();
    let app_secret = env::var("APP_SECRET").unwrap();
    // 创建 Client
    let client = LarkClient::new(&app_id, &app_secret).build();
    let mut req = ListChatReqBuilder::new().build();

    // 发起请求
    let resp = client.im.v1.chats.list(&mut req, &[]).unwrap();
    if resp.success() {
        // 业务处理
        info!("response: {:?}", resp.data);
    } else {
        error!("list chat failed: {} ", resp.error_msg());
    }

    // 使用迭代器
    client.im.v1.chats.list_iter(req, vec![]).for_each(|chats| {
        for chat in chats {
            info!("chat {:?}", chat);
        }
    })
}

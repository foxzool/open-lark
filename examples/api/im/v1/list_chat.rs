use std::env;

use dotenvy::dotenv;
use log::{error, info};

use open_lark::client::LarkClient;
use open_lark::service::im::v1::chats::ListChatReqBuilder;

// GET /open-apis/im/v1/messages
fn main() {
    dotenv().expect(".env file not found");
    env_logger::init();
    let app_id = env::var("APP_ID").unwrap();
    let app_secret = env::var("APP_SECRET").unwrap();
    // 创建 Client
    let client = LarkClient::new(&app_id, &app_secret).build();
    let req = ListChatReqBuilder::new().build();

    // 发起请求
    let resp = client.im.v1.chats.list(req).unwrap();

    if resp.success() {
        // 业务处理
        info!("response: {:?}", resp.data);
    } else {
        error!(
            "send message failed: {} {} {}",
            resp.code_error.code,
            resp.code_error.msg,
            resp.api_resp.request_id()
        );
    }
}

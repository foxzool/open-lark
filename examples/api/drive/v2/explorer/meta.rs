use std::env;

use dotenvy::dotenv;

use open_lark::{
    client::LarkClientBuilder,
    service::{drive::v1::files::UploadAllRequest, im::v1::chats::ListChatReqBuilder},
};

/// 获取我的空间（root folder）元信息
fn main() {
    dotenv().expect(".env file not found");
    env_logger::init();
    let app_id = env::var("APP_ID").unwrap();
    let app_secret = env::var("APP_SECRET").unwrap();
    // 创建 Client
    let client = LarkClientBuilder::new(&app_id, &app_secret).build();


    // 发起请求
    let resp = client.drive.v2.explorer.meta_data().unwrap();
    println!("response: {:?}", resp);
}

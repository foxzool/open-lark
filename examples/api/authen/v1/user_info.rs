use dotenvy::dotenv;
use std::env;

use open_lark::client::LarkClient;

/// 获取登录用户信息
#[tokio::main]
async fn main() {
    dotenv().expect(".env file not found");
    env_logger::init();
    let app_id = env::var("APP_ID").unwrap();
    let app_secret = env::var("APP_SECRET").unwrap();
    let user_access_token = env::var("USER_ACCESS_TOKEN").unwrap();
    // 创建 Client
    let client = LarkClient::builder(&app_id, &app_secret).build();

    // 发起请求
    let resp = client
        .auth
        .v1
        .user_info
        .get(user_access_token)
        .await
        .unwrap();
    match resp.data.as_ref() {
        Some(data) => {
            println!("user_info: {:#?}", data);
        }
        None => {
            println!("user_info error: {:#?}", resp);
        }
    }
}

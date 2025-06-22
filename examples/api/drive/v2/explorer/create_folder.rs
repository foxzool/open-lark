use std::env;

use dotenvy::dotenv;

use open_lark::{prelude::LarkClient, service::drive::v2::explorer::CreateFolderRequest};

/// 新建文件夹
#[tokio::main]
async fn main() {
    dotenv().expect(".env file not found");
    env_logger::init();
    let app_id = env::var("APP_ID").unwrap();
    let app_secret = env::var("APP_SECRET").unwrap();
    // 创建 Client
    let client = LarkClient::builder(&app_id, &app_secret).build();

    let req = CreateFolderRequest::builder()
        .name("收入报表")
        .folder_token("")
        .build();
    // 发起请求
    let resp = client
        .drive
        .v2
        .explorer
        .create_folder(req, None)
        .await
        .unwrap();
    if let Some(data) = resp.data {
        println!("response: {:#?}", data);
    }
}

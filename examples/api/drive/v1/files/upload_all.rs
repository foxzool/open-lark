use std::env;

use dotenvy::dotenv;

use open_lark::{
    client::LarkClientBuilder,
    service::{drive::v1::files::UploadAllRequest, im::v1::chats::ListChatReq},
};

/// 向云空间指定目录下上传一个小文件
fn main() {
    dotenv().expect(".env file not found");
    env_logger::init();
    let app_id = env::var("APP_ID").unwrap();
    let app_secret = env::var("APP_SECRET").unwrap();
    // 创建 Client
    let client = LarkClientBuilder::new(&app_id, &app_secret).build();

    let req = UploadAllRequest::default();
    // 发起请求
    let resp = client.drive.v1.files.upload_all(req).unwrap();
    println!("response: {:?}", resp);
}

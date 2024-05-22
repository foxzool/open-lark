use std::env;

use dotenvy::dotenv;

use open_lark::{
    client::LarkClientBuilder,
};
use open_lark::core::api_resp::ApiResponse;

/// 获取我的空间（root folder）元信息
fn main() {
    dotenv().expect(".env file not found");
    env_logger::init();
    let app_id = env::var("APP_ID").unwrap();
    let app_secret = env::var("APP_SECRET").unwrap();
    // 创建 Client
    let client = LarkClientBuilder::new(&app_id, &app_secret).build();


    // 发起请求
    let resp = client.drive.v2.explorer.root_folder_meta().unwrap();
    if let ApiResponse::Success { data, ..} = resp {
        println!("root_meta: {:#?}", data);

        let resp = client.drive.v2.explorer.folder_meta(&data.token).unwrap();
        println!("folder_meta: {:?}", resp)
    }
}

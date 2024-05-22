use std::env;

use dotenvy::dotenv;

use open_lark::{
    client::LarkClientBuilder, core::api_resp::ApiResponse,
    service::drive::v2::explorer::CreateFolderReq,
};

/// 新建文件夹
fn main() {
    dotenv().expect(".env file not found");
    env_logger::init();
    let app_id = env::var("APP_ID").unwrap();
    let app_secret = env::var("APP_SECRET").unwrap();
    // 创建 Client
    let client = LarkClientBuilder::new(&app_id, &app_secret).build();

    let req = CreateFolderReq::new()
        .name("测试文件夹")
        .folder_token("nodcnBh4MAgg2GpI5IkRVZuw3Jd")
        .build();
    // 发起请求
    let resp = client.drive.v2.explorer.create_folder(req, None).unwrap();
    if let ApiResponse::Success { data, .. } = resp {
        println!("response: {:#?}", data);
    }
}

use std::env;

use dotenvy::dotenv;

use open_lark::prelude::LarkClient;

/// 获取我的空间（root folder）元信息
#[tokio::main]
async fn main() {
    dotenv().expect(".env file not found");
    env_logger::init();
    let app_id = env::var("APP_ID").unwrap();
    let app_secret = env::var("APP_SECRET").unwrap();
    // 创建 Client
    let client = LarkClient::builder(&app_id, &app_secret).build();

    // 发起请求
    let resp = client
        .drive
        .v2
        .explorer
        .root_folder_meta(None)
        .await
        .unwrap();
    if let Some(data) = resp.data {
        println!("root_meta: {:#?}", data);

        // 获取文件夹元信息
        let resp = client
            .drive
            .v2
            .explorer
            .folder_meta(&data.token, None)
            .await
            .unwrap();
        println!("folder_meta: {:?}", resp)
    }
}

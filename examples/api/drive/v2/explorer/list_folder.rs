use std::env;

use dotenvy::dotenv;

use open_lark::{client::LarkClient, service::drive::v2::explorer::ListFolderRequest};

/// 获取文件夹下的清单
#[tokio::main]
async fn main() {
    dotenv().expect(".env file not found");
    env_logger::init();
    let app_id = env::var("APP_ID").unwrap();
    let app_secret = env::var("APP_SECRET").unwrap();
    // let user_access_token = env::var("USER_ACCESS_TOKEN").unwrap();
    // 创建 Client
    let client = LarkClient::builder(&app_id, &app_secret).build();

    let req = ListFolderRequest::builder().folder_token("").build();
    // 发起请求
    let resp = client
        .drive
        .v2
        .explorer
        .list_folder(
            req.clone(),
            None,
            // Some(
            //     RequestOption::builder()
            //         .user_access_token(user_access_token)
            //         .build(),
            // ),
        )
        .await
        .unwrap();
    if let Some(data) = resp.data {
        println!("response: {:#?}", data);
    }

    // 使用迭代器
    let mut iterator = client.drive.v2.explorer.list_folder_iter(req, None);
    while let Some(folders) = iterator.next().await {
        for folder in folders {
            println!("folder {:?}", folder);
        }
    }
}

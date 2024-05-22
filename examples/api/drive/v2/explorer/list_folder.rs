use std::env;

use dotenvy::dotenv;

use open_lark::{
    client::LarkClientBuilder,
    core::{api_resp::ApiResponse, req_option::RequestOption},
    service::drive::v2::explorer::ListFolderReq,
};

/// 获取文件夹下的清单
fn main() {
    dotenv().expect(".env file not found");
    env_logger::init();
    let app_id = env::var("APP_ID").unwrap();
    let app_secret = env::var("APP_SECRET").unwrap();
    // 创建 Client
    let client = LarkClientBuilder::new(&app_id, &app_secret).build();

    let req = ListFolderReq::new().build();
    // 发起请求
    let resp = client
        .drive
        .v2
        .explorer
        .list_folder(req.clone(), None)
        .unwrap();
    if let ApiResponse::Success { data, .. } = resp {
        println!("response: {:#?}", data);
    }

    // 使用迭代器
    client
        .drive
        .v2
        .explorer
        .list_folder_iter(
            req,
            Some(RequestOption {
                user_access_token: env::var("USER_ACCESS_TOKEN").unwrap(),
                ..Default::default()
            }),
        )
        .for_each(|folders| {
            for folder in folders {
                println!("folder {:?}", folder);
            }
        })
}

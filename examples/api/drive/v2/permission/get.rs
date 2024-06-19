use std::env;

use dotenvy::dotenv;

use open_lark::{
    client::LarkClientBuilder, core::req_option::RequestOption,
    service::drive::v1::permissions::GetPermissionRequest,
};

/// 获取云文档权限设置
#[tokio::main]
async fn main() {
    dotenv().expect(".env file not found");
    env_logger::init();
    let app_id = env::var("APP_ID").unwrap();
    let app_secret = env::var("APP_SECRET").unwrap();
    let user_access_token = env::var("USER_ACCESS_TOKEN").unwrap();
    // 创建 Client
    let client = LarkClientBuilder::new(&app_id, &app_secret).build();
    // 构建请求体
    let req = GetPermissionRequest::builder()
        .token("UONOs1jSahmgoGtOlKHcRCaPnKd")
        .r#type("sheet")
        .build();
    // 发起请求
    let resp = client
        .drive
        .v2
        .permission
        .get(
            req,
            Some(
                RequestOption::builder()
                    .user_access_token(user_access_token.clone())
                    .build(),
            ),
        )
        .await
        .unwrap();
    if let Some(data) = resp.data {
        println!("patch spreadsheet response: {:#?}", data);
    }
}

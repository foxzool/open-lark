use std::env;

use dotenvy::dotenv;

use open_lark::{prelude::LarkClient, service::sheets::v3::spreadsheet::CreateSpreedSheetRequest};

/// 创建表格
#[tokio::main]
async fn main() {
    dotenv().expect(".env file not found");
    env_logger::init();
    let app_id = env::var("APP_ID").unwrap();
    let app_secret = env::var("APP_SECRET").unwrap();
    // let user_access_token = env::var("USER_ACCESS_TOKEN").unwrap();
    // 创建 Client
    let client = LarkClient::builder(&app_id, &app_secret).build();

    let req = CreateSpreedSheetRequest::builder()
        .folder_token("FQmNfKO5plqA0sdnSXYcDifknEe")
        .title("2024年收入")
        .build();

    // 发起请求
    let resp = client
        .sheets
        .v3
        .spreadsheet
        .create(
            req,
            None,
            // Some(
            //     RequestOption::builder()
            //         .user_access_token(user_access_token.clone())
            //         .build(),
            // ),
        )
        .await
        .unwrap();
    if let Some(data) = resp.data {
        println!("create spread response: {:#?}", data.spreadsheet);
    }
}

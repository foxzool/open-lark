use std::env;

use dotenvy::dotenv;

use open_lark::{
    client::LarkClientBuilder,
    core::{api_resp::ApiResponse, req_option::RequestOption},
    service::sheets::v3::spreadsheet::CreateSpreedSheetRequest,
};

/// 创建表格
fn main() {
    dotenv().expect(".env file not found");
    env_logger::init();
    let app_id = env::var("APP_ID").unwrap();
    let app_secret = env::var("APP_SECRET").unwrap();
    let user_access_token = env::var("USER_ACCESS_TOKEN").unwrap();
    // 创建 Client
    let client = LarkClientBuilder::new(&app_id, &app_secret).build();

    let req = CreateSpreedSheetRequest::builder().title("xx").build();

    // 发起请求
    let resp = client
        .sheets
        .v3
        .spreadsheet
        .create(
            req,
            Some(
                RequestOption::builder()
                    .user_access_token(user_access_token.clone())
                    .build(),
            ),
        )
        .unwrap();
    if let ApiResponse::Success { data, .. } = resp {
        println!("create spread response: {:#?}", data);
    }
}

use std::env;

use dotenvy::dotenv;

use open_lark::{
    client::LarkClientBuilder, service::sheets::v3::spreadsheet::GetSpreadsheetRequest,
};

/// 获取电子表格信息
#[tokio::main]
async fn main() {
    dotenv().expect(".env file not found");
    env_logger::init();
    let app_id = env::var("APP_ID").unwrap();
    let app_secret = env::var("APP_SECRET").unwrap();
    // 创建 Client
    let client = LarkClientBuilder::new(&app_id, &app_secret).build();
    // 构建请求体
    let req = GetSpreadsheetRequest::builder()
        .spreadsheet_token("MSK5sCjxdhprI3toTNDcN61rnbf")
        .build();
    // 发起请求
    let resp = client
        .sheets
        .v3
        .spreadsheet
        .get(
            req,
            None, /* Some(
                  *     open_lark::core::req_option::RequestOption::builder()
                  *         .user_access_token(env::var("USER_ACCESS_TOKEN").unwrap())
                  *         .build(),
                  * ), */
        )
        .await
        .unwrap();
    if let Some(data) = resp.data {
        println!("patch spread response: {:#?}", data);
    }
}

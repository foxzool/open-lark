use std::env;

use dotenvy::dotenv;

use open_lark::{
    client::LarkClient, service::sheets::v2::data_operation::ReadingMultipleRangeRequest,
};

/// 读取多个范围
#[tokio::main]
async fn main() {
    dotenv().expect(".env file not found");
    env_logger::init();
    let app_id = env::var("APP_ID").unwrap();
    let app_secret = env::var("APP_SECRET").unwrap();
    // 创建 Client
    let client = LarkClient::builder(&app_id, &app_secret).build();

    let req = ReadingMultipleRangeRequest::builder()
        .spreadsheet_token("O21wsTInWht7sUtRj77cFwRXnme")
        .ranges("0ae03b!C4:D6")
        .build();
    let resp = client
        .sheets
        .v2
        .spreadsheet
        .reading_multi_ranges(req, None)
        .await;

    match resp {
        Ok(base_resp) => {
            println!("sheet reading_multi_ranges response: {:#?}", base_resp.data);
        }
        Err(err) => {
            println!("sheet reading_multi_ranges error: {:#?}", err);
        }
    }
}

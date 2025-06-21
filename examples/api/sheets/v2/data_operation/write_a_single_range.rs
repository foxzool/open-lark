use std::env;

use dotenvy::dotenv;
use serde_json::json;

use open_lark::{
    client::LarkClient, service::sheets::v2::data_operation::WriteDataToSingleRangeRequest,
};

/// 读取单个范围
#[tokio::main]
async fn main() {
    dotenv().expect(".env file not found");
    env_logger::init();
    let app_id = env::var("APP_ID").unwrap();
    let app_secret = env::var("APP_SECRET").unwrap();
    // 创建 Client
    let client = LarkClient::builder(&app_id, &app_secret).build();

    let req = WriteDataToSingleRangeRequest::builder()
        .spreadsheet_token("O21wsTInWht7sUtRj77cFwRXnme")
        .range("0ae03b!C4:D4")
        .values(json!([[111, 222]]))
        .build();
    let resp = client
        .sheets
        .v2
        .spreadsheet
        .write_data_to_single_range(req, None)
        .await;

    match resp {
        Ok(base_resp) => {
            println!("sheet write_a_single_range response: {:#?}", base_resp);
        }
        Err(err) => {
            println!("sheet write_a_single_range error: {:#?}", err);
        }
    }
}

use std::env;

use dotenvy::dotenv;
use serde_json::json;

use open_lark::{
    client::LarkClientBuilder, service::sheets::v2::data_operation::AppendDataRequest,
};

/// 追加数据
#[tokio::main]
async fn main() {
    dotenv().expect(".env file not found");
    env_logger::init();
    let app_id = env::var("APP_ID").unwrap();
    let app_secret = env::var("APP_SECRET").unwrap();
    // 创建 Client
    let client = LarkClientBuilder::new(&app_id, &app_secret).build();

    let req = AppendDataRequest::builder()
        .spreadsheet_token("O21wsTInWht7sUtRj77cFwRXnme")
        .range("0ae03b!B2:C2")
        .values(json!([[123, 456,]]))
        .build();
    let resp = client
        .sheets
        .v2
        .spreadsheet_sheet
        .append_data(req, None)
        .await
        .unwrap();

    if let Some(data) = resp.data {
        println!("sheet append data response: {:#?}", data);
    }
}

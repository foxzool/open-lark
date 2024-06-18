use std::env;

use dotenvy::dotenv;
use serde_json::json;

use open_lark::{
    client::LarkClientBuilder, service::sheets::v2::data_operation::PrependDataRequest,
};

/// 插入数据
#[tokio::main]
async fn main() {
    dotenv().expect(".env file not found");
    env_logger::init();
    let app_id = env::var("APP_ID").unwrap();
    let app_secret = env::var("APP_SECRET").unwrap();
    // 创建 Client
    let client = LarkClientBuilder::new(&app_id, &app_secret).build();

    let req = PrependDataRequest::builder()
        .spreadsheet_token("O21wsTInWht7sUtRj77cFwRXnme")
        .range("0ae03b!G2:H2")
        .values(json!([["2021-09-01", "2021-09-02",]]))
        .build();
    let resp = client
        .sheets
        .v2
        .spreadsheet_sheet
        .prepend_data(req, None)
        .await
        .unwrap();

    if let Some(data) = resp.data {
        println!("update spreadsheet sheet properties response: {:#?}", data);
    }
}

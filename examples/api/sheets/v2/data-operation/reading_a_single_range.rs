use std::env;

use dotenvy::dotenv;

use open_lark::{
    client::LarkClientBuilder, service::sheets::v2::data_operation::ReadingSingleRangeRequest,
};

/// 读取单个范围
#[tokio::main]
async fn main() {
    dotenv().expect(".env file not found");
    env_logger::init();
    let app_id = env::var("APP_ID").unwrap();
    let app_secret = env::var("APP_SECRET").unwrap();
    // 创建 Client
    let client = LarkClientBuilder::new(&app_id, &app_secret).build();

    let req = ReadingSingleRangeRequest::builder()
        .spreadsheet_token("O21wsTInWht7sUtRj77cFwRXnme")
        .range("0ae03b!C4:D6")
        .build();
    let resp = client
        .sheets
        .v2
        .spreadsheet
        .reading_a_single_range(req, None)
        .await;

    match resp {
        Ok(base_resp) => {
            println!(
                "sheet reading_a_single_range response: {:#?}",
                base_resp.data
            );
        }
        Err(err) => {
            println!("sheet reading_a_single_range error: {:#?}", err);
        }
    }
}

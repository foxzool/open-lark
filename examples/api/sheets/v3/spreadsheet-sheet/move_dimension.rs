use std::env;

use dotenvy::dotenv;

use open_lark::{
    client::LarkClientBuilder, service::sheets::v3::sheet_row_col::MoveDimensionRequest,
};

/// 移动行列
#[tokio::main]
async fn main() {
    dotenv().expect(".env file not found");
    env_logger::init();
    let app_id = env::var("APP_ID").unwrap();
    let app_secret = env::var("APP_SECRET").unwrap();
    // 创建 Client
    let client = LarkClientBuilder::new(&app_id, &app_secret).build();

    let req = MoveDimensionRequest::builder()
        .spreadsheet_token("O21wsTInWht7sUtRj77cFwRXnme")
        .sheet_id("0ae03b")
        .major_dimension("ROWS")
        .start_index(1)
        .end_index(2)
        .destination_index(4)
        .build();
    let resp = client
        .sheets
        .v3
        .spreadsheet_sheet
        .move_dimension(req, None)
        .await
        .unwrap();

    if let Some(data) = resp.data {
        println!("move dimension spreadsheet response: {:#?}", data);
    }
}

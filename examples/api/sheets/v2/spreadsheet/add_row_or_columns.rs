use std::env;

use dotenvy::dotenv;

use open_lark::{
    client::LarkClientBuilder, service::sheets::v2::sheet_row_col::AddDimensionRangeRequest,
};

/// 操作工作表
#[tokio::main]
async fn main() {
    dotenv().expect(".env file not found");
    env_logger::init();
    let app_id = env::var("APP_ID").unwrap();
    let app_secret = env::var("APP_SECRET").unwrap();
    // 创建 Client
    let client = LarkClientBuilder::new(&app_id, &app_secret).build();

    let req = AddDimensionRangeRequest::builder()
        .spreadsheet_token("O21wsTInWht7sUtRj77cFwRXnme")
        .sheet_id("0ae03b")
        .major_dimension("ROWS")
        .length(1)
        .build();
    let resp = client
        .sheets
        .v2
        .spreadsheet
        .add_dimension_range(req, None)
        .await
        .unwrap();

    if let Some(data) = resp.data {
        println!("add-rows-or-columns spreadsheet response: {:#?}", data);
    }
}

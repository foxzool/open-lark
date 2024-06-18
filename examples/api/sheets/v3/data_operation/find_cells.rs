use std::env;

use dotenvy::dotenv;

use open_lark::{
    client::LarkClientBuilder,
    service::sheets::v3::data_operation::FindCellsRequest,
};

/// 查找单元格
#[tokio::main]
async fn main() {
    dotenv().expect(".env file not found");
    env_logger::init();
    let app_id = env::var("APP_ID").unwrap();
    let app_secret = env::var("APP_SECRET").unwrap();
    // 创建 Client
    let client = LarkClientBuilder::new(&app_id, &app_secret).build();

    let req = FindCellsRequest::builder()
        .spreadsheet_token("O21wsTInWht7sUtRj77cFwRXnme")
        .range("0ae03b!C1:D2")
        .sheet_id("0ae03b")
        .find("Props")
        .match_case(false)
        .build();

    let resp = client
        .sheets
        .v3
        .spreadsheet_sheet
        .find_cells(req, None)
        .await;

    match resp {
        Ok(base_resp) => {
            println!("sheet find_cells response: {:#?}", base_resp.data);
        }
        Err(err) => {
            println!("sheet find_cells error: {:#?}", err);
        }
    }
}

use std::env;

use dotenvy::dotenv;

use open_lark::{prelude::LarkClient, service::sheets::v2::data_operation::MergeCellsRequest};

/// 合并单元格
#[tokio::main]
async fn main() {
    dotenv().expect(".env file not found");
    env_logger::init();
    let app_id = env::var("APP_ID").unwrap();
    let app_secret = env::var("APP_SECRET").unwrap();
    // 创建 Client
    let client = LarkClient::builder(&app_id, &app_secret).build();

    let req = MergeCellsRequest::builder()
        .spreadsheet_token("O21wsTInWht7sUtRj77cFwRXnme")
        .range("0ae03b!G2:G3")
        .merge_type("MERGE_COLUMNS")
        .build();
    let resp = client.sheets.v2.spreadsheet.merge_cells(req, None).await;

    match resp {
        Ok(base_resp) => {
            println!("sheet merge_cells response: {:#?}", base_resp.data);
        }
        Err(err) => {
            println!("sheet merge_cells error: {:#?}", err);
        }
    }
}

use std::env;

use dotenvy::dotenv;

use open_lark::{client::LarkClient, service::sheets::v2::data_operation::SplitCellsRequest};

/// 拆分单元格
#[tokio::main]
async fn main() {
    dotenv().expect(".env file not found");
    env_logger::init();
    let app_id = env::var("APP_ID").unwrap();
    let app_secret = env::var("APP_SECRET").unwrap();
    // 创建 Client
    let client = LarkClient::builder(&app_id, &app_secret).build();

    let req = SplitCellsRequest::builder()
        .spreadsheet_token("O21wsTInWht7sUtRj77cFwRXnme")
        .range("0ae03b!G2:G3")
        .build();
    let resp = client.sheets.v2.spreadsheet.split_cells(req, None).await;

    match resp {
        Ok(base_resp) => {
            println!("sheet split_cells response: {:#?}", base_resp.data);
        }
        Err(err) => {
            println!("sheet split_cells error: {:#?}", err);
        }
    }
}

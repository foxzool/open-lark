use std::env;

use dotenvy::dotenv;

use open_lark::{
    client::LarkClientBuilder,
    service::sheets::v2::data_operation::{
        SetCellStyleRequest, StyleFont,
    },
};

/// 设置单元格样式
#[tokio::main]
async fn main() {
    dotenv().expect(".env file not found");
    env_logger::init();
    let app_id = env::var("APP_ID").unwrap();
    let app_secret = env::var("APP_SECRET").unwrap();
    // 创建 Client
    let client = LarkClientBuilder::new(&app_id, &app_secret).build();

    let req = SetCellStyleRequest::builder()
        .spreadsheet_token("O21wsTInWht7sUtRj77cFwRXnme")
        .range("0ae03b!C4:D4")
        .font(StyleFont::builder().bold(true).build())
        .build();
    let resp = client
        .sheets
        .v2
        .spreadsheet
        .set_cell_style(req, None)
        .await;

    match resp {
        Ok(base_resp) => {
            println!("sheet set_cell_style response: {:#?}", base_resp.data);
        }
        Err(err) => {
            println!("sheet set_cell_style error: {:#?}", err);
        }
    }
}

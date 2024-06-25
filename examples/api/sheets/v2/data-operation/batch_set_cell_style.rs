use std::env;

use dotenvy::dotenv;

use open_lark::{
    client::LarkClient,
    service::sheets::v2::data_operation::{BatchSetCellStyleRequest, CellStyle, StyleFont},
};

/// 批量设置单元格样式
#[tokio::main]
async fn main() {
    dotenv().expect(".env file not found");
    env_logger::init();
    let app_id = env::var("APP_ID").unwrap();
    let app_secret = env::var("APP_SECRET").unwrap();
    // 创建 Client
    let client = LarkClient::builder(&app_id, &app_secret).build();

    let req = BatchSetCellStyleRequest::builder()
        .spreadsheet_token("O21wsTInWht7sUtRj77cFwRXnme")
        .add_style(
            "0ae03b!C4:D4",
            CellStyle::builder()
                .font(StyleFont::builder().bold(false).build())
                .build(),
        )
        .build();
    let resp = client
        .sheets
        .v2
        .spreadsheet
        .batch_set_cell_style(req, None)
        .await;

    match resp {
        Ok(base_resp) => {
            println!("sheet batch_set_cell_style response: {:#?}", base_resp.data);
        }
        Err(err) => {
            println!("sheet batch_set_cell_style error: {:#?}", err);
        }
    }
}

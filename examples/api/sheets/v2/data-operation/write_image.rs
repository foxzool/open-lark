use std::env;

use dotenvy::dotenv;

use open_lark::{client::LarkClient, service::sheets::v2::data_operation::WriteImageRequest};

/// 写入图片
#[tokio::main]
async fn main() {
    dotenv().expect(".env file not found");
    env_logger::init();
    let app_id = env::var("APP_ID").unwrap();
    let app_secret = env::var("APP_SECRET").unwrap();
    // 创建 Client
    let client = LarkClient::builder(&app_id, &app_secret).build();

    let req = WriteImageRequest::builder()
        .spreadsheet_token("O21wsTInWht7sUtRj77cFwRXnme")
        .range("0ae03b!C2:D2")
        .image(vec![
            137, 80, 78, 71, 13, 10, 26, 10, 0, 0, 0, 13, 73, 72, 68, 82, 0, 0, 0, 2, 0, 0, 0, 1,
            1, 0, 0, 0, 0, 220, 89, 66, 39, 0, 0, 0, 2, 116, 82, 78, 83, 0, 0, 118, 147, 205, 56,
            0, 0, 0, 10, 73, 68, 65, 84, 8, 215, 99, 104, 0, 0, 0, 130, 0, 129, 221, 67, 106, 244,
            0, 0, 0, 0, 73, 69, 78, 68, 174, 66, 96, 130,
        ])
        .name("test.png")
        .build();

    let resp = client.sheets.v2.spreadsheet.write_image(req, None).await;

    match resp {
        Ok(base_resp) => {
            println!("sheet write_multi_ranges response: {:#?}", base_resp.data);
        }
        Err(err) => {
            println!("sheet write_multi_ranges error: {:#?}", err);
        }
    }
}

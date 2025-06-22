use std::env;

use dotenvy::dotenv;

use open_lark::{prelude::LarkClient, service::sheets::v3::spreadsheet::PatchSpreadSheetRequest};

/// 修改电子表格属性
#[tokio::main]
async fn main() {
    dotenv().expect(".env file not found");
    env_logger::init();
    let app_id = env::var("APP_ID").unwrap();
    let app_secret = env::var("APP_SECRET").unwrap();
    // 创建 Client
    let client = LarkClient::builder(&app_id, &app_secret).build();
    // 构建请求体
    let req = PatchSpreadSheetRequest::builder()
        .spreadsheet_token("MSK5sCjxdhprI3toTNDcN61rnbf")
        .title("title_edited")
        .build();
    // 发起请求
    let resp = client.sheets.v3.spreadsheet.patch(req, None).await.unwrap();
    if let Some(data) = resp.data {
        println!("patch spread response: {:#?}", data);
    }
}

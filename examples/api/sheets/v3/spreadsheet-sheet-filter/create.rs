use std::env;

use dotenvy::dotenv;

use open_lark::{
    client::LarkClientBuilder,
    service::sheets::v3::spreadsheet_sheet_filter::{
        CreateSheetFilterRequest, SheetFilterCondition,
    },
};

/// 创建筛选
#[tokio::main]
async fn main() {
    dotenv().expect(".env file not found");
    env_logger::init();
    let app_id = env::var("APP_ID").unwrap();
    let app_secret = env::var("APP_SECRET").unwrap();
    // 创建 Client
    let client = LarkClientBuilder::new(&app_id, &app_secret).build();
    // 构建请求体
    let req = CreateSheetFilterRequest::builder()
        .spreadsheet_token("O21wsTInWht7sUtRj77cFwRXnme")
        .sheet_id("0ae03b")
        .range("0ae03b")
        .col("D")
        .condition(
            SheetFilterCondition::builder()
                .filter_type("number")
                .compare_type("equal")
                .expected(vec!["444".to_string()])
                .build(),
        )
        .build();
    // 发起请求
    let resp = client
        .sheets
        .v3
        .spreadsheet_sheet_filter
        .create(req, None)
        .await
        .unwrap();
    if let Some(data) = resp.data {
        println!("create spreadsheet_sheet_filter response: {:#?}", data);
    }
}

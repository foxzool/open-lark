use std::env;

use dotenvy::dotenv;

use open_lark::{
    client::LarkClient,
    service::sheets::v3::spreadsheet_sheet_filter::{
        SheetFilterCondition, UpdateSheetFilterRequest,
    },
};

/// 更新筛选
#[tokio::main]
async fn main() {
    dotenv().expect(".env file not found");
    env_logger::init();
    let app_id = env::var("APP_ID").unwrap();
    let app_secret = env::var("APP_SECRET").unwrap();
    // 创建 Client
    let client = LarkClient::builder(&app_id, &app_secret).build();
    // 构建请求体
    let req = UpdateSheetFilterRequest::builder()
        .spreadsheet_token("O21wsTInWht7sUtRj77cFwRXnme")
        .sheet_id("0ae03b")
        .col("D")
        .condition(
            SheetFilterCondition::builder()
                .filter_type("number")
                .compare_type("greaterOrEqual")
                .expected(vec!["333".to_string()])
                .build(),
        )
        .build();
    // 发起请求
    let resp = client
        .sheets
        .v3
        .spreadsheet_sheet_filter
        .update(req, None)
        .await;

    match resp {
        Ok(base_resp) => {
            println!(
                "update spreadsheet_sheet_filter response: {:#?}",
                base_resp.data
            );
        }
        Err(err) => {
            println!("update spreadsheet_sheet_filter  error: {:#?}", err);
        }
    }
}

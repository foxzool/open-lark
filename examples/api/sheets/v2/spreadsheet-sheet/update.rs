use std::env;

use dotenvy::dotenv;

use open_lark::{
    client::LarkClientBuilder,
    service::sheets::{
        v2::spreadsheet_sheet::{UpdateSheetPropertiesRequest, UpdateSheetProperty},
        v3::spreadsheet_sheet::QuerySpreadsheetSheetRequest,
    },
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

    // 查询原测试工作表的sheet id
    let req = QuerySpreadsheetSheetRequest::builder()
        .spreadsheet_token("O21wsTInWht7sUtRj77cFwRXnme")
        .build();
    let resp = client
        .sheets
        .v3
        .spreadsheet_sheet
        .query(req, None)
        .await
        .unwrap();

    let mut update_builder =
        UpdateSheetPropertiesRequest::builder().spreadsheet_token("O21wsTInWht7sUtRj77cFwRXnme");
    if let Some(data) = resp.data {
        for sheet in data.sheets {
            if sheet.title == "Sheet1" {
                update_builder = update_builder.add_request(UpdateSheetProperty {
                    sheet_id: sheet.sheet_id,
                    title: "Sheet1 edit".to_string(),

                    hidden: Some(true),
                    ..Default::default()
                })
            }
        }
    }

    let req = update_builder.build();

    // 发起请求
    let resp = client
        .sheets
        .v2
        .spreadsheet_sheet
        .update_sheet_properties(
            req,
            None, /* Some(
                  * open_lark::core::req_option::RequestOption::builder()
                  * .user_access_token(env::var("USER_ACCESS_TOKEN").unwrap())
                  * .build(),
                  * ), */
        )
        .await
        .unwrap();
    if let Some(data) = resp.data {
        println!("update spreadsheet sheet properties response: {:#?}", data);
    }
}

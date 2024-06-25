use std::env;

use dotenvy::dotenv;

use open_lark::{
    client::LarkClient,
    service::sheets::{
        v2::spreadsheet_sheet::OperateSheetsRequest,
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
    let client = LarkClient::builder(&app_id, &app_secret).build();

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

    let mut operate_builder =
        OperateSheetsRequest::builder().spreadsheet_token("O21wsTInWht7sUtRj77cFwRXnme");
    if let Some(data) = resp.data {
        for sheet in data.sheets {
            if sheet.title.contains("test") {
                operate_builder = operate_builder.delete_sheet(&sheet.sheet_id);
            }
            if sheet.title == "Sheet1" {
                operate_builder = operate_builder
                    .add_sheet(
                        format!("test for add sheet {}", chrono::Local::now().timestamp()),
                        None,
                    )
                    .copy_sheet(
                        &sheet.sheet_id,
                        Some(format!(
                            "test for copy sheet {}",
                            chrono::Local::now().timestamp()
                        )),
                    )
            }
        }
    }

    let req = operate_builder.build();

    // 发起请求
    let resp = client
        .sheets
        .v2
        .spreadsheet_sheet
        .operate(
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
        println!("operate spreadsheet sheet response: {:#?}", data);
    }
}

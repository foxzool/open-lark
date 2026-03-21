//! 文档 helper 示例
//!
//! 演示 `openlark` 根 crate 在 `rc.2` 中提供的通用 docs helper：
//! - 文件夹自动分页遍历
//! - 按标题查找工作表
//! - 读取多个单元格范围
//! - 多维表格全量读取
//!
//! 运行方式：
//! ```bash
//! export OPENLARK_APP_ID="your_app_id"
//! export OPENLARK_APP_SECRET="your_app_secret"
//! cargo run --example docs_helpers --features "auth,docs-bitable"
//! ```

use open_lark::prelude::*;

#[tokio::main]
async fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();

    let client = Client::from_env()?;

    println!("docs helper 示例");
    println!("当前 app_id: {}", client.config().app_id);

    if let Ok(folder_token) = std::env::var("OPENLARK_FOLDER_TOKEN") {
        let items = client
            .docs
            .list_folder_children_all(&folder_token, None)
            .await?;
        println!("文件夹子项数量: {}", items.len());
    } else {
        println!("未设置 OPENLARK_FOLDER_TOKEN，跳过文件夹遍历示例");
    }

    if let (Ok(spreadsheet_token), Ok(sheet_title)) = (
        std::env::var("OPENLARK_SPREADSHEET_TOKEN"),
        std::env::var("OPENLARK_SHEET_TITLE"),
    ) {
        let sheet = client
            .docs
            .find_sheet_by_title(&spreadsheet_token, &sheet_title)
            .await?;
        let data = client
            .docs
            .read_multiple_ranges(
                &spreadsheet_token,
                vec![format!("{}!A1:C5", sheet.sheet_id)],
            )
            .await?;
        println!("工作表标题: {}", sheet.title);
        println!("读取范围数量: {}", data.value_ranges.len());
    } else {
        println!("未设置 OPENLARK_SPREADSHEET_TOKEN / OPENLARK_SHEET_TITLE，跳过表格示例");
    }

    if let (Ok(app_token), Ok(table_id)) = (
        std::env::var("OPENLARK_BITABLE_APP_TOKEN"),
        std::env::var("OPENLARK_BITABLE_TABLE_ID"),
    ) {
        let records = client
            .docs
            .search_bitable_records_all(&app_token, &table_id)
            .await?;
        println!("多维表格记录数: {}", records.len());
    } else {
        println!("未设置 OPENLARK_BITABLE_APP_TOKEN / OPENLARK_BITABLE_TABLE_ID，跳过多维表格示例");
    }

    Ok(())
}

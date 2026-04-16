//! 文档 helper 示例
//!
//! 演示 `openlark` 根 crate 在 `0.15.0` 中提供的通用 docs helper：
//! - 文件夹自动分页遍历
//! - 文件夹 typed pagination helper
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
        let mut pager = client
            .docs
            .folder_children_pager(&folder_token)
            .page_size(50);
        let first_page = pager.fetch_next_page().await?;
        println!("第一页子项数量: {}", first_page.items.len());
        println!("第一页是否还有更多: {}", first_page.has_more);
        if let Some(next_page_token) = &first_page.next_page_token {
            println!("下一页 token: {}", next_page_token);
        }

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
        let range = open_lark::docs::SheetRange::from_range_expr(sheet.sheet_id.clone(), "A1:C5")?;
        let data = client
            .docs
            .read_sheet_ranges(&spreadsheet_token, vec![range.clone()])
            .await?;
        println!("工作表标题: {}", sheet.title);
        println!("读取范围数量: {}", data.value_ranges.len());

        if std::env::var("OPENLARK_SHEETS_WRITE_DEMO").ok().as_deref() == Some("1") {
            let write = open_lark::docs::SheetWriteRange::new(
                range.clone(),
                vec![vec![serde_json::json!("demo"), serde_json::json!(2026)]],
            );
            let result = client
                .docs
                .write_sheet_ranges(&spreadsheet_token, vec![write])
                .await?;
            println!("批量写入更新单元格数: {}", result.total_updated_cells);
        }
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

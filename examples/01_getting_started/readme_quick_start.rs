//! 根 README 对齐示例。
//!
//! 这个示例专门用于覆盖 README 中主推的 `openlark` 快速开始路径，
//! 让文档中的 helper 调用能够通过 CI 编译校验。

use open_lark::prelude::*;

#[tokio::main]
async fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    let client = Client::builder()
        .app_id("your_app_id")
        .app_secret("your_app_secret")
        .build()?;

    let folder_items = client
        .docs
        .list_folder_children_all("folder_token", Some("sheet"))
        .await?;
    println!("文件夹子项数量: {}", folder_items.len());

    let sheet = client
        .docs
        .find_sheet_by_title("spreadsheet_token", "汇总表")
        .await?;
    let ranges = client
        .docs
        .read_multiple_ranges(
            "spreadsheet_token",
            vec![format!("{}!A1:C10", sheet.sheet_id)],
        )
        .await?;
    println!("读取范围数量: {}", ranges.value_ranges.len());

    let records = client
        .docs
        .search_bitable_records_all("app_token", "table_id")
        .await?;
    println!("记录数量: {}", records.len());

    Ok(())
}

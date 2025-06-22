use dotenvy::dotenv;
use open_lark::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    
    let app_id = std::env::var("APP_ID").expect("APP_ID not found");
    let app_secret = std::env::var("APP_SECRET").expect("APP_SECRET not found");

    let client = LarkClient::builder(app_id, app_secret)
        .with_enable_token_cache(true)
        .build();

    // 合并单元格示例
    let req = open_lark::service::sheets::v3::data_operation::MergeCellsRequest::builder()
        .spreadsheet_token("shtcnmBA*****yGehy8") // 替换为实际的表格 token
        .sheet_id("Sheet1") // 替换为实际的工作表 ID
        .range("A1:C3") // 合并 A1 到 C3 范围的单元格
        .merge_type("MERGE_ALL") // 合并所有单元格
        .build();

    match client.sheets.v3.data_operation.merge_cells(req, None).await {
        Ok(resp) => {
            println!("✅ 合并单元格成功!");
            println!("📊 合并后的范围: {}", resp.data.merged_range);
        }
        Err(e) => {
            eprintln!("❌ 合并单元格失败: {:?}", e);
        }
    }

    Ok(())
}
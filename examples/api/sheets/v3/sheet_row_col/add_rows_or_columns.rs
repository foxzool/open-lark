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

    // 增加行列示例
    let req = open_lark::service::sheets::v3::sheet_row_col::AddRowsOrColumnsRequest::builder()
        .spreadsheet_token("shtcnmBA*****yGehy8") // 替换为实际的表格 token
        .sheet_id("Sheet1") // 替换为实际的工作表 ID
        .dimension("ROWS") // 增加行
        .length(5) // 增加 5 行
        .build();

    match client.sheets.v3.sheet_row_col.add_rows_or_columns(req, None).await {
        Ok(resp) => {
            println!("✅ 增加行列成功!");
            println!("📊 维度: {}", resp.data.add_range.dimension);
            println!("📍 起始位置: {}", resp.data.add_range.start_index);
            println!("📍 结束位置: {}", resp.data.add_range.end_index);
        }
        Err(e) => {
            eprintln!("❌ 增加行列失败: {:?}", e);
        }
    }

    Ok(())
}
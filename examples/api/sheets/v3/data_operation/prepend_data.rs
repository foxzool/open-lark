use dotenv::dotenv;
use open_lark::prelude::*;
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    
    let app_id = std::env::var("APP_ID").expect("APP_ID not found");
    let app_secret = std::env::var("APP_SECRET").expect("APP_SECRET not found");

    let client = LarkClient::builder(app_id, app_secret)
        .with_app_type(AppType::SelfBuilt)
        .with_enable_token_cache(true)
        .build();

    // 准备要插入的数据（在开头插入）
    let data = vec![
        vec![json!("类型"), json!("姓名"), json!("年龄"), json!("部门")],
        vec![json!("实习生"), json!("王五"), json!(22), json!("设计部")],
    ];

    // 插入数据示例
    let req = open_lark::service::sheets::v3::data_operation::PrependDataRequest::builder()
        .spreadsheet_token("shtcnmBA*****yGehy8") // 替换为实际的表格 token
        .range("Sheet1!A:D") // 在 A 到 D 列范围内插入数据
        .insert_data_option("INSERT_ROWS") // 插入行的方式
        .values(data)
        .build();

    match client.sheets.v3.data_operation.prepend_data(req, None).await {
        Ok(resp) => {
            println!("✅ 插入数据成功!");
            println!("📊 表格 Token: {}", resp.data.spreadsheet_token);
            println!("📍 更新范围: {}", resp.data.table_range);
            println!("🔄 版本号: {}", resp.data.revision);
            println!("📝 更新信息:");
            println!("  - 更新范围: {}", resp.data.updates.updated_range);
            println!("  - 更新行数: {}", resp.data.updates.updated_rows);
            println!("  - 更新列数: {}", resp.data.updates.updated_columns);
            println!("  - 更新单元格数: {}", resp.data.updates.updated_cells);
        }
        Err(e) => {
            eprintln!("❌ 插入数据失败: {:?}", e);
        }
    }

    Ok(())
}
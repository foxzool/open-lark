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

    // 准备要追加的数据
    let data = vec![
        vec![json!("新员工"), json!("张三"), json!(25), json!("研发部")],
        vec![json!("新员工"), json!("李四"), json!(28), json!("产品部")],
    ];

    // 追加数据示例
    let req = open_lark::service::sheets::v3::data_operation::AppendDataRequest::builder()
        .spreadsheet_token("shtcnmBA*****yGehy8") // 替换为实际的表格 token
        .range("Sheet1!A:D") // 在 A 到 D 列范围内追加数据
        .insert_data_option("INSERT_ROWS") // 插入行的方式
        .values(data)
        .build();

    match client.sheets.v3.data_operation.append_data(req, None).await {
        Ok(resp) => {
            println!("✅ 追加数据成功!");
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
            eprintln!("❌ 追加数据失败: {:?}", e);
        }
    }

    Ok(())
}
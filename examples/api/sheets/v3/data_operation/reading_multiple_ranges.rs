use dotenvy::dotenv;
use open_lark::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    
    let app_id = std::env::var("APP_ID").expect("APP_ID not found");
    let app_secret = std::env::var("APP_SECRET").expect("APP_SECRET not found");

    let client = LarkClient::builder(&app_id, &app_secret)
        .with_enable_token_cache(true)
        .build();

    // 读取多个范围示例
    let ranges = vec![
        "Sheet1!A1:B5".to_string(),   // 读取第一个范围
        "Sheet1!D1:E5".to_string(),   // 读取第二个范围
        "Sheet2!A1:C3".to_string(),   // 读取第三个范围
    ];

    let req = open_lark::service::sheets::v3::data_operation::ReadingMultipleRangesRequest::builder()
        .spreadsheet_token("shtcnmBA*****yGehy8") // 替换为实际的表格 token
        .ranges(ranges)
        .value_render_option("FormattedValue") // 返回格式化的值
        .date_time_render_option("FormattedString") // 日期时间格式化为字符串
        .user_id_type("open_id") // 用户 ID 类型
        .build();

    match client.sheets.v3.data_operation.reading_multiple_ranges(req, None).await {
        Ok(resp) => {
            println!("✅ 读取多个范围成功!");
            println!("📊 总共读取了 {} 个范围", resp.data.value_ranges.len());
            
            // 打印每个范围的数据
            for (i, value_range) in resp.data.value_ranges.iter().enumerate() {
                println!("\n📋 范围 {}: {}", i + 1, value_range.range);
                println!("🔄 版本号: {}", value_range.revision);
                println!("📝 数据行数: {}", value_range.values.len());
                
                // 打印数据内容（限制显示前几行）
                for (row_idx, row) in value_range.values.iter().take(3).enumerate() {
                    println!("  第{}行: {:?}", row_idx + 1, row);
                }
                
                if value_range.values.len() > 3 {
                    println!("  ... 还有 {} 行数据", value_range.values.len() - 3);
                }
            }
        }
        Err(e) => {
            eprintln!("❌ 读取多个范围失败: {:?}", e);
        }
    }

    Ok(())
}
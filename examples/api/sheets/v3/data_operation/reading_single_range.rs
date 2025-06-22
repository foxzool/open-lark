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

    // 读取单个范围示例
    let req = open_lark::service::sheets::v3::data_operation::ReadingSingleRangeRequest::builder()
        .spreadsheet_token("shtcnmBA*****yGehy8") // 替换为实际的表格 token
        .range("Sheet1!A1:B10") // 读取 A1 到 B10 范围的数据
        .value_render_option("FormattedValue") // 返回格式化的值
        .date_time_render_option("FormattedString") // 日期时间格式化为字符串
        .user_id_type("open_id") // 用户 ID 类型
        .build();

    match client.sheets.v3.data_operation.reading_single_range(req, None).await {
        Ok(resp) => {
            println!("✅ 读取单个范围成功!");
            println!("📊 范围: {}", resp.data.value_range.range);
            println!("🔄 版本号: {}", resp.data.value_range.revision);
            println!("📋 数据行数: {}", resp.data.value_range.values.len());
            
            // 打印数据内容
            for (i, row) in resp.data.value_range.values.iter().enumerate() {
                println!("第{}行: {:?}", i + 1, row);
            }
        }
        Err(e) => {
            eprintln!("❌ 读取单个范围失败: {:?}", e);
        }
    }

    Ok(())
}
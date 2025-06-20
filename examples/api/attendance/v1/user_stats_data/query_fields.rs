#![allow(clippy::field_reassign_with_default)]

use dotenvy::dotenv;
use open_lark::{prelude::LarkClient, service::attendance::v1::models::QueryStatsFieldsRequest};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let app_id = env::var("APP_ID").expect("APP_ID must be set");
    let app_secret = env::var("APP_SECRET").expect("APP_SECRET must be set");

    let client = LarkClient::builder(&app_id, &app_secret).build();

    // 构建查询统计表头请求
    let mut req = QueryStatsFieldsRequest::default();
    req.employee_type = "employee_id".to_string();
    req.locale = Some("zh-CN".to_string()); // 中文

    println!("发送查询统计表头请求...");
    println!("语言: 中文 (zh-CN)");

    match client
        .attendance
        .v1
        .user_stats_data
        .query_fields(req, None)
        .await
    {
        Ok(resp) => {
            println!("✅ 查询统计表头成功!");
            if let Some(data) = resp.data {
                println!("📊 可用统计字段 (共{}个):", data.fields.len());

                for (index, field) in data.fields.iter().enumerate() {
                    println!("  {}. 📈 字段信息:", index + 1);
                    println!("     字段标识: {}", field.field_key);
                    println!("     字段名称: {}", field.field_name);

                    if let Some(zh_name) = &field.field_name_zh {
                        println!("     中文名称: {}", zh_name);
                    }
                    if let Some(en_name) = &field.field_name_en {
                        println!("     英文名称: {}", en_name);
                    }
                    if let Some(ja_name) = &field.field_name_ja {
                        println!("     日文名称: {}", ja_name);
                    }

                    let field_type_name = match field.field_type {
                        0 => "文本",
                        1 => "数字",
                        2 => "时间",
                        _ => "未知",
                    };
                    println!("     字段类型: {} ({})", field.field_type, field_type_name);
                    println!("     ---");
                }

                println!("💡 提示: 你可以使用这些字段标识在统计设置中指定需要查询的字段");
            }
        }
        Err(e) => {
            eprintln!("❌ 查询统计表头失败: {:?}", e);
            eprintln!("💡 提示: 请检查员工ID类型和语言设置是否正确");
        }
    }

    Ok(())
}

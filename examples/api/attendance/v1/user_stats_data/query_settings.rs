#![allow(clippy::field_reassign_with_default)]

use dotenvy::dotenv;
use open_lark::{prelude::LarkClient, service::attendance::v1::models::QueryStatsSettingsRequest};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let app_id = env::var("APP_ID").expect("APP_ID must be set");
    let app_secret = env::var("APP_SECRET").expect("APP_SECRET must be set");

    let client = LarkClient::builder(&app_id, &app_secret).build();

    // 构建查询统计设置请求
    let mut req = QueryStatsSettingsRequest::default();
    req.employee_type = "employee_id".to_string();

    println!("发送查询统计设置请求...");

    match client
        .attendance
        .v1
        .user_stats_data
        .query_settings(req, None)
        .await
    {
        Ok(resp) => {
            println!("✅ 查询统计设置成功!");
            if let Some(data) = resp.data {
                let setting = &data.stats_setting;
                println!("📊 当前统计设置:");
                println!(
                    "  统计范围: {}",
                    match setting.stats_scope {
                        1 => "自定义范围",
                        2 => "全部",
                        _ => "未知",
                    }
                );
                println!("  统计起始日期: {}", setting.start_date);
                println!("  统计结束日期: {}", setting.end_date);
                println!("  用户数量: {}", setting.user_ids.len());

                if !setting.user_ids.is_empty() {
                    println!("  用户ID列表: {:?}", setting.user_ids);
                }

                println!("  统计字段数量: {}", setting.need_fields.len());
                if !setting.need_fields.is_empty() {
                    println!("  统计字段:");
                    for (index, field) in setting.need_fields.iter().enumerate() {
                        println!("    {}. {}", index + 1, field);
                    }
                }
            }
        }
        Err(e) => {
            eprintln!("❌ 查询统计设置失败: {:?}", e);
            eprintln!("💡 提示: 请确保有权限查询统计设置，或者先设置统计配置");
        }
    }

    Ok(())
}

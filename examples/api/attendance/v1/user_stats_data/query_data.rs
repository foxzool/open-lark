#![allow(clippy::field_reassign_with_default)]

use dotenvy::dotenv;
use open_lark::{prelude::LarkClient, service::attendance::v1::models::QueryUserStatsDataRequest};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let app_id = env::var("APP_ID").expect("APP_ID must be set");
    let app_secret = env::var("APP_SECRET").expect("APP_SECRET must be set");

    let client = LarkClient::builder(&app_id, &app_secret).build();

    // 构建查询统计数据请求
    let mut req = QueryUserStatsDataRequest::default();
    req.employee_type = "employee_id".to_string();
    req.start_date = "2024-06-01".to_string();
    req.end_date = "2024-06-30".to_string();
    req.user_ids = vec![
        "employee_123".to_string(),
        "employee_456".to_string(),
        "employee_789".to_string(),
    ];
    req.need_fields = vec![
        "actual_work_day".to_string(),   // 实际工作天数
        "normal_work_day".to_string(),   // 正常工作天数
        "late_count".to_string(),        // 迟到次数
        "early_leave_count".to_string(), // 早退次数
        "absence_count".to_string(),     // 缺勤次数
        "overtime_duration".to_string(), // 加班时长
    ];
    req.locale = Some("zh-CN".to_string()); // 中文

    println!("发送查询统计数据请求...");
    println!("统计时间: {} - {}", req.start_date, req.end_date);
    println!("用户数量: {}", req.user_ids.len());
    println!("统计字段: {:?}", req.need_fields);

    match client
        .attendance
        .v1
        .user_stats_data
        .query_data(req, None)
        .await
    {
        Ok(resp) => {
            println!("✅ 查询统计数据成功!");
            if let Some(data) = resp.data {
                println!("📊 用户统计数据 (共{}个用户):", data.datas.len());

                for (index, user_data) in data.datas.iter().enumerate() {
                    println!("  {}. 👤 用户统计:", index + 1);
                    println!("     用户ID: {}", user_data.user_id);

                    if let Some(user_name) = &user_data.user_name {
                        println!("     用户姓名: {}", user_name);
                    }

                    println!("     📈 统计数据:");
                    for (field_key, field_value) in &user_data.datas {
                        println!("       {}: {}", field_key, field_value);
                    }
                    println!("     ---");
                }

                // 汇总统计
                if !data.datas.is_empty() {
                    println!("📈 数据汇总:");
                    println!("  总用户数: {}", data.datas.len());
                    println!(
                        "  统计字段数: {}",
                        data.datas.first().map(|d| d.datas.len()).unwrap_or(0)
                    );
                    println!("  💡 提示: 可以使用这些数据进行考勤分析和报表生成");
                }
            }
        }
        Err(e) => {
            eprintln!("❌ 查询统计数据失败: {:?}", e);
            eprintln!("💡 提示: 请检查用户ID、日期范围和统计字段是否正确");
        }
    }

    Ok(())
}

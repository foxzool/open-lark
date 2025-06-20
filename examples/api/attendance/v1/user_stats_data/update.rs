#![allow(clippy::field_reassign_with_default)]

use dotenvy::dotenv;
use open_lark::{
    prelude::LarkClient,
    service::attendance::v1::models::{StatsSettings, UpdateUserStatsDataRequest},
};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let app_id = env::var("APP_ID").expect("APP_ID must be set");
    let app_secret = env::var("APP_SECRET").expect("APP_SECRET must be set");

    let client = LarkClient::builder(&app_id, &app_secret).build();

    // 构建统计设置
    let stats_setting = StatsSettings {
        stats_scope: 1, // 自定义范围
        start_date: "2024-06-01".to_string(),
        end_date: "2024-06-30".to_string(),
        user_ids: vec![
            "employee_123".to_string(),
            "employee_456".to_string(),
            "employee_789".to_string(),
        ],
        need_fields: vec![
            "actual_work_day".to_string(),   // 实际工作天数
            "normal_work_day".to_string(),   // 正常工作天数
            "late_count".to_string(),        // 迟到次数
            "early_leave_count".to_string(), // 早退次数
            "absence_count".to_string(),     // 缺勤次数
            "overtime_duration".to_string(), // 加班时长
        ],
    };

    // 构建更新统计设置请求
    let mut req = UpdateUserStatsDataRequest::default();
    req.employee_type = "employee_id".to_string();
    req.stats_setting = stats_setting;

    println!("发送更新统计设置请求...");
    println!("统计范围: 自定义范围");
    println!(
        "统计时间: {} - {}",
        req.stats_setting.start_date, req.stats_setting.end_date
    );
    println!("用户数量: {}", req.stats_setting.user_ids.len());
    println!("统计字段数量: {}", req.stats_setting.need_fields.len());

    match client.attendance.v1.user_stats_data.update(req, None).await {
        Ok(resp) => {
            println!("✅ 更新统计设置成功!");
            if let Some(data) = resp.data {
                println!("📊 更新结果:");
                println!("  更新状态: {}", if data.success { "成功" } else { "失败" });
                println!("  💡 提示: 统计设置已更新，可以使用查询接口获取统计数据");
            }
        }
        Err(e) => {
            eprintln!("❌ 更新统计设置失败: {:?}", e);
            eprintln!("💡 提示: 请检查员工ID是否正确，统计日期范围是否合理");
        }
    }

    Ok(())
}

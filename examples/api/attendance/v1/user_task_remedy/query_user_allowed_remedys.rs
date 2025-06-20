#![allow(clippy::field_reassign_with_default)]

use dotenvy::dotenv;
use open_lark::{
    prelude::LarkClient, service::attendance::v1::models::QueryUserAllowedRemedysRequest,
};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let app_id = env::var("APP_ID").expect("APP_ID must be set");
    let app_secret = env::var("APP_SECRET").expect("APP_SECRET must be set");

    let client = LarkClient::builder(&app_id, &app_secret).build();

    // 构建获取可补卡时间请求
    let mut req = QueryUserAllowedRemedysRequest::default();
    req.employee_type = "employee_id".to_string();
    req.user_id = "employee_123".to_string();
    req.date_from = Some("2024-06-01".to_string());
    req.date_to = Some("2024-06-30".to_string());

    println!("发送获取可补卡时间请求...");
    println!("查询用户: {}", req.user_id);
    println!(
        "查询时间: {} - {}",
        req.date_from.as_ref().unwrap(),
        req.date_to.as_ref().unwrap()
    );

    match client
        .attendance
        .v1
        .user_task_remedy
        .query_user_allowed_remedys(req, None)
        .await
    {
        Ok(resp) => {
            println!("✅ 获取可补卡时间成功!");
            if let Some(data) = resp.data {
                println!(
                    "📋 可补卡时间 (共{}天有补卡时间):",
                    data.allowed_remedys.len()
                );

                for (index, allowed_remedy) in data.allowed_remedys.iter().enumerate() {
                    println!("  {}. 📅 日期: {}", index + 1, allowed_remedy.date);
                    println!(
                        "     班次: {} ({})",
                        allowed_remedy.shift_id, allowed_remedy.shift_name
                    );

                    if allowed_remedy.remedy_periods.is_empty() {
                        println!("     ⚠️  该日期无可补卡时间段");
                    } else {
                        println!(
                            "     🕐 可补卡时间段 ({}个):",
                            allowed_remedy.remedy_periods.len()
                        );

                        for (period_index, period) in
                            allowed_remedy.remedy_periods.iter().enumerate()
                        {
                            println!("       {}. {}", period_index + 1, period.remedy_type_name);
                            println!("          标准时间: {}", period.standard_time);
                            println!(
                                "          可补卡时间: {} - {}",
                                period.remedy_start_time, period.remedy_end_time
                            );

                            if period.can_remedy {
                                println!("          状态: ✅ 可以补卡");
                            } else {
                                println!("          状态: ❌ 不可补卡");
                                if let Some(reason) = &period.block_reason {
                                    println!("          原因: {}", reason);
                                }
                            }
                        }
                    }
                    println!("     ---");
                }

                if data.allowed_remedys.is_empty() {
                    println!("  📝 无可补卡时间");
                    println!("  💡 提示: 可能已超过补卡时限或无需补卡的记录");
                } else {
                    println!("💡 提示: 可以选择合适的时间段提交补卡申请");
                }
            }
        }
        Err(e) => {
            eprintln!("❌ 获取可补卡时间失败: {:?}", e);
            eprintln!("💡 提示: 请检查用户ID和查询时间范围是否正确");
        }
    }

    Ok(())
}

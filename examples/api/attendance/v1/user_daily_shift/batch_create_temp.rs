#![allow(clippy::field_reassign_with_default)]

use dotenvy::dotenv;
use open_lark::{
    prelude::LarkClient,
    service::attendance::v1::models::{BatchCreateTempUserDailyShiftRequest, UserDailyShift},
};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let app_id = env::var("APP_ID").expect("APP_ID must be set");
    let app_secret = env::var("APP_SECRET").expect("APP_SECRET must be set");

    let client = LarkClient::builder(&app_id, &app_secret).build();

    // 构建创建临时排班请求
    let mut req = BatchCreateTempUserDailyShiftRequest::default();
    req.employee_type = "employee_id".to_string();
    req.user_daily_shifts = vec![
        UserDailyShift {
            user_id: "employee_123".to_string(),
            shift_date: "2024-06-21".to_string(),
            shift_id: "7517943152473964546".to_string(),
        },
        UserDailyShift {
            user_id: "employee_456".to_string(),
            shift_date: "2024-06-21".to_string(),
            shift_id: "shift_holiday".to_string(),
        },
    ];

    println!("发送批量创建临时排班请求...");
    println!("请求参数: {:?}", req.employee_type);
    println!("临时排班信息数量: {}", req.user_daily_shifts.len());

    match client
        .attendance
        .v1
        .user_daily_shift
        .batch_create_temp(req, None)
        .await
    {
        Ok(resp) => {
            println!("✅ 批量创建临时排班成功!");
            if let Some(data) = resp.data {
                if let Some(success_count) = data.success_count {
                    println!("成功数量: {}", success_count);
                }
                if let Some(failed_count) = data.failed_count {
                    println!("失败数量: {}", failed_count);
                }
                if let Some(failed_shifts) = &data.failed_user_daily_shifts {
                    if !failed_shifts.is_empty() {
                        println!("失败的临时排班信息:");
                        for failed in failed_shifts {
                            println!(
                                "  - 用户: {}, 日期: {}, 班次: {}",
                                failed.user_id, failed.shift_date, failed.shift_id
                            );
                        }
                    }
                }
            }
        }
        Err(e) => {
            eprintln!("❌ 批量创建临时排班失败: {:?}", e);
        }
    }

    Ok(())
}

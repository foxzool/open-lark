use dotenvy::dotenv;
use open_lark::{prelude::LarkClient, service::attendance::v1::models::QueryUserDailyShiftRequest};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let app_id = env::var("APP_ID").expect("APP_ID must be set");
    let app_secret = env::var("APP_SECRET").expect("APP_SECRET must be set");

    let client = LarkClient::builder(&app_id, &app_secret).build();

    // 构建查询排班表请求
    let mut req = QueryUserDailyShiftRequest::default();
    req.employee_type = "employee_id".to_string();
    req.user_ids = vec!["employee_123".to_string(), "employee_456".to_string()];
    req.check_date_from = "2024-06-01".to_string();
    req.check_date_to = "2024-06-30".to_string();

    println!("发送查询排班表请求...");
    println!("查询用户数量: {}", req.user_ids.len());
    println!(
        "查询日期范围: {} - {}",
        req.check_date_from, req.check_date_to
    );

    match client.attendance.v1.user_daily_shift.query(req, None).await {
        Ok(resp) => {
            println!("✅ 查询排班表成功!");
            if let Some(data) = resp.data {
                println!("找到 {} 条排班记录", data.user_daily_shift_list.len());

                for shift_data in &data.user_daily_shift_list {
                println!("📋 排班信息:");
                println!("  用户ID: {}", shift_data.user_id);
                println!("  日期: {}", shift_data.shift_date);
                println!("  班次ID: {}", shift_data.shift_id);
                println!("  班次名称: {}", shift_data.shift_name);
                if let Some(is_temp) = shift_data.is_temp {
                    println!("  是否临时班次: {}", if is_temp { "是" } else { "否" });
                }
                if let Some(create_time) = &shift_data.create_time {
                    println!("  创建时间: {}", create_time);
                }
                if let Some(update_time) = &shift_data.update_time {
                    println!("  更新时间: {}", update_time);
                }
                println!("  ---");
                }
            }
        }
        Err(e) => {
            eprintln!("❌ 查询排班表失败: {:?}", e);
        }
    }

    Ok(())
}

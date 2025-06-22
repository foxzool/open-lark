use chrono::Utc;
use open_lark::{
    prelude::*,
    service::attendance::v1::models::{CreateShiftRequest, PunchTimeRule},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();
    env_logger::init();

    let app_id = std::env::var("APP_ID").expect("APP_ID is required");
    let app_secret = std::env::var("APP_SECRET").expect("APP_SECRET is required");

    // 创建客户端
    let client = LarkClient::builder(&app_id, &app_secret)
        .with_enable_token_cache(true)
        .build();

    println!("=== 创建班次接口示例 ===\n");

    // 定义打卡时间规则
    let punch_time_rules = vec![PunchTimeRule {
        on_time: "09:00".to_string(),
        off_time: "18:00".to_string(),
        on_advance_minutes: 30, // 上班提前30分钟可打卡
        off_delay_minutes: 30,  // 下班延迟30分钟可打卡
        late_minutes_as_late: Some(30),
        late_minutes_as_lack: Some(60),
        early_minutes_as_early: Some(30),
        early_minutes_as_lack: Some(60),
    }];

    // 创建标准班次
    println!("📝 创建标准班次...");
    let request = CreateShiftRequest::builder()
        .employee_type("employee_id") // 使用员工 employee ID
        .shift_name(format!("API示例-标准班次-{}", Utc::now().timestamp()))
        .punch_times(2)
        .is_flexible(false)
        .no_need_off(false)
        .punch_time_rule(punch_time_rules.clone())
        .late_minutes_as_late(30)
        .late_minutes_as_lack(60)
        .early_minutes_as_early(60) // 增加到60分钟
        .early_minutes_as_lack(120)
        .allow_outside_apply(true)
        .outside_apply_limit(2)
        .build();

    match client.attendance.v1.shift.create(request, None).await {
        Ok(response) => {
            if response.success() {
                if let Some(data) = response.data {
                    let shift = &data.shift;
                    println!("✅ 标准班次创建成功!");
                    println!("   班次ID: {}", shift.shift_id);
                    println!("   班次名称: {}", shift.shift_name);
                    println!("   打卡次数: {}", shift.punch_times);
                    println!("   是否弹性打卡: {:?}", shift.is_flexible);
                } else {
                    println!("⚠️ 响应成功但无数据");
                }
            } else {
                println!("❌ 创建失败: {} - {} \n", response.code(), response.msg());
            }
        }
        Err(e) => {
            eprintln!("❌ 请求失败: {:?}", e);
        }
    }

    println!();

    // 创建弹性班次
    println!("📝 创建弹性班次...");
    let flexible_request = CreateShiftRequest::builder()
        .employee_type("employee_id") // 使用员工 employee ID
        .shift_name(format!("API示例-弹性班次-{}", Utc::now().timestamp()))
        .punch_times(2)
        .is_flexible(true)
        .flexible_minutes(30)
        .punch_time_rule(punch_time_rules)
        .allow_outside_apply(true)
        .outside_apply_limit(3)
        .build();

    match client
        .attendance
        .v1
        .shift
        .create(flexible_request, None)
        .await
    {
        Ok(response) => {
            if response.success() {
                if let Some(data) = response.data {
                    let shift = &data.shift;
                    println!("✅ 弹性班次创建成功!");
                    println!("   班次ID: {}", shift.shift_id);
                    println!("   班次名称: {}", shift.shift_name);
                    println!("   弹性打卡时间: {:?} 分钟", shift.flexible_minutes);
                    println!("   在家办公限制: {:?} 次", shift.outside_apply_limit);
                } else {
                    println!("⚠️ 响应成功但无数据");
                }
            } else {
                println!("❌ 创建失败: {} - {}", response.code(), response.msg());
            }
        }
        Err(e) => {
            eprintln!("❌ 请求失败: {:?}", e);
        }
    }

    println!("\n=== 示例结束 ===");
    Ok(())
}

use dotenvy::dotenv;
use open_lark::{
    prelude::LarkClient,
    service::attendance::v1::models::{LeaveAccrualRecordPatch, PatchLeaveAccrualRecordRequest},
};

/// 修改发放记录示例
///
/// 该接口用于修改指定的休假发放记录信息，包括发放数量、过期时间、发放原因等。
/// 支持部分字段更新，只需要传入需要修改的字段即可。
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    // 从环境变量获取应用配置
    let app_id = std::env::var("APP_ID").expect("APP_ID must be set");
    let app_secret = std::env::var("APP_SECRET").expect("APP_SECRET must be set");

    // 创建 LarkClient
    let client = LarkClient::builder(&app_id, &app_secret).build();

    // 设置新的过期时间（示例：设置为90天后过期）
    let ninety_days_later = chrono::Utc::now().timestamp_millis() + (90 * 24 * 60 * 60 * 1000);

    // 构建发放记录修改信息
    let leave_record_patch = LeaveAccrualRecordPatch {
        granted_amount: Some(80.0),           // 修改发放数量为80小时
        expire_time: Some(ninety_days_later), // 设置新的过期时间
        granted_reason: Some("年度假期调整 - 增加发放数量并延长有效期".to_string()),
        validity_type: Some(2), // 2-指定过期时间
        granted_type: Some(2),  // 2-管理员手动发放
        granted_description: Some("因工作表现优秀，额外增加年假发放量".to_string()),
        ..Default::default()
    };

    // 构建修改发放记录请求
    let req = PatchLeaveAccrualRecordRequest {
        employee_type: "employee_id".to_string(),
        leave_accrual_record_id: "leave_record_123".to_string(), // 实际使用时需要替换为真实的记录ID
        leave_accrual_record: leave_record_patch,
        ..Default::default()
    };

    println!("修改发放记录请求:");
    println!("  记录ID: {}", req.leave_accrual_record_id);
    println!("  员工ID类型: {}", req.employee_type);
    println!("  修改内容:");
    if let Some(amount) = req.leave_accrual_record.granted_amount {
        println!("    发放数量: {} 小时", amount);
    }
    if let Some(expire_time) = req.leave_accrual_record.expire_time {
        println!(
            "    过期时间: {} ({})",
            expire_time,
            chrono::DateTime::from_timestamp_millis(expire_time)
                .unwrap()
                .format("%Y-%m-%d %H:%M:%S")
        );
    }
    if let Some(reason) = &req.leave_accrual_record.granted_reason {
        println!("    发放原因: {}", reason);
    }
    if let Some(description) = &req.leave_accrual_record.granted_description {
        println!("    发放说明: {}", description);
    }
    if let Some(validity_type) = req.leave_accrual_record.validity_type {
        println!(
            "    有效期类型: {}",
            match validity_type {
                1 => "永久有效",
                2 => "指定过期时间",
                _ => "未知类型",
            }
        );
    }
    if let Some(granted_type) = req.leave_accrual_record.granted_type {
        println!(
            "    发放类型: {}",
            match granted_type {
                1 => "系统自动发放",
                2 => "管理员手动发放",
                3 => "员工申请发放",
                _ => "未知类型",
            }
        );
    }

    // 调用 API
    match client
        .attendance
        .v1
        .leave_accrual_record
        .patch(req, None)
        .await
    {
        Ok(resp) => {
            println!("✅ 修改发放记录成功!");
            if let Some(data) = resp.data {
                println!("📝 修改后的记录信息:");
                let record = &data.leave_accrual_record;
                println!("  记录ID: {}", record.record_id);
                println!("  员工ID: {}", record.employee_id);
                if let Some(name) = &record.employee_name {
                    println!("  员工姓名: {}", name);
                }
                println!(
                    "  休假类型: {} ({})",
                    record.leave_type_name, record.leave_type_id
                );
                println!("  发放数量: {} 小时", record.granted_amount);
                println!("  剩余数量: {} 小时", record.remaining_amount);
                println!("  已使用数量: {} 小时", record.used_amount);
                println!(
                    "  状态: {}",
                    match record.status {
                        1 => "有效",
                        2 => "已过期",
                        3 => "已使用完",
                        _ => "未知状态",
                    }
                );

                if let Some(expire_time) = record.expire_time {
                    println!(
                        "  过期时间: {}",
                        chrono::DateTime::from_timestamp_millis(expire_time)
                            .unwrap()
                            .format("%Y-%m-%d %H:%M:%S")
                    );
                } else {
                    println!("  过期时间: 永久有效");
                }

                println!(
                    "  发放时间: {}",
                    chrono::DateTime::from_timestamp_millis(record.granted_time)
                        .unwrap()
                        .format("%Y-%m-%d %H:%M:%S")
                );

                if let Some(reason) = &record.granted_reason {
                    println!("  发放原因: {}", reason);
                }

                if let Some(description) = &record.granted_description {
                    println!("  发放说明: {}", description);
                }

                println!(
                    "  有效期类型: {}",
                    match record.validity_type {
                        1 => "永久有效",
                        2 => "指定过期时间",
                        _ => "未知类型",
                    }
                );

                println!(
                    "  发放类型: {}",
                    match record.granted_type {
                        1 => "系统自动发放",
                        2 => "管理员手动发放",
                        3 => "员工申请发放",
                        _ => "未知类型",
                    }
                );

                println!(
                    "  创建时间: {}",
                    chrono::DateTime::from_timestamp_millis(record.create_time)
                        .unwrap()
                        .format("%Y-%m-%d %H:%M:%S")
                );
                println!(
                    "  更新时间: {}",
                    chrono::DateTime::from_timestamp_millis(record.update_time)
                        .unwrap()
                        .format("%Y-%m-%d %H:%M:%S")
                );

                println!("\n💡 提示: 发放记录已成功更新，员工可以看到最新的休假余额");
            }
        }
        Err(e) => {
            eprintln!("❌ 修改发放记录失败: {:?}", e);
            eprintln!("💡 提示: 请检查记录ID是否存在，以及是否有修改权限");
        }
    }

    Ok(())
}

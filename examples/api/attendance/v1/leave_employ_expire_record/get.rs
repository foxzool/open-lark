use dotenvy::dotenv;
use open_lark::{
    prelude::LarkClient, service::attendance::v1::models::GetLeaveEmployExpireRecordRequest,
};

/// 通过过期时间获取发放记录示例
///
/// 该接口用于通过过期时间范围查询员工休假发放记录，支持分页查询。
/// 可以查询指定时间范围内即将过期或已过期的休假发放记录。
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    // 从环境变量获取应用配置
    let app_id = std::env::var("APP_ID").expect("APP_ID must be set");
    let app_secret = std::env::var("APP_SECRET").expect("APP_SECRET must be set");

    // 创建 LarkClient
    let client = LarkClient::builder(&app_id, &app_secret).build();

    // 设置查询时间范围（示例：查询未来30天内过期的记录）
    let now = chrono::Utc::now().timestamp_millis();
    let thirty_days_later = now + (30 * 24 * 60 * 60 * 1000); // 30天后

    // 构建获取过期发放记录请求
    let req = GetLeaveEmployExpireRecordRequest {
        employee_type: "employee_id".to_string(),
        start_time: now,
        end_time: thirty_days_later,
        page_size: Some(20),
        page_token: None,
        ..Default::default()
    };

    println!("查询时间范围: {} 到 {}", now, thirty_days_later);
    println!("查询参数:");
    println!("  员工ID类型: {}", req.employee_type);
    println!(
        "  开始时间: {} ({})",
        req.start_time,
        chrono::DateTime::from_timestamp_millis(req.start_time)
            .unwrap()
            .format("%Y-%m-%d %H:%M:%S")
    );
    println!(
        "  结束时间: {} ({})",
        req.end_time,
        chrono::DateTime::from_timestamp_millis(req.end_time)
            .unwrap()
            .format("%Y-%m-%d %H:%M:%S")
    );
    println!("  分页大小: {:?}", req.page_size);

    // 调用 API
    match client
        .attendance
        .v1
        .leave_employ_expire_record
        .get(req, None)
        .await
    {
        Ok(resp) => {
            println!("✅ 查询过期发放记录成功!");
            if let Some(data) = resp.data {
                println!("📊 查询结果:");
                println!("  总记录数: {}", data.total_count);
                println!("  当前页记录数: {}", data.records.len());
                println!("  是否有更多数据: {}", data.has_more);

                if !data.records.is_empty() {
                    println!("\n📝 发放记录详情:");
                    for (index, record) in data.records.iter().enumerate() {
                        println!("  记录 {}:", index + 1);
                        println!("    记录ID: {}", record.record_id);
                        println!("    员工ID: {}", record.employee_id);
                        if let Some(name) = &record.employee_name {
                            println!("    员工姓名: {}", name);
                        }
                        println!(
                            "    休假类型: {} ({})",
                            record.leave_type_name, record.leave_type_id
                        );
                        println!("    发放数量: {} 小时", record.granted_amount);
                        println!("    剩余数量: {} 小时", record.remaining_amount);
                        println!(
                            "    状态: {}",
                            match record.status {
                                1 => "有效",
                                2 => "已过期",
                                3 => "已使用完",
                                _ => "未知状态",
                            }
                        );
                        println!(
                            "    过期时间: {}",
                            chrono::DateTime::from_timestamp_millis(record.expire_time)
                                .unwrap()
                                .format("%Y-%m-%d %H:%M:%S")
                        );
                        println!(
                            "    发放时间: {}",
                            chrono::DateTime::from_timestamp_millis(record.granted_time)
                                .unwrap()
                                .format("%Y-%m-%d %H:%M:%S")
                        );
                        if let Some(reason) = &record.granted_reason {
                            println!("    发放原因: {}", reason);
                        }
                        println!();
                    }
                } else {
                    println!("  📝 在指定时间范围内没有找到即将过期的发放记录");
                }

                // 检查是否有更多数据
                if data.has_more {
                    if let Some(page_token) = data.page_token {
                        println!(
                            "💡 提示: 还有更多数据，可以使用 page_token '{}' 获取下一页",
                            page_token
                        );
                    }
                }
            }
        }
        Err(e) => {
            eprintln!("❌ 查询过期发放记录失败: {:?}", e);
            eprintln!("💡 提示: 请检查时间范围参数是否正确，以及是否有相应的权限");
        }
    }

    Ok(())
}

//! 查询用户考勤记录示例
//!
//! 演示如何使用考勤API查询用户的日常考勤记录，包括打卡结果、异常信息等。

use chrono::NaiveDate;
use open_lark::{client::LarkClient, service::attendance::v1::models::*};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 从 .env 文件加载环境变量
    dotenvy::dotenv().ok();

    // 从环境变量读取配置
    let app_id = std::env::var("APP_ID").expect("APP_ID is required in .env file");
    let app_secret = std::env::var("APP_SECRET").expect("APP_SECRET is required in .env file");

    // 创建客户端
    let client = LarkClient::builder(&app_id, &app_secret).build();

    println!("=== 飞书考勤记录查询示例 ===\n");

    // 1. 基础查询示例
    basic_query_example(&client).await?;

    // 2. 分页查询示例
    pagination_query_example(&client).await?;

    // 3. 带详细信息查询示例
    detailed_query_example(&client).await?;

    Ok(())
}

/// 基础查询示例
async fn basic_query_example(client: &LarkClient) -> Result<(), Box<dyn std::error::Error>> {
    println!("1. 基础查询示例");
    println!("----------------");

    let req = UserTaskQueryRequest::builder()
        .user_ids(vec!["ou_xxx".to_string()]) // 替换为实际的用户ID
        .check_date_from(NaiveDate::from_ymd_opt(2025, 1, 1).unwrap())
        .check_date_to(NaiveDate::from_ymd_opt(2025, 1, 31).unwrap())
        .page_size(10)
        .build();

    match client.attendance.v1.user_task.query(req, None).await {
        Ok(response) => {
            println!("查询成功，共找到 {} 条记录", response.records.len());
            for record in response.records.iter().take(3) {
                println!("用户 {} 在 {} 的考勤记录:", record.user_id, record.date);
                println!("  班次: {}", record.shift_id);
                println!("  上班打卡: {:?}", record.check_in_result.result);
                println!("  下班打卡: {:?}", record.check_out_result.result);
                if let Some(absent) = &record.absent_info {
                    println!("  请假类型: {}", absent.absent_type);
                }
                println!();
            }

            if response.has_more {
                println!("还有更多数据，页面令牌: {:?}", response.page_token);
            }
        }
        Err(e) => {
            eprintln!("查询失败: {:?}", e);
        }
    }

    println!();
    Ok(())
}

/// 分页查询示例
async fn pagination_query_example(client: &LarkClient) -> Result<(), Box<dyn std::error::Error>> {
    println!("2. 分页查询示例");
    println!("----------------");

    let req = UserTaskQueryRequest::builder()
        .user_ids(vec!["ou_xxx".to_string()]) // 替换为实际的用户ID
        .check_date_from(NaiveDate::from_ymd_opt(2025, 1, 1).unwrap())
        .check_date_to(NaiveDate::from_ymd_opt(2025, 1, 31).unwrap())
        .page_size(5) // 小页面用于演示分页
        .build();

    let mut iter = client.attendance.v1.user_task.query_iter(req);
    let mut page_count = 0;
    let mut total_records = 0;

    while let Some(records) = iter.next_page().await? {
        page_count += 1;
        total_records += records.len();

        println!("第 {} 页，包含 {} 条记录", page_count, records.len());

        // 只处理前3页作为示例
        if page_count >= 3 {
            break;
        }
    }

    println!("共处理 {} 页，总计 {} 条记录", page_count, total_records);
    println!();
    Ok(())
}

/// 带详细信息查询示例
async fn detailed_query_example(client: &LarkClient) -> Result<(), Box<dyn std::error::Error>> {
    println!("3. 带详细信息查询示例");
    println!("--------------------");

    let req = UserTaskQueryRequest::builder()
        .user_ids(vec!["ou_xxx".to_string()]) // 替换为实际的用户ID
        .check_date_from(NaiveDate::from_ymd_opt(2025, 1, 15).unwrap())
        .check_date_to(NaiveDate::from_ymd_opt(2025, 1, 20).unwrap())
        .need_absent_info(true)
        .need_supplement_info(true)
        .page_size(20)
        .build();

    match client.attendance.v1.user_task.query(req, None).await {
        Ok(response) => {
            println!("详细查询成功，共找到 {} 条记录", response.records.len());

            for record in response.records {
                println!("📅 {} ({})", record.date, record.user_id);
                println!(
                    "   班次: {} - {}",
                    record.shift_id,
                    record.shift_name.as_deref().unwrap_or("未知")
                );

                // 打卡结果分析
                print_check_in_result("上班", &record.check_in_result);
                print_check_out_result("下班", &record.check_out_result);

                // 请假信息
                if let Some(absent) = &record.absent_info {
                    println!(
                        "   🏠 请假: {} ({:.1}小时)",
                        absent.absent_type, absent.duration
                    );
                }

                // 补卡信息
                if let Some(supplement) = &record.supplement_info {
                    println!("   🔄 补卡状态: {:?}", supplement.status);
                    if let Some(reason) = &supplement.reason {
                        println!("   💬 补卡原因: {}", reason);
                    }
                }

                println!();
            }
        }
        Err(e) => {
            eprintln!("详细查询失败: {:?}", e);
        }
    }

    Ok(())
}

/// 打印上班打卡结果信息
fn print_check_in_result(check_type: &str, result: &CheckInResult) {
    let status_emoji = match result.result {
        CheckInStatus::Normal => "✅",
        CheckInStatus::Late => "🟡",
        CheckInStatus::Early => "🟠",
        CheckInStatus::Lack => "❌",
        CheckInStatus::NoNeed => "⚪",
    };

    print!("   {} {} {:?}", status_emoji, check_type, result.result);

    if let Some(time) = &result.time {
        print!(" ({})", time.format("%H:%M:%S"));
    }

    if let Some(location) = &result.location_info {
        if let Some(name) = &location.name {
            print!(" @ {}", name);
        }
    }

    if let Some(exception) = &result.exception_type {
        print!(" [异常: {:?}]", exception);
    }

    println!();
}

/// 打印下班打卡结果信息
fn print_check_out_result(check_type: &str, result: &CheckOutResult) {
    let status_emoji = match result.result {
        CheckInStatus::Normal => "✅",
        CheckInStatus::Late => "🟡",
        CheckInStatus::Early => "🟠",
        CheckInStatus::Lack => "❌",
        CheckInStatus::NoNeed => "⚪",
    };

    print!("   {} {} {:?}", status_emoji, check_type, result.result);

    if let Some(time) = &result.time {
        print!(" ({})", time.format("%H:%M:%S"));
    }

    if let Some(location) = &result.location_info {
        if let Some(name) = &location.name {
            print!(" @ {}", name);
        }
    }

    if let Some(exception) = &result.exception_type {
        print!(" [异常: {:?}]", exception);
    }

    println!();
}

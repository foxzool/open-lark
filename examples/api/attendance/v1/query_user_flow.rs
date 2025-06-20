//! 查询用户打卡流水示例
//!
//! 演示如何使用考勤API查询用户的打卡流水记录，包括打卡时间、位置、方式等详细信息。

use chrono::Utc;
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

    println!("=== 飞书打卡流水查询示例 ===\n");
    
    println!("📋 使用说明:");
    println!("   1. 此示例使用占位符用户ID 'ou_xxx'，仅用于演示API调用结构");
    println!("   2. 要实际测试，请将代码中的 'ou_xxx' 替换为真实的用户ID");
    println!("   3. 确保在 .env 文件中配置了有效的 APP_ID 和 APP_SECRET");
    println!("   4. 确保应用拥有考勤相关权限\n");

    // 1. 基础流水查询示例
    basic_flow_query_example(&client).await?;

    // 2. 时间范围流水查询
    time_range_flow_query_example(&client).await?;

    // 3. 分页流水查询
    pagination_flow_query_example(&client).await?;

    Ok(())
}

/// 基础流水查询示例
async fn basic_flow_query_example(client: &LarkClient) -> Result<(), Box<dyn std::error::Error>> {
    println!("1. 基础流水查询示例");
    println!("------------------");

    let req = UserFlowQueryRequest::builder()
        .user_ids(vec!["ou_xxx".to_string()]) // 替换为实际的用户ID
        .check_time_from(Utc::now() - chrono::Duration::days(1)) // 最近1天
        .check_time_to(Utc::now())
        .page_size(20)
        .build();

    println!("🔍 正在查询用户打卡流水...");
    println!("⚠️  注意：示例使用占位符用户ID 'ou_xxx'，实际使用时请替换为真实用户ID");
    
    match client.attendance.v1.user_flow.query(req, None).await {
        Ok(response) => {
            println!("✅ 查询成功，共找到 {} 条打卡记录", response.records.len());

            for record in response.records.iter().take(10) {
                print_flow_record(record);
            }

            if response.has_more {
                println!("📄 还有更多数据，页面令牌: {:?}", response.page_token);
            }
        }
        Err(e) => {
            eprintln!("❌ 查询失败: {:?}", e);
            println!("💡 可能的原因：");
            println!("   1. 用户ID 'ou_xxx' 是占位符，请替换为真实的用户ID");
            println!("   2. 应用没有考勤相关权限");
            println!("   3. APP_ID 或 APP_SECRET 配置错误");
            println!("   4. 网络连接问题");
        }
    }

    println!();
    Ok(())
}

/// 时间范围流水查询示例
async fn time_range_flow_query_example(
    client: &LarkClient,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("2. 时间范围流水查询示例");
    println!("----------------------");

    // 查询本周的打卡记录
    let week_start = Utc::now() - chrono::Duration::days(7);
    let week_end = Utc::now();

    let req = UserFlowQueryRequest::builder()
        .user_ids(vec!["ou_xxx".to_string()]) // 替换为实际的用户ID
        .check_time_from(week_start)
        .check_time_to(week_end)
        .page_size(50)
        .build();

    println!("🔍 正在查询本周打卡记录...");
    
    match client.attendance.v1.user_flow.query(req, None).await {
        Ok(response) => {
            println!("✅ 本周打卡记录查询成功，共 {} 条记录", response.records.len());

            // 按日期分组统计
            let mut daily_stats = std::collections::HashMap::new();
            for record in &response.records {
                let date = record.check_time.date_naive();
                let entry = daily_stats.entry(date).or_insert(Vec::new());
                entry.push(record);
            }

            for (date, records) in daily_stats {
                println!("📅 {}: {} 次打卡", date, records.len());
                for record in records {
                    let check_type_emoji = match record.check_type {
                        CheckType::CheckIn => "🟢",
                        CheckType::CheckOut => "🔴",
                    };
                    println!(
                        "  {} {:?} {} ({})",
                        check_type_emoji,
                        record.check_type,
                        record.check_time.format("%H:%M:%S"),
                        format_check_method(record.check_method)
                    );
                }
                println!();
            }
        }
        Err(e) => {
            eprintln!("❌ 时间范围查询失败: {:?}", e);
            println!("💡 请检查用户ID和权限配置");
        }
    }

    println!();
    Ok(())
}

/// 分页流水查询示例
async fn pagination_flow_query_example(
    client: &LarkClient,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("3. 分页流水查询示例");
    println!("------------------");

    let req = UserFlowQueryRequest::builder()
        .user_ids(vec!["ou_xxx".to_string()]) // 替换为实际的用户ID
        .check_time_from(Utc::now() - chrono::Duration::days(30)) // 最近30天
        .check_time_to(Utc::now())
        .page_size(10) // 小页面用于演示分页
        .build();

    println!("🔍 正在分页查询最近30天的打卡记录...");
    println!("⚠️  注意：使用占位符用户ID，实际调用会失败");
    
    let mut iter = client.attendance.v1.user_flow.query_iter(req);
    let mut page_count = 0;
    let mut total_records = 0;
    let mut check_in_count = 0;
    let mut check_out_count = 0;

    while let Some(records) = iter.next_page().await? {
        page_count += 1;
        total_records += records.len();

        for record in &records {
            match record.check_type {
                CheckType::CheckIn => check_in_count += 1,
                CheckType::CheckOut => check_out_count += 1,
            }
        }

        println!("第 {} 页，包含 {} 条记录", page_count, records.len());

        // 只处理前5页作为示例
        if page_count >= 5 {
            break;
        }
    }

    println!("统计结果:");
    println!("  总页数: {}", page_count);
    println!("  总记录数: {}", total_records);
    println!("  上班打卡: {} 次", check_in_count);
    println!("  下班打卡: {} 次", check_out_count);

    Ok(())
}

/// 打印打卡流水记录
fn print_flow_record(record: &UserFlowRecord) {
    let check_type_emoji = match record.check_type {
        CheckType::CheckIn => "🟢",
        CheckType::CheckOut => "🔴",
    };

    println!(
        "{}用户 {} {:?} {}",
        check_type_emoji,
        record.user_id,
        record.check_type,
        record.check_time.format("%Y-%m-%d %H:%M:%S")
    );

    println!("   打卡方式: {}", format_check_method(record.check_method));

    // 位置信息
    if let Some(location) = &record.location_info {
        if let Some(name) = &location.name {
            print!("   位置: {}", name);
        }
        if let Some(address) = &location.address {
            print!(" ({})", address);
        }
        if location.longitude.is_some() && location.latitude.is_some() {
            print!(
                " [GPS: {:.6}, {:.6}]",
                location.longitude.unwrap(),
                location.latitude.unwrap()
            );
        }
        println!();
    }

    // 设备信息
    if let Some(device) = &record.device_info {
        if let Some(name) = &device.device_name {
            println!("   设备: {}", name);
        }
    }

    // 照片信息
    if let Some(photo) = &record.photo_info {
        if let Some(url) = &photo.photo_url {
            println!("   📷 照片: {}", url);
        }
    }

    println!();
}

/// 格式化打卡方式
fn format_check_method(method: CheckMethod) -> &'static str {
    match method {
        CheckMethod::GPS => "GPS定位",
        CheckMethod::WiFi => "WiFi",
        CheckMethod::Machine => "考勤机",
        CheckMethod::PC => "电脑",
        CheckMethod::Mobile => "手机",
    }
}

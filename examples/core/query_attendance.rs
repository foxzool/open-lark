/// 考勤数据查询示例
///
/// 这个示例演示如何使用飞书SDK查询员工的考勤相关信息。
///
/// 使用方法：
/// cargo run --example core_query_attendance
///
/// 环境变量：
/// APP_ID=your_app_id
/// APP_SECRET=your_app_secret
use open_lark::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 加载环境变量
    dotenvy::dotenv().ok();

    let app_id = std::env::var("APP_ID").expect("APP_ID environment variable not set");
    let app_secret = std::env::var("APP_SECRET").expect("APP_SECRET environment variable not set");

    // 创建客户端
    let client = LarkClient::builder(&app_id, &app_secret)
        .with_enable_token_cache(true)
        .build();

    println!("📊 飞书考勤数据查询示例");
    println!("{}", "=".repeat(50));

    // 查询考勤统计数据
    query_attendance_stats(&client).await?;

    // 查询审批数据
    query_approval_data(&client).await?;

    Ok(())
}

/// 查询考勤统计数据
async fn query_attendance_stats(client: &LarkClient) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n📈 查询考勤统计数据...");

    // 设置查询时间范围（最近7天）
    let end_date = chrono::Utc::now().date_naive();
    let start_date = end_date - chrono::Duration::days(7);

    println!("   查询时间范围: {start_date} 到 {end_date}");

    // 构建查询请求
    let request = open_lark::service::attendance::v1::models::QueryUserStatsDataRequest {
        api_req: Default::default(),
        employee_type: "employee_id".to_string(),
        start_date: start_date.format("%Y-%m-%d").to_string(),
        end_date: end_date.format("%Y-%m-%d").to_string(),
        user_ids: vec![], // 空数组查询所有用户
        need_fields: vec![
            "user_id".to_string(),
            "date".to_string(),
            "work_duration".to_string(),
        ],
        locale: Some("zh-CN".to_string()),
    };

    // 注意：实际API调用需要直接调用service方法，不是通过builder模式
    match client
        .attendance
        .v1
        .user_stats_data
        .query_data(request, None)
        .await
    {
        Ok(response) => {
            if let Some(data) = &response.data {
                println!("✅ 考勤统计数据查询成功!");
                println!("   返回记录数: {}", data.datas.len());

                if !data.datas.is_empty() {
                    println!("\n📊 考勤统计结果:");
                    for (index, stats) in data.datas.iter().enumerate() {
                        println!("   {}. 用户ID: {}", index + 1, stats.user_id);

                        if let Some(user_name) = &stats.user_name {
                            println!("      用户姓名: {user_name}");
                        }

                        // 显示统计字段数据
                        for (field_name, field_value) in &stats.datas {
                            println!("      {field_name}: {field_value}");
                        }

                        println!(); // 空行分隔
                    }
                } else {
                    println!("📭 没有找到考勤统计数据");
                    println!("💡 可能的原因:");
                    println!("   1. 查询时间范围内没有考勤记录");
                    println!("   2. 考勤统计数据尚未生成");
                    println!("   3. 应用权限不足");
                }
            } else {
                println!("⚠️ 查询请求成功，但未返回数据");
            }
        }
        Err(e) => {
            println!("❌ 查询考勤统计数据失败: {e:?}");
            println!("\n💡 常见错误解决方案:");
            println!("   1. 检查应用是否有考勤数据访问权限");
            println!("   2. 确认时间范围格式是否正确 (YYYY-MM-DD)");
            println!("   3. 验证应用访问令牌权限");
            println!("   4. 检查employee_type参数是否正确");
            return Err(e.into());
        }
    }

    Ok(())
}

/// 查询审批数据
async fn query_approval_data(client: &LarkClient) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n📋 查询审批数据...");

    // 设置查询时间范围（最近30天）
    let end_date = chrono::Utc::now().date_naive();
    let start_date = end_date - chrono::Duration::days(30);

    println!("   查询时间范围: {start_date} 到 {end_date}");

    // 构建查询请求
    let request = open_lark::service::attendance::v1::models::QueryUserApprovalRequest {
        api_req: Default::default(),
        employee_type: "employee_id".to_string(),
        status: None, // 查询所有状态的审批
        date_from: Some(start_date.format("%Y-%m-%d").to_string()),
        date_to: Some(end_date.format("%Y-%m-%d").to_string()),
        user_ids: None, // 查询所有用户
        page_token: None,
        page_size: Some(20),
    };

    match client
        .attendance
        .v1
        .user_approval
        .query(request, None)
        .await
    {
        Ok(response) => {
            if let Some(data) = &response.data {
                println!("✅ 审批数据查询成功!");
                println!("   总记录数: {}", data.approvals.len());
                println!("   是否有更多: {}", data.has_more);

                if !data.approvals.is_empty() {
                    println!("\n📄 审批记录列表:");
                    for (index, approval) in data.approvals.iter().enumerate() {
                        let status_text = match approval.status {
                            1 => "审批中",
                            2 => "已通过",
                            3 => "已拒绝",
                            _ => "未知状态",
                        };

                        println!("   {}. 审批ID: {}", index + 1, approval.approval_id);
                        println!("      用户ID: {}", approval.user_id);
                        println!("      状态: {status_text}");

                        println!("      开始时间: {}", approval.start_time);
                        println!("      结束时间: {}", approval.end_time);

                        if let Some(reason) = &approval.reason {
                            let display_reason = if reason.len() > 50 {
                                format!("{}...", &reason[..50])
                            } else {
                                reason.clone()
                            };
                            println!("      申请原因: {display_reason}");
                        }

                        println!(); // 空行分隔
                    }
                } else {
                    println!("📭 没有找到审批记录");
                }

                if data.has_more {
                    println!("💡 提示: 还有更多审批记录可以通过分页获取");
                    if let Some(next_page_token) = &data.page_token {
                        println!("   下一页Token: {next_page_token}");
                    }
                }
            } else {
                println!("⚠️ 查询请求成功，但未返回数据");
            }
        }
        Err(e) => {
            println!("❌ 查询审批数据失败: {e:?}");
            println!("\n💡 常见错误解决方案:");
            println!("   1. 检查应用是否有审批数据访问权限");
            println!("   2. 确认时间范围格式是否正确");
            println!("   3. 验证应用访问令牌权限");
            println!("   4. 检查查询参数是否有效");
        }
    }

    Ok(())
}

/// 演示查询统计字段信息
#[allow(dead_code)]
async fn query_stats_fields(client: &LarkClient) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🔧 查询可用的统计字段...");

    let request = open_lark::service::attendance::v1::models::QueryStatsFieldsRequest {
        api_req: Default::default(),
        employee_type: "employee_id".to_string(),
        locale: Some("zh-CN".to_string()),
    };

    match client
        .attendance
        .v1
        .user_stats_data
        .query_fields(request, None)
        .await
    {
        Ok(response) => {
            if let Some(data) = &response.data {
                println!("✅ 统计字段查询成功!");
                println!("   可用字段数: {}", data.fields.len());

                for field in &data.fields {
                    println!("   - {}: {}", field.field_key, field.field_name);
                }
            }
        }
        Err(e) => {
            println!("❌ 查询统计字段失败: {e:?}");
        }
    }

    Ok(())
}

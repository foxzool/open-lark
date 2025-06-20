//! 考勤模块完整集成演示
//!
//! 展示如何使用 open-lark SDK 的考勤功能

use chrono::NaiveDate;
use open_lark::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🎯 飞书考勤模块集成演示");
    println!("====================\n");

    // 从 .env 文件加载环境变量
    dotenvy::dotenv().ok();

    // 检查环境变量
    let app_id =
        std::env::var("APP_ID").map_err(|_| "⚠️  请在 .env 文件中设置 APP_ID=your_app_id")?;
    let app_secret = std::env::var("APP_SECRET")
        .map_err(|_| "⚠️  请在 .env 文件中设置 APP_SECRET=your_app_secret")?;

    // 创建客户端
    let client = LarkClient::builder(&app_id, &app_secret)
        .with_enable_token_cache(true)
        .build();

    println!("✅ 客户端创建成功！");
    println!("📋 可用的考勤功能：");
    println!("   • client.attendance.v1.user_task  - 用户考勤任务查询");
    println!("   • client.attendance.v1.user_flow  - 用户打卡流水查询");
    println!("   • client.attendance.v1.shift      - 排班信息查询");
    println!();

    // 演示数据模型构建
    demo_data_models().await?;

    // 演示 API 调用结构
    demo_api_structure(&client).await?;

    Ok(())
}

/// 演示数据模型的构建和使用
async fn demo_data_models() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔧 数据模型演示");
    println!("----------------");

    // 1. 用户考勤任务查询请求
    let task_req = UserTaskQueryRequest::builder()
        .user_ids(vec!["ou_user_123".to_string()])
        .check_date_from(NaiveDate::from_ymd_opt(2025, 1, 1).unwrap())
        .check_date_to(NaiveDate::from_ymd_opt(2025, 1, 31).unwrap())
        .need_absent_info(true)
        .need_supplement_info(true)
        .page_size(50)
        .build();

    println!("✅ UserTaskQueryRequest 构建成功");
    println!("   用户数量: {}", task_req.user_ids.len());
    println!(
        "   查询范围: {} ~ {}",
        task_req.check_date_from, task_req.check_date_to
    );

    // 2. 用户打卡流水查询请求
    let flow_req = UserFlowQueryRequest::builder()
        .user_ids(vec!["ou_user_123".to_string()])
        .check_time_from(chrono::Utc::now() - chrono::Duration::days(7))
        .check_time_to(chrono::Utc::now())
        .page_size(100)
        .build();

    println!("✅ UserFlowQueryRequest 构建成功");
    println!("   用户数量: {}", flow_req.user_ids.len());
    println!("   查询时长: 最近7天");

    // 3. 排班查询请求
    let shift_req = ShiftQueryRequest::builder()
        .shift_ids(vec!["shift_123".to_string()])
        .date(NaiveDate::from_ymd_opt(2025, 1, 20).unwrap())
        .build();

    println!("✅ ShiftQueryRequest 构建成功");
    println!("   班次数量: {}", shift_req.shift_ids.len());
    println!("   查询日期: {}", shift_req.date);

    // 4. 枚举类型演示
    println!("\n📊 枚举类型演示：");
    println!("   打卡状态: {:?}", CheckInStatus::Normal);
    println!("   打卡方式: {:?}", CheckMethod::Mobile);
    println!("   打卡类型: {:?}", CheckType::CheckIn);
    println!("   异常类型: {:?}", ExceptionType::NoException);

    println!();
    Ok(())
}

/// 演示 API 调用结构（不实际发送请求）
async fn demo_api_structure(_client: &LarkClient) -> Result<(), Box<dyn std::error::Error>> {
    println!("🌐 API 调用结构演示");
    println!("-------------------");

    // 演示如何构建查询请求
    let _task_req = UserTaskQueryRequest::builder()
        .user_ids(vec!["test_user".to_string()])
        .check_date_from(NaiveDate::from_ymd_opt(2025, 1, 1).unwrap())
        .check_date_to(NaiveDate::from_ymd_opt(2025, 1, 31).unwrap())
        .build();

    println!("📝 API 调用示例代码：");
    println!();
    println!("   // 1. 查询用户考勤记录");
    println!("   let response = client.attendance.v1.user_task");
    println!("       .query(task_req, None).await?;");
    println!();
    println!("   // 2. 使用迭代器分页查询");
    println!("   let mut iter = client.attendance.v1.user_task");
    println!("       .query_iter(task_req);");
    println!("   while let Some(records) = iter.next_page().await? {{");
    println!("       // 处理每页数据");
    println!("   }}");
    println!();
    println!("   // 3. 查询打卡流水");
    println!("   let flow_response = client.attendance.v1.user_flow");
    println!("       .query(flow_req, None).await?;");
    println!();
    println!("   // 4. 查询排班信息");
    println!("   let shift_response = client.attendance.v1.shift");
    println!("       .query(shift_req, None).await?;");

    println!("\n✨ 所有 API 调用都是异步的，支持错误处理和重试机制");
    println!("📚 详细使用方法请参考 examples/api/attendance/ 目录中的示例");

    // 注意：这里不实际调用 API，因为需要真实的访问令牌
    println!("\n💡 提示：要实际调用 API，您需要：");
    println!("   1. 在飞书开放平台申请相应的考勤权限");
    println!("   2. 在 .env 文件中配置正确的 APP_ID 和 APP_SECRET");
    println!("   3. 使用真实的用户ID和日期范围");
    println!("   4. 复制 .env-example 为 .env 并填写您的配置");

    Ok(())
}

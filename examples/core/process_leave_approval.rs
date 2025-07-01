/// 请假审批流程示例
///
/// 这个示例演示如何使用飞书SDK处理员工的请假申请审批流程。
///
/// 使用方法：
/// cargo run --example core_process_leave_approval
///
/// 环境变量：
/// APP_ID=your_app_id
/// APP_SECRET=your_app_secret
/// USER_ID=applicant_user_id (可选，默认使用示例用户)
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

    println!("📝 飞书请假审批流程示例");
    println!("{}", "=".repeat(50));

    // 查询当前待审批的请假申请
    query_pending_leave_requests(&client).await?;

    // 演示创建请假申请
    create_leave_request(&client).await?;

    // 演示处理审批
    demonstrate_approval_process(&client).await?;

    Ok(())
}

/// 查询待审批的请假申请
async fn query_pending_leave_requests(
    client: &LarkClient,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🔍 查询待审批的请假申请...");

    // 设置查询时间范围（最近30天）
    let end_date = chrono::Utc::now().date_naive();
    let start_date = end_date - chrono::Duration::days(30);

    println!("   查询时间范围: {start_date} 到 {end_date}");

    let request = open_lark::service::attendance::v1::models::QueryUserApprovalRequest {
        api_req: Default::default(),
        employee_type: "employee_id".to_string(),
        status: Some(1), // 1 = 审批中
        date_from: Some(start_date.format("%Y-%m-%d").to_string()),
        date_to: Some(end_date.format("%Y-%m-%d").to_string()),
        user_ids: None,
        page_token: None,
        page_size: Some(10),
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
                println!("✅ 待审批请假申请查询成功!");
                println!("   待审批数量: {}", data.approvals.len());

                if !data.approvals.is_empty() {
                    println!("\n📋 待审批请假列表:");
                    for (index, approval) in data.approvals.iter().enumerate() {
                        println!("   {}. 申请ID: {}", index + 1, approval.approval_id);
                        println!("      申请人: {}", approval.user_id);

                        println!("      请假开始: {}", approval.start_time);
                        println!("      请假结束: {}", approval.end_time);

                        if let Some(duration) = &approval.duration {
                            println!("      请假时长: {duration}小时");
                        }

                        if let Some(reason) = &approval.reason {
                            let display_reason = if reason.len() > 30 {
                                format!("{}...", &reason[..30])
                            } else {
                                reason.clone()
                            };
                            println!("      请假原因: {display_reason}");
                        }

                        let type_name = match approval.approval_type {
                            1 => "请假",
                            2 => "出差",
                            3 => "外出",
                            4 => "加班",
                            5 => "调休",
                            _ => "其他",
                        };
                        println!("      申请类型: {type_name}");

                        println!(); // 空行分隔
                    }
                } else {
                    println!("📭 当前没有待审批的请假申请");
                }
            } else {
                println!("⚠️ 查询请求成功，但未返回数据");
            }
        }
        Err(e) => {
            println!("❌ 查询待审批请假申请失败: {e:?}");
            println!("\n💡 常见错误解决方案:");
            println!("   1. 检查应用是否有审批数据访问权限");
            println!("   2. 确认应用访问令牌权限");
            println!("   3. 验证查询参数格式是否正确");
            return Err(e.into());
        }
    }

    Ok(())
}

/// 演示创建请假申请
async fn create_leave_request(client: &LarkClient) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n📝 演示创建请假申请...");

    let user_id = std::env::var("USER_ID").unwrap_or_else(|_| "ou_example_user_id".to_string());

    // 设置请假时间（明天请假一天）
    let start_date = chrono::Utc::now().date_naive() + chrono::Duration::days(1);
    let end_date = start_date + chrono::Duration::days(1);

    println!("   申请人: {user_id}");
    println!("   请假时间: {start_date} 到 {end_date}");
    println!("   请假类型: 年假");
    println!("   请假原因: 家庭事务处理");

    // 注意：这里演示的是审批处理结构，实际创建请假申请可能需要不同的API
    println!("💡 请假申请通常通过飞书客户端或企业内部系统提交");
    println!("   这里演示的是审批处理流程，而非申请创建流程");

    // 演示审批处理请求结构（通过已存在的审批ID）
    let approval_id = format!("demo_approval_{user_id}");
    let request = open_lark::service::attendance::v1::models::CreateUserApprovalRequest {
        api_req: Default::default(),
        employee_type: "employee_id".to_string(),
        approval_id: approval_id.clone(),
        status: 2, // 2 = 已通过
        approval_note: Some("同意请假申请".to_string()),
    };

    match client
        .attendance
        .v1
        .user_approval
        .create(request, None)
        .await
    {
        Ok(response) => {
            if let Some(data) = &response.data {
                println!("✅ 请假申请创建成功!");
                println!("   申请ID: {}", data.approval_id);
                println!("   当前状态: 待审批");
                println!(
                    "   提交时间: {}",
                    chrono::Utc::now().format("%Y-%m-%d %H:%M:%S")
                );

                println!("\n💡 申请已提交，等待管理员审批");
            } else {
                println!("⚠️ 创建请求成功，但未返回申请信息");
            }
        }
        Err(e) => {
            println!("❌ 创建请假申请失败: {e:?}");
            println!("\n💡 这是演示模式，实际的创建请假申请可能需要:");
            println!("   1. 不同的API端点");
            println!("   2. 特定的权限配置");
            println!("   3. 企业内部的审批流程设置");
            println!("   4. 用户身份验证");
        }
    }

    Ok(())
}

/// 演示审批处理流程
async fn demonstrate_approval_process(
    client: &LarkClient,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n⚖️ 演示审批处理流程...");

    // 模拟一个审批ID
    let approval_id = "example_approval_123456";
    let approval_action = "approve"; // approve 或 reject
    let approval_comment = "同意请假申请，注意工作交接";

    println!("   审批ID: {approval_id}");
    println!(
        "   审批动作: {}",
        if approval_action == "approve" {
            "通过"
        } else {
            "拒绝"
        }
    );
    println!("   审批意见: {approval_comment}");

    // 演示处理审批请求
    let action_code = match approval_action {
        "approve" => 1, // 1 = 审批通过
        "reject" => 2,  // 2 = 审批拒绝
        _ => 1,
    };

    let request = open_lark::service::attendance::v1::models::ProcessUserApprovalRequest {
        api_req: Default::default(),
        employee_type: "employee_id".to_string(),
        approval_id: approval_id.to_string(),
        action: action_code,
        message: Some(approval_comment.to_string()),
    };

    match client
        .attendance
        .v1
        .user_approval
        .process(request, None)
        .await
    {
        Ok(response) => {
            if let Some(_data) = &response.data {
                println!("✅ 审批处理成功!");
                println!(
                    "   处理结果: 审批已{}",
                    if approval_action == "approve" {
                        "通过"
                    } else {
                        "拒绝"
                    }
                );
                println!(
                    "   处理时间: {}",
                    chrono::Utc::now().format("%Y-%m-%d %H:%M:%S")
                );

                if approval_action == "approve" {
                    println!("\n📧 系统将自动通知申请人审批通过");
                    println!("📅 请假时间将被记录到考勤系统");
                } else {
                    println!("\n📧 系统将自动通知申请人审批被拒绝");
                    println!("💬 拒绝原因已发送给申请人");
                }
            } else {
                println!("⚠️ 处理请求成功，但未返回处理结果");
            }
        }
        Err(e) => {
            println!("❌ 审批处理失败: {e:?}");
            println!("\n💡 这是演示模式，实际的审批处理需要:");
            println!("   1. 有效的审批ID");
            println!("   2. 审批权限");
            println!("   3. 正确的审批状态");
            println!("   4. 符合企业审批流程的操作");
        }
    }

    Ok(())
}

/// 展示审批统计信息
#[allow(dead_code)]
async fn show_approval_statistics(client: &LarkClient) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n📊 审批统计信息...");

    // 查询最近30天的审批统计
    let end_date = chrono::Utc::now().date_naive();
    let start_date = end_date - chrono::Duration::days(30);

    let mut total_approvals = 0;
    let mut pending_count = 0;
    let mut approved_count = 0;
    let mut rejected_count = 0;

    // 分别查询不同状态的审批
    for status in [1, 2, 3] {
        // 1=审批中, 2=通过, 3=拒绝
        let request = open_lark::service::attendance::v1::models::QueryUserApprovalRequest {
            api_req: Default::default(),
            employee_type: "employee_id".to_string(),
            status: Some(status),
            date_from: Some(start_date.format("%Y-%m-%d").to_string()),
            date_to: Some(end_date.format("%Y-%m-%d").to_string()),
            user_ids: None,
            page_token: None,
            page_size: Some(100),
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
                    let count = data.approvals.len();
                    total_approvals += count;

                    match status {
                        1 => pending_count = count,
                        2 => approved_count = count,
                        3 => rejected_count = count,
                        _ => {}
                    }
                }
            }
            Err(_) => {
                // 忽略错误，继续统计
            }
        }
    }

    println!("   统计时间范围: {start_date} 到 {end_date}");
    println!("   总申请数: {total_approvals}");
    println!("   待审批: {pending_count}");
    println!("   已通过: {approved_count}");
    println!("   已拒绝: {rejected_count}");

    if total_approvals > 0 {
        let approval_rate =
            (approved_count as f64 / (approved_count + rejected_count) as f64) * 100.0;
        println!("   通过率: {approval_rate:.1}%");
    }

    Ok(())
}

/// 演示批量审批处理（供参考）
#[allow(dead_code)]
async fn batch_approval_processing(client: &LarkClient) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n🔄 批量审批处理演示...");

    println!("   💡 批量审批功能包括:");
    println!("     - 批量查询待审批项目");
    println!("     - 按条件筛选审批项目");
    println!("     - 批量执行审批操作");
    println!("     - 生成审批处理报告");
    println!("     - 发送批量通知");

    // 示例：查询所有待审批的年假申请
    println!("\n📋 查询所有待审批的年假申请...");

    let request = open_lark::service::attendance::v1::models::QueryUserApprovalRequest {
        api_req: Default::default(),
        employee_type: "employee_id".to_string(),
        status: Some(1), // 审批中
        date_from: None,
        date_to: None,
        user_ids: None,
        page_token: None,
        page_size: Some(50),
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
                println!("   待审批申请: {}个", data.approvals.len());

                // 这里可以实现批量处理逻辑
                for approval in &data.approvals {
                    println!(
                        "     - {}: {} ({} 到 {})",
                        approval.user_id,
                        approval.approval_id,
                        approval.start_time,
                        approval.end_time
                    );
                }
            }
        }
        Err(e) => {
            println!("   查询失败: {e:?}");
        }
    }

    Ok(())
}

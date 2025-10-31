//! EHR Leave Management Demo
//!
//! 演示如何使用open-lark SDK的请假管理功能
//! 展示请假申请、审批、余额查询等核心功能的使用方法

use open_lark::prelude::*;
use open_lark::service::ehr::v1::leave::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 初始化客户端
    let client = LarkClient::builder()
        .app_id("your_app_id")
        .app_secret("your_app_secret")
        .build()?;

    println!("=== EHR请假管理演示 ===");

    // ==================== 创建请假申请 ====================
    println!("\n1. 创建年假申请");

    let leave_response = client.ehr.v1.leave
        .create_leave_builder()
        .employee_id("emp_001")
        .leave_type(LeaveType::Annual)
        .start_time("2024-02-15T09:00:00Z")
        .end_time("2024-02-17T18:00:00Z")
        .reason("家庭聚会，需要回老家")
        .remarks("会提前安排好工作交接")
        .execute(&client.ehr.v1.leave)
        .await?;

    println!("✅ 请假申请创建成功");
    println!("   请假ID: {}", leave_response.data.leave_id);
    println!("   请假类型: {:?}", leave_response.data.leave_type);
    println!("   请假天数: {}", leave_response.data.leave_days);
    println!("   状态: {:?}", leave_response.data.status);

    let leave_id = leave_response.data.leave_id.clone();

    // ==================== 查询请假记录 ====================
    println!("\n2. 查询员工请假记录");

    let records_response = client.ehr.v1.leave
        .query_leave_records_builder()
        .employee_id("emp_001")
        .status(LeaveStatus::Approved)
        .start_date("2024-01-01")
        .end_date("2024-12-31")
        .page_size(20)
        .execute(&client.ehr.v1.leave)
        .await?;

    println!("✅ 找到 {} 条请假记录", records_response.data.items.len());
    for (i, record) in records_response.data.items.iter().enumerate() {
        println!("   {}. {} - {}天 ({})",
            i + 1,
            format_leave_type(&record.leave_type),
            record.leave_days,
            format_leave_status(&record.status)
        );
    }

    // ==================== 查询请假余额 ====================
    println!("\n3. 查询年假余额");

    let balance_response = client.ehr.v1.leave
        .query_leave_balance_builder("emp_001")
        .leave_type(LeaveType::Annual)
        .year(2024)
        .execute(&client.ehr.v1.leave)
        .await?;

    if let Some(balance) = balance_response.data.items.first() {
        println!("✅ 年假余额信息");
        println!("   总额度: {} 天", balance.total_days);
        println!("   已使用: {} 天", balance.used_days);
        println!("   剩余: {} 天", balance.remaining_days);
    }

    // ==================== 请假审批（管理员视角） ====================
    println!("\n4. 请假审批演示");

    let approval_response = client.ehr.v1.leave
        .approve_leave_builder(&leave_id)
        .decision(LeaveApprovalDecision::Approve)
        .comment("同意请假，请注意安全并安排好工作交接")
        .execute(&client.ehr.v1.leave)
        .await?;

    println!("✅ 请假审批完成");
    println!("   审批人ID: {}", approval_response.data.approver_id);
    println!("   审批结果: {:?}", approval_response.data.decision);
    println!("   审批意见: {:?}", approval_response.data.comment);

    // ==================== 获取请假统计 ====================
    println!("\n5. 获取员工请假统计");

    let stats_response = client.ehr.v1.leave
        .get_leave_statistics_builder()
        .employee_id("emp_001")
        .year(2024)
        .execute(&client.ehr.v1.leave)
        .await?;

    println!("✅ 2024年请假统计");
    println!("   总请假天数: {}", stats_response.data.total_leave_days);
    println!("   总请假次数: {}", stats_response.data.total_leave_count);

    if let Some(type_stats) = &stats_response.data.leave_type_stats {
        println!("   按类型统计:");
        for stat in type_stats {
            println!("     {}: {}天 ({}次)",
                format_leave_type(&stat.leave_type),
                stat.leave_days,
                stat.leave_count
            );
        }
    }

    // ==================== 创建请假规则 ====================
    println!("\n6. 创建请假规则（管理员功能）");

    let rule_response = client.ehr.v1.leave
        .create_leave_rule_builder()
        .leave_type(LeaveType::Annual)
        .name("标准年假规则")
        .description("公司标准年假管理规则，适用于所有正式员工")
        .requires_approval(true)
        .max_leave_days(15.0)
        .allow_partial_days(true)
        .advance_notice_days(3)
        .add_applicable_department("dept_001")
        .add_applicable_department("dept_002")
        .execute(&client.ehr.v1.leave)
        .await?;

    println!("✅ 请假规则创建成功");
    println!("   规则ID: {}", rule_response.data.rule_id);
    println!("   规则名称: {}", rule_response.data.name);
    println!("   需要审批: {}", rule_response.data.requires_approval);
    println!("   最大请假天数: {}", rule_response.data.max_leave_days.unwrap_or(0.0));

    // ==================== 获取待审批列表 ====================
    println!("\n7. 获取待审批列表");

    let pending_response = client.ehr.v1.leave
        .get_pending_approvals_builder()
        .page_size(10)
        .execute(&client.ehr.v1.leave)
        .await?;

    println!("✅ 找到 {} 个待审批申请", pending_response.data.items.len());
    for (i, record) in pending_response.data.items.iter().enumerate() {
        println!("   {}. {} - {} ({})",
            i + 1,
            record.employee_id,
            format_leave_type(&record.leave_type),
            format_leave_status(&record.status)
        );
    }

    // ==================== 调整请假余额 ====================
    println!("\n8. 调整员工请假余额（HR功能）");

    let adjust_response = client.ehr.v1.leave
        .adjust_leave_balance_builder("emp_001")
        .leave_type(LeaveType::Annual)
        .year(2024)
        .adjustment_days(2.0)
        .reason("年度福利调整，额外增加2天年假")
        .execute(&client.ehr.v1.leave)
        .await?;

    println!("✅ 请假余额调整完成");
    println!("   调整后余额: {} 天", adjust_response.data.remaining_days);

    // ==================== 获取请假规则列表 ====================
    println!("\n9. 获取请假规则列表");

    let rules_response = client.ehr.v1.leave
        .get_leave_rules_builder()
        .leave_type(LeaveType::Annual)
        .page_size(20)
        .execute(&client.ehr.v1.leave)
        .await?;

    println!("✅ 找到 {} 条年假规则", rules_response.data.items.len());
    for (i, rule) in rules_response.data.items.iter().enumerate() {
        println!("   {}. {} - 需要: {}, 最大: {}天",
            i + 1,
            rule.name,
            if rule.requires_approval { "是" } else { "否" },
            rule.max_leave_days.unwrap_or(0.0)
        );
    }

    println!("\n=== EHR请假管理演示完成 ===");
    println!("本演示展示了请假管理的核心功能:");
    println!("✓ 请假申请创建和管理");
    println!("✓ 请假记录查询和筛选");
    println!("✓ 请假余额查询和统计");
    println!("✓ 请假审批流程");
    println!("✓ 请假规则配置");
    println!("✓ 请假统计分析");
    println!("✓ 余额调整管理");

    Ok(())
}

/// 格式化请假类型显示
fn format_leave_type(leave_type: &LeaveType) -> &'static str {
    match leave_type {
        LeaveType::Annual => "年假",
        LeaveType::Sick => "病假",
        LeaveType::Personal => "事假",
        LeaveType::Marriage => "婚假",
        LeaveType::Maternity => "产假",
        LeaveType::Paternity => "陪产假",
        LeaveType::Bereavement => "丧假",
        LeaveType::Compensatory => "调休",
        LeaveType::Prenatal => "产检假",
        LeaveType::Lactation => "哺乳假",
        LeaveType::Other => "其他",
    }
}

/// 格式化请假状态显示
fn format_leave_status(status: &LeaveStatus) -> &'static str {
    match status {
        LeaveStatus::Draft => "草稿",
        LeaveStatus::PendingApproval => "审批中",
        LeaveStatus::Approved => "已批准",
        LeaveStatus::Rejected => "已拒绝",
        LeaveStatus::Cancelled => "已撤销",
        LeaveStatus::InProgress => "进行中",
        LeaveStatus::Completed => "已完成",
    }
}
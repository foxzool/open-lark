#![allow(clippy::field_reassign_with_default)]

use dotenvy::dotenv;
use open_lark::{prelude::LarkClient, service::attendance::v1::models::ProcessUserApprovalRequest};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let app_id = env::var("APP_ID").expect("APP_ID must be set");
    let app_secret = env::var("APP_SECRET").expect("APP_SECRET must be set");

    let client = LarkClient::builder(&app_id, &app_secret).build();

    // 构建通知审批状态更新请求
    let mut req = ProcessUserApprovalRequest::default();
    req.employee_type = "employee_id".to_string();
    req.approval_id = "approval_123456789".to_string(); // 审批ID
    req.action = 1; // 1：审批通过，2：审批拒绝，3：撤回申请
    req.message = Some("您的请假申请已经通过审批，请合理安排工作交接。".to_string());

    println!("发送通知审批状态更新请求...");
    println!("审批ID: {}", req.approval_id);
    println!(
        "通知类型: {}",
        match req.action {
            1 => "审批通过",
            2 => "审批拒绝",
            3 => "撤回申请",
            _ => "未知",
        }
    );
    if let Some(message) = &req.message {
        println!("通知消息: {}", message);
    }

    match client.attendance.v1.user_approval.process(req, None).await {
        Ok(resp) => {
            println!("✅ 通知审批状态更新成功!");
            if let Some(data) = resp.data {
                println!("📤 通知发送结果:");
                println!("  通知状态: {}", if data.success { "成功" } else { "失败" });
                println!("  审批ID: {}", data.approval_id);
                println!("  💡 提示: 审批状态通知已发送给申请人");
            }
        }
        Err(e) => {
            eprintln!("❌ 通知审批状态更新失败: {:?}", e);
            eprintln!("💡 提示: 请检查审批ID和通知类型是否正确");
        }
    }

    Ok(())
}

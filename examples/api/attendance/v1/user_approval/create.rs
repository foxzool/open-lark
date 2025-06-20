#![allow(clippy::field_reassign_with_default)]

use dotenvy::dotenv;
use open_lark::{prelude::LarkClient, service::attendance::v1::models::CreateUserApprovalRequest};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let app_id = env::var("APP_ID").expect("APP_ID must be set");
    let app_secret = env::var("APP_SECRET").expect("APP_SECRET must be set");

    let client = LarkClient::builder(&app_id, &app_secret).build();

    // 构建写入审批结果请求
    let mut req = CreateUserApprovalRequest::default();
    req.employee_type = "employee_id".to_string();
    req.approval_id = "approval_123456789".to_string(); // 审批ID（从查询接口获得）
    req.status = 2; // 2：已通过，3：已拒绝
    req.approval_note = Some("经审核，该请假申请符合公司规定，予以批准。".to_string());

    println!("发送写入审批结果请求...");
    println!("审批ID: {}", req.approval_id);
    println!(
        "审批状态: {}",
        match req.status {
            2 => "已通过",
            3 => "已拒绝",
            _ => "未知",
        }
    );
    if let Some(note) = &req.approval_note {
        println!("审批备注: {}", note);
    }

    match client.attendance.v1.user_approval.create(req, None).await {
        Ok(resp) => {
            println!("✅ 写入审批结果成功!");
            if let Some(data) = resp.data {
                println!("📝 审批处理结果:");
                println!("  处理状态: {}", if data.success { "成功" } else { "失败" });
                println!("  审批ID: {}", data.approval_id);
                println!("  💡 提示: 审批结果已记录，可以使用通知接口告知申请人");
            }
        }
        Err(e) => {
            eprintln!("❌ 写入审批结果失败: {:?}", e);
            eprintln!("💡 提示: 请检查审批ID是否正确，审批状态是否合法");
        }
    }

    Ok(())
}

#![allow(clippy::field_reassign_with_default)]

use dotenvy::dotenv;
use open_lark::{prelude::LarkClient, service::attendance::v1::models::QueryUserApprovalRequest};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let app_id = env::var("APP_ID").expect("APP_ID must be set");
    let app_secret = env::var("APP_SECRET").expect("APP_SECRET must be set");

    let client = LarkClient::builder(&app_id, &app_secret).build();

    // 构建获取审批数据请求
    let mut req = QueryUserApprovalRequest::default();
    req.employee_type = "employee_id".to_string();
    req.status = Some(1); // 1：待审批
    req.date_from = Some("2024-06-01".to_string());
    req.date_to = Some("2024-06-30".to_string());
    req.user_ids = Some(vec![
        "employee_123".to_string(),
        "employee_456".to_string(),
        "employee_789".to_string(),
    ]);
    req.page_size = Some(50);

    println!("发送获取审批数据请求...");
    println!("审批状态: 待审批");
    println!(
        "查询时间: {} - {}",
        req.date_from.as_ref().unwrap(),
        req.date_to.as_ref().unwrap()
    );
    println!("用户数量: {}", req.user_ids.as_ref().unwrap().len());

    match client.attendance.v1.user_approval.query(req, None).await {
        Ok(resp) => {
            println!("✅ 获取审批数据成功!");
            if let Some(data) = resp.data {
                println!("📋 审批数据 (共{}条):", data.approvals.len());

                for (index, approval) in data.approvals.iter().enumerate() {
                    println!("  {}. 📝 审批信息:", index + 1);
                    println!("     审批ID: {}", approval.approval_id);
                    println!("     用户ID: {}", approval.user_id);

                    if let Some(user_name) = &approval.user_name {
                        println!("     用户姓名: {}", user_name);
                    }

                    let approval_type_name = match approval.approval_type {
                        1 => "请假",
                        2 => "出差",
                        3 => "外出",
                        4 => "加班",
                        5 => "调休",
                        _ => "未知",
                    };
                    println!(
                        "     审批类型: {} ({})",
                        approval.approval_type, approval_type_name
                    );

                    let status_name = match approval.status {
                        1 => "待审批",
                        2 => "已通过",
                        3 => "已拒绝",
                        _ => "未知",
                    };
                    println!("     审批状态: {} ({})", approval.status, status_name);

                    println!(
                        "     申请时间: {} - {}",
                        approval.start_time, approval.end_time
                    );

                    if let Some(duration) = approval.duration {
                        println!("     申请时长: {:.1} 小时", duration);
                    }

                    if let Some(reason) = &approval.reason {
                        println!("     申请理由: {}", reason);
                    }

                    if let Some(approval_note) = &approval.approval_note {
                        println!("     审批备注: {}", approval_note);
                    }

                    if let Some(created_at) = &approval.created_at {
                        println!("     提交时间: {}", created_at);
                    }

                    if let Some(approved_at) = &approval.approved_at {
                        println!("     审批时间: {}", approved_at);
                    }

                    println!("     ---");
                }

                // 分页信息
                println!("📄 分页信息:");
                println!("  是否有更多数据: {}", data.has_more);
                if let Some(page_token) = &data.page_token {
                    println!("  下一页令牌: {}", page_token);
                }

                println!("💡 提示: 可以使用写入审批结果接口来处理这些待审批的申请");
            }
        }
        Err(e) => {
            eprintln!("❌ 获取审批数据失败: {:?}", e);
            eprintln!("💡 提示: 请检查员工ID类型和查询条件是否正确");
        }
    }

    Ok(())
}

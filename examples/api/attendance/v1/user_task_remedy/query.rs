#![allow(clippy::field_reassign_with_default)]

use dotenvy::dotenv;
use open_lark::{prelude::LarkClient, service::attendance::v1::models::QueryUserTaskRemedyRequest};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let app_id = env::var("APP_ID").expect("APP_ID must be set");
    let app_secret = env::var("APP_SECRET").expect("APP_SECRET must be set");

    let client = LarkClient::builder(&app_id, &app_secret).build();

    // 构建获取补卡记录请求
    let mut req = QueryUserTaskRemedyRequest::default();
    req.employee_type = "employee_id".to_string();
    req.user_ids = Some(vec![
        "employee_123".to_string(),
        "employee_456".to_string(),
        "employee_789".to_string(),
    ]);
    req.date_from = Some("2024-06-01".to_string());
    req.date_to = Some("2024-06-30".to_string());
    req.status = Some(1); // 1：待审批，2：已通过，3：已拒绝
    req.page_size = Some(50);

    println!("发送获取补卡记录请求...");
    println!("查询用户: {:?}", req.user_ids.as_ref().unwrap());
    println!(
        "查询时间: {} - {}",
        req.date_from.as_ref().unwrap(),
        req.date_to.as_ref().unwrap()
    );
    println!(
        "筛选状态: {}",
        match req.status.unwrap() {
            1 => "待审批",
            2 => "已通过",
            3 => "已拒绝",
            _ => "全部",
        }
    );

    match client.attendance.v1.user_task_remedy.query(req, None).await {
        Ok(resp) => {
            println!("✅ 获取补卡记录成功!");
            if let Some(data) = resp.data {
                println!("📋 补卡记录 (共{}条):", data.remedys.len());

                for (index, remedy) in data.remedys.iter().enumerate() {
                    println!("  {}. 📝 补卡申请:", index + 1);
                    println!("     申请ID: {}", remedy.remedy_id);
                    println!("     用户ID: {}", remedy.user_id);

                    if let Some(user_name) = &remedy.user_name {
                        println!("     用户姓名: {}", user_name);
                    }

                    println!("     补卡日期: {}", remedy.remedy_date);
                    println!("     补卡时间: {}", remedy.remedy_time);

                    let remedy_type_name = match remedy.remedy_type {
                        1 => "上班补卡",
                        2 => "下班补卡",
                        _ => "未知",
                    };
                    println!(
                        "     补卡类型: {} ({})",
                        remedy.remedy_type, remedy_type_name
                    );

                    let status_name = match remedy.status {
                        1 => "待审批",
                        2 => "已通过",
                        3 => "已拒绝",
                        _ => "未知",
                    };
                    println!("     申请状态: {} ({})", remedy.status, status_name);

                    println!("     申请原因: {}", remedy.reason);

                    if let Some(comment) = &remedy.comment {
                        println!("     补充说明: {}", comment);
                    }

                    println!("     申请时间: {}", remedy.apply_time);

                    if let Some(approve_time) = &remedy.approve_time {
                        println!("     审批时间: {}", approve_time);
                    }

                    if let Some(approver_id) = &remedy.approver_id {
                        println!("     审批人ID: {}", approver_id);
                    }

                    if let Some(approve_comment) = &remedy.approve_comment {
                        println!("     审批备注: {}", approve_comment);
                    }

                    println!("     ---");
                }

                // 分页信息
                println!("📄 分页信息:");
                println!("  是否有更多数据: {}", data.has_more);
                if let Some(page_token) = &data.page_token {
                    println!("  下一页令牌: {}", page_token);
                }

                if data.remedys.is_empty() {
                    println!("  📝 无补卡记录");
                    println!("  💡 提示: 指定条件下没有找到补卡申请记录");
                } else {
                    println!("💡 提示: 可以使用通知补卡审批发起接口提交新的补卡申请");
                }
            }
        }
        Err(e) => {
            eprintln!("❌ 获取补卡记录失败: {:?}", e);
            eprintln!("💡 提示: 请检查查询条件和用户权限");
        }
    }

    Ok(())
}

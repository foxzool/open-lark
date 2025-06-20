#![allow(clippy::field_reassign_with_default)]

use dotenvy::dotenv;
use open_lark::{
    prelude::LarkClient,
    service::attendance::v1::models::{CreateUserTaskRemedyRequest, UserTaskRemedyApplication},
};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let app_id = env::var("APP_ID").expect("APP_ID must be set");
    let app_secret = env::var("APP_SECRET").expect("APP_SECRET must be set");

    let client = LarkClient::builder(&app_id, &app_secret).build();

    // 构建通知补卡审批发起请求
    let mut req = CreateUserTaskRemedyRequest::default();
    req.employee_type = "employee_id".to_string();
    req.remedy_application = UserTaskRemedyApplication {
        user_id: "employee_123".to_string(),
        remedy_date: "2024-06-20".to_string(),
        remedy_time: "09:30".to_string(),
        remedy_type: 1, // 1：上班补卡，2：下班补卡
        reason: "因交通拥堵迟到30分钟，申请补卡".to_string(),
        comment: Some("地铁故障导致延误，已提供证明材料".to_string()),
    };

    println!("发送通知补卡审批发起请求...");
    println!("申请用户: {}", req.remedy_application.user_id);
    println!("补卡日期: {}", req.remedy_application.remedy_date);
    println!("补卡时间: {}", req.remedy_application.remedy_time);
    println!(
        "补卡类型: {}",
        match req.remedy_application.remedy_type {
            1 => "上班补卡",
            2 => "下班补卡",
            _ => "未知",
        }
    );
    println!("申请原因: {}", req.remedy_application.reason);
    if let Some(comment) = &req.remedy_application.comment {
        println!("补充说明: {}", comment);
    }

    match client
        .attendance
        .v1
        .user_task_remedy
        .create(req, None)
        .await
    {
        Ok(resp) => {
            println!("✅ 补卡申请提交成功!");
            if let Some(data) = resp.data {
                println!("📝 申请结果:");
                println!("  补卡申请ID: {}", data.remedy_id);
                println!("  提交状态: {}", if data.success { "成功" } else { "失败" });
                if data.success {
                    println!("  💡 提示: 补卡申请已提交，请等待审批结果");
                    println!("  📋 下一步: 可以使用获取补卡记录接口查询审批进度");
                }
            }
        }
        Err(e) => {
            eprintln!("❌ 补卡申请提交失败: {:?}", e);
            eprintln!("💡 提示: 请检查用户ID、补卡时间格式等参数是否正确");
        }
    }

    Ok(())
}

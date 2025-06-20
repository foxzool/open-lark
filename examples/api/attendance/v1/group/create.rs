#![allow(clippy::field_reassign_with_default)]

use dotenvy::dotenv;
use open_lark::{
    prelude::LarkClient,
    service::attendance::v1::models::{CreateGroupRequest, MemberRule, WorkDayRule},
};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let app_id = env::var("APP_ID").expect("APP_ID must be set");
    let app_secret = env::var("APP_SECRET").expect("APP_SECRET must be set");

    let client = LarkClient::builder(&app_id, &app_secret).build();

    // 构建创建考勤组请求
    let mut req = CreateGroupRequest::default();
    req.employee_type = "employee_id".to_string();
    req.dept_type = Some("open_id".to_string());
    req.group_name = format!("API示例考勤组-{}", chrono::Utc::now().timestamp());
    req.time_zone = Some("Asia/Shanghai".to_string());
    req.attendance_type = Some(1); // 1-固定班制
    req.punch_type = Some(1); // 1-GPS打卡
    req.allow_late_minutes = Some(30); // 允许迟到30分钟
    req.allow_early_leave_minutes = Some(30); // 允许早退30分钟

    // 设置工作日规则（周一到周五使用同一个班次）
    req.work_day_rule = Some(vec![
        WorkDayRule {
            week_day: 1,                                 // 周一
            shift_id: "7517943152473964546".to_string(), // 需要先创建班次
        },
        WorkDayRule {
            week_day: 2, // 周二
            shift_id: "7517943152473964546".to_string(),
        },
        WorkDayRule {
            week_day: 3, // 周三
            shift_id: "7517943152473964546".to_string(),
        },
        WorkDayRule {
            week_day: 4, // 周四
            shift_id: "7517943152473964546".to_string(),
        },
        WorkDayRule {
            week_day: 5, // 周五
            shift_id: "7517943152473964546".to_string(),
        },
    ]);

    // 设置成员规则（添加用户）
    req.member_rule = Some(MemberRule {
        member_type: 2, // 2-用户
        member_ids: vec!["employee_123".to_string(), "employee_456".to_string()],
    });

    println!("发送创建考勤组请求...");
    println!("考勤组名称: {}", req.group_name);
    println!("时区: {:?}", req.time_zone);
    println!("考勤方式: {:?}", req.attendance_type);

    match client.attendance.v1.group.create(req, None).await {
        Ok(resp) => {
            println!("✅ 创建考勤组成功!");
            if let Some(data) = resp.data {
                let group = &data.group;
                println!("🏢 考勤组信息:");
                println!("  考勤组ID: {}", group.group_id);
                println!("  考勤组名称: {}", group.group_name);
                if let Some(time_zone) = &group.time_zone {
                    println!("  时区: {}", time_zone);
                }
                if let Some(attendance_type) = group.attendance_type {
                    println!("  考勤方式: {}", attendance_type);
                }
                if let Some(punch_type) = group.punch_type {
                    println!("  打卡方式: {}", punch_type);
                }
                if let Some(allow_late_minutes) = group.allow_late_minutes {
                    println!("  允许迟到时间: {}分钟", allow_late_minutes);
                }
                if let Some(work_day_rule) = &group.work_day_rule {
                    println!("  工作日设置: {} 条规则", work_day_rule.len());
                }
                if let Some(member_rule) = &group.member_rule {
                    println!("  成员设置: {} 个成员", member_rule.member_ids.len());
                }
                if let Some(create_time) = &group.create_time {
                    println!("  创建时间: {}", create_time);
                }
            }
        }
        Err(e) => {
            eprintln!("❌ 创建考勤组失败: {:?}", e);
        }
    }

    Ok(())
}

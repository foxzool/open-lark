#![allow(clippy::field_reassign_with_default)]

use dotenvy::dotenv;
use open_lark::{prelude::LarkClient, service::attendance::v1::models::ListGroupRequest};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let app_id = env::var("APP_ID").expect("APP_ID must be set");
    let app_secret = env::var("APP_SECRET").expect("APP_SECRET must be set");

    let client = LarkClient::builder(&app_id, &app_secret).build();

    // 构建列出考勤组请求
    let mut req = ListGroupRequest::default();
    req.employee_type = "employee_id".to_string();
    req.dept_type = Some("open_id".to_string());
    req.page_size = Some(20); // 每页20个

    println!("发送列出所有考勤组请求...");
    println!("每页数量: {:?}", req.page_size);

    match client.attendance.v1.group.list(req, None).await {
        Ok(resp) => {
            println!("✅ 列出考勤组成功!");
            if let Some(data) = resp.data {
                println!("找到 {} 个考勤组", data.group_list.len());
                println!("是否还有更多: {}", data.has_more);

                for (index, group) in data.group_list.iter().enumerate() {
                    println!("{}. 🏢 考勤组信息:", index + 1);
                    println!("   考勤组ID: {}", group.group_id);
                    println!("   考勤组名称: {}", group.group_name);
                    if let Some(time_zone) = &group.time_zone {
                        println!("   时区: {}", time_zone);
                    }
                    if let Some(attendance_type) = group.attendance_type {
                        let type_name = match attendance_type {
                            1 => "固定班制",
                            2 => "排班制",
                            3 => "自由班制",
                            _ => "未知类型",
                        };
                        println!("   考勤方式: {}", type_name);
                    }
                    if let Some(work_day_rule) = &group.work_day_rule {
                        println!("   工作日规则: {} 条", work_day_rule.len());
                    }
                    if let Some(member_rule) = &group.member_rule {
                        let member_type_name = match member_rule.member_type {
                            1 => "部门",
                            2 => "用户",
                            _ => "未知",
                        };
                        println!(
                            "   成员: {} 个{}",
                            member_rule.member_ids.len(),
                            member_type_name
                        );
                    }
                    if let Some(create_time) = &group.create_time {
                        println!("   创建时间: {}", create_time);
                    }
                    println!();
                }

                if let Some(page_token) = &data.page_token {
                    println!("分页标记: {}", page_token);
                }
            }
        }
        Err(e) => {
            eprintln!("❌ 列出考勤组失败: {:?}", e);
        }
    }

    Ok(())
}

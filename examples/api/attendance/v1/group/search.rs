#![allow(clippy::field_reassign_with_default)]

use dotenvy::dotenv;
use open_lark::{prelude::LarkClient, service::attendance::v1::models::SearchGroupRequest};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let app_id = env::var("APP_ID").expect("APP_ID must be set");
    let app_secret = env::var("APP_SECRET").expect("APP_SECRET must be set");

    let client = LarkClient::builder(&app_id, &app_secret).build();

    // 构建搜索考勤组请求
    let mut req = SearchGroupRequest::default();
    req.employee_type = "employee_id".to_string();
    req.dept_type = Some("open_id".to_string());
    req.group_name = "默认考勤组".to_string(); // 要搜索的考勤组名称

    println!("发送搜索考勤组请求...");
    println!("搜索名称: {}", req.group_name);

    match client.attendance.v1.group.search(req, None).await {
        Ok(resp) => {
            println!("✅ 搜索考勤组成功!");
            if let Some(data) = resp.data {
                println!("找到 {} 个匹配的考勤组", data.group_list.len());

                for group in &data.group_list {
                    println!("🏢 考勤组信息:");
                    println!("  考勤组ID: {}", group.group_id);
                    println!("  考勤组名称: {}", group.group_name);
                    if let Some(time_zone) = &group.time_zone {
                        println!("  时区: {}", time_zone);
                    }
                    if let Some(attendance_type) = group.attendance_type {
                        let type_name = match attendance_type {
                            1 => "固定班制",
                            2 => "排班制",
                            3 => "自由班制",
                            _ => "未知类型",
                        };
                        println!("  考勤方式: {} ({})", attendance_type, type_name);
                    }
                    if let Some(work_day_rule) = &group.work_day_rule {
                        println!("  工作日规则: {} 条", work_day_rule.len());
                    }
                    if let Some(member_rule) = &group.member_rule {
                        println!("  成员数量: {} 个", member_rule.member_ids.len());
                    }
                    if let Some(create_time) = &group.create_time {
                        println!("  创建时间: {}", create_time);
                    }
                    println!("  ---");
                }
            }
        }
        Err(e) => {
            eprintln!("❌ 搜索考勤组失败: {:?}", e);
        }
    }

    Ok(())
}

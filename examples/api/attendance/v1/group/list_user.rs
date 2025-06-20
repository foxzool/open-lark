#![allow(clippy::field_reassign_with_default)]

use dotenvy::dotenv;
use open_lark::{prelude::LarkClient, service::attendance::v1::models::ListGroupUserRequest};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let app_id = env::var("APP_ID").expect("APP_ID must be set");
    let app_secret = env::var("APP_SECRET").expect("APP_SECRET must be set");

    let client = LarkClient::builder(&app_id, &app_secret).build();

    // 构建查询考勤组成员请求
    let mut req = ListGroupUserRequest::default();
    req.group_id = "7373168808276312066".to_string(); // 考勤组ID
    req.employee_type = "employee_id".to_string();
    req.dept_type = Some("open_id".to_string());
    req.page_size = Some(10);

    println!("发送查询考勤组成员请求...");
    println!("考勤组ID: {}", req.group_id);
    println!("员工ID类型: {}", req.employee_type);

    match client.attendance.v1.group.list_user(req, None).await {
        Ok(resp) => {
            println!("✅ 查询考勤组成员成功!");
            if let Some(data) = resp.data {
                println!("找到 {} 个成员", data.user_list.len());
                println!("是否还有更多: {}", data.has_more);

                for user in &data.user_list {
                    println!("👤 成员信息:");
                    println!("  用户ID: {}", user.user_id);
                    if let Some(user_name) = &user.user_name {
                        println!("  用户姓名: {}", user_name);
                    }
                    if let Some(employee_no) = &user.employee_no {
                        println!("  员工工号: {}", employee_no);
                    }
                    if let Some(department_id) = &user.department_id {
                        println!("  部门ID: {}", department_id);
                    }
                    if let Some(join_time) = &user.join_time {
                        println!("  加入时间: {}", join_time);
                    }
                    println!("  ---");
                }

                if let Some(page_token) = &data.page_token {
                    println!("分页标记: {}", page_token);
                }
            }
        }
        Err(e) => {
            eprintln!("❌ 查询考勤组成员失败: {:?}", e);
        }
    }

    Ok(())
}

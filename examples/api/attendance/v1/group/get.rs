use dotenvy::dotenv;
use open_lark::{prelude::LarkClient, service::attendance::v1::models::GetGroupRequest};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let app_id = env::var("APP_ID").expect("APP_ID must be set");
    let app_secret = env::var("APP_SECRET").expect("APP_SECRET must be set");

    let client = LarkClient::builder(&app_id, &app_secret).build();

    // 构建查询考勤组请求
    let mut req = GetGroupRequest::default();
    req.group_id = "7373168808276312066".to_string(); // 考勤组ID
    req.employee_type = "employee_id".to_string();
    req.dept_type = Some("open_id".to_string());

    println!("发送查询考勤组请求...");
    println!("考勤组ID: {}", req.group_id);

    match client.attendance.v1.group.get(req, None).await {
        Ok(resp) => {
            println!("✅ 查询考勤组成功!");
            if let Some(group) = resp.data {
                println!("🏢 考勤组详细信息:");
                println!("  考勤组ID: {}", group.group_id);
                println!("  考勤组名称: {}", group.group_name);
                if let Some(time_zone) = &group.time_zone {
                    println!("  时区: {}", time_zone);
                }
                if let Some(bind_dept_ids) = &group.bind_dept_ids {
                    println!("  绑定部门: {} 个", bind_dept_ids.len());
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
                if let Some(punch_type) = group.punch_type {
                    let punch_name = match punch_type {
                        1 => "GPS打卡",
                        2 => "Wi-Fi打卡",
                        3 => "考勤机打卡",
                        4 => "人脸识别打卡",
                        _ => "未知方式",
                    };
                    println!("  打卡方式: {} ({})", punch_type, punch_name);
                }
                if let Some(allow_late_minutes) = group.allow_late_minutes {
                    println!("  允许迟到: {}分钟", allow_late_minutes);
                }
                if let Some(allow_early_leave_minutes) = group.allow_early_leave_minutes {
                    println!("  允许早退: {}分钟", allow_early_leave_minutes);
                }
                if let Some(work_day_rule) = &group.work_day_rule {
                    println!("  工作日规则: {} 条", work_day_rule.len());
                    for rule in work_day_rule {
                        let day_name = match rule.week_day {
                            1 => "周一",
                            2 => "周二",
                            3 => "周三",
                            4 => "周四",
                            5 => "周五",
                            6 => "周六",
                            7 => "周日",
                            _ => "未知",
                        };
                        println!("    {}: 班次ID {}", day_name, rule.shift_id);
                    }
                }
                if let Some(member_rule) = &group.member_rule {
                    let member_type_name = match member_rule.member_type {
                        1 => "部门",
                        2 => "用户",
                        _ => "未知类型",
                    };
                    println!(
                        "  成员规则: {} ({} 个)",
                        member_type_name,
                        member_rule.member_ids.len()
                    );
                }
                if let Some(create_time) = &group.create_time {
                    println!("  创建时间: {}", create_time);
                }
                if let Some(update_time) = &group.update_time {
                    println!("  更新时间: {}", update_time);
                }
            }
        }
        Err(e) => {
            eprintln!("❌ 查询考勤组失败: {:?}", e);
        }
    }

    Ok(())
}

//! Attendance模块使用示例
//!
//! 演示如何使用attendance模块进行考勤管理操作，包括：
//! - 用户打卡任务查询
//! - 班次信息管理
//! - 考勤统计数据获取

use open_lark::prelude::*;
use open_lark::service::attendance::v1::{
    GetShiftRequest, GetUserStatsRequest, GetUserTaskRequest, ListShiftsRequest,
    QueryUserTasksRequest,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 初始化日志
    env_logger::init();

    println!("🚀 Attendance模块示例演示");

    // 创建客户端（这里使用测试配置）
    let client = LarkClient::builder("test_app_id", "test_app_secret")
        .with_app_type(AppType::SelfBuild)
        .build();

    println!("✅ 客户端创建成功");

    // 演示获取用户打卡任务
    println!("\n📋 获取用户打卡任务");
    let get_task_request = GetUserTaskRequest {
        user_id: "user_12345".to_string(),
        task_id: "task_12345".to_string(),
        user_id_type: Some("open_id".to_string()),
    };

    match client.attendance.v1.get_user_task(&get_task_request).await {
        Ok(response) => {
            println!("✅ 用户打卡任务获取成功");
            if let Some(data) = response.data {
                println!("   任务ID: {}", data.task_id);
                println!("   用户ID: {}", data.user_id);
                println!("   打卡日期: {}", data.check_date);
                println!("   班次名称: {}", data.shift_name);
                println!(
                    "   上班时间: {:?}",
                    data.check_in_time
                        .clone()
                        .unwrap_or_else(|| "未打卡".to_string())
                );
                println!(
                    "   下班时间: {:?}",
                    data.check_out_time
                        .clone()
                        .unwrap_or_else(|| "未打卡".to_string())
                );
                println!(
                    "   工作时长: {}",
                    data.work_hours.clone().unwrap_or_else(|| "0".to_string())
                );
                println!("   考勤状态: {:?}", data.status);
                if let Some(location) = data.location {
                    println!("   打卡位置: {}", location);
                }
            }
        }
        Err(e) => {
            println!("❌ 用户打卡任务获取失败: {}", e);
        }
    }

    // 演示查询用户打卡任务列表
    println!("\n📋 查询用户打卡任务列表");
    let query_tasks_request = QueryUserTasksRequest {
        user_id: "user_12345".to_string(),
        check_date_from: Some("2024-01-01".to_string()),
        check_date_to: Some("2024-01-07".to_string()),
        check_type: Some("all".to_string()),
        page_size: Some(10),
        page_token: None,
        user_id_type: Some("open_id".to_string()),
    };

    match client
        .attendance
        .v1
        .query_user_tasks(&query_tasks_request)
        .await
    {
        Ok(response) => {
            println!("✅ 用户打卡任务列表查询成功");
            if let Some(data) = response.data {
                println!("   找到 {} 个打卡任务:", data.tasks.len());
                println!("   总数: {}", data.total);
                println!("   是否有更多: {}", data.has_more);

                for (i, task) in data.tasks.iter().enumerate() {
                    println!("\n   {}. {}", i + 1, task.shift_name);
                    println!("      日期: {}", task.check_date);
                    println!(
                        "      上班: {:?}",
                        task.check_in_time
                            .clone()
                            .unwrap_or_else(|| "未打卡".to_string())
                    );
                    println!(
                        "      下班: {:?}",
                        task.check_out_time
                            .clone()
                            .unwrap_or_else(|| "未打卡".to_string())
                    );
                    println!("      状态: {:?}", task.status);
                    if let Some(work_hours) = &task.work_hours {
                        println!("      工时: {} 小时", work_hours);
                    }
                }
            }
        }
        Err(e) => {
            println!("❌ 用户打卡任务列表查询失败: {}", e);
        }
    }

    // 演示获取班次信息
    println!("\n📋 获取班次信息");
    let get_shift_request = GetShiftRequest {
        shift_id: "shift_001".to_string(),
    };

    match client.attendance.v1.get_shift(&get_shift_request).await {
        Ok(response) => {
            println!("✅ 班次信息获取成功");
            if let Some(data) = response.data {
                println!("   班次ID: {}", data.shift_id);
                println!("   班次名称: {}", data.shift_name);
                println!("   打卡次数: {}", data.punch_times);
                println!("   是否弹性打卡: {:?}", data.is_flexible.unwrap_or(false));
                if let Some(flexible_minutes) = data.flexible_minutes {
                    println!("   弹性时间: {} 分钟", flexible_minutes);
                }
                if let Some(allow_outside) = data.allow_outside_apply {
                    println!("   允许外勤: {}", allow_outside);
                }
                if let Some(outside_limit) = data.outside_apply_limit {
                    println!("   外勤限制: {} 次", outside_limit);
                }
                if let Some(punch_rules) = data.punch_time_rule {
                    for (i, rule) in punch_rules.iter().enumerate() {
                        println!("\n   打卡规则 {}:", i + 1);
                        println!("      上班时间: {}", rule.on_time);
                        println!("      下班时间: {}", rule.off_time);
                        println!("      提前打卡: {} 分钟", rule.on_advance_minutes);
                        println!("      延迟打卡: {} 分钟", rule.off_delay_minutes);
                        if let Some(late_as_late) = rule.late_minutes_as_late {
                            println!("      迟到时间: {} 分钟", late_as_late);
                        }
                        if let Some(early_as_early) = rule.early_minutes_as_early {
                            println!("      早到时间: {} 分钟", early_as_early);
                        }
                    }
                }
            }
        }
        Err(e) => {
            println!("❌ 班次信息获取失败: {}", e);
        }
    }

    // 演示获取班次列表
    println!("\n📋 获取班次列表");
    let list_shifts_request = ListShiftsRequest {
        page_size: Some(10),
        page_token: None,
    };

    match client.attendance.v1.list_shifts(&list_shifts_request).await {
        Ok(response) => {
            println!("✅ 班次列表获取成功");
            if let Some(data) = response.data {
                println!("   找到 {} 个班次:", data.shifts.len());
                println!("   总数: {}", data.total);
                println!("   是否有更多: {}", data.has_more);

                for (i, shift) in data.shifts.iter().enumerate() {
                    println!("\n   {}. {}", i + 1, shift.shift_name);
                    println!("      班次ID: {}", shift.shift_id);
                    println!("      打卡次数: {}", shift.punch_times);
                    println!("      弹性班次: {:?}", shift.is_flexible.unwrap_or(false));
                    if let Some(outside_apply) = shift.allow_outside_apply {
                        println!("      允许外勤: {}", outside_apply);
                    }
                }
            }
        }
        Err(e) => {
            println!("❌ 班次列表获取失败: {}", e);
        }
    }

    // 演示获取用户考勤统计
    println!("\n📊 获取用户考勤统计");
    let get_stats_request = GetUserStatsRequest {
        user_id: "user_12345".to_string(),
        stats_period: "2024-01".to_string(),
        user_id_type: Some("open_id".to_string()),
    };

    match client
        .attendance
        .v1
        .get_user_stats(&get_stats_request)
        .await
    {
        Ok(response) => {
            println!("✅ 用户考勤统计获取成功");
            if let Some(data) = response.data {
                println!("   用户ID: {}", data.user_id);
                println!("   统计周期: {}", data.stats_period);
                println!("   总工作日: {} 天", data.total_work_days);
                println!("   实际工作日: {} 天", data.actual_work_days);
                println!("   请假天数: {} 天", data.leave_days);
                println!("   迟到天数: {} 天", data.late_days);
                println!("   早退天数: {} 天", data.early_leave_days);
                println!("   缺勤天数: {} 天", data.absent_days);
                if let Some(overtime_hours) = data.overtime_hours {
                    println!("   加班时长: {} 小时", overtime_hours);
                }
                if let Some(average_hours) = data.average_work_hours {
                    println!("   平均工时: {} 小时", average_hours);
                }
                if let Some(total_hours) = data.total_work_hours {
                    println!("   总工作时长: {} 小时", total_hours);
                }
            }
        }
        Err(e) => {
            println!("❌ 用户考勤统计获取失败: {}", e);
        }
    }

    println!("\n🎉 Attendance模块示例演示完成！");
    Ok(())
}

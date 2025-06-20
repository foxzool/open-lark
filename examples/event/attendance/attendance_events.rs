use dotenvy::dotenv;
use open_lark::{
    event::dispatcher::EventDispatcherHandler,
    service::attendance::v1::{
        p2_attendance_user_task_status_change_v1::P2AttendanceUserTaskStatusChangeV1,
        p2_attendance_user_task_updated_v1::P2AttendanceUserTaskUpdatedV1,
    },
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    // 初始化日志
    env_logger::init();

    println!("🚀 启动考勤事件处理器示例...");

    // 创建事件分发处理器
    let handler = EventDispatcherHandler::builder()
        // 注册考勤打卡流水事件处理器
        .register_p2_attendance_user_task_updated_v1(|event: P2AttendanceUserTaskUpdatedV1| {
            println!("📋 收到考勤打卡流水事件:");
            println!("  事件ID: {:?}", event.header.event_id);
            println!("  事件类型: {:?}", event.header.event_type);
            println!("  租户Key: {}", event.event.tenant_key);

            // 打印用户信息
            let user = &event.event.user_id;
            println!("  👤 用户信息:");
            println!("    用户ID: {}", user.user_id);
            println!("    Open ID: {}", user.open_id);
            println!("    Union ID: {}", user.union_id);
            if let Some(employee_id) = &user.employee_id {
                println!("    员工ID: {}", employee_id);
            }

            // 打印任务信息
            let task = &event.event.task;
            println!("  📝 打卡任务信息:");
            println!("    任务ID: {}", task.task_id);
            println!("    考勤组ID: {}", task.group_id);
            println!("    班次ID: {} ({})", task.shift_id, task.shift_name);
            println!("    记录日期: {}", task.record_date);
            println!("    打卡时间: {}", task.check_time);

            let type_name = match task.type_ {
                1 => "上班打卡",
                2 => "下班打卡",
                _ => "未知",
            };
            println!("    打卡类型: {} ({})", task.type_, type_name);

            let result_name = match task.result {
                1 => "正常",
                2 => "早退",
                3 => "迟到",
                4 => "严重迟到",
                5 => "缺卡",
                6 => "无效",
                7 => "无班次",
                8 => "休息",
                _ => "未知",
            };
            println!("    打卡结果: {} ({})", task.result, result_name);

            println!("    是否外勤: {}", if task.is_field { "是" } else { "否" });
            println!("    是否补卡: {}", if task.is_remedy { "是" } else { "否" });

            if let Some(location) = &task.location {
                println!("    📍 位置信息:");
                println!("      纬度: {}", location.latitude);
                println!("      经度: {}", location.longitude);
                if let Some(address) = &location.address {
                    println!("      地址: {}", address);
                }
            }

            if let Some(comment) = &task.comment {
                println!("    备注: {}", comment);
            }

            println!("    创建时间: {}", task.create_time);
            println!("    更新时间: {}", task.update_time);
            println!("  ---");
        })?
        // 注册考勤任务状态变更事件处理器
        .register_p2_attendance_user_task_status_change_v1(
            |event: P2AttendanceUserTaskStatusChangeV1| {
                println!("🔄 收到考勤任务状态变更事件:");
                println!("  事件ID: {:?}", event.header.event_id);
                println!("  事件类型: {:?}", event.header.event_type);
                println!("  租户Key: {}", event.event.tenant_key);

                // 打印用户信息
                let user = &event.event.user_id;
                println!("  👤 用户信息:");
                println!("    用户ID: {}", user.user_id);
                println!("    Open ID: {}", user.open_id);
                println!("    Union ID: {}", user.union_id);
                if let Some(employee_id) = &user.employee_id {
                    println!("    员工ID: {}", employee_id);
                }

                // 打印状态变更信息
                let change = &event.event.task_status_change;
                println!("  📊 状态变更信息:");
                println!("    任务ID: {}", change.task_id);
                println!("    考勤组ID: {}", change.group_id);
                println!("    班次ID: {} ({})", change.shift_id, change.shift_name);
                println!("    记录日期: {}", change.record_date);

                let old_status_name = match change.old_status {
                    1 => "正常",
                    2 => "早退",
                    3 => "迟到",
                    4 => "严重迟到",
                    5 => "缺卡",
                    6 => "无效",
                    7 => "无班次",
                    8 => "休息",
                    _ => "未知",
                };
                let new_status_name = match change.new_status {
                    1 => "正常",
                    2 => "早退",
                    3 => "迟到",
                    4 => "严重迟到",
                    5 => "缺卡",
                    6 => "无效",
                    7 => "无班次",
                    8 => "休息",
                    _ => "未知",
                };
                println!(
                    "    状态变更: {} ({}) → {} ({})",
                    change.old_status, old_status_name, change.new_status, new_status_name
                );

                let change_type_name = match change.change_type {
                    1 => "管理员修改",
                    2 => "补卡",
                    3 => "审批通过",
                    4 => "系统自动调整",
                    _ => "未知",
                };
                println!(
                    "    变更类型: {} ({})",
                    change.change_type, change_type_name
                );
                println!("    变更原因: {}", change.change_reason);
                println!("    变更时间: {}", change.change_time);

                if let Some(operator_id) = &change.operator_id {
                    println!("    操作人ID: {}", operator_id);
                }

                if let Some(comment) = &change.change_comment {
                    println!("    变更备注: {}", comment);
                }

                // 打印相关打卡记录
                if let Some(records) = &change.check_records {
                    println!("    📋 相关打卡记录 ({}条):", records.len());
                    for (index, record) in records.iter().enumerate() {
                        println!("      {}. 记录ID: {}", index + 1, record.record_id);
                        println!("         打卡时间: {}", record.check_time);

                        let check_type_name = match record.check_type {
                            1 => "上班打卡",
                            2 => "下班打卡",
                            _ => "未知",
                        };
                        println!(
                            "         打卡类型: {} ({})",
                            record.check_type, check_type_name
                        );

                        let check_result_name = match record.check_result {
                            1 => "正常",
                            2 => "早退",
                            3 => "迟到",
                            4 => "严重迟到",
                            5 => "缺卡",
                            6 => "无效",
                            _ => "未知",
                        };
                        println!(
                            "         打卡结果: {} ({})",
                            record.check_result, check_result_name
                        );
                        println!(
                            "         是否外勤: {}",
                            if record.is_field { "是" } else { "否" }
                        );
                        println!(
                            "         是否补卡: {}",
                            if record.is_remedy { "是" } else { "否" }
                        );

                        if let Some(location) = &record.location {
                            println!(
                                "         📍 位置: {}, {} ({})",
                                location.latitude,
                                location.longitude,
                                location.address.as_deref().unwrap_or("未知")
                            );
                        }

                        if let Some(comment) = &record.comment {
                            println!("         备注: {}", comment);
                        }
                    }
                }

                println!("  ---");
            },
        )?
        .build();

    println!("✅ 考勤事件处理器注册完成!");
    println!("💡 说明: 当收到考勤相关事件时，会自动处理并打印详细信息");
    println!("📋 支持的事件:");
    println!("  - attendance.user_task.updated_v1: 考勤打卡流水事件");
    println!("  - attendance.user_task.status_change_v1: 考勤任务状态变更事件");
    println!();

    // 模拟接收事件数据进行测试
    println!("🧪 模拟事件数据测试...");

    // 模拟考勤打卡流水事件
    let attendance_event_data = r#"{
        "schema": "2.0",
        "header": {
            "event_id": "attendance_test_001",
            "event_type": "attendance.user_task.updated_v1",
            "create_time": "1719211482721",
            "token": "test_token",
            "app_id": "test_app",
            "tenant_key": "test_tenant"
        },
        "event": {
            "user_id": {
                "open_id": "ou_test_user",
                "union_id": "on_test_union",
                "user_id": "test_user_123",
                "employee_id": "emp_001"
            },
            "task": {
                "task_id": "task_123456",
                "user_id": "test_user_123",
                "employee_id": "emp_001",
                "group_id": "group_001",
                "shift_id": "shift_001",
                "record_date": "2024-06-20",
                "shift_name": "标准工作班次",
                "check_time": "2024-06-20 09:00:00",
                "result": 1,
                "type_": 1,
                "location": {
                    "latitude": 39.908822,
                    "longitude": 116.397128,
                    "address": "北京市朝阳区望京街道"
                },
                "is_field": false,
                "is_remedy": false,
                "comment": "正常上班打卡",
                "create_time": "1719211482485",
                "update_time": "1719211482485"
            },
            "tenant_key": "test_tenant"
        }
    }"#;

    println!("处理考勤打卡流水事件:");
    handler.do_without_validation(attendance_event_data.as_bytes().to_vec())?;

    // 模拟考勤任务状态变更事件
    let status_change_event_data = r#"{
        "schema": "2.0",
        "header": {
            "event_id": "attendance_status_test_001",
            "event_type": "attendance.user_task.status_change_v1",
            "create_time": "1719211482721",
            "token": "test_token",
            "app_id": "test_app",
            "tenant_key": "test_tenant"
        },
        "event": {
            "user_id": {
                "open_id": "ou_test_user",
                "union_id": "on_test_union",
                "user_id": "test_user_123",
                "employee_id": "emp_001"
            },
            "task_status_change": {
                "task_id": "task_123456",
                "user_id": "test_user_123",
                "employee_id": "emp_001",
                "group_id": "group_001",
                "shift_id": "shift_001",
                "record_date": "2024-06-20",
                "shift_name": "标准工作班次",
                "old_status": 3,
                "new_status": 1,
                "change_reason": "员工补卡申请审批通过",
                "change_type": 3,
                "operator_id": "admin_001",
                "change_time": "1719211582485",
                "change_comment": "员工因交通拥堵迟到30分钟，提交补卡申请并获得批准",
                "check_records": [
                    {
                        "record_id": "record_001",
                        "check_time": "2024-06-20 09:30:00",
                        "check_type": 1,
                        "check_result": 1,
                        "location": {
                            "latitude": 39.908822,
                            "longitude": 116.397128,
                            "address": "北京市朝阳区望京街道"
                        },
                        "is_field": false,
                        "is_remedy": true,
                        "comment": "补卡 - 交通拥堵"
                    }
                ]
            },
            "tenant_key": "test_tenant"
        }
    }"#;

    println!("处理考勤任务状态变更事件:");
    handler.do_without_validation(status_change_event_data.as_bytes().to_vec())?;

    println!("✅ 考勤事件处理测试完成!");

    Ok(())
}

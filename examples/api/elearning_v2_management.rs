//! 飞书在线学习 v2 - 学习管理示例
//!
//! 该示例演示如何使用在线学习 API 进行学习管理操作

use dotenvy::dotenv;
use open_lark::{
    core::req_option::RequestOption, prelude::*, service::elearning::course_registration::*,
};
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let app_id = env::var("APP_ID").expect("APP_ID 环境变量未设置");
    let app_secret = env::var("APP_SECRET").expect("APP_SECRET 环境变量未设置");

    let client = LarkClient::builder(&app_id, &app_secret)
        .with_app_type(AppType::SelfBuild)
        .build();

    println!("===== 获取学习进度详情 =====");

    // 获取指定学习记录的详情
    let registration_id = "registration_123456";

    let get_response = client
        .elearning
        .course_registration
        .get(registration_id, Option::<RequestOption>::None)
        .await?;

    if get_response.code() == 0 {
        println!("获取学习详情成功！");
        if let Some(data) = get_response.data {
            let registration = &data.registration;
            println!("  - 记录ID: {}", registration.registration_id);
            println!("  - 课程ID: {}", registration.course_id);
            println!("  - 用户ID: {}", registration.user_id);

            if let Some(status) = &registration.status {
                println!("  - 状态: {status}");
            }

            if let Some(progress) = registration.progress {
                println!("  - 学习进度: {progress:.1}%");
            }

            if let Some(total_duration) = registration.total_duration {
                println!("  - 总课程时长: {total_duration} 秒");
            }

            if let Some(studied_duration) = registration.studied_duration {
                println!("  - 已学时长: {studied_duration} 秒");
            }

            if let Some(score) = registration.score {
                println!("  - 学习成绩: {score:.1}");
            }

            if let Some(passed) = registration.passed {
                println!("  - 是否通过: {}", if passed { "是" } else { "否" });
            }

            // 显示课程信息
            if let Some(course_info) = &registration.course_info {
                println!("\n  课程信息:");
                if let Some(name) = &course_info.course_name {
                    println!("    - 课程名称: {name}");
                }
                if let Some(category) = &course_info.category {
                    println!("    - 课程分类: {category}");
                }
                if let Some(difficulty) = &course_info.difficulty {
                    println!("    - 难度等级: {difficulty}");
                }
            }

            // 显示用户信息
            if let Some(user_info) = &registration.user_info {
                println!("\n  用户信息:");
                if let Some(name) = &user_info.name {
                    println!("    - 用户姓名: {name}");
                }
                if let Some(department) = &user_info.department {
                    println!("    - 所属部门: {department}");
                }
            }
        }
    } else {
        eprintln!(
            "获取详情失败: {} - {}",
            get_response.code(),
            get_response.msg()
        );
    }

    println!("\n===== 按状态筛选学习记录 =====");

    // 查询进行中的学习记录
    let in_progress_request = CourseRegistrationListRequest {
        status: Some("in_progress".to_string()),
        page_size: Some(20),
        ..Default::default()
    };

    let in_progress_response = client
        .elearning
        .course_registration
        .list(in_progress_request, Option::<RequestOption>::None)
        .await?;

    if in_progress_response.code() == 0 {
        println!("查询进行中学习记录成功！");
        if let Some(data) = in_progress_response.data {
            println!("进行中的学习记录数: {}", data.registrations.items.len());

            for registration in &data.registrations.items {
                println!(
                    "  - 用户: {} | 课程: {} | 进度: {:.1}%",
                    registration.user_id,
                    registration.course_id,
                    registration.progress.unwrap_or(0.0)
                );
            }
        }
    } else {
        eprintln!(
            "查询失败: {} - {}",
            in_progress_response.code(),
            in_progress_response.msg()
        );
    }

    println!("\n===== 按时间范围查询学习记录 =====");

    // 查询最近30天的学习记录
    let now = chrono::Utc::now().timestamp();
    let thirty_days_ago = now - (30 * 24 * 60 * 60); // 30天前的时间戳

    let time_range_request = CourseRegistrationListRequest {
        start_time: Some(thirty_days_ago),
        end_time: Some(now),
        page_size: Some(15),
        ..Default::default()
    };

    let time_range_response = client
        .elearning
        .course_registration
        .list(time_range_request, Option::<RequestOption>::None)
        .await?;

    if time_range_response.code() == 0 {
        println!("按时间范围查询成功！");
        if let Some(data) = time_range_response.data {
            println!("最近30天学习记录数: {}", data.registrations.items.len());

            for registration in data.registrations.items.iter().take(5) {
                println!(
                    "  - 记录ID: {} | 状态: {:?} | 创建时间: {:?}",
                    registration.registration_id,
                    registration.status,
                    registration.created_at.map(|t| {
                        chrono::DateTime::from_timestamp(t, 0)
                            .unwrap_or_default()
                            .format("%Y-%m-%d %H:%M:%S")
                            .to_string()
                    })
                );
            }
        }
    } else {
        eprintln!(
            "时间范围查询失败: {} - {}",
            time_range_response.code(),
            time_range_response.msg()
        );
    }

    println!("\n===== 获取部门学习统计 =====");

    // 获取部门级别的学习统计
    let dept_stats_request = CourseRegistrationStatisticsRequest {
        department_id: Some("dept_12345".to_string()),
        start_time: Some(thirty_days_ago),
        end_time: Some(now),
        ..Default::default()
    };

    let dept_stats_response = client
        .elearning
        .course_registration
        .get_statistics(dept_stats_request, Option::<RequestOption>::None)
        .await?;

    if dept_stats_response.code() == 0 {
        println!("获取部门统计成功！");
        if let Some(data) = dept_stats_response.data {
            let stats = &data.statistics;

            println!("部门学习统计数据:");
            if let Some(total) = stats.total_courses {
                println!("  - 参与课程总数: {total}");
            }
            if let Some(completed) = stats.completed_courses {
                println!("  - 完成课程数: {completed}");
            }
            if let Some(in_progress) = stats.in_progress_courses {
                println!("  - 进行中课程数: {in_progress}");
            }
            if let Some(total_time) = stats.total_study_time {
                let hours = total_time / 3600;
                let minutes = (total_time % 3600) / 60;
                println!("  - 总学习时长: {hours}小时{minutes}分钟");
            }
            if let Some(avg_progress) = stats.average_progress {
                println!("  - 平均学习进度: {avg_progress:.1}%");
            }
            if let Some(pass_rate) = stats.pass_rate {
                println!("  - 通过率: {:.1}%", pass_rate * 100.0);
            }
        }
    } else {
        eprintln!(
            "获取部门统计失败: {} - {}",
            dept_stats_response.code(),
            dept_stats_response.msg()
        );
    }

    println!("\n===== 删除学习记录（慎用）=====");

    // 注意：这是演示删除操作，实际使用时请谨慎
    let delete_registration_id = "registration_to_delete";

    println!("准备删除学习记录: {delete_registration_id}");
    println!("注意：这将永久删除学习记录！");

    // 在实际应用中，你可能想要添加确认步骤
    // println!("请输入 'YES' 确认删除:");
    // let mut input = String::new();
    // std::io::stdin().read_line(&mut input)?;
    // if input.trim() != "YES" {
    //     println!("删除操作已取消");
    //     return Ok(());
    // }

    let delete_response = client
        .elearning
        .course_registration
        .delete(delete_registration_id, Option::<RequestOption>::None)
        .await?;

    if delete_response.code() == 0 {
        println!("删除操作结果：");
        if let Some(data) = delete_response.data {
            if data.success {
                println!("  ✓ 学习记录删除成功");
                if let Some(deleted_id) = &data.registration_id {
                    println!("  - 被删除的记录ID: {deleted_id}");
                }
            } else {
                println!("  ✗ 学习记录删除失败");
                if let Some(error_msg) = &data.error_message {
                    println!("  - 错误信息: {error_msg}");
                }
            }
        }
    } else {
        eprintln!(
            "删除失败: {} - {}",
            delete_response.code(),
            delete_response.msg()
        );
    }

    Ok(())
}

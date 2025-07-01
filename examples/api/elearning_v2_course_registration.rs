//! 飞书在线学习 v2 - 课程学习进度管理示例
//!
//! 该示例演示如何使用在线学习 API 管理课程学习进度

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

    println!("===== 创建课程学习进度记录 =====");

    // 创建新的学习进度记录
    let create_request = CourseRegistrationCreateRequest {
        course_id: "course_12345".to_string(),
        user_id: "user_67890".to_string(),
        registration_type: Some("manual".to_string()),
        status: Some("enrolled".to_string()),
        notes: Some("用户主动报名参加课程".to_string()),
    };

    let create_response = client
        .elearning
        .course_registration
        .create(create_request, Option::<RequestOption>::None)
        .await?;

    if create_response.code() == 0 {
        println!("学习进度记录创建成功！");
        if let Some(data) = &create_response.data {
            let registration = &data.registration;
            println!("  - 记录ID: {}", registration.registration_id);
            println!("  - 课程ID: {}", registration.course_id);
            println!("  - 用户ID: {}", registration.user_id);
            if let Some(status) = &registration.status {
                println!("  - 状态: {status}");
            }
        }
    } else {
        eprintln!(
            "创建失败: {} - {}",
            create_response.code(),
            create_response.msg()
        );
    }

    println!("\n===== 查询课程学习进度列表 =====");

    // 查询学习进度列表
    let list_request = CourseRegistrationListRequest {
        page_size: Some(10),
        course_id: Some("course_12345".to_string()),
        ..Default::default()
    };

    let list_response = client
        .elearning
        .course_registration
        .list(list_request, Option::<RequestOption>::None)
        .await?;

    if list_response.code() == 0 {
        println!("查询成功！");
        if let Some(data) = list_response.data {
            println!("学习进度记录总数: {}", data.registrations.items.len());

            for (index, registration) in data.registrations.items.iter().enumerate() {
                println!("\n学习记录 {}:", index + 1);
                println!("  - 记录ID: {}", registration.registration_id);
                println!("  - 课程ID: {}", registration.course_id);
                println!("  - 用户ID: {}", registration.user_id);
                if let Some(status) = &registration.status {
                    println!("  - 状态: {status}");
                }
                if let Some(progress) = registration.progress {
                    println!("  - 进度: {progress:.1}%");
                }
                if let Some(score) = registration.score {
                    println!("  - 成绩: {score:.1}");
                }
            }

            if let Some(has_more) = data.registrations.has_more {
                if has_more {
                    println!("\n还有更多学习记录...");
                }
            }
        }
    } else {
        eprintln!(
            "查询失败: {} - {}",
            list_response.code(),
            list_response.msg()
        );
    }

    println!("\n===== 按用户ID查询学习进度 =====");

    // 按用户ID查询
    let user_request = CourseRegistrationListRequest {
        user_id: Some("user_67890".to_string()),
        page_size: Some(5),
        ..Default::default()
    };

    let user_response = client
        .elearning
        .course_registration
        .list(user_request, Option::<RequestOption>::None)
        .await?;

    if user_response.code() == 0 {
        println!("按用户查询成功！");
        if let Some(data) = user_response.data {
            println!("用户学习记录数: {}", data.registrations.items.len());

            for registration in &data.registrations.items {
                println!(
                    "  - 课程ID: {} (状态: {:?})",
                    registration.course_id, registration.status
                );
            }
        }
    } else {
        eprintln!(
            "按用户查询失败: {} - {}",
            user_response.code(),
            user_response.msg()
        );
    }

    println!("\n===== 更新学习进度 =====");

    // 假设我们有一个学习记录ID
    let registration_id = "registration_123456";

    let update_request = CourseRegistrationUpdateRequest {
        status: Some("in_progress".to_string()),
        progress: Some(75.5),
        studied_duration: Some(3600), // 1小时
        score: Some(85.0),
        notes: Some("学习进度更新".to_string()),
        ..Default::default()
    };

    let update_response = client
        .elearning
        .course_registration
        .update(
            registration_id,
            update_request,
            Option::<RequestOption>::None,
        )
        .await?;

    if update_response.code() == 0 {
        println!("学习进度更新成功！");
        if let Some(data) = update_response.data {
            let registration = &data.registration;
            println!("  - 记录ID: {}", registration.registration_id);
            if let Some(progress) = registration.progress {
                println!("  - 新进度: {progress:.1}%");
            }
            if let Some(score) = registration.score {
                println!("  - 新成绩: {score:.1}");
            }
        }
    } else {
        eprintln!(
            "更新失败: {} - {}",
            update_response.code(),
            update_response.msg()
        );
    }

    println!("\n===== 获取学习统计数据 =====");

    // 获取用户学习统计
    let stats_request = CourseRegistrationStatisticsRequest {
        user_id: Some("user_67890".to_string()),
        ..Default::default()
    };

    let stats_response = client
        .elearning
        .course_registration
        .get_statistics(stats_request, Option::<RequestOption>::None)
        .await?;

    if stats_response.code() == 0 {
        println!("获取统计数据成功！");
        if let Some(data) = stats_response.data {
            let stats = &data.statistics;
            if let Some(total) = stats.total_courses {
                println!("  - 总课程数: {total}");
            }
            if let Some(completed) = stats.completed_courses {
                println!("  - 已完成课程数: {completed}");
            }
            if let Some(avg_progress) = stats.average_progress {
                println!("  - 平均进度: {avg_progress:.1}%");
            }
            if let Some(avg_score) = stats.average_score {
                println!("  - 平均成绩: {avg_score:.1}");
            }
        }
    } else {
        eprintln!(
            "获取统计失败: {} - {}",
            stats_response.code(),
            stats_response.msg()
        );
    }

    Ok(())
}

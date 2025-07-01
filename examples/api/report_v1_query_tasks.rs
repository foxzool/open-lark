//! 飞书汇报 v1 - 查询任务示例
//!
//! 该示例演示如何使用汇报 API 查询任务列表

use dotenvy::dotenv;
use open_lark::{
    core::req_option::RequestOption, prelude::*, service::report::task::TaskQueryRequest,
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

    println!("===== 查询任务列表 =====");

    // 查询所有任务
    let request = TaskQueryRequest {
        page_size: Some(10),
        ..Default::default()
    };

    let response = client
        .report
        .task
        .query(request, Option::<RequestOption>::None)
        .await?;

    if response.code() == 0 {
        println!("查询成功！");
        if let Some(data) = response.data {
            println!("任务总数: {}", data.tasks.items.len());

            for (index, task) in data.tasks.items.iter().enumerate() {
                println!("\n任务 {}:", index + 1);
                println!("  - 任务ID: {}", task.task_id);
                println!("  - 规则ID: {}", task.rule_id);
                if let Some(name) = &task.name {
                    println!("  - 任务名称: {name}");
                }
                if let Some(status) = &task.status {
                    println!("  - 状态: {status}");
                }
                if let Some(task_type) = &task.task_type {
                    println!("  - 类型: {task_type}");
                }
                if let Some(reporter_id) = &task.reporter_id {
                    println!("  - 汇报者ID: {reporter_id}");
                }
            }

            if let Some(has_more) = data.tasks.has_more {
                if has_more {
                    println!("\n还有更多任务数据...");
                }
            }
        }
    } else {
        eprintln!("查询失败: {} - {}", response.code(), response.msg());
    }

    println!("\n===== 按规则ID查询任务 =====");

    // 按规则ID查询任务
    let filtered_request = TaskQueryRequest {
        rule_id: Some("rule_123456".to_string()), // 替换为实际的规则ID
        page_size: Some(5),
        ..Default::default()
    };

    let filtered_response = client
        .report
        .task
        .query(filtered_request, Option::<RequestOption>::None)
        .await?;

    if filtered_response.code() == 0 {
        println!("按规则ID查询成功！");
        if let Some(data) = filtered_response.data {
            println!("指定规则的任务数: {}", data.tasks.items.len());

            for task in &data.tasks.items {
                println!("  - 任务ID: {} (状态: {:?})", task.task_id, task.status);
            }
        }
    } else {
        eprintln!(
            "按规则ID查询失败: {} - {}",
            filtered_response.code(),
            filtered_response.msg()
        );
    }

    println!("\n===== 按状态查询任务 =====");

    // 按状态查询任务
    let status_request = TaskQueryRequest {
        status: Some("pending".to_string()),
        page_size: Some(3),
        ..Default::default()
    };

    let status_response = client
        .report
        .task
        .query(status_request, Option::<RequestOption>::None)
        .await?;

    if status_response.code() == 0 {
        println!("按状态查询成功！");
        if let Some(data) = status_response.data {
            println!("待处理任务数: {}", data.tasks.items.len());

            for task in &data.tasks.items {
                if let Some(name) = &task.name {
                    println!("  - {} ({})", name, task.task_id);
                } else {
                    println!("  - 任务ID: {}", task.task_id);
                }
            }
        }
    } else {
        eprintln!(
            "按状态查询失败: {} - {}",
            status_response.code(),
            status_response.msg()
        );
    }

    Ok(())
}

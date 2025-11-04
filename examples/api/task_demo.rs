//! 任务模块使用示例
//!
//! 演示如何使用task模块进行任务管理操作，包括：
//! - 任务创建
//! - 任务列表查询
//! - 任务状态更新
//! - 任务删除

use open_lark::prelude::*;
use open_lark::service::task::v1::{
    CreateTaskRequest, DeleteTaskRequest, ListTasksRequest, UpdateTaskRequest,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 初始化日志
    env_logger::init();

    println!("🚀 任务模块示例演示");

    // 创建客户端（这里使用测试配置）
    let client = LarkClient::builder("test_app_id", "test_app_secret")
        .with_app_type(AppType::SelfBuild)
        .build();

    println!("✅ 客户端创建成功");

    // 演示创建任务
    println!("\n📋 创建任务");
    let create_request = CreateTaskRequest {
        title: "完成项目文档".to_string(),
        description: Some("编写项目技术文档和API说明".to_string()),
        priority: "high".to_string(),
        assignee: Some("user_001".to_string()),
        due_date: Some("2024-01-15T23:59:59Z".to_string()),
    };

    match client.task.v1.create_task(&create_request).await {
        Ok(response) => {
            println!("✅ 任务创建成功");
            if let Some(data) = response.data {
                println!("   任务ID: {}", data.task_id);
                println!("   任务标题: {}", data.title);
                println!("   任务状态: {}", data.status);
                println!("   优先级: {}", data.priority);
                println!("   创建者: {}", data.creator);
                if let Some(assignee) = data.assignee {
                    println!("   指派给: {}", assignee);
                }
                if let Some(due_date) = data.due_date {
                    println!("   截止日期: {}", due_date);
                }
            }
        }
        Err(e) => {
            println!("❌ 任务创建失败: {}", e);
        }
    }

    // 演示获取任务列表
    println!("\n📋 获取任务列表");
    let list_request = ListTasksRequest {
        status: None,
        assignee: Some("user_001".to_string()),
        priority: None,
        page_size: Some(10),
        page_token: None,
    };

    match client.task.v1.list_tasks(&list_request).await {
        Ok(response) => {
            println!("✅ 任务列表获取成功");
            if let Some(data) = response.data {
                println!("   找到 {} 个任务:", data.tasks.len());
                println!("   总数: {}", data.total);
                println!("   是否有更多: {}", data.has_more);

                for (i, task) in data.tasks.iter().enumerate() {
                    println!("\n   {}. {}", i + 1, task.title);
                    println!("      状态: {}", task.status);
                    println!("      优先级: {}", task.priority);
                    println!("      创建时间: {}", task.created_time);
                    if let Some(assignee) = &task.assignee {
                        println!("      指派给: {}", assignee);
                    }
                }
            }
        }
        Err(e) => {
            println!("❌ 任务列表获取失败: {}", e);
        }
    }

    // 演示更新任务状态
    println!("\n📋 更新任务状态");
    let update_request = UpdateTaskRequest {
        task_id: "task_001".to_string(),
        title: None,
        description: Some("任务进度：已完成50%".to_string()),
        status: "in_progress".to_string(),
        priority: None,
        assignee: Some("user_002".to_string()),
        due_date: Some("2024-01-20T23:59:59Z".to_string()),
    };

    match client.task.v1.update_task(&update_request).await {
        Ok(response) => {
            println!("✅ 任务更新成功");
            if let Some(data) = response.data {
                println!("   任务ID: {}", data.task_id);
                println!("   新状态: {}", data.status);
                if let Some(description) = data.description {
                    println!("   描述: {}", description);
                }
                println!("   更新时间: {}", data.updated_time);
            }
        }
        Err(e) => {
            println!("❌ 任务更新失败: {}", e);
        }
    }

    // 演示删除任务
    println!("\n📋 删除任务");
    let delete_request = DeleteTaskRequest {
        task_id: "task_002".to_string(),
    };

    match client.task.v1.delete_task(&delete_request).await {
        Ok(response) => {
            println!("✅ 任务删除成功");
            if let Some(data) = response.data {
                println!("   任务ID: {}", data.task_id);
                println!("   删除状态: {}", data.deleted);
            }
        }
        Err(e) => {
            println!("❌ 任务删除失败: {}", e);
        }
    }

    println!("\n🎉 任务模块示例演示完成！");
    Ok(())
}

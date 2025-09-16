use dotenvy::dotenv;
use open_lark::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 加载环境变量
    dotenv().ok();

    let app_id = std::env::var("APP_ID").expect("需要设置 APP_ID 环境变量");
    let app_secret = std::env::var("APP_SECRET").expect("需要设置 APP_SECRET 环境变量");

    // 创建客户端
    let client = LarkClient::builder(&app_id, &app_secret)
        .with_app_type(AppType::SelfBuild)
        .build();

    // 演示任务管理功能
    demo_task_management(&client).await?;

    Ok(())
}

async fn demo_task_management(client: &LarkClient) -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 开始演示 Task v2 API");

    // 1. 创建清单
    println!("\n📝 创建任务清单...");
    let create_tasklist_req = CreateTasklistRequest {
        name: "开发任务清单".to_string(),
        members: None,
    };

    let tasklist_response = client
        .task
        .tasklist
        .create(create_tasklist_req, None, None)
        .await;

    match tasklist_response {
        Ok(response) => {
            let tasklist = response.data.unwrap().tasklist;
            println!("✅ 清单创建成功: {:?}", tasklist.name);
            let tasklist_guid = tasklist.guid.clone().unwrap_or_default();

            // 2. 创建任务
            println!("\n📋 创建任务...");
            let create_task_req = CreateTaskRequest {
                summary: "实现用户认证功能".to_string(),
                description: Some("需要实现JWT认证和权限管理".to_string()),
                tasklist_guid: Some(tasklist_guid.clone()),
                due: Some(TaskDue {
                    timestamp: Some("1672531200000".to_string()), // 2023-01-01
                    is_all_day: Some(false),
                }),
                start: None,
                members: None,
                repeat_rule: None,
                custom_complete: None,
                source: Some(1),
            };

            let task_response = client.task.task.create(create_task_req, None, None).await;

            match task_response {
                Ok(response) => {
                    let task = response.data.unwrap().task;
                    println!("✅ 任务创建成功: {:?}", task.summary);
                    let task_guid = task.guid.clone().unwrap_or_default();

                    // 3. 创建子任务
                    println!("\n📌 创建子任务...");
                    let create_subtask_req = CreateSubtaskRequest {
                        summary: "设计数据库架构".to_string(),
                        description: Some("设计用户认证相关的数据库表".to_string()),
                    };

                    let subtask_response = client
                        .task
                        .task_subtask
                        .create(&task_guid, create_subtask_req, None, None)
                        .await;

                    match subtask_response {
                        Ok(response) => {
                            let subtask = response.data.unwrap().task;
                            println!("✅ 子任务创建成功: {:?}", subtask.summary);
                        }
                        Err(e) => println!("❌ 子任务创建失败: {e:?}"),
                    }

                    // 4. 添加任务评论
                    println!("\n💬 添加评论...");
                    let create_comment_req = CreateCommentRequest {
                        content: "已经开始分析需求，预计明天完成设计".to_string(),
                        parent_id: None,
                    };

                    let comment_response = client
                        .task
                        .comment
                        .create(&task_guid, create_comment_req, None, None)
                        .await;

                    match comment_response {
                        Ok(response) => {
                            let comment = response.data.unwrap().comment;
                            println!("✅ 评论添加成功: {:?}", comment.content);
                        }
                        Err(e) => println!("❌ 评论添加失败: {e:?}"),
                    }

                    // 5. 创建自定义分组
                    println!("\n🗂️ 创建自定义分组...");
                    let create_section_req = CreateSectionRequest {
                        name: "开发阶段".to_string(),
                        tasklist_guid: tasklist_guid.clone(),
                    };

                    let section_response = client
                        .task
                        .section
                        .create(create_section_req, None, None)
                        .await;

                    match section_response {
                        Ok(response) => {
                            let section = response.data.unwrap().section;
                            println!("✅ 分组创建成功: {:?}", section.name);
                        }
                        Err(e) => println!("❌ 分组创建失败: {e:?}"),
                    }

                    // 6. 创建活动订阅
                    println!("\n🔔 创建活动订阅...");
                    let create_subscription_req = CreateActivitySubscriptionRequest {
                        name: "开发进度通知".to_string(),
                        subscribers: None,
                        include_completed: Some(true),
                    };

                    let subscription_response = client
                        .task
                        .tasklist_activity_subscription
                        .create(&tasklist_guid, create_subscription_req, None, None)
                        .await;

                    match subscription_response {
                        Ok(response) => {
                            let subscription = response.data.unwrap().subscription;
                            println!("✅ 订阅创建成功: {:?}", subscription.name);
                        }
                        Err(e) => println!("❌ 订阅创建失败: {e:?}"),
                    }

                    // 7. 获取任务列表
                    println!("\n📋 获取任务列表...");
                    let tasks_response = client
                        .task
                        .task
                        .list(
                            Some(10),    // page_size
                            None,        // page_token
                            Some(false), // completed
                            None,        // created_from
                            None,        // created_to
                            None,        // updated_from
                            None,        // updated_to
                            None,        // due_from
                            None,        // due_to
                            None,        // user_id_type
                            None,        // option
                        )
                        .await;

                    match tasks_response {
                        Ok(response) => {
                            let tasks_data = response.data.unwrap();
                            println!("✅ 获取到 {} 个任务", tasks_data.items.len());
                            for task in tasks_data.items {
                                println!("  - {}", task.summary.unwrap_or_default());
                            }
                        }
                        Err(e) => println!("❌ 获取任务列表失败: {e:?}"),
                    }

                    // 8. 获取任务详情
                    println!("\n🔍 获取任务详情...");
                    let task_detail_response = client.task.task.get(&task_guid, None, None).await;

                    match task_detail_response {
                        Ok(response) => {
                            let task = response.data.unwrap().task;
                            println!("✅ 任务详情获取成功:");
                            println!("  标题: {:?}", task.summary);
                            println!("  描述: {:?}", task.description);
                            println!("  状态: {:?}", task.status);
                        }
                        Err(e) => println!("❌ 获取任务详情失败: {e:?}"),
                    }

                    // 9. 更新任务
                    println!("\n✏️ 更新任务...");
                    let update_task_req = UpdateTaskRequest {
                        summary: Some("实现用户认证功能 (进行中)".to_string()),
                        description: Some("需要实现JWT认证和权限管理 - 已开始开发".to_string()),
                        due: None,
                        start: None,
                        completed_at: None,
                        repeat_rule: None,
                        custom_complete: None,
                    };

                    let update_response = client
                        .task
                        .task
                        .patch(&task_guid, update_task_req, None, None)
                        .await;

                    match update_response {
                        Ok(response) => {
                            let updated_task = response.data.unwrap().task;
                            println!("✅ 任务更新成功: {:?}", updated_task.summary);
                        }
                        Err(e) => println!("❌ 任务更新失败: {e:?}"),
                    }

                    println!("\n🎉 Task v2 API 演示完成!");
                }
                Err(e) => println!("❌ 任务创建失败: {e:?}"),
            }
        }
        Err(e) => println!("❌ 清单创建失败: {e:?}"),
    }

    Ok(())
}

// 引入必要的类型
use open_lark::service::task::v2::{
    comment::CreateCommentRequest,
    section::CreateSectionRequest,
    task::{CreateTaskRequest, TaskDue, UpdateTaskRequest},
    task_subtask::CreateSubtaskRequest,
    tasklist::CreateTasklistRequest,
    tasklist_activity_subscription::CreateActivitySubscriptionRequest,
};

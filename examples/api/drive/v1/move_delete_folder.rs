use dotenv::dotenv;
use open_lark::prelude::*;
use open_lark::service::drive::v1::folder::{
    CheckAsyncTaskRequest, CreateFolderRequest, MoveOrDeleteFolderRequest,
};
use std::env;
use tokio::time::{sleep, Duration};
use tracing::info;

/// 移动或删除文件夹示例
///
/// 该接口用于根据文件夹的token移动或删除文件夹。
///
/// <https://open.feishu.cn/document/server-docs/docs/drive-v1/folder/move-delete-folder>
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 从环境变量获取配置
    dotenv().ok();
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let app_id = env::var("APP_ID").expect("APP_ID 必须设置");
    let app_secret = env::var("APP_SECRET").expect("APP_SECRET 必须设置");
    let user_access_token = env::var("USER_ACCESS_TOKEN").expect("USER_ACCESS_TOKEN 必须设置");

    // 创建客户端，使用用户访问凭证
    let client = LarkClient::builder(app_id, app_secret)
        .with_user_access_token(user_access_token)
        .build();

    info!("开始演示移动和删除文件夹...");

    // 首先获取根目录的token
    match client.drive.v1.folder.get_root_folder_meta(None).await {
        Ok(root_response) => {
            if let Some(root_data) = root_response.data {
                let root_token = root_data.token;
                info!("获取到根目录token: {}", root_token);

                // 步骤1: 创建第一个测试文件夹
                let folder1_name = format!("测试文件夹1_{}", chrono::Utc::now().timestamp());
                let create_request1 = CreateFolderRequest::new(folder1_name.clone(), root_token.clone());
                
                let folder1_token = match client.drive.v1.folder.create_folder(create_request1, None).await {
                    Ok(response) => {
                        if let Some(data) = response.data {
                            println!("✅ 创建文件夹1成功: {} (Token: {})", folder1_name, data.token);
                            data.token
                        } else {
                            eprintln!("❌ 创建文件夹1失败：没有返回数据");
                            return Ok(());
                        }
                    }
                    Err(e) => {
                        eprintln!("❌ 创建文件夹1失败: {}", e);
                        return Ok(());
                    }
                };

                // 步骤2: 创建第二个测试文件夹
                let folder2_name = format!("测试文件夹2_{}", chrono::Utc::now().timestamp());
                let create_request2 = CreateFolderRequest::new(folder2_name.clone(), root_token.clone());
                
                let folder2_token = match client.drive.v1.folder.create_folder(create_request2, None).await {
                    Ok(response) => {
                        if let Some(data) = response.data {
                            println!("✅ 创建文件夹2成功: {} (Token: {})", folder2_name, data.token);
                            data.token
                        } else {
                            eprintln!("❌ 创建文件夹2失败：没有返回数据");
                            return Ok(());
                        }
                    }
                    Err(e) => {
                        eprintln!("❌ 创建文件夹2失败: {}", e);
                        return Ok(());
                    }
                };

                // 步骤3: 将文件夹2移动到文件夹1下
                println!("\n📁 将文件夹2移动到文件夹1下...");
                let move_request = MoveOrDeleteFolderRequest::move_folder(folder2_token.clone(), folder1_token.clone());
                
                match client.drive.v1.folder.move_or_delete_folder(move_request, None).await {
                    Ok(response) => {
                        println!("✅ 移动文件夹操作已提交");
                        if let Some(data) = response.data {
                            if let Some(task_id) = data.task_id {
                                println!("📋 异步任务ID: {}", task_id);
                                
                                // 查询任务状态
                                let mut attempts = 0;
                                loop {
                                    attempts += 1;
                                    let check_request = CheckAsyncTaskRequest::new(task_id.clone());
                                    
                                    match client.drive.v1.folder.check_async_task(check_request, None).await {
                                        Ok(task_response) => {
                                            if let Some(task_data) = task_response.data {
                                                println!("🔍 任务状态检查 (第{}次): {}", attempts, task_data.status);
                                                
                                                match task_data.status.as_str() {
                                                    "SUCCESS" => {
                                                        println!("✅ 文件夹移动成功！");
                                                        break;
                                                    }
                                                    "FAILURE" => {
                                                        let error_msg = task_data.error_msg.unwrap_or_else(|| "未知错误".to_string());
                                                        eprintln!("❌ 文件夹移动失败: {}", error_msg);
                                                        break;
                                                    }
                                                    "PENDING" => {
                                                        if attempts >= 10 {
                                                            println!("⏰ 任务仍在执行中，停止等待");
                                                            break;
                                                        }
                                                        println!("⏳ 任务进行中，2秒后重试...");
                                                        sleep(Duration::from_secs(2)).await;
                                                    }
                                                    _ => {
                                                        println!("❓ 未知任务状态: {}", task_data.status);
                                                        break;
                                                    }
                                                }
                                            }
                                        }
                                        Err(e) => {
                                            eprintln!("❌ 查询任务状态失败: {}", e);
                                            break;
                                        }
                                    }
                                }
                            } else {
                                println!("⚡ 移动操作立即完成（无异步任务）");
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("❌ 移动文件夹失败: {}", e);
                    }
                }

                // 步骤4: 等待一会儿，然后删除文件夹1（会连同其下的文件夹2一起删除）
                println!("\n🗑️  删除文件夹1（连同其子文件夹）...");
                sleep(Duration::from_secs(3)).await;
                
                let delete_request = MoveOrDeleteFolderRequest::delete_folder(folder1_token);
                
                match client.drive.v1.folder.move_or_delete_folder(delete_request, None).await {
                    Ok(response) => {
                        println!("✅ 删除文件夹操作已提交");
                        if let Some(data) = response.data {
                            if let Some(task_id) = data.task_id {
                                println!("📋 异步任务ID: {}", task_id);
                                
                                // 查询删除任务状态
                                let mut attempts = 0;
                                loop {
                                    attempts += 1;
                                    let check_request = CheckAsyncTaskRequest::new(task_id.clone());
                                    
                                    match client.drive.v1.folder.check_async_task(check_request, None).await {
                                        Ok(task_response) => {
                                            if let Some(task_data) = task_response.data {
                                                println!("🔍 删除任务状态检查 (第{}次): {}", attempts, task_data.status);
                                                
                                                match task_data.status.as_str() {
                                                    "SUCCESS" => {
                                                        println!("✅ 文件夹删除成功！");
                                                        break;
                                                    }
                                                    "FAILURE" => {
                                                        let error_msg = task_data.error_msg.unwrap_or_else(|| "未知错误".to_string());
                                                        eprintln!("❌ 文件夹删除失败: {}", error_msg);
                                                        break;
                                                    }
                                                    "PENDING" => {
                                                        if attempts >= 10 {
                                                            println!("⏰ 删除任务仍在执行中，停止等待");
                                                            break;
                                                        }
                                                        println!("⏳ 删除任务进行中，2秒后重试...");
                                                        sleep(Duration::from_secs(2)).await;
                                                    }
                                                    _ => {
                                                        println!("❓ 未知任务状态: {}", task_data.status);
                                                        break;
                                                    }
                                                }
                                            }
                                        }
                                        Err(e) => {
                                            eprintln!("❌ 查询删除任务状态失败: {}", e);
                                            break;
                                        }
                                    }
                                }
                            } else {
                                println!("⚡ 删除操作立即完成（无异步任务）");
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("❌ 删除文件夹失败: {}", e);
                    }
                }

                println!("\n🎉 文件夹移动和删除演示完成！");
            } else {
                eprintln!("❌ 获取根目录信息失败：没有返回数据");
            }
        }
        Err(e) => {
            eprintln!("❌ 获取根目录信息失败: {}", e);
        }
    }

    Ok(())
}
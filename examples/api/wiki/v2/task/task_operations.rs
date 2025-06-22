use open_lark::{
    service::wiki::v2::task::{GetTaskRequest, MoveDocsToWikiRequest, TaskStatus},
    prelude::*,
};
use std::time::Duration;
use tokio::time::sleep;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 加载环境变量
    dotenvy::dotenv().ok();

    let app_id = std::env::var("APP_ID").expect("APP_ID environment variable not set");
    let app_secret = std::env::var("APP_SECRET").expect("APP_SECRET environment variable not set");

    // 创建客户端
    let client = LarkClient::builder(&app_id, &app_secret)
        .with_enable_token_cache(true)
        .build();

    // 配置必要参数
    let space_id = "spcxxxxxx"; // 需要一个真实的知识空间ID
    let doc_tokens = vec![
        "doccnxxxxxx".to_string(), // 需要真实的文档token
        "shtcnxxxxxx".to_string(), // 需要真实的电子表格token
    ];

    // 1. 移动云空间文档至知识空间
    println!("1. 移动云空间文档至知识空间...");

    let move_request = MoveDocsToWikiRequest::builder()
        .space_id(space_id)
        .obj_tokens(doc_tokens.clone())
        .to_root() // 移动到知识空间根目录
        .build();

    let task_id = match client
        .wiki
        .v2
        .task
        .move_docs_to_wiki(move_request, None)
        .await
    {
        Ok(move_response) => {
            let task = &move_response.task;
            println!("移动任务创建成功:");
            println!("  - 任务ID: {}", task.task_id);
            println!("  - 正在移动 {} 个文档到知识空间", doc_tokens.len());
            task.task_id.clone()
        }
        Err(e) => {
            println!("创建移动任务失败: {:?}", e);
            println!("可能的原因：");
            println!("- 文档token不存在或无效");
            println!("- 没有对文档的编辑权限");
            println!("- 知识空间ID不存在或没有权限");
            println!("- 文档已经在知识空间中");
            return Ok(());
        }
    };

    // 2. 轮询任务状态直到完成
    println!("\n2. 监控任务执行进度...");

    let mut max_polls = 30; // 最多轮询30次
    let poll_interval = Duration::from_secs(2); // 每2秒轮询一次

    loop {
        let get_request = GetTaskRequest::builder().task_id(&task_id).build();

        match client.wiki.v2.task.get(get_request, None).await {
            Ok(task_response) => {
                let task = &task_response.task;

                println!("任务状态更新:");
                println!("  - 任务ID: {}", task.task_id);
                println!("  - 状态: {:?}", task.status);

                if let Some(space_id) = &task.space_id {
                    println!("  - 目标空间: {}", space_id);
                }

                if let (Some(processed), Some(total)) = (task.processed_count, task.total_count) {
                    println!(
                        "  - 进度: {}/{} ({}%)",
                        processed,
                        total,
                        task.progress_percentage().unwrap_or(0.0) as i32
                    );
                }

                // 检查任务是否完成
                if task.status.is_finished() {
                    if task.status.is_success() {
                        println!("\n✅ 任务执行成功!");

                        if let Some(results) = &task.move_results {
                            println!("移动成功的文档:");
                            for (index, result) in results.iter().enumerate() {
                                println!("  文档 {}:", index + 1);
                                println!("    - 原始Token: {}", result.obj_token);
                                println!("    - 节点Token: {}", result.node_token);
                                if let Some(title) = &result.title {
                                    println!("    - 标题: {}", title);
                                }
                                if let Some(obj_type) = &result.obj_type {
                                    println!("    - 类型: {}", obj_type);
                                }
                            }
                        }

                        println!("成功移动 {} 个文档", task.success_count());
                    } else if task.status.is_failed() {
                        println!("\n❌ 任务执行失败!");
                        if let Some(error) = &task.error_message {
                            println!("错误信息: {}", error);
                        }
                    }

                    if let Some(finish_time) = &task.finish_time {
                        println!("完成时间: {}", finish_time);
                    }

                    break;
                } else {
                    println!("  - 任务进行中，等待下次检查...\n");
                }
            }
            Err(e) => {
                println!("获取任务状态失败: {:?}", e);
                break;
            }
        }

        max_polls -= 1;
        if max_polls == 0 {
            println!("已达到最大轮询次数，停止监控");
            break;
        }

        sleep(poll_interval).await;
    }

    // 3. 示例：移动文档到指定父节点
    println!("\n3. 移动文档到指定父节点...");

    let parent_node_token = "wikcnparent"; // 需要一个真实的父节点token
    let specific_docs = vec!["doccnspecific".to_string()]; // 需要真实的文档token

    let move_to_parent_request = MoveDocsToWikiRequest::builder()
        .space_id(space_id)
        .obj_tokens(specific_docs.clone())
        .parent_node_token(parent_node_token)
        .build();

    match client
        .wiki
        .v2
        .task
        .move_docs_to_wiki(move_to_parent_request, None)
        .await
    {
        Ok(response) => {
            println!("移动到指定父节点的任务创建成功:");
            println!("  - 任务ID: {}", response.task.task_id);
            println!("  - 目标父节点: {}", parent_node_token);
        }
        Err(e) => {
            println!("创建移动到父节点的任务失败: {:?}", e);
        }
    }

    // 4. 示例：批量添加文档的不同方式
    println!("\n4. 批量添加文档的不同方式...");

    let batch_request = MoveDocsToWikiRequest::builder()
        .space_id(space_id)
        .add_obj_token("doccn001")
        .add_obj_token("shtcn002")
        .add_obj_tokens(vec!["bblcn003", "mindcn004", "wikicn005"])
        .to_root()
        .build();

    println!("批量请求配置完成:");
    println!("  - 空间ID: {}", space_id);
    // println!("  - 文档数量: {} 个", batch_request.obj_tokens.len()); // 私有字段，暂时注释
    println!("  - 目标位置: 根目录");

    println!("\n云文档任务操作示例完成！");
    println!("提示：");
    println!("- 移动文档需要对源文档有编辑权限");
    println!("- 移动后原云空间中的文档会被删除");
    println!("- 支持文档、电子表格、思维笔记、多维表格等类型");
    println!("- 任务是异步执行的，需要轮询获取结果");
    println!("- 建议添加适当的超时和错误处理机制");
    println!("- 移动后的文档会成为知识空间中的节点");

    Ok(())
}

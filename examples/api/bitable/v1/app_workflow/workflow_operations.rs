use open_lark::{
    service::bitable::v1::app_workflow::{ListWorkflowRequest, UpdateWorkflowRequest},
    LarkClient,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 加载环境变量
    dotenvy::dotenv().ok();

    let app_id = std::env::var("APP_ID").expect("APP_ID environment variable not set");
    let app_secret = std::env::var("APP_SECRET").expect("APP_SECRET environment variable not set");

    // 创建客户端
    let client = LarkClient::builder(app_id, app_secret)
        .with_enable_token_cache(true)
        .build();

    // 配置必要参数
    let app_token = "bascnmBA*****yGehy8";

    // 1. 列出自动化流程
    println!("1. 列出自动化流程...");

    let list_request = ListWorkflowRequest::builder()
        .app_token(app_token)
        .page_size(20)
        .build();

    match client
        .bitable
        .v1
        .app_workflow
        .list(list_request, None)
        .await
    {
        Ok(list_response) => {
            println!("自动化流程列表:");
            for workflow in &list_response.data.items {
                println!("  流程信息:");
                println!("    - ID: {}", workflow.id);
                println!("    - 名称: {}", workflow.name);
                println!(
                    "    - 状态: {}",
                    if workflow.is_enabled == 1 {
                        "已启用"
                    } else {
                        "未启用"
                    }
                );
                println!("    - 触发器类型: {}", workflow.trigger_type);
                if let Some(desc) = &workflow.description {
                    println!("    - 描述: {}", desc);
                }
                println!("    - 创建时间: {}", workflow.created_time);
                println!("    - 更新时间: {}", workflow.updated_time);
                if let Some(last_exec) = workflow.last_execution_time {
                    println!("    - 最后执行时间: {}", last_exec);
                } else {
                    println!("    - 最后执行时间: 未执行过");
                }
                println!();
            }
            println!(
                "总计 {} 个自动化流程",
                list_response
                    .data
                    .total
                    .unwrap_or(list_response.data.items.len() as i32)
            );

            // 如果有自动化流程，演示状态更新
            if let Some(first_workflow) = list_response.data.items.first() {
                println!("\n2. 更新自动化流程状态...");
                let workflow_id = &first_workflow.id;
                let current_enabled = first_workflow.is_enabled == 1;

                println!("选择第一个流程进行状态切换:");
                println!("  - 流程名称: {}", first_workflow.name);
                println!(
                    "  - 当前状态: {}",
                    if current_enabled {
                        "已启用"
                    } else {
                        "未启用"
                    }
                );
                println!(
                    "  - 将切换为: {}",
                    if current_enabled { "停用" } else { "启用" }
                );

                // 切换状态
                let update_request = UpdateWorkflowRequest::builder()
                    .app_token(app_token)
                    .workflow_id(workflow_id)
                    .is_enabled(!current_enabled)
                    .build();

                match client
                    .bitable
                    .v1
                    .app_workflow
                    .update(update_request, None)
                    .await
                {
                    Ok(update_response) => {
                        println!("状态更新成功:");
                        println!("  - 流程ID: {}", update_response.data.workflow_id);
                        println!(
                            "  - 新状态: {}",
                            if update_response.data.is_enabled == 1 {
                                "已启用"
                            } else {
                                "未启用"
                            }
                        );
                        println!("  - 更新时间: {}", update_response.data.updated_time);

                        // 再次切换回原状态（演示完整操作）
                        println!("\n3. 恢复原状态...");

                        let restore_request = UpdateWorkflowRequest::builder()
                            .app_token(app_token)
                            .workflow_id(workflow_id)
                            .is_enabled(current_enabled)
                            .build();

                        match client
                            .bitable
                            .v1
                            .app_workflow
                            .update(restore_request, None)
                            .await
                        {
                            Ok(restore_response) => {
                                println!("状态恢复成功:");
                                println!("  - 流程ID: {}", restore_response.data.workflow_id);
                                println!(
                                    "  - 恢复状态: {}",
                                    if restore_response.data.is_enabled == 1 {
                                        "已启用"
                                    } else {
                                        "未启用"
                                    }
                                );
                                println!("  - 更新时间: {}", restore_response.data.updated_time);
                            }
                            Err(e) => {
                                println!("恢复状态失败: {:?}", e);
                            }
                        }
                    }
                    Err(e) => {
                        println!("更新状态失败: {:?}", e);
                    }
                }

                println!("\n4. 再次列出流程（验证状态）...");

                let final_list_request = ListWorkflowRequest::builder()
                    .app_token(app_token)
                    .page_size(20)
                    .build();

                let final_list_response = client
                    .bitable
                    .v1
                    .app_workflow
                    .list(final_list_request, None)
                    .await?;

                println!("最终流程状态:");
                for workflow in &final_list_response.data.items {
                    println!(
                        "  - {}: {}",
                        workflow.name,
                        if workflow.is_enabled == 1 {
                            "已启用"
                        } else {
                            "未启用"
                        }
                    );
                }
            } else {
                println!("\n没有找到自动化流程，跳过状态更新演示");
            }
        }
        Err(e) => {
            println!("列出自动化流程失败: {:?}", e);
            println!("请确保多维表格存在且有自动化流程");
        }
    }

    println!("\n自动化流程操作示例完成！");
    println!("提示：");
    println!("- 自动化流程需要在飞书多维表格中预先创建");
    println!("- 启用/停用操作会影响流程的实际执行");
    println!("- 可以通过触发器类型了解流程的触发条件");
    println!("- 最后执行时间帮助了解流程的活跃度");

    Ok(())
}

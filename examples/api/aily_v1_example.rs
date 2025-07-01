use dotenvy::dotenv;
use open_lark::prelude::*;
use std::env;

#[tokio::main]
async fn main() -> SDKResult<()> {
    dotenv().ok();

    let app_id = env::var("APP_ID").expect("APP_ID is required");
    let app_secret = env::var("APP_SECRET").expect("APP_SECRET is required");

    let client = LarkClient::builder(&app_id, &app_secret)
        .with_app_type(AppType::SelfBuild)
        .with_enable_token_cache(true)
        .build();

    // 示例应用ID（实际使用时需要替换为真实的智能伙伴应用ID）
    let aily_app_id = "your_aily_app_id".to_string();

    println!("=== 飞书智能伙伴创建平台 API 示例 ===\n");

    // 1. 会话管理示例
    println!("1. 创建智能伙伴会话");
    let session_request = open_lark::service::aily::models::SessionCreateRequest {
        app_id: aily_app_id.clone(),
        metadata: None,
        tool_set: None,
    };

    match client
        .aily
        .session
        .create_session(session_request, None)
        .await
    {
        Ok(response) => {
            println!("创建会话成功: {response:?}");
            let session_id = "demo_session_id".to_string(); // 使用示例会话ID

            // 2. 消息管理示例
            println!("\n2. 发送消息到智能伙伴");
            let message_request = open_lark::service::aily::models::MessageCreateRequest {
                app_id: aily_app_id.clone(),
                session_id: session_id.clone(),
                content: "你好，请帮我分析一下市场趋势".to_string(),
                message_type: Some("text".to_string()),
                metadata: None,
            };

            match client
                .aily
                .message
                .create_message(message_request, None)
                .await
            {
                Ok(msg_response) => {
                    println!("发送消息成功: {msg_response:?}");

                    // 3. 运行管理示例
                    println!("\n3. 创建智能伙伴运行");
                    let run_request = open_lark::service::aily::models::RunCreateRequest {
                        app_id: aily_app_id.clone(),
                        session_id: session_id.clone(),
                        instructions: Some("请基于用户问题进行详细分析".to_string()),
                        model: Some("your_model_name".to_string()),
                        additional_messages: None,
                        tool_set: None,
                    };

                    match client.aily.run.create_run(run_request, None).await {
                        Ok(run_response) => {
                            println!("创建运行成功: {run_response:?}");
                            let _run_id = "demo_run_id".to_string(); // 使用示例运行ID

                            // 4. 技能管理示例
                            println!("\n4. 查询可用技能列表");
                            let skill_list_request =
                                open_lark::service::aily::models::SkillListRequest {
                                    app_id: aily_app_id.clone(),
                                    page_size: Some(10),
                                    page_token: None,
                                };

                            match client
                                .aily
                                .skill
                                .list_skills(skill_list_request, None)
                                .await
                            {
                                Ok(skills_response) => {
                                    println!("技能列表查询成功: {skills_response:?}");
                                }
                                Err(e) => {
                                    println!("技能列表查询失败: {e:?}");
                                }
                            }

                            // 5. 知识问答示例
                            println!("\n5. 执行数据知识问答");
                            let knowledge_ask_request =
                                open_lark::service::aily::models::DataKnowledgeAskRequest {
                                    app_id: aily_app_id.clone(),
                                    question: "什么是人工智能？".to_string(),
                                    knowledge_base_ids: Some(vec!["kb_001".to_string()]),
                                    chat_history: None,
                                    retrieval_config: None,
                                };

                            match client
                                .aily
                                .knowledge
                                .ask_data_knowledge(knowledge_ask_request, None)
                                .await
                            {
                                Ok(knowledge_response) => {
                                    println!("知识问答成功: {knowledge_response:?}");
                                }
                                Err(e) => {
                                    println!("知识问答失败: {e:?}");
                                }
                            }

                            // 6. 查询会话信息
                            println!("\n6. 查询会话信息");
                            let session_get_request =
                                open_lark::service::aily::models::SessionGetRequest {
                                    app_id: aily_app_id.clone(),
                                    session_id: session_id.clone(),
                                };

                            match client
                                .aily
                                .session
                                .get_session(session_get_request, None)
                                .await
                            {
                                Ok(session_response) => {
                                    println!("查询会话成功: {session_response:?}");
                                }
                                Err(e) => {
                                    println!("查询会话失败: {e:?}");
                                }
                            }
                        }
                        Err(e) => {
                            println!("创建运行失败: {e:?}");
                        }
                    }
                }
                Err(e) => {
                    println!("发送消息失败: {e:?}");
                }
            }
        }
        Err(e) => {
            println!("创建会话失败: {e:?}");
        }
    }

    println!("\n=== 智能伙伴 API 示例完成 ===");
    Ok(())
}

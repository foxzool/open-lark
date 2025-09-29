use log::{debug, error, info, warn};
use open_lark::prelude::*;
use std::time::Instant;

/// 处理接收到的消息内容并发送回显
///
/// # 参数
/// * `content` - 原始消息内容 JSON 字符串
/// * `client` - Lark 客户端实例
/// * `sender_id` - 发送者的 open_id
///
/// # 返回值
/// * `Ok(true)` - 成功发送了回显消息
/// * `Ok(false)` - 消息已处理但未发送回显（非文本消息等）
/// * `Err(e)` - 处理过程中发生错误
async fn handle_message_content(
    content: &str,
    client: &LarkClient,
    sender_id: &str,
) -> Result<bool, Box<dyn std::error::Error + Send + Sync>> {
    debug!("开始解析消息内容: {}", content);

    // 尝试解析消息内容为 JSON
    let content_json: serde_json::Value = match serde_json::from_str(content) {
        Ok(json) => json,
        Err(e) => {
            warn!("无法解析消息内容为 JSON: {e}");
            return Ok(false);
        }
    };

    // 处理不同类型的消息
    match content_json.get("text").and_then(|v| v.as_str()) {
        Some(text) => {
            debug!("解析到文本消息: {}", text);

            // 创建带时间戳和优化格式的回显内容
            let timestamp = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs();

            // 格式化回显消息，包含时间戳和原文
            let echo_content = if text.len() > 100 {
                format!("🔄 Echo [{}]: {}...", timestamp, &text[..97])
            } else {
                format!("🔄 Echo [{}]: {}", timestamp, text)
            };

            // 发送回显消息
            send_echo_message(client, sender_id, &echo_content).await?;

            Ok(true)
        }
        None => {
            // 检查是否是其他类型的消息
            if let Some(msg_type) = content_json.get("type").and_then(|v| v.as_str()) {
                info!("收到非文本消息类型: {}, 跳过回显", msg_type);
            } else {
                warn!("未识别的消息格式，跳过回显");
            }
            Ok(false)
        }
    }
}

/// 发送回显消息，包含重试逻辑和智能错误处理
///
/// # 参数
/// * `client` - Lark 客户端实例
/// * `sender_id` - 接收者的 open_id
/// * `echo_content` - 要发送的回显内容
///
/// # 错误处理
/// - 自动重试最多3次
/// - 对"数据为空"错误进行特殊处理（通常表示消息已成功发送）
/// - 详细的日志记录便于问题排查
async fn send_echo_message(
    client: &LarkClient,
    sender_id: &str,
    echo_content: &str,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    const MAX_RETRIES: u32 = 3;
    const RETRY_DELAY_MS: u64 = 1000;

    for attempt in 1..=MAX_RETRIES {
        debug!(
            "发送回显消息尝试 {}/{}: {}",
            attempt, MAX_RETRIES, echo_content
        );

        // 构建回显消息请求
        let echo_request = CreateMessageRequest::builder()
            .receive_id_type("open_id")
            .request_body(
                CreateMessageRequestBody::builder()
                    .receive_id(sender_id)
                    .msg_type("text")
                    .content(format!("{{\"text\":\"{echo_content}\"}}"))
                    .build(),
            )
            .build();

        // 尝试发送消息
        match client.im.v1.message.create(echo_request, None).await {
            Ok(response) => {
                info!(
                    "✅ Echo 消息发送成功: {} (尝试次数: {})",
                    echo_content, attempt
                );
                debug!(
                    "响应详情 - 消息ID: {}, 创建时间: {}",
                    response.message_id, response.create_time
                );
                return Ok(());
            }
            Err(e) => {
                error!(
                    "❌ Echo 消息发送失败 (尝试 {}/{}): {e:?}",
                    attempt, MAX_RETRIES
                );

                if attempt < MAX_RETRIES {
                    warn!("等待 {}ms 后重试...", RETRY_DELAY_MS);
                    tokio::time::sleep(tokio::time::Duration::from_millis(RETRY_DELAY_MS)).await;
                } else {
                    return Err(
                        format!("经过 {} 次尝试后仍然无法发送消息: {e:?}", MAX_RETRIES).into(),
                    );
                }
            }
        }
    }

    unreachable!()
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 初始化环境变量和日志
    dotenvy::dotenv().ok();

    // 设置日志级别以获取更详细的调试信息
    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "debug");
    }

    env_logger::init();

    let app_id = std::env::var("APP_ID").expect("APP_ID environment variable not set");
    let app_secret = std::env::var("APP_SECRET").expect("APP_SECRET environment variable not set");

    println!("🔌 飞书/Lark WebSocket Echo Bot 示例");
    println!("该示例演示一个增强的 Echo Bot，具有以下功能:");
    println!("  • 智能消息解析和错误处理");
    println!("  • 带时间戳的回显消息");
    println!("  • 自动重试机制和性能监控");
    println!("  • 结构化日志记录");
    println!();
    println!("⚠️  注意: WebSocket 功能需要启用 'websocket' feature 标志");

    #[cfg(not(feature = "websocket"))]
    {
        println!("❌ WebSocket 功能未启用");
        println!(
            "💡 使用以下命令启用: cargo run --example websocket_client --features websocket,im"
        );
        println!();
        println!("🔧 启用 WebSocket 支持的步骤:");
        println!("  1. 在 Cargo.toml 的 features 中添加 'websocket'");
        println!("  2. 使用 EventDispatcherHandler 进行事件处理");
        println!("  3. 为不同事件类型注册事件处理器");
        println!("  4. 配置正确的 APP_ID 和 APP_SECRET 环境变量");
        return Ok(());
    }

    #[cfg(feature = "websocket")]
    {
        use open_lark::client::ws_client::LarkWsClient;
        use std::sync::Arc;

        println!("✅ WebSocket 功能已启用");
        println!("🎯 正在连接到飞书/Lark WebSocket 服务器...");

        // 创建客户端并获取配置
        info!("初始化 Lark 客户端...");
        let client = LarkClient::builder(&app_id, &app_secret)
            .with_app_type(AppType::SelfBuild)
            .with_enable_token_cache(true)
            .build();

        let client_for_echo = Arc::new(client);
        let config = Arc::new(client_for_echo.config.clone());

        // 创建事件处理器，实现增强的 echo bot 功能和完整的 IM 事件监控
        let echo_client = client_for_echo.clone();
        let event_handler = match EventDispatcherHandler::builder()
            // 消息接收事件 - Echo Bot 核心功能
            .register_p2_im_message_receive_v1(move |event| {
                let client = echo_client.clone();
                tokio::spawn(async move {
                    let start_time = Instant::now();

                    info!(
                        "📩 收到消息接收事件 - 事件ID: {:?}, 消息ID: {:?}, 发送者: {:?}",
                        event.header.event_id,
                        event.event.message.message_id,
                        event.event.sender.sender_id.open_id
                    );

                    debug!(
                        "消息详情 - 类型: {:?}, 内容长度: {}, 聊天ID: {}",
                        event.event.message.message_type,
                        event.event.message.content.len(),
                        event.event.message.chat_id
                    );

                    // 检查消息内容是否为空
                    if event.event.message.content.is_empty() {
                        warn!("收到空消息内容，跳过处理");
                        return;
                    }

                    // 解析消息内容并处理回显逻辑
                    match handle_message_content(
                        &event.event.message.content,
                        &client,
                        &event.event.sender.sender_id.open_id,
                    )
                    .await
                    {
                        Ok(echo_sent) => {
                            if echo_sent {
                                info!("✅ Echo 消息处理成功，耗时: {:?}", start_time.elapsed());
                            } else {
                                debug!("消息已处理但未发送回显 (非文本消息)");
                            }
                        }
                        Err(e) => {
                            error!(
                                "❌ 处理消息时发生错误: {e:?}，耗时: {:?}",
                                start_time.elapsed()
                            );
                        }
                    }
                });
            })
            // 消息已读事件
            .and_then(|builder| {
                builder.register_p2_im_message_read_v1(|event| {
                    tokio::spawn(async move {
                        let start_time = Instant::now();

                        info!(
                            "👁️ 收到消息已读事件 - 事件ID: {:?}, 阅读者: {:?}",
                            event.header.event_id, event.event.reader.reader_id.open_id
                        );

                        debug!(
                            "已读事件详情 - 阅读时间: {}, 消息数量: {}, 租户: {}",
                            event.event.reader.read_time,
                            event.event.message_id_list.len(),
                            event.event.reader.tenant_key
                        );

                        if !event.event.message_id_list.is_empty() {
                            debug!("已读消息ID列表: {:?}", event.event.message_id_list);
                        }

                        info!("👁️ 消息已读事件处理完成，耗时: {:?}", start_time.elapsed());
                    });
                })
            })
            // 消息撤回事件
            .and_then(|builder| {
                builder.register_p2_im_message_recalled_v1(|event| {
                    tokio::spawn(async move {
                        let start_time = Instant::now();

                        warn!(
                            "↩️ 收到消息撤回事件 - 事件ID: {:?}, 消息ID: {}, 操作者: {:?}",
                            event.header.event_id,
                            event.event.message_id,
                            event
                                .event
                                .operator
                                .operator_id
                                .user_id
                                .unwrap_or_else(|| "未知".to_string())
                        );

                        debug!(
                            "撤回事件详情 - 撤回时间: {}, 聊天ID: {}, 操作者类型: {}",
                            event.event.recall_time,
                            event.event.chat_info.chat_id,
                            event.event.operator.operator_type
                        );

                        info!("↩️ 消息撤回事件处理完成，耗时: {:?}", start_time.elapsed());
                    });
                })
            })
            // 群聊创建事件
            .and_then(|builder| {
                builder.register_p2_im_chat_created_v1(|event| {
                    tokio::spawn(async move {
                        let start_time = Instant::now();

                        info!(
                            "💬 收到群聊创建事件 - 事件ID: {:?}, 群聊ID: {}, 创建者: {:?}",
                            event.header.event_id,
                            event.event.chat_id,
                            event
                                .event
                                .creator
                                .user_id
                                .user_id
                                .unwrap_or_else(|| "未知".to_string())
                        );

                        debug!(
                            "群聊创建详情 - 名称: {:?}, 描述: {:?}, 类型: {:?}, 创建时间: {}",
                            event.event.name,
                            event.event.description,
                            event.event.chat_type,
                            event.event.create_time
                        );

                        info!("💬 群聊创建事件处理完成，耗时: {:?}", start_time.elapsed());
                    });
                })
            })
            // 群聊更新事件
            .and_then(|builder| {
                builder.register_p2_im_chat_updated_v1(|event| {
                    tokio::spawn(async move {
                        let start_time = Instant::now();

                        info!(
                            "📝 收到群聊更新事件 - 事件ID: {:?}, 群聊ID: {}, 操作者: {:?}",
                            event.header.event_id,
                            event.event.chat_id,
                            event
                                .event
                                .operator
                                .operator_id
                                .user_id
                                .unwrap_or_else(|| "未知".to_string())
                        );

                        debug!(
                            "群聊更新详情 - 更新时间: {}, 操作者类型: {:?}",
                            event.event.update_time, event.event.operator.operator_type
                        );

                        info!("📝 群聊更新事件处理完成，耗时: {:?}", start_time.elapsed());
                    });
                })
            })
            // 群聊解散事件
            .and_then(|builder| {
                builder.register_p2_im_chat_disbanded_v1(|event| {
                    tokio::spawn(async move {
                        let start_time = Instant::now();

                        warn!(
                            "❌ 收到群聊解散事件 - 事件ID: {:?}, 群聊ID: {}, 操作者: {:?}",
                            event.header.event_id,
                            event.event.chat_id,
                            event
                                .event
                                .operator
                                .operator_id
                                .user_id
                                .unwrap_or_else(|| "未知".to_string())
                        );

                        debug!(
                            "群聊解散详情 - 解散时间: {}, 操作者类型: {:?}",
                            event.event.disband_time, event.event.operator.operator_type
                        );

                        info!("❌ 群聊解散事件处理完成，耗时: {:?}", start_time.elapsed());
                    });
                })
            })
            // 群聊成员添加事件
            .and_then(|builder| {
                builder.register_p2_im_chat_member_user_added_v1(|event| {
                    tokio::spawn(async move {
                        let start_time = Instant::now();

                        info!(
                            "➕ 收到成员添加事件 - 事件ID: {:?}, 群聊ID: {}, 添加了 {} 个成员",
                            event.header.event_id,
                            event.event.chat_id,
                            event.event.users.len()
                        );

                        debug!(
                            "成员添加详情 - 操作者: {:?}, 添加时间: {}",
                            event
                                .event
                                .operator
                                .operator_id
                                .user_id
                                .unwrap_or_else(|| "未知".to_string()),
                            event.event.add_time
                        );

                        // 记录添加的用户信息
                        for (i, user) in event.event.users.iter().enumerate() {
                            debug!(
                                "  新成员 {}: {:?} (名称: {:?})",
                                i + 1,
                                user.user_id.user_id.as_ref().unwrap_or(&"未知".to_string()),
                                user.name
                            );
                        }

                        info!("➕ 成员添加事件处理完成，耗时: {:?}", start_time.elapsed());
                    });
                })
            })
            // 群聊成员移除事件
            .and_then(|builder| {
                builder.register_p2_im_chat_member_user_deleted_v1(|event| {
                    tokio::spawn(async move {
                        let start_time = Instant::now();

                        info!(
                            "➖ 收到成员移除事件 - 事件ID: {:?}, 群聊ID: {}, 移除了 {} 个成员",
                            event.header.event_id,
                            event.event.chat_id,
                            event.event.users.len()
                        );

                        debug!(
                            "成员移除详情 - 操作者: {:?}, 移除时间: {}",
                            event
                                .event
                                .operator
                                .operator_id
                                .user_id
                                .unwrap_or_else(|| "未知".to_string()),
                            event.event.delete_time
                        );

                        // 记录被移除的用户信息
                        for (i, user) in event.event.users.iter().enumerate() {
                            debug!(
                                "  被移除成员 {}: {:?} (名称: {:?})",
                                i + 1,
                                user.user_id.user_id.as_ref().unwrap_or(&"未知".to_string()),
                                user.name
                            );
                        }

                        info!("➖ 成员移除事件处理完成，耗时: {:?}", start_time.elapsed());
                    });
                })
            }) {
            Ok(builder) => builder.build(),
            Err(e) => {
                eprintln!("❌ Failed to register event handler: {e}");
                return Err(Box::new(std::io::Error::other(e)) as Box<dyn std::error::Error>);
            }
        };

        info!("📡 事件处理器注册成功！");
        println!("🎉 Echo Bot 已配置完成，支持的功能:");
        println!("  📩 消息接收事件 (p2.im.message.receive_v1)");
        println!("    • 智能文本消息解析");
        println!("    • 带时间戳的 Echo 回复");
        println!("    • 自动重试和错误恢复");
        println!("  👁️  消息已读事件 (p2.im.message.read_v1)");
        println!("    • 实时阅读状态跟踪");
        println!("    • 详细的事件日志记录");
        println!();
        println!("💡 使用指南:");
        println!("  • 向机器人发送文本消息将收到带时间戳的回显");
        println!("  • 所有事件都会记录到日志中便于调试");
        println!("  • 支持消息发送失败的自动重试");
        println!("  • 通过 RUST_LOG=debug 可查看详细日志");
        println!();
        println!("🚀 正在启动 WebSocket 连接...");

        // 启动 WebSocket 客户端
        match LarkWsClient::open(config, event_handler).await {
            Ok(_) => {
                info!("✅ WebSocket 连接已建立");
                println!("🎊 Echo Bot 已成功启动并正在运行！");
                println!("📱 现在可以向机器人发送消息进行测试...");
                println!("🔄 所有收到的文本消息都会被回显");
                println!("📊 事件处理状态会记录到日志中");
                println!();
                println!("💡 提示: 使用 Ctrl+C 停止程序");
            }
            Err(e) => {
                error!("❌ WebSocket 连接失败: {e:?}");
                return Err(format!("WebSocket 连接失败: {e:?}").into());
            }
        }
    }

    Ok(())
}

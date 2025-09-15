/// IM服务现代Builder模式示例
///
/// 这个示例展示了飞书IM消息服务的现代化Builder模式使用方法，
/// 包括消息发送、消息历史查询等操作的统一API调用方式。
use dotenvy::dotenv;
use open_lark::{
    client::LarkClient,
    core::{constants::AppType, trait_system::ExecutableBuilder},
    service::im::v1::message::{CreateMessageRequestBody, MessageText, SendMessageTrait},
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    env_logger::init();

    // 创建客户端
    let client = LarkClient::builder(
        &std::env::var("APP_ID").expect("APP_ID is required"),
        &std::env::var("APP_SECRET").expect("APP_SECRET is required"),
    )
    .with_app_type(AppType::SelfBuild)
    .build();

    println!("=== 飞书IM服务现代Builder模式示例 ===\n");

    // ==========================================
    // 方式一: 传统API调用方式 (仅演示，返回值已改变)
    // ==========================================
    println!("📋 方式一: 传统API调用方式");
    println!("适用于: 现有代码迁移、简单消息发送\n");

    // 构建文本消息
    let text_message = MessageText::new("Hello from traditional API!")
        .add_text(" 这是一条测试消息")
        .at_user("all")
        .build();

    let message_body = CreateMessageRequestBody::builder()
        .receive_id("test_chat_id") // 实际使用时需要真实的chat_id
        .msg_type(text_message.msg_type())
        .content(text_message.content())
        .uuid("demo-uuid-traditional")
        .build();

    let traditional_request = open_lark::service::im::v1::message::CreateMessageRequest::builder()
        .receive_id_type("chat_id")
        .request_body(message_body)
        .build();

    // 注意：现在返回的是Message而不是BaseResponse<Message>
    match client.im.v1.message.create(traditional_request, None).await {
        Ok(message) => {
            println!("✅ 传统方式消息发送成功");
            println!("   消息ID: {}", message.message_id);
            println!("   消息类型: {}", message.msg_type);
        }
        Err(e) => {
            println!("❌ 传统方式发送失败: {e}");
            println!("   这可能是因为权限问题或测试环境限制");
        }
    }

    println!();

    // ==========================================
    // 方式二: 现代Builder模式 (推荐)
    // ==========================================
    println!("🏗️  方式二: 现代Builder模式 (推荐)");
    println!("适用于: 新代码开发、复杂消息构建、链式调用\n");

    // 构建富文本消息
    let rich_text = MessageText::new("📢 现代Builder模式消息:")
        .text_line("✨ 支持链式调用")
        .text_line("🔄 统一错误处理")
        .text_line("⚡ 类型安全保证")
        .at_user("all")
        .build();

    let modern_body = CreateMessageRequestBody::builder()
        .receive_id("test_chat_id") // 实际使用时需要真实的chat_id
        .msg_type(rich_text.msg_type())
        .content(rich_text.content())
        .uuid("demo-uuid-modern")
        .build();

    // 使用现代Builder模式
    let builder_result = open_lark::service::im::v1::message::CreateMessageRequest::builder()
        .receive_id_type("chat_id")
        .request_body(modern_body)
        .execute(&client.im.v1.message)
        .await;

    match builder_result {
        Ok(message) => {
            println!("✅ Builder模式消息发送成功");
            println!("   消息ID: {}", message.message_id);
            println!("   消息类型: {}", message.msg_type);
            println!("   创建时间: {}", message.create_time);

            // 演示消息历史查询
            println!("\n📜 尝试查询消息历史...");
            let list_result = open_lark::service::im::v1::message::ListMessageRequest::builder()
                .container_id_type("chat")
                .container_id("test_chat_id") // 实际使用时需要真实的chat_id
                .page_size(10)
                .sort_type("ByCreateTimeDesc")
                .execute(&client.im.v1.message)
                .await;

            match list_result {
                Ok(list_data) => {
                    println!("✅ 消息历史查询成功");
                    println!("   消息数量: {}", list_data.items.len());
                    println!("   是否有更多: {}", list_data.has_more);

                    // 显示最近的几条消息
                    for (i, msg) in list_data.items.iter().take(3).enumerate() {
                        println!("   消息 {}: {} ({})", i + 1, msg.message_id, msg.msg_type);
                    }
                }
                Err(e) => {
                    println!("❌ 消息历史查询失败: {e}");
                }
            }
        }
        Err(e) => {
            println!("❌ Builder模式发送失败: {e}");
            println!("   这可能是因为权限问题或测试环境限制");
        }
    }

    println!();

    // ==========================================
    // 方式三: Builder模式的高级用法
    // ==========================================
    println!("⚡ 方式三: Builder模式高级用法");
    println!("展示: 条件构建、消息类型多样性、批量操作\n");

    // 模拟不同类型的消息
    let message_types = [
        ("text", "这是文本消息"),
        ("text", "这是另一条文本消息 📝"),
        ("text", "最后一条测试消息 🎉"),
    ];

    for (i, (_msg_type, content)) in message_types.iter().enumerate() {
        let text_msg = MessageText::new(content)
            .add_text(&format!(" [批量消息 {}]", i + 1))
            .build();

        let body = CreateMessageRequestBody::builder()
            .receive_id("test_chat_id")
            .msg_type(text_msg.msg_type())
            .content(text_msg.content())
            .uuid(format!("demo-batch-{i}"))
            .build();

        let message_builder = open_lark::service::im::v1::message::CreateMessageRequest::builder()
            .receive_id_type("chat_id")
            .request_body(body);

        // 条件性配置 - 为某些消息添加特殊标识
        if i == 2 {
            // 最后一条消息可以添加特殊处理
            println!("📌 为最后一条消息添加特殊标识");
        }

        // 执行发送
        match message_builder.execute(&client.im.v1.message).await {
            Ok(message) => {
                println!("✅ 批量消息 {} 发送成功: {}", i + 1, message.message_id);
            }
            Err(e) => {
                println!("❌ 批量消息 {} 发送失败: {}", i + 1, e);
            }
        }
    }

    println!();

    // ==========================================
    // 错误处理最佳实践
    // ==========================================
    println!("🛡️  错误处理最佳实践");
    println!("展示: 统一错误处理、详细错误信息、重试策略\n");

    // 故意创建一个可能失败的请求（无效接收者ID）
    let invalid_body = CreateMessageRequestBody::builder()
        .receive_id("") // 空接收者ID可能导致错误
        .msg_type("text")
        .content("{\"text\":\"测试错误处理\"}")
        .uuid("demo-error-test")
        .build();

    let error_demo_result = open_lark::service::im::v1::message::CreateMessageRequest::builder()
        .receive_id_type("chat_id")
        .request_body(invalid_body)
        .execute(&client.im.v1.message)
        .await;

    match error_demo_result {
        Ok(message) => {
            println!("✅ 意外成功: {}", message.message_id);
        }
        Err(e) => {
            println!("❌ 预期错误示例:");
            println!("   错误信息: {e}");

            // 使用新的错误处理方法
            use open_lark::core::error::LarkAPIError;
            match &e {
                LarkAPIError::APIError { code, msg, .. } => {
                    println!("   错误码: {code}");
                    println!("   错误消息: {msg}");

                    // 根据错误码决定处理策略
                    match *code {
                        9499 => {
                            println!("   💡 建议: 应用没有权限操作指定资源");
                        }
                        1061002 => {
                            println!("   💡 建议: 参数错误，检查receive_id和消息格式");
                        }
                        400 => {
                            println!("   💡 建议: 请求参数错误，检查消息体格式");
                        }
                        _ => {
                            println!("   💡 建议: 检查网络连接和API配置");
                        }
                    }
                }
                LarkAPIError::DataError(msg) => {
                    println!("   数据错误: {msg}");
                    println!("   💡 建议: 检查消息内容和格式");
                }
                _ => {
                    println!("   其他错误类型");
                    println!("   💡 建议: 查看详细日志获取更多信息");
                }
            }
        }
    }

    println!();

    // ==========================================
    // 最佳实践总结
    // ==========================================
    println!("📚 IM服务现代Builder模式最佳实践:");
    println!("1. 🔄 消息发送推荐使用Builder模式，支持链式调用");
    println!("2. 🔧 条件性构建适用于不同消息类型和接收者");
    println!("3. 🛡️  统一错误处理提供详细的消息发送错误诊断");
    println!("4. ⚡ 类型安全保证确保消息格式正确");
    println!("5. 🎯 批量消息发送时建议使用Builder模式提高代码复用性");
    println!("6. 🔍 使用.execute()方法获得一致的异步执行体验");
    println!("7. 📝 实际使用时记得使用真实的chat_id和receive_id");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use open_lark::service::im::v1::message::{
        CreateMessageRequestBody, MessageText, SendMessageTrait,
    };

    #[test]
    fn test_im_builder_pattern_creation() {
        let client = LarkClient::builder("test_app_id", "test_app_secret")
            .with_app_type(AppType::SelfBuild)
            .build();

        // 测试文本消息构建
        let text_message = MessageText::new("Hello World!")
            .add_text(" 测试消息")
            .at_user("test_user")
            .build();

        assert_eq!(text_message.msg_type(), "text");
        assert!(text_message.content().contains("Hello World!"));

        // 测试消息体Builder
        let message_body = CreateMessageRequestBody::builder()
            .receive_id("test_chat_id")
            .msg_type(text_message.msg_type())
            .content(text_message.content())
            .uuid("test-uuid")
            .build();

        assert_eq!(message_body.receive_id, "test_chat_id");
        assert_eq!(message_body.msg_type, "text");
        assert_eq!(message_body.uuid, Some("test-uuid".to_string()));

        // 测试请求Builder
        let request_builder = open_lark::service::im::v1::message::CreateMessageRequest::builder()
            .receive_id_type("chat_id")
            .request_body(message_body);

        let request = request_builder.build();
        // 验证请求构建成功
        assert!(!request.api_req.body.is_empty());
    }

    #[test]
    fn test_message_types() {
        // 测试不同类型的消息构建
        let text_msg = MessageText::new("Simple text").build();
        assert_eq!(text_msg.msg_type(), "text");

        let rich_text = MessageText::new("Rich text")
            .text_line("with line break")
            .at_user("user123")
            .build();
        assert_eq!(rich_text.msg_type(), "text");
        assert!(rich_text.content().contains("\\n"));
        assert!(rich_text.content().contains("user123"));
    }

    #[test]
    fn test_list_message_builder() {
        // 测试消息列表查询Builder
        let list_request = open_lark::service::im::v1::message::ListMessageRequest::builder()
            .container_id_type("chat")
            .container_id("test_chat_id")
            .page_size(20)
            .sort_type("ByCreateTimeDesc")
            .build();

        // 验证查询参数设置正确
        assert!(list_request
            .api_req
            .query_params
            .contains_key("container_id_type"));
        assert!(list_request
            .api_req
            .query_params
            .contains_key("container_id"));
        assert!(list_request.api_req.query_params.contains_key("page_size"));
        assert!(list_request.api_req.query_params.contains_key("sort_type"));
    }
}

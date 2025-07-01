/// 增强错误处理示例
///
/// 这个示例演示如何使用飞书SDK新的错误处理功能，包括：
/// - 智能错误分类和处理建议
/// - 用户友好的错误消息
/// - 自动重试策略
/// - 详细的错误上下文信息
///
/// 使用方法：
/// cargo run --example enhanced_error_handling
///
/// 环境变量：
/// APP_ID=your_app_id (可选，可以使用无效值测试错误处理)
/// APP_SECRET=your_app_secret (可选，可以使用无效值测试错误处理)
use open_lark::core::{error_helper::ErrorHelper, trait_system::ExecutableBuilder};
use open_lark::prelude::*;
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 加载环境变量
    dotenvy::dotenv().ok();

    println!("🛡️ 飞书SDK增强错误处理示例");
    println!("{}", "=".repeat(60));

    // 演示各种错误处理场景
    demonstrate_authentication_errors().await;
    demonstrate_api_error_handling().await;
    demonstrate_retry_strategies().await;
    demonstrate_response_analysis().await;

    Ok(())
}

/// 演示认证错误处理
async fn demonstrate_authentication_errors() {
    println!("\n📋 场景1: 认证错误处理");
    println!("{}", "-".repeat(40));

    // 使用无效的应用凭据创建客户端
    let client = LarkClient::builder("invalid_app_id", "invalid_app_secret")
        .with_enable_token_cache(false)
        .build();

    // 尝试发送消息以触发认证错误
    let message_body = open_lark::service::im::v1::message::CreateMessageRequestBody::builder()
        .receive_id("test_user")
        .msg_type("text")
        .content(json!({"text": "测试消息"}).to_string())
        .build();

    match open_lark::service::im::v1::message::CreateMessageRequest::builder()
        .receive_id_type("open_id")
        .request_body(message_body)
        .execute(&client.im.v1.message)
        .await
    {
        Ok(_) => println!("✅ 意外成功"),
        Err(error) => {
            println!("❌ 捕获到错误（预期）");
            handle_error_with_enhanced_features(&error);
        }
    }
}

/// 演示API错误处理
async fn demonstrate_api_error_handling() {
    println!("\n📋 场景2: API错误分析");
    println!("{}", "-".repeat(40));

    // 模拟不同类型的API错误
    let api_errors = vec![
        (403, "权限不足", "模拟权限错误"),
        (429, "请求频率过高", "模拟限流错误"),
        (500, "内部服务器错误", "模拟服务器错误"),
        (404, "资源不存在", "模拟资源不存在错误"),
    ];

    for (code, message, description) in api_errors {
        println!("\n🔍 {description}: ");
        let error = LarkAPIError::api_error(code, message, Some("req_123456".to_string()));

        // 使用ErrorHelper分析错误
        let advice = ErrorHelper::handle_error(&error);
        println!("   分类: {:?}", advice.category);
        println!(
            "   可恢复: {}",
            if advice.is_recoverable { "是" } else { "否" }
        );
        println!(
            "   可重试: {}",
            if advice.is_retryable { "是" } else { "否" }
        );

        if let Some(delay) = advice.retry_delay {
            println!("   建议延迟: {delay}秒");
        }

        if !advice.actions.is_empty() {
            println!("   建议操作:");
            for action in &advice.actions {
                println!("     - {action}");
            }
        }
    }
}

/// 演示重试策略
async fn demonstrate_retry_strategies() {
    println!("\n📋 场景3: 智能重试策略");
    println!("{}", "-".repeat(40));

    let retryable_errors = vec![
        LarkAPIError::api_error(429, "Too Many Requests", None),
        LarkAPIError::api_error(500, "Internal Server Error", None),
        LarkAPIError::api_error(503, "Service Unavailable", None),
    ];

    for error in retryable_errors {
        println!("\n🔄 错误: {}", ErrorHelper::format_user_error(&error));

        if let Some(strategy) = ErrorHelper::create_retry_strategy(&error) {
            println!("   重试策略:");
            println!("   - 最大重试次数: {}", strategy.max_attempts);
            println!("   - 基础延迟: {:?}", strategy.base_delay);
            println!(
                "   - 指数退避: {}",
                if strategy.use_exponential_backoff {
                    "启用"
                } else {
                    "禁用"
                }
            );

            // 展示延迟计算
            println!("   - 重试延迟序列:");
            for attempt in 0..strategy.max_attempts {
                let delay = strategy.calculate_delay(attempt);
                println!("     第{}次重试: {:?}", attempt + 1, delay);
            }
        } else {
            println!("   此错误不建议重试");
        }
    }
}

/// 演示响应分析
async fn demonstrate_response_analysis() {
    println!("\n📋 场景4: 响应分析和错误诊断");
    println!("{}", "-".repeat(40));

    // 模拟各种API响应
    let mock_responses = vec![
        (0, "success", "成功响应"),
        (403, "Forbidden", "权限不足响应"),
        (99991671, "access_token_invalid", "访问令牌无效响应"),
    ];

    for (code, message, description) in mock_responses {
        println!("\n🔍 分析{description}: ");

        // 创建模拟响应
        let raw_response = open_lark::core::api_resp::RawResponse {
            code,
            msg: message.to_string(),
            err: None,
        };

        let response = open_lark::core::api_resp::BaseResponse {
            raw_response,
            data: Some("mock_data".to_string()),
        };

        // 使用新的便利方法分析响应
        if response.success() {
            println!("   ✅ 请求成功");
        } else {
            println!("   ❌ 请求失败");

            if let Some(error_code) = response.error_code() {
                println!("   错误码: {error_code} ({code})");
                println!("   详细描述: {}", error_code.detailed_description());
                println!("   错误分类: {:?}", error_code.category());
            }

            // 获取用户友好的错误信息
            if let Some(friendly_error) = response.user_friendly_error() {
                println!("   用户友好错误: {friendly_error}");
            }

            // 获取解决方案建议
            let solutions = response.error_solutions();
            if !solutions.is_empty() {
                println!("   解决方案:");
                for solution in solutions {
                    println!("     - {solution}");
                }
            }

            // 检查重试建议
            if response.is_retryable() {
                if let Some(delay) = response.suggested_retry_delay() {
                    println!("   ⏱️ 建议重试延迟: {delay}秒");
                }
            }

            // 获取帮助链接
            let help_links = response.help_links();
            if !help_links.is_empty() {
                println!("   📚 相关文档:");
                for (name, url) in help_links {
                    println!("     {name}: {url}");
                }
            }
        }
    }
}

/// 使用增强功能处理错误
fn handle_error_with_enhanced_features(error: &LarkAPIError) {
    println!("\n🔧 增强错误处理分析:");

    // 1. 获取用户友好的错误消息
    let user_message = ErrorHelper::format_user_error(error);
    println!("   用户消息: {user_message}");

    // 2. 创建完整的错误上下文
    let context = ErrorHelper::create_error_context(error);
    println!("   错误分类: {:?}", context.category);
    println!(
        "   可恢复性: {}",
        if context.is_recoverable {
            "可恢复"
        } else {
            "需人工干预"
        }
    );

    // 3. 展示建议操作
    if !context.suggested_actions.is_empty() {
        println!("   建议操作:");
        for (i, action) in context.suggested_actions.iter().enumerate() {
            println!("     {}. {}", i + 1, action);
        }
    }

    // 4. 展示重试策略
    if let Some(strategy) = &context.retry_strategy {
        println!(
            "   重试策略: 最多{}次，基础延迟{:?}",
            strategy.max_attempts, strategy.base_delay
        );
    }

    // 5. 展示帮助链接
    if let Some(help_url) = &context.help_url {
        println!("   帮助文档: {help_url}");
    }

    // 6. 使用便利方法打印详细信息
    println!("\n📊 详细错误信息:");
    context.print_details();
}

/// 演示实际API调用中的错误处理模式
#[allow(dead_code)]
async fn enhanced_api_call_example() -> Result<(), Box<dyn std::error::Error>> {
    let app_id = std::env::var("APP_ID").unwrap_or_else(|_| "demo_app_id".to_string());
    let app_secret = std::env::var("APP_SECRET").unwrap_or_else(|_| "demo_app_secret".to_string());

    let client = LarkClient::builder(&app_id, &app_secret)
        .with_enable_token_cache(true)
        .build();

    let message_body = open_lark::service::im::v1::message::CreateMessageRequestBody::builder()
        .receive_id("demo_user")
        .msg_type("text")
        .content(json!({"text": "测试消息"}).to_string())
        .build();

    // 使用增强的错误处理模式
    match open_lark::service::im::v1::message::CreateMessageRequest::builder()
        .receive_id_type("open_id")
        .request_body(message_body)
        .execute(&client.im.v1.message)
        .await
    {
        Ok(response) => {
            // 处理响应
            println!("✅ 消息发送成功: {}", response.message_id);
        }
        Err(error) => {
            println!("❌ API调用失败");

            // 创建错误上下文并处理
            let context = ErrorHelper::create_error_context(&error);

            // 判断是否可以重试
            if context.is_retryable {
                if let Some(strategy) = &context.retry_strategy {
                    println!("🔄 错误可重试，建议延迟 {:?} 后重试", strategy.base_delay);

                    // 模拟重试逻辑（这里只是展示概念）
                    println!("⏰ 建议延迟 {:?} 后重试", strategy.base_delay);
                }
            } else {
                println!("🚫 错误不可重试，需要人工干预");

                // 打印详细的错误分析
                context.print_details();
            }

            return Err(error.into());
        }
    }

    Ok(())
}

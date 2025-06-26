/// 响应处理模式示例
///
/// 这个示例演示如何使用BaseResponse的新增便利方法来处理API响应：
/// - data_or_error() - 获取数据或友好错误消息
/// - data_or_api_error() - 获取数据或转换为LarkAPIError
/// - handle_common_errors() - 处理常见错误场景
/// - print_error_details() - 打印详细错误信息
/// - 各种错误检查方法
///
/// 使用方法：
/// cargo run --example response_handling_patterns
use open_lark::core::{
    api_resp::{BaseResponse, RawResponse},
    error_codes::LarkErrorCode,
};
use open_lark::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();

    println!("📊 飞书SDK响应处理模式示例");
    println!("{}", "=".repeat(50));

    // 演示不同的响应处理模式
    demonstrate_data_extraction_patterns().await;
    demonstrate_error_checking_patterns().await;
    demonstrate_common_error_handling().await;
    demonstrate_response_analysis().await;

    Ok(())
}

/// 演示数据提取模式
async fn demonstrate_data_extraction_patterns() {
    println!("\n📋 模式1: 数据提取和错误处理");
    println!("{}", "-".repeat(40));

    // 模拟不同的响应场景
    let scenarios = vec![
        (0, "success", Some("valid_data".to_string()), "成功响应"),
        (0, "success", None, "成功但无数据响应"),
        (404, "not found", None, "资源不存在响应"),
        (403, "forbidden", None, "权限不足响应"),
    ];

    for (code, message, data, description) in scenarios {
        println!("\n🔍 测试{}: ", description);

        let response = create_mock_response(code, message, data.clone());

        // 模式1: 使用 data_or_error() 获取友好错误消息
        match response.data_or_error() {
            Ok(data) => println!("   ✅ 成功获取数据: {}", data),
            Err(error_msg) => println!("   ❌ 友好错误: {}", error_msg),
        }

        let response = create_mock_response(code, message, data.clone());

        // 模式2: 使用 data_or_api_error() 获取详细错误
        match response.data_or_api_error() {
            Ok(data) => println!("   ✅ 成功获取数据: {}", data),
            Err(api_error) => {
                println!("   ❌ API错误: {}", api_error);
                // 可以进一步使用ErrorHelper分析这个错误
            }
        }
    }
}

/// 演示错误检查模式
async fn demonstrate_error_checking_patterns() {
    println!("\n📋 模式2: 错误类型检查");
    println!("{}", "-".repeat(40));

    let error_codes = vec![
        (99991671, "访问令牌无效"),
        (403, "权限不足"),
        (429, "请求频率过高"),
        (500, "内部服务器错误"),
        (404, "资源不存在"),
    ];

    for (code, description) in error_codes {
        println!("\n🔍 检查错误: {} ({})", description, code);

        let response = create_mock_response::<String>(code, description, None);

        // 使用便利方法检查错误类型
        println!("   认证错误: {}", response.is_auth_error());
        println!("   权限错误: {}", response.is_permission_error());
        println!("   客户端错误: {}", response.is_client_error());
        println!("   服务器错误: {}", response.is_server_error());
        println!("   可重试: {}", response.is_retryable());

        if let Some(delay) = response.suggested_retry_delay() {
            println!("   建议重试延迟: {}秒", delay);
        }

        // 检查特定错误码
        if let Some(error_code) = response.error_code() {
            println!("   错误码类型: {}", error_code);
            println!(
                "   是否为访问令牌无效: {}",
                response.is_error_code(LarkErrorCode::AccessTokenInvalid)
            );
        }
    }
}

/// 演示通用错误处理
async fn demonstrate_common_error_handling() {
    println!("\n📋 模式3: 通用错误处理");
    println!("{}", "-".repeat(40));

    let common_errors = vec![
        (99991671, "access_token_invalid", "访问令牌无效"),
        (99991664, "app_access_token_invalid", "应用访问令牌无效"),
        (403, "forbidden", "权限不足"),
        (429, "too_many_requests", "请求频率过高"),
        (404, "not_found", "资源不存在"),
        (1001, "unknown_error", "未知错误"),
    ];

    for (code, message, description) in common_errors {
        println!("\n🔧 处理{}: ", description);

        let response = create_mock_response::<String>(code, message, None);

        // 使用 handle_common_errors() 处理常见错误
        match response.handle_common_errors() {
            Ok(handled_response) => {
                if handled_response.success() {
                    println!("   ✅ 响应正常");
                } else {
                    println!("   ⚠️ 需要调用者自行处理的错误");
                    println!("   错误信息: {}", handled_response.msg());
                }
            }
            Err(handled_error) => {
                println!("   🛠️ 已转换为具体错误: {}", handled_error);

                // 展示错误的用户友好消息
                let friendly_msg =
                    open_lark::core::error_helper::ErrorHelper::format_user_error(&handled_error);
                println!("   用户友好消息: {}", friendly_msg);
            }
        }
    }
}

/// 演示响应分析
async fn demonstrate_response_analysis() {
    println!("\n📋 模式4: 详细响应分析");
    println!("{}", "-".repeat(40));

    let analysis_cases = vec![
        (403, "权限不足测试"),
        (99991671, "令牌无效测试"),
        (429, "限流测试"),
    ];

    for (code, description) in analysis_cases {
        println!("\n📊 分析场景: {}", description);

        let response = create_mock_response::<String>(code, "error_message", None);

        // 获取用户友好的错误描述
        if let Some(friendly_error) = response.user_friendly_error() {
            println!("   用户友好错误: {}", friendly_error);
        }

        // 获取解决方案建议
        let solutions = response.error_solutions();
        if !solutions.is_empty() {
            println!("   解决方案建议:");
            for (i, solution) in solutions.iter().enumerate() {
                println!("     {}. {}", i + 1, solution);
            }
        }

        // 获取帮助链接
        let help_links = response.help_links();
        if !help_links.is_empty() {
            println!("   相关文档:");
            for (name, url) in help_links {
                println!("     {}: {}", name, url);
            }
        }

        // 使用详细错误打印功能
        println!("\n   📋 详细错误报告:");
        response.print_error_details();
    }
}

/// 创建模拟响应的助手函数
fn create_mock_response<T>(code: i32, message: &str, data: Option<T>) -> BaseResponse<T> {
    BaseResponse {
        raw_response: RawResponse {
            code,
            msg: message.to_string(),
            err: None,
        },
        data,
    }
}

/// 实际API调用中的最佳实践示例
#[allow(dead_code)]
async fn api_call_best_practices() -> Result<(), Box<dyn std::error::Error>> {
    let app_id = std::env::var("APP_ID").unwrap_or_else(|_| "demo_app_id".to_string());
    let app_secret = std::env::var("APP_SECRET").unwrap_or_else(|_| "demo_app_secret".to_string());

    let _client = LarkClient::builder(&app_id, &app_secret)
        .with_enable_token_cache(true)
        .build();

    // 示例：获取用户信息
    // let user_request = ...;

    // 最佳实践模式
    println!("🚀 API调用最佳实践");

    // 模拟API调用
    // match client.some_service.some_operation(&request).await {
    //     Ok(response) => {
    //         // 模式1: 快速数据提取
    //         match response.data_or_error() {
    //             Ok(data) => {
    //                 println!("✅ 获取数据成功");
    //                 // 处理数据...
    //             }
    //             Err(error_msg) => {
    //                 println!("❌ 获取数据失败: {}", error_msg);
    //
    //                 // 模式2: 详细错误分析
    //                 if response.is_retryable() {
    //                     println!("🔄 可以重试");
    //                     if let Some(delay) = response.suggested_retry_delay() {
    //                         println!("   建议延迟: {}秒", delay);
    //                     }
    //                 }
    //
    //                 // 模式3: 显示解决方案
    //                 let solutions = response.error_solutions();
    //                 if !solutions.is_empty() {
    //                     println!("💡 解决方案:");
    //                     for solution in solutions {
    //                         println!("   - {}", solution);
    //                     }
    //                 }
    //
    //                 return Err(error_msg.into());
    //             }
    //         }
    //     }
    //     Err(error) => {
    //         println!("❌ API调用失败");
    //
    //         // 使用ErrorHelper进行完整的错误分析
    //         let context = open_lark::core::error_helper::ErrorHelper::create_error_context(&error);
    //         context.print_details();
    //
    //         return Err(error.into());
    //     }
    // }

    Ok(())
}

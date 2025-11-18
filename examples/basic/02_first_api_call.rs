/**
 * OpenLark SDK 第一个API调用示例
 *
 * 本示例展示了如何使用 OpenLark SDK 进行第一次API调用，包括：
 * - 完整的API调用流程
 * - 请求参数构建和响应处理
 * - 不同服务模块的API调用示例
 * - 构建器模式 vs 传统模式
 * - 错误处理和重试机制
 *
 * 运行方法：
 * cargo run --example 02_first_api_call
 */

use openlark_core::prelude::*;
use openlark_client::minimal::{MinimalLarkClient, AuthClient};
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 加载环境变量
    dotenvy::dotenv().ok();

    println!("🚀 OpenLark SDK 第一个API调用示例");
    println!("===================================");
    println!();

    // 从环境变量获取配置
    let app_id = std::env::var("OPENLARK_APP_ID")
        .expect("请设置环境变量 OPENLARK_APP_ID");
    let app_secret = std::env::var("OPENLARK_APP_SECRET")
        .expect("请设置环境变量 OPENLARK_APP_SECRET");

    // 创建客户端
    let client = create_client(&app_id, &app_secret)?;

    println!("📋 API调用流程说明");
    println!("==================");
    println!("1. 🔐 客户端认证 - SDK自动处理");
    println!("2. 📝 构建请求 - 使用请求结构体或构建器");
    println!("3. 📞 发送API - 调用对应服务方法");
    println!("4. 📊 处理响应 - 解析返回数据");
    println!("5. ⚠️  错误处理 - 处理可能的异常");
    println!();

    // === 示例1: 应用信息API ===
    println!("📋 示例1: 获取应用信息");
    println!("--------------------");

    match demo_get_app_info(&client).await {
        Ok(_) => println!("✅ 应用信息API调用成功"),
        Err(e) => println!("❌ 应用信息API调用失败: {}", e),
    }
    println!();

    // === 示例2: 构建器模式API调用 ===
    println!("📋 示例2: 构建器模式调用");
    println!("----------------------");

    if let Some(user_id) = std::env::var("OPENLARK_TEST_USER_ID").ok() {
        match demo_builder_pattern(&client, &user_id).await {
            Ok(_) => println!("✅ 构建器模式调用成功"),
            Err(e) => println!("❌ 构建器模式调用失败: {}", e),
        }
    } else {
        println!("ℹ️  跳过构建器模式演示");
        println!("💡 设置 OPENLARK_TEST_USER_ID 环境变量来运行此示例");
    }
    println!();

    // === 示例3: 分页API调用 ===
    println!("📋 示例3: 分页数据处理");
    println!("----------------------");

    match demo_pagination(&client).await {
        Ok(_) => println!("✅ 分页API调用成功"),
        Err(e) => println!("❌ 分页API调用失败: {}", e),
    }
    println!();

    // === 示例4: 批量操作 ===
    println!("📋 示例4: 批量操作处理");
    println!("----------------------");

    match demo_batch_operations(&client).await {
        Ok(_) => println!("✅ 批量操作演示成功"),
        Err(e) => println!("❌ 批量操作演示失败: {}", e),
    }
    println!();

    // === API调用最佳实践总结 ===
    println!("💡 API调用最佳实践");
    println!("==================");
    println!("1. 🔄 错误处理:");
    println!("   • 使用 ? 操作符或 match 处理错误");
    println!("   • 检查 StandardResponse 的 success 字段");
    println!("   • 实现重试机制处理临时错误");
    println!();
    println!("2. 📊 数据验证:");
    println!("   • 验证响应数据的存在性");
    println!("   • 检查数据字段的合理性");
    println!("   • 处理空数据或缺失字段");
    println!();
    println!("3. 🚀 性能优化:");
    println!("   • 启用令牌缓存减少认证请求");
    println!("   • 使用并发处理提升效率");
    println!("   • 合理设置超时和重试参数");
    println!();
    println!("4. 🛡️ 安全考虑:");
    println!("   • 不在代码中硬编码敏感信息");
    println!("   • 使用环境变量存储配置");
    println!("   • 定期轮换应用密钥");

    Ok(())
}

/**
 * 创建客户端
 */
fn create_client(app_id: &str, app_secret: &str) -> Result<MinimalLarkClient, Box<dyn std::error::Error>> {
    let client = MinimalLarkClient::new(app_id.to_string(), app_secret.to_string())?;
    Ok(client)
}

/**
 * 示例1: 获取应用访问令牌
 * 演示最基本的认证API调用流程
 */
async fn demo_get_app_info(client: &MinimalLarkClient) -> Result<(), Box<dyn std::error::Error>> {
    println!("📞 调用应用认证API...");

    // 发送API请求获取应用访问令牌
    match client.get_app_access_token().await {
        Ok(token) => {
            println!("✅ API调用成功");
            println!("📱 令牌信息:");
            println!("   • 访问令牌: {}...", &token.access_token[..token.access_token.len().min(20)]);
            println!("   • 令牌类型: {:?}", token.token_type);
            println!("   • 过期时间: {} 秒", token.expires_in);

            println!("💡 这是最基础的API调用，认证成功后即可调用其他业务API");
        }
        Err(e) => {
            println!("❌ API调用失败");
            println!("   错误信息: {}", e);
        }
    }

    Ok(())
}

/**
 * 示例2: API调用模式说明
 * 演示当前最小客户端支持的使用模式
 */
async fn demo_builder_pattern(_client: &MinimalLarkClient, _user_id: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("📞 API调用模式说明...");

    // 当前最小客户端主要专注于认证功能
    println!("🔄 当前支持的模式:");
    println!("   • 直接方法调用: client.get_app_access_token().await");
    println!("   • 环境变量创建: MinimalLarkClient::from_env()");
    println!("   • 手动配置创建: MinimalLarkClient::new(app_id, app_secret)");

    // 未来扩展计划
    println!("🏗️  未来扩展计划:");
    println!("   • 构建器模式: 链式调用，参数清晰");
    println!("   • 类型安全: 编译时检查");
    println!("   • 条件构建: 灵活配置");
    println!("   • 业务API: 消息、文档、联系人等");

    Ok(())
}

/**
 * 示例3: 分页数据处理概念
 * 演示分页API的处理思路
 */
async fn demo_pagination(_client: &MinimalLarkClient) -> Result<(), Box<dyn std::error::Error>> {
    println!("📞 分页数据处理概念演示...");

    println!("📋 分页API的一般处理模式:");
    println!("   1️⃣ 设置页面大小: page_size = 10");
    println!("   2️⃣ 第一页请求: page_token = None");
    println!("   3️⃣ 检查响应: has_more, page_token");
    println!("   4️⃣ 循环获取: 使用下一页token继续");
    println!("   5️⃣ 合并数据: 将所有页面数据合并");

    // 模拟分页处理逻辑
    println!("🔄 模拟分页处理:");
    let mut page_count = 0;
    let mut total_items = 0;
    let mut has_more = true;
    let mut page_token: Option<String> = None;

    while has_more && page_count < 3 {  // 最多模拟3页
        page_count += 1;
        let page_size = 10;
        let items_count = std::cmp::min(page_size, 25 - total_items); // 模拟总共25条数据

        println!("   📄 第{}页: {}条记录", page_count, items_count);

        total_items += items_count;
        has_more = total_items < 25;

        if has_more {
            page_token = Some(format!("page_token_{}", page_count));
            println!("      • 还有更多数据，下一页token: {:?}", page_token);
        } else {
            println!("      • 已获取全部数据");
        }
    }

    println!("📊 分页结果: 共{}页，{}条记录", page_count, total_items);
    Ok(())
}

/**
 * 示例4: 批量操作处理概念
 * 演示批量操作的设计思路和性能优化
 */
async fn demo_batch_operations(_client: &MinimalLarkClient) -> Result<(), Box<dyn std::error::Error>> {
    println!("📞 批量操作处理概念演示...");

    // 模拟批量数据
    let items = vec![
        "item_1", "item_2", "item_3", "item_4", "item_5"
    ];

    println!("🔄 批量处理 {} 个项目...", items.len());

    // 方法1：串行处理（简单但慢）
    println!("📌 串行处理方式:");
    let start_time = std::time::Instant::now();

    let mut serial_success = 0;
    for (i, item) in items.iter().enumerate() {
        // 模拟API调用延迟
        tokio::time::sleep(std::time::Duration::from_millis(100)).await;

        // 模拟90%成功率
        if i % 10 != 0 {
            serial_success += 1;
            println!("   ✅ 项目 {} 处理完成", item);
        } else {
            println!("   ❌ 项目 {} 处理失败", item);
        }
    }

    let serial_time = start_time.elapsed();
    println!("   📊 串行处理耗时: {:?}，成功: {}/{}", serial_time, serial_success, items.len());

    // 方法2：并发处理（高效但复杂）
    println!("📌 并发处理方式:");

    let start_time = std::time::Instant::now();

    // 使用 futures::stream 处理并发请求
    use futures::stream::{self, StreamExt};

    let futures: Vec<_> = items.iter().enumerate().map(|(i, item)| {
        let item = item.to_string();
        async move {
            // 模拟API调用延迟
            tokio::time::sleep(std::time::Duration::from_millis(100)).await;

            // 模拟90%成功率
            if i % 10 != 0 {
                Ok(item)
            } else {
                Err("模拟失败".to_string())
            }
        }
    }).collect();

    let results: Vec<_> = stream::iter(futures)
        .buffer_unordered(3) // 最多3个并发请求
        .collect()
        .await;

    let concurrent_time = start_time.elapsed();

    let success_count = results.iter().filter(|r: &&Result<_, _>| r.is_ok()).count();
    println!("   📊 并发处理耗时: {:?}", concurrent_time);
    println!("   ✅ 成功处理: {}/{}", success_count, items.len());

    if concurrent_time < serial_time {
        let speedup = serial_time.as_secs_f64() / concurrent_time.as_secs_f64();
        println!("   🚀 性能提升: {:.2}x", speedup);
    }

    println!("💡 批量操作最佳实践:");
    println!("   • 控制并发数量，避免压垮服务器");
    println!("   • 实现错误重试机制");
    println!("   • 使用流式处理处理大数据集");
    println!("   • 考虑断点续传和进度跟踪");

    Ok(())
}

/**
 * API响应处理说明
 */
fn handle_api_response_example<T: std::fmt::Debug>(result: &Result<T, openlark_core::error::LarkAPIError>, operation: &str) -> bool {
    match result {
        Ok(data) => {
            println!("✅ {} 成功", operation);
            println!("📊 响应数据: {:?}", data);
            true
        }
        Err(error) => {
            println!("❌ {} 失败", operation);
            println!("   错误信息: {}", error);
            false
        }
    }
}

/**
 * 重试机制示例
 */
async fn retry_with_backoff<F, T, E>(
    operation: F,
    max_retries: u32,
    initial_delay: Duration
) -> Result<T, E>
where
    F: Fn() -> futures::future::BoxFuture<'static, Result<T, E>>,
    E: std::fmt::Debug,
{
    let mut delay = initial_delay;

    for attempt in 1..=max_retries {
        match operation().await {
            Ok(result) => return Ok(result),
            Err(e) => {
                if attempt == max_retries {
                    return Err(e);
                }

                println!("⚠️  第{}次尝试失败，{:?}后重试...", attempt, delay);
                tokio::time::sleep(delay).await;
                delay *= 2; // 指数退避
            }
        }
    }

    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_creation() {
        let app_id = "test_app_id";
        let app_secret = "test_app_secret";

        let result = create_client(app_id, app_secret);
        assert!(result.is_ok(), "客户端创建应该成功");
    }

    #[test]
    fn test_response_handling() {
        // 创建测试结果
        let success_result: Result<String, openlark_core::error::LarkAPIError> = Ok("test data".to_string());
        let failure_result: Result<String, openlark_core::error::LarkAPIError> =
            Err(openlark_core::error::LarkAPIError::illegal_param("test error"));

        let success_handled = handle_api_response_example(&success_result, "测试成功操作");
        let failure_handled = handle_api_response_example(&failure_result, "测试失败操作");

        assert!(success_handled, "成功结果应该被正确处理");
        assert!(!failure_handled, "失败结果应该被正确处理");
    }

    #[tokio::test]
    async fn test_retry_mechanism() {
        use futures::future::BoxFuture;

        let mut attempt_count = 0;
        let operation = || -> BoxFuture<Result<String, String>> {
            Box::pin(async move {
                attempt_count += 1;
                if attempt_count < 3 {
                    Err("模拟失败".to_string())
                } else {
                    Ok("成功".to_string())
                }
            })
        };

        let result = retry_with_backoff(operation, 5, Duration::from_millis(10)).await;
        assert!(result.is_ok(), "重试机制应该最终成功");
        assert_eq!(attempt_count, 3, "应该重试3次");
    }
}
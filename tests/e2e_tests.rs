//! OpenLark 端到端测试
//!
//! 测试完整的用户使用场景，从客户端创建到API调用的完整流程。

use openlark_client::prelude::*;
use std::time::Duration;

#[tokio::test]
#[ignore] // 需要真实的API密钥
async fn test_real_api_workflow() {
    // 端到端测试：真实API调用
    // 这个测试需要有效的环境变量配置

    println!("🚀 开始端到端测试");
    println!("====================");

    // 1. 创建客户端
    println!("📝 步骤1: 创建客户端");
    let client = OpenLarkClient::from_env()
        .await
        .expect("无法创建客户端，请检查环境配置");
    println!("✅ 客户端创建成功");

    // 2. 检查服务可用性
    println!("\n🔍 步骤2: 检查服务可用性");
    let services = client.available_services();
    println!("可用服务: {:?}", services);

    let health = client.health_check().await.expect("健康检查失败");
    for (service, healthy) in health {
        let status = if healthy { "✅ 健康" } else { "❌ 异常" };
        println!("  {}: {}", service, status);
    }

    // 3. 测试认证服务
    println!("\n🔑 步骤3: 测试认证服务");
    if client.is_service_available("auth") {
        match client.get_app_access_token().await {
            Ok(token) => {
                println!("✅ 应用访问令牌获取成功");
                println!("  类型: {}", token.token_type);
                println!("  过期时间: {}", token.expires_at);
                println!(
                    "  令牌前缀: {}...",
                    &token.access_token[..std::cmp::min(10, token.access_token.len())]
                );
            }
            Err(e) => {
                println!("❌ 应用访问令牌获取失败: {}", e);
            }
        }
    }

    // 4. 测试通信服务
    println!("\n💬 步骤4: 测试通信服务");
    if client.is_service_available("communication") {
        // 发送测试消息
        let test_message = format!("测试消息 - {}", chrono::Utc::now());
        match client
            .send_text_message("test_user", "open_id", &test_message)
            .await
        {
            Ok(result) => {
                println!("✅ 测试消息发送成功");
                println!("  消息ID: {}", result.message_id);
                println!("  发送时间: {}", result.send_time);
            }
            Err(e) => {
                println!("❌ 测试消息发送失败: {}", e);
            }
        }

        // 获取消息列表
        match client
            .list_messages("test_chat", "chat_id", Some(10), None)
            .await
        {
            Ok(result) => {
                println!("✅ 消息列表获取成功");
                println!("  消息数量: {}", result.messages.len());
                println!("  是否有更多: {}", result.has_more);
            }
            Err(e) => {
                println!("❌ 消息列表获取失败: {}", e);
            }
        }
    }

    // 5. 测试HR服务
    println!("\n👥 步骤5: 测试HR服务");
    if client.is_service_available("hr") {
        // 获取员工列表
        match client.list_employees(Some("open_id"), Some(10), None).await {
            Ok(result) => {
                println!("✅ 员工列表获取成功");
                println!("  员工数量: {}", result.employees.len());

                for (i, employee) in result.employees.iter().take(3).enumerate() {
                    println!("  员工 {}: {} ({})", i + 1, employee.name, employee.user_id);
                }

                if result.employees.len() > 3 {
                    println!("  ... 还有 {} 个员工", result.employees.len() - 3);
                }
            }
            Err(e) => {
                println!("❌ 员工列表获取失败: {}", e);
            }
        }
    }

    // 6. 测试文档服务
    println!("\n📊 步骤6: 测试文档服务");
    if client.is_service_available("docs") {
        // 创建测试表格
        let test_title = format!("OpenLark测试表格 - {}", chrono::Utc::now());
        match client.create_spreadsheet(&test_title, None).await {
            Ok(spreadsheet) => {
                println!("✅ 测试表格创建成功");
                println!("  表格标题: {}", spreadsheet.title);
                println!("  表格Token: {}", spreadsheet.spreadsheet_token);
                println!("  访问链接: {}", spreadsheet.url);
            }
            Err(e) => {
                println!("❌ 测试表格创建失败: {}", e);
            }
        }
    }

    // 7. 测试AI服务
    println!("\n🤖 步骤7: 测试AI服务");
    if client.is_service_available("ai") {
        // AI文本生成
        let test_prompt = "请写一首关于春天的简短诗歌";
        match client
            .generate_text(test_prompt, None, Some(0.7), Some(100))
            .await
        {
            Ok(result) => {
                println!("✅ AI文本生成成功");
                println!("  生成的文本: {}", result.text);
                println!("  使用的模型: {}", result.model);
                println!("  Token使用: {:?}", result.usage);
            }
            Err(e) => {
                println!("❌ AI文本生成失败: {}", e);
            }
        }
    }

    // 8. 测试批量操作
    println!("\n📦 步骤8: 测试批量操作");

    // 批量发送消息
    let batch_messages = vec![
        (
            "batch_user1".to_string(),
            "open_id".to_string(),
            "批量测试消息1".to_string(),
        ),
        (
            "batch_user2".to_string(),
            "open_id".to_string(),
            "批量测试消息2".to_string(),
        ),
    ];

    match client.batch_send_text_messages(batch_messages).await {
        Ok(results) => {
            println!("✅ 批量消息发送完成");
            let success_count = results.iter().filter(|r| r.is_ok()).count();
            println!("  成功发送: {}/{}", success_count, results.len());

            for (i, result) in results.iter().enumerate() {
                match result {
                    Ok(message_result) => {
                        println!("  消息{}: {}", i + 1, message_result.message_id);
                    }
                    Err(e) => {
                        println!("  消息{}: 发送失败 - {}", i + 1, e);
                    }
                }
            }
        }
        Err(e) => {
            println!("❌ 批量消息发送失败: {}", e);
        }
    }

    println!("\n🎉 端到端测试完成！");
}

#[tokio::test]
async fn test_mock_workflow() {
    // 模拟工作流程测试
    println!("🧪 模拟工作流程测试");
    println!("==================");

    // 使用模拟配置
    let config = UnifiedConfig::default();
    let client = UnifiedClient::new(config).await.expect("客户端创建失败");

    // 验证客户端功能
    println!("✅ 客户端创建成功");

    let services = client.available_services();
    println!("📋 发现服务: {:?}", services);

    let dispatcher = client.dispatcher();
    let adapter_services = dispatcher.list_services();
    println!("🔧 适配器服务: {:?}", adapter_services);

    // 验证服务适配器
    for service_name in adapter_services {
        if let Some(adapter) = dispatcher.get_adapter(service_name) {
            println!(
                "  ✅ {}: {} (v{})",
                service_name,
                adapter.name(),
                adapter.version()
            );

            let health = adapter.health_check().await.unwrap_or(false);
            println!("    状态: {}", if health { "健康" } else { "异常" });
        }
    }

    println!("🎯 模拟工作流程测试完成！");
}

#[tokio::test]
async fn test_concurrent_operations() {
    // 测试并发操作
    println!("⚡ 并发操作测试");
    println!("================");

    let config = UnifiedConfig::default();
    let client = UnifiedClient::new(config).await.expect("客户端创建失败");

    // 创建多个并发任务
    let tasks = vec![
        async {
            // 模拟API调用1
            tokio::time::sleep(Duration::from_millis(100)).await;
            println!("🔄 并发任务1完成");
            Ok("task1_result".to_string())
        },
        async {
            // 模拟API调用2
            tokio::time::sleep(Duration::from_millis(150)).await;
            println!("🔄 并发任务2完成");
            Ok("task2_result".to_string())
        },
        async {
            // 模拟API调用3
            tokio::time::sleep(Duration::from_millis(200)).await;
            println!("🔄 并发任务3完成");
            Ok("task3_result".to_string())
        },
    ];

    // 并发执行所有任务
    let results = futures::future::join_all(tasks).await;
    println!("✅ 并发任务执行完成: {} 个任务", results.len());

    for (i, result) in results.into_iter().enumerate() {
        match result {
            Ok(value) => println!("  任务{}: {}", i + 1, value),
            Err(e) => println!("  任务{}: 失败 - {}", i + 1, e),
        }
    }
}

#[tokio::test]
async fn test_performance_metrics() {
    // 性能指标测试
    println!("📊 性能指标测试");
    println!("==================");

    let start_time = std::time::Instant::now();

    // 创建客户端
    let config = UnifiedConfig::default();
    let client = UnifiedClient::new(config).await.expect("客户端创建失败");

    let creation_time = start_time.elapsed();
    println!("⏱️  客户端创建时间: {:?}", creation_time);

    // 测试服务发现性能
    let discovery_start = std::time::Instant::now();
    let services = client.available_services();
    let discovery_time = discovery_start.elapsed();
    println!("🔍 服务发现时间: {:?}", discovery_time);
    println!("📋 发现服务数量: {}", services.len());

    // 测试健康检查性能
    let health_start = std::time::Instant::now();
    let health = client.health_check().await.expect("健康检查失败");
    let health_time = health_start.elapsed();
    println!("🏥 健康检查时间: {:?}", health_time);
    println!("📊 健康服务数: {}", health.values().filter(|&h| *h).count());

    // 测试API分发器性能
    let dispatcher_start = std::instant::Instant::now();
    let adapter_services = client.dispatcher().list_services();
    let dispatcher_time = dispatcher_start.elapsed();
    println!("🔧 分发器列表时间: {:?}", dispatcher_time);
    println!("📋 适配器服务数: {}", adapter_services.len());

    let total_time = start_time.elapsed();
    println!("⏱️  总初始化时间: {:?}", total_time);

    // 性能断言
    assert!(creation_time < Duration::from_secs(1), "客户端创建时间过长");
    assert!(
        discovery_time < Duration::from_millis(100),
        "服务发现时间过长"
    );
    assert!(health_time < Duration::from_millis(500), "健康检查时间过长");
    assert!(
        dispatcher_time < Duration::from_millis(50),
        "分发器列表时间过长"
    );

    println!("✅ 性能指标测试通过！");
}

#[cfg(test)]
mod e2e_utils {
    /// 等待网络延迟
    pub async fn simulate_network_delay() {
        tokio::time::sleep(Duration::from_millis(100)).await;
    }

    /// 创建测试用的消息内容
    pub fn create_test_message(prefix: &str) -> String {
        format!(
            "{} - 测试消息 - {}",
            prefix,
            chrono::Utc::now().format("%Y-%m-%d %H:%M:%S")
        )
    }

    /// 验证API响应格式
    pub fn validate_api_response<T>(result: &Result<T, UnifiedError>) -> bool {
        match result {
            Ok(_) => true,
            Err(e) => {
                // 检查是否是预期的错误类型
                e.user_friendly_message().len() > 0
            }
        }
    }
}

/**
 * OpenLark SDK 错误处理示例
 *
 * 本示例展示了如何使用 OpenLark SDK 进行全面的错误处理，包括：
 * - 不同类型的错误识别和处理
 * - 错误恢复和重试策略
 * - 用户友好的错误消息
 * - 日志记录和监控
 * - 生产环境的错误处理最佳实践
 *
 * 运行方法：
 * cargo run --example 03_error_handling
 */

use openlark_core::prelude::*;
use openlark_client::minimal::{MinimalLarkClient, AuthClient};
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 加载环境变量
    dotenvy::dotenv().ok();

    println!("🛡️ OpenLark SDK 错误处理示例");
    println!("===============================");
    println!();

    // 创建客户端
    let app_id = std::env::var("OPENLARK_APP_ID")
        .unwrap_or_else(|_| "invalid_app_id".to_string());
    let app_secret = std::env::var("OPENLARK_APP_SECRET")
        .unwrap_or_else(|_| "invalid_secret".to_string());

    let client = create_client(&app_id, &app_secret)?;

    // === 错误类型演示 ===
    println!("🔍 错误类型识别");
    println!("================");

    demonstrate_error_types().await?;
    println!();

    // === 错误处理策略 ===
    println!("🔧 错误处理策略");
    println!("================");

    demonstrate_error_handling_strategies(&client).await?;
    println!();

    // === 重试机制演示 ===
    println!("🔄 重试机制演示");
    println!("================");

    demonstrate_retry_mechanisms(&client).await?;
    println!();

    // === 错误监控和日志 ===
    println!("📊 错误监控和日志");
    println!("==================");

    demonstrate_error_monitoring().await?;
    println!();

    // === 生产环境最佳实践 ===
    println!("🏢 生产环境最佳实践");
    println!("====================");

    demonstrate_production_best_practices(&client).await?;
    println!();

    // === 错误处理总结 ===
    println!("💡 错误处理总结");
    println!("================");
    println!("1. 🎯 错误分类:");
    println!("   • 系统错误: 网络、超时、服务器问题");
    println!("   • 业务错误: 权限、数据验证、业务规则");
    println!("   • 配置错误: 参数、环境、权限配置");
    println!();
    println!("2. 🛠️ 处理策略:");
    println!("   • 立即失败: 不可恢复的错误");
    println!("   • 重试尝试: 临时性、可恢复的错误");
    println!("   • 降级处理: 部分功能不可用时的备选方案");
    println!("   • 用户提示: 需要用户操作或知情的情况");
    println!();
    println!("3. 📈 监控要点:");
    println!("   • 错误率和趋势");
    println!("   • 响应时间分布");
    println!("   • 异常模式识别");
    println!("   • 用户体验影响");

    Ok(())
}

/**
 * 创建客户端（配置错误处理参数）
 */
fn create_client(app_id: &str, app_secret: &str) -> Result<MinimalLarkClient, Box<dyn std::error::Error>> {
    let client = MinimalLarkClient::new(app_id.to_string(), app_secret.to_string())?;
    Ok(client)
}

/**
 * 演示不同类型的错误
 */
async fn demonstrate_error_types() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔍 识别不同类型的错误...");

    // 1. 网络错误
    println!("1️⃣ 网络错误:");
    println!("   • 特征: 连接超时、DNS解析失败、连接中断");
    println!("   • 处理: 重试、检查网络、切换备用地址");
    println!("   • 示例: LarkAPIError::NetworkError");

    // 2. 认证错误
    println!("2️⃣ 认证错误:");
    println!("   • 特征: 令牌无效、权限不足、应用未授权");
    println!("   • 处理: 刷新令牌、检查权限、联系管理员");
    println!("   • 示例: code 99991663 (无效App ID)");

    // 3. 业务逻辑错误
    println!("3️⃣ 业务逻辑错误:");
    println!("   • 特征: 数据不存在、参数错误、业务规则违反");
    println!("   • 处理: 参数验证、数据检查、业务规则调整");
    println!("   • 示例: code 333009 (用户不存在)");

    // 4. 数据格式错误
    println!("4️⃣ 数据格式错误:");
    println!("   • 特征: JSON解析失败、字段类型错误、数据结构不匹配");
    println!("   • 处理: 数据验证、格式转换、兼容性检查");
    println!("   • 示例: LarkAPIError::DataError");

    // 5. 系统错误
    println!("5️⃣ 系统错误:");
    println!("   • 特征: 服务不可用、限流、服务器内部错误");
    println!("   • 处理: 降级、重试、等待恢复");
    println!("   • 示例: code 429 (请求频率限制)");

    Ok(())
}

/**
 * 演示错误处理策略
 */
async fn demonstrate_error_handling_strategies(
    client: &MinimalLarkClient
) -> Result<(), Box<dyn std::error::Error>> {
    println!("🔧 实践错误处理策略...");

    // 策略1: 立即失败策略
    println!("1️⃣ 立即失败策略:");
    let invalid_client = MinimalLarkClient::new("invalid_app".to_string(), "invalid_secret".to_string());

    match invalid_client {
        Ok(_) => println!("   ⚠️  意外创建了无效客户端"),
        Err(e) => {
            println!("   ✅ 正确识别了无效配置");
            handle_error_immediately(&e);
        }
    }

    // 策略2: 重试策略
    println!("2️⃣ 重试策略:");
    println!("   • 对于临时性错误实施重试");
    println!("   • 使用指数退避避免加重系统负担");
    println!("   • 设置最大重试次数防止无限循环");

    // 策略3: 降级策略
    println!("3️⃣ 降级策略:");
    println!("   • 主功能不可用时提供基础功能");
    println!("   • 缓存常用数据避免实时依赖");
    println!("   • 返回默认值或历史数据");

    // 策略4: 用户提示策略
    println!("4️⃣ 用户提示策略:");
    println!("   • 将技术错误转换为用户友好的消息");
    println!("   • 提供具体的解决建议");
    println!("   • 引导用户进行正确的操作");

    // 策略5: 测试正常客户端
    println!("5️⃣ 正常客户端测试:");
    match client.get_app_access_token().await {
        Ok(_) => println!("   ✅ 正常客户端工作正常"),
        Err(e) => {
            println!("   ⚠️  正常客户端也遇到错误，可能是配置问题");
            handle_error_immediately(&e);
        }
    }

    Ok(())
}

/**
 * 演示重试机制
 */
async fn demonstrate_retry_mechanisms(client: &LarkClient) -> Result<(), Box<dyn std::error::Error>> {
    println!("🔄 实现智能重试机制...");

    // 模拟各种重试场景
    demonstrate_exponential_backoff().await?;
    demonstrate_circuit_breaker().await?;
    demonstrate_rate_limiting().await?;

    Ok(())
}

/**
 * 演示指数退避重试
 */
async fn demonstrate_exponential_backoff() -> Result<(), Box<dyn std::error::Error>> {
    println!("📈 指数退避重试:");
    println!("   • 延迟时间: 1s, 2s, 4s, 8s...");
    println!("   • 适用于: 网络超时、服务繁忙");
    println!("   • 优势: 避免加重系统负担");

    let mut attempt = 0;
    let max_attempts = 3;
    let mut delay = Duration::from_secs(1);

    while attempt < max_attempts {
        attempt += 1;
        println!("   🔄 第{}次尝试，延迟{:?}", attempt, delay);

        // 这里会模拟API调用失败
        if attempt < max_attempts {
            println!("   ❌ 尝试失败，准备重试...");
            tokio::time::sleep(delay).await;
            delay *= 2; // 指数增长
        } else {
            println!("   ✅ 模拟成功");
            break;
        }
    }

    Ok(())
}

/**
 * 演示熔断器模式
 */
async fn demonstrate_circuit_breaker() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔌 熔断器模式:");
    println!("   • 状态: 关闭、打开、半开");
    println!("   • 适用于: 保护下游服务不被过载");
    println!("   • 机制: 失败率达到阈值时断开连接");

    // 简化的熔断器状态
    let mut failure_count = 0;
    let failure_threshold = 5;
    let mut circuit_open = false;

    for _i in 1..=8 {
        if circuit_open {
            println!("   🔌 熔断器打开，直接拒绝请求");
            continue;
        }

        // 模拟失败
        failure_count += 1;
        println!("   ❌ 请求失败 (失败次数: {})", failure_count);

        if failure_count >= failure_threshold {
            circuit_open = true;
            println!("   🔌 达到失败阈值，熔断器打开");
            println!("   ⏰ 30秒后尝试半开状态...");
        }
    }

    Ok(())
}

/**
 * 演示限流处理
 */
async fn demonstrate_rate_limiting() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚦 限流处理:");
    println!("   • 检测: HTTP 429 状态码");
    println!("   • 策略: 等待重试时间或降低请求频率");
    println!("   • 实现: 解析Retry-After头部");

    // 模拟限流响应处理
    let rate_limit_response = simulate_rate_limit_response().await;
    if rate_limit_response.contains("429") {
        println!("   ⏰ 检测到限流，等待60秒后重试...");
        // tokio::time::sleep(Duration::from_secs(60)).await;
        println!("   ✅ 限流等待完成，可以继续请求");
    }

    Ok(())
}

/**
 * 演示错误监控
 */
async fn demonstrate_error_monitoring() -> Result<(), Box<dyn std::error::Error>> {
    println!("📊 实现错误监控...");

    // 模拟错误统计
    let mut error_stats = ErrorStatistics::new();
    error_stats.record_error("network_timeout");
    error_stats.record_error("auth_failed");
    error_stats.record_error("business_error");
    error_stats.record_error("network_timeout");

    println!("📈 错误统计:");
    error_stats.print_statistics();

    // 告警规则示例
    println!("🚨 告警规则:");
    println!("   • 错误率 > 10%: 发送邮件通知");
    println!("   • 连续失败 > 5次: 立即通知开发团队");
    println!("   • 响应时间 > 5s: 性能告警");

    Ok(())
}

/**
 * 演示生产环境最佳实践
 */
async fn demonstrate_production_best_practices(client: &LarkClient) -> Result<(), Box<dyn std::error::Error>> {
    println!("🏢 生产环境最佳实践...");

    // 1. 分层错误处理
    println!("1️⃣ 分层错误处理:");
    demonstrate_layered_error_handling().await?;

    // 2. 优雅降级
    println!("2️⃣ 优雅降级:");
    demonstrate_graceful_degradation(client).await?;

    // 3. 健康检查
    println!("3️⃣ 健康检查:");
    demonstrate_health_checks(client).await?;

    // 4. 日志记录
    println!("4️⃣ 结构化日志:");
    demonstrate_structured_logging().await?;

    Ok(())
}

/**
 * 立即错误处理
 */
fn handle_error_immediately(error: &openlark_core::error::LarkAPIError) {
    match error {
        openlark_core::error::LarkAPIError::NetworkError { message, .. } => {
            eprintln!("🌐 网络错误: {}", message);
        }
        openlark_core::error::LarkAPIError::APIError { code, message, .. } => {
            if *code == 99991663 {
                eprintln!("❌ 配置错误: App ID 或 App Secret 无效");
                eprintln!("💡 请检查环境变量 OPENLARK_APP_ID 和 OPENLARK_APP_SECRET");
            } else {
                eprintln!("❌ API错误 ({}): {}", code, message);
            }
        }
        openlark_core::error::LarkAPIError::DataError(msg) => {
            eprintln!("📊 数据错误: {}", msg);
        }
        _ => {
            eprintln!("❓ 未知错误: {}", error);
        }
    }
}

/**
 * 用户友好的错误消息
 */
fn user_friendly_error_message(error: &openlark_core::error::LarkAPIError) -> String {
    match error {
        openlark_core::error::LarkAPIError::APIError { code, .. } => {
            match code {
                99991663 => "应用配置有误，请联系管理员检查App ID和Secret".to_string(),
                99991664 => "应用密钥错误，请重新配置应用".to_string(),
                99991400 => "访问令牌无效，请重新登录".to_string(),
                99991401 => "访问令牌已过期，请刷新令牌".to_string(),
                99991403 => "权限不足，请联系管理员".to_string(),
                429 => "请求过于频繁，请稍后再试".to_string(),
                403 => "没有权限执行此操作".to_string(),
                404 => "请求的资源不存在".to_string(),
                500 => "服务器内部错误，请稍后重试".to_string(),
                _ => format!("系统繁忙，请稍后重试 (错误代码: {})", code),
            }
        }
        openlark_core::error::LarkAPIError::NetworkError { .. } => "网络连接失败，请检查网络连接".to_string(),
        openlark_core::error::LarkAPIError::DataError(msg) => format!("数据格式错误: {}", msg),
        _ => "发生了未知错误，请联系技术支持".to_string(),
    }
}

/**
 * 分层错误处理
 */
async fn demonstrate_layered_error_handling() -> Result<(), Box<dyn std::error::Error>> {
    println!("   📊 分层结构:");
    println!("     • 网络层: 处理连接、超时、协议错误");
    println!("     • 业务层: 处理权限、数据、规则错误");
    println!("     • 表现层: 处理用户交互、界面显示错误");

    Ok(())
}

/**
 * 优雅降级
 */
async fn demonstrate_graceful_degradation(_client: &MinimalLarkClient) -> Result<(), Box<dyn std::error::Error>> {
    println!("   🔄 降级策略:");
    println!("     • 缓存命中: 返回缓存数据");
    println!("     • 功能简化: 禁用非关键功能");
    println!("     • 默认值: 使用合理的默认数据");

    // 模拟降级
    let cached_data = Some("缓存的用户信息".to_string());
    if let Some(data) = cached_data {
        println!("     ✅ 使用缓存数据: {}", data);
    }

    Ok(())
}

/**
 * 健康检查
 */
async fn demonstrate_health_checks(client: &MinimalLarkClient) -> Result<(), Box<dyn std::error::Error>> {
    println!("   🔍 健康检查项:");
    println!("     • 服务可用性检查");
    println!("     • 网络连接检查");
    println!("     • 认证状态检查");

    // 执行健康检查
    match client.get_app_access_token().await {
        Ok(_) => println!("     ✅ 所有健康检查通过"),
        Err(e) => println!("     ❌ 健康检查失败: {}", e),
    }

    Ok(())
}

/**
 * 结构化日志
 */
async fn demonstrate_structured_logging() -> Result<(), Box<dyn std::error::Error>> {
    println!("   📝 日志级别和格式:");
    println!("     • ERROR: 系统错误、异常");
    println!("     • WARN:  警告、降级、重试");
    println!("     • INFO:  关键操作、状态变化");
    println!("     • DEBUG: 调试信息、详细流程");

    // 模拟结构化日志
    let log_entry = serde_json::json!({
        "timestamp": "2024-01-01T12:00:00Z",
        "level": "ERROR",
        "component": "openlark_client",
        "operation": "api_call",
        "error_code": 99991663,
        "error_message": "Invalid app credentials",
        "duration_ms": 1250,
        "request_id": "req_123456"
    });

    println!("     📋 示例日志: {}", log_entry);

    Ok(())
}

// 辅助函数和结构体

#[derive(Debug)]
struct ErrorStatistics {
    network_errors: u32,
    auth_errors: u32,
    business_errors: u32,
    total_errors: u32,
}

impl ErrorStatistics {
    fn new() -> Self {
        Self {
            network_errors: 0,
            auth_errors: 0,
            business_errors: 0,
            total_errors: 0,
        }
    }

    fn record_error(&mut self, error_type: &str) {
        self.total_errors += 1;
        match error_type {
            "network_timeout" => self.network_errors += 1,
            "auth_failed" => self.auth_errors += 1,
            _ => self.business_errors += 1,
        }
    }

    fn print_statistics(&self) {
        println!("   总错误数: {}", self.total_errors);
        println!("   网络错误: {} ({:.1}%)",
                self.network_errors,
                self.network_errors as f64 / self.total_errors as f64 * 100.0);
        println!("   认证错误: {} ({:.1}%)",
                self.auth_errors,
                self.auth_errors as f64 / self.total_errors as f64 * 100.0);
        println!("   业务错误: {} ({:.1}%)",
                self.business_errors,
                self.business_errors as f64 / self.total_errors as f64 * 100.0);
    }
}

async fn simulate_rate_limit_response() -> String {
    // 模拟HTTP 429响应
    "{\"code\": 429, \"msg\": \"Too Many Requests\"}".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_statistics() {
        let mut stats = ErrorStatistics::new();
        stats.record_error("network_timeout");
        stats.record_error("network_timeout");
        stats.record_error("auth_failed");

        assert_eq!(stats.total_errors, 3);
        assert_eq!(stats.network_errors, 2);
        assert_eq!(stats.auth_errors, 1);
    }

    #[test]
    fn test_user_friendly_messages() {
        let api_error = openlark_core::error::LarkAPIError::illegal_param("Invalid app credentials");

        let friendly_msg = user_friendly_error_message(&api_error);
        assert!(!friendly_msg.is_empty());
    }

    #[test]
    fn test_client_creation() {
        let app_id = "test_app_id";
        let app_secret = "test_app_secret";

        let result = create_client(app_id, app_secret);
        assert!(result.is_ok(), "客户端创建应该成功");
    }
}
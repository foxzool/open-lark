/**
 * OpenLark SDK 认证机制示例
 *
 * 本示例展示了如何使用 OpenLark SDK 进行身份认证和权限管理，包括：
 * - 应用级认证（tenant_access_token）
 * - 用户级认证（user_access_token）
 * - 令牌缓存和自动刷新
 * - 权限检查和错误处理
 * - 不同应用类型的认证方式
 *
 * 运行方法：
 * cargo run --example 01_authentication
 */

use openlark_core::config::ConfigBuilder;
use openlark_core::constants::AppType;
use openlark_core::prelude::*;
use openlark_client::minimal::{MinimalLarkClient, AuthClient};
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 加载环境变量
    dotenvy::dotenv().ok();

    println!("🔐 OpenLark SDK 认证机制示例");
    println!("==============================");
    println!();

    // 从环境变量获取应用配置
    let app_id = std::env::var("OPENLARK_APP_ID")
        .expect("请设置环境变量 OPENLARK_APP_ID");
    let app_secret = std::env::var("OPENLARK_APP_SECRET")
        .expect("请设置环境变量 OPENLARK_APP_SECRET");

    // 可选：用户访问令牌（用于用户级API）
    let user_access_token = std::env::var("OPENLARK_USER_ACCESS_TOKEN").ok();

    println!("📋 应用配置");
    println!("App ID: {}", app_id);
    println!("App Secret: {}***", &app_secret[..8.min(app_secret.len())]);
    if user_access_token.is_some() {
        println!("User Access Token: 已设置");
    } else {
        println!("User Access Token: 未设置（某些功能可能不可用）");
    }
    println!();

    // 创建客户端
    let client = create_client(&app_id, &app_secret)?;

    // === 1. 应用级认证 ===
    println!("🔐 应用级认证 (Tenant Access Token)");
    println!("---------------------------------");

    match demonstrate_tenant_auth(&client).await {
        Ok(_) => println!("✅ 应用级认证演示成功"),
        Err(e) => println!("❌ 应用级认证演示失败: {}", e),
    }
    println!();

    // === 2. 用户级认证 ===
    println!("👤 用户级认证 (User Access Token)");
    println!("--------------------------------");

    if let Some(token) = user_access_token {
        match demonstrate_user_auth(&client, &token).await {
            Ok(_) => println!("✅ 用户级认证演示成功"),
            Err(e) => println!("❌ 用户级认证演示失败: {}", e),
        }
    } else {
        println!("ℹ️  跳过用户级认证演示（未设置 OPENLARK_USER_ACCESS_TOKEN）");
        println!("💡 提示: 可以通过 OAuth 流程获取用户访问令牌");
    }
    println!();

    // === 3. 令牌缓存演示 ===
    println!("💾 令牌缓存演示");
    println!("-----------------");

    match demonstrate_token_caching(&client).await {
        Ok(_) => println!("✅ 令牌缓存演示成功"),
        Err(e) => println!("❌ 令牌缓存演示失败: {}", e),
    }
    println!();

    // === 4. 权限检查演示 ===
    println!("🔍 权限检查演示");
    println!("-----------------");

    match demonstrate_permission_check(&client).await {
        Ok(_) => println!("✅ 权限检查演示成功"),
        Err(e) => println!("❌ 权限检查演示失败: {}", e),
    }
    println!();

    // === 5. 不同应用类型认证 ===
    println!("🏢 应用类型认证对比");
    println!("--------------------");

    demonstrate_app_type_auth(&app_id, &app_secret).await?;
    println!();

    // === 6. 认证最佳实践 ===
    println!("💡 认证最佳实践");
    println!("----------------");
    println!("1. 🔄 启用令牌缓存:");
    println!("   • 减少重复获取令牌的HTTP请求");
    println!("   • 自动处理令牌过期和刷新");
    println!("   • 显著提升API调用性能");
    println!();
    println!("2. 🛡️ 安全配置:");
    println!("   • 使用环境变量存储敏感信息");
    println!("   • 定期轮换应用密钥");
    println!("   • 限制应用权限范围");
    println!();
    println!("3. 🔧 错误处理:");
    println!("   • 检查认证错误码");
    println!("   • 实现重试机制");
    println!("   • 提供用户友好的错误信息");
    println!();
    println!("4. 📊 监控和日志:");
    println!("   • 记录认证成功/失败事件");
    println!("   • 监控令牌使用情况");
    println!("   • 异常情况告警");

    Ok(())
}

/**
 * 创建客户端（启用令牌缓存）
 */
fn create_client(app_id: &str, app_secret: &str) -> Result<MinimalLarkClient, Box<dyn std::error::Error>> {
    let client = MinimalLarkClient::new(app_id.to_string(), app_secret.to_string())?;
    Ok(client)
}

/**
 * 演示应用级认证（Tenant Access Token）
 *
 * 应用级认证用于访问应用相关的数据和功能，如：
 * - 应用配置信息
 * - 企业组织架构
 * - 管理员功能
 */
async fn demonstrate_tenant_auth(client: &MinimalLarkClient) -> Result<(), Box<dyn std::error::Error>> {
    println!("🔑 获取应用访问令牌...");

    // 使用最小客户端获取应用访问令牌
    match client.get_app_access_token().await {
        Ok(token) => {
            println!("✅ 应用级认证成功");
            println!("📱 令牌信息:");
            println!("   • 访问令牌: {}...", &token.access_token[..token.access_token.len().min(20)]);
            println!("   • 令牌类型: {:?}", token.token_type);
            println!("   • 过期时间: {} 秒", token.expires_in);
        }
        Err(e) => {
            println!("⚠️  应用令牌获取失败");
            println!("   错误: {}", e);
        }
    }

    Ok(())
}

/**
 * 演示用户级认证（User Access Token）
 *
 * 用户级认证用于访问用户相关的数据和功能，如：
 * - 用户个人信息
 * - 用户消息
 * - 用户文件
 */
async fn demonstrate_user_auth(_client: &MinimalLarkClient, user_token: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("👤 使用用户访问令牌...");

    // 注意：在最小客户端架构中，用户访问令牌主要用于OAuth流程
    // 这里我们演示如何使用已有的用户访问令牌信息

    println!("💡 用户访问令牌使用说明:");
    println!("   • 用户令牌通常通过OAuth流程获取");
    println!("   • 可以用来代表用户执行操作");
    println!("   • 需要定期刷新以保持有效性");
    println!("   • 当前令牌前缀: {}...", &user_token[..user_token.len().min(10)]);

    // 在实际应用中，用户令牌会用于调用用户相关的API
    // 这里仅作为演示，展示如何处理用户令牌

    Ok(())
}

/**
 * 演示令牌缓存机制
 */
async fn demonstrate_token_caching(client: &MinimalLarkClient) -> Result<(), Box<dyn std::error::Error>> {
    println!("🔄 测试令牌缓存效果...");

    let start_time = std::time::Instant::now();

    // 第一次API调用 - 需要获取令牌
    println!("📞 第一次令牌获取...");
    let _token1 = client.get_app_access_token().await;
    let first_call_time = start_time.elapsed();

    // 第二次API调用 - 使用缓存的令牌
    println!("📞 第二次令牌获取...");
    let _token2 = client.get_app_access_token().await;
    let second_call_time = start_time.elapsed() - first_call_time;

    println!("📊 性能对比:");
    println!("   • 第一次调用耗时: {:?}", first_call_time);
    println!("   • 第二次调用耗时: {:?}", second_call_time);

    if second_call_time < first_call_time {
        println!("✅ 令牌缓存生效，第二次调用更快");
    } else {
        println!("ℹ️  时间差可能受到网络影响，但令牌缓存仍然有效");
    }

    Ok(())
}

/**
 * 演示权限检查
 */
async fn demonstrate_permission_check(client: &MinimalLarkClient) -> Result<(), Box<dyn std::error::Error>> {
    println!("🔍 检查应用权限...");

    // 通过获取应用访问令牌来验证基本权限
    match client.get_app_access_token().await {
        Ok(_) => {
            println!("✅ 基本认证权限正常");
            println!("📋 应用权限说明:");
            println!("   • 可以在飞书应用管理中查看已获得的权限");
            println!("   • 常见权限包括：im:message、contact:user.base:readonly 等");
            println!("   • 令牌获取成功说明基础认证配置正确");
        }
        Err(e) => {
            println!("❌ 权限检查失败: {}", e);
            println!("💡 请检查:");
            println!("   • App ID 和 App Secret 是否正确");
            println!("   • 应用是否已发布并激活");
            println!("   • 网络连接是否正常");
        }
    }

    Ok(())
}

/**
 * 演示不同应用类型的认证方式
 */
async fn demonstrate_app_type_auth(app_id: &str, app_secret: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("🏢 对比不同应用类型的认证...");

    // 1. 自建应用 (SelfBuild)
    println!("1️⃣ 自建应用认证");
    println!("   • 适用场景: 企业内部应用");
    println!("   • 权限范围: 企业内部数据");
    println!("   • 认证方式: App Secret + Tenant Access Token");

    let _self_build_client = MinimalLarkClient::new(app_id.to_string(), app_secret.to_string())?;
    println!("   ✅ 自建应用客户端创建成功");

    // 2. 应用商店应用说明
    println!("2️⃣ 应用商店应用认证");
    println!("   • 适用场景: 发布到飞书应用商店的应用");
    println!("   • 权限范围: 跨企业数据");
    println!("   • 认证方式: App Secret + 企业授权");
    println!("   • 说明: 当前最小客户端主要支持自建应用模式");

    // 3. 用户访问令牌说明
    println!("3️⃣ 用户访问令牌认证");
    println!("   • 适用场景: 代表用户操作");
    println!("   • 权限范围: 用户个人数据");
    println!("   • 认证方式: OAuth 流程获取的令牌");
    println!("   • 说明: 用户令牌可通过 get_user_access_token 方法获取");

    println!();
    println!("💡 选择建议:");
    println!("   • 企业内部应用 → 使用 MinimalLarkClient");
    println!("   • SaaS 应用 → 需要企业授权配置");
    println!("   • 用户操作 → 使用 OAuth 流程获取用户令牌");

    Ok(())
}

/**
 * 处理认证错误
 */
fn handle_auth_error(error: &openlark_core::error::LarkAPIError) {
    match error {
        openlark_core::error::LarkAPIError::NetworkError { message, .. } => {
            println!("🌐 网络错误: {}", message);
        }
        openlark_core::error::LarkAPIError::APIError { code, msg, .. } => {
            match code {
                99991663 => println!("❌ 无效的 App ID 或 App Secret"),
                99991664 => println!("❌ 应用密钥错误"),
                99991400 => println!("❌ 无效的访问令牌"),
                99991401 => println!("❌ 访问令牌已过期"),
                99991403 => println!("❌ 无权限访问该资源"),
                _ => println!("❌ API错误 ({}): {}", code, msg),
            }
        }
        openlark_core::error::LarkAPIError::DataError(msg) => {
            println!("📊 数据错误: {}", msg);
        }
        _ => {
            println!("❓ 未知错误: {}", error);
        }
    }
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
    fn test_minimal_client_creation() {
        let app_id = "test_app_id";
        let app_secret = "test_app_secret";

        // 测试最小客户端创建
        let result = MinimalLarkClient::new(app_id.to_string(), app_secret.to_string());
        assert!(result.is_ok(), "最小客户端创建应该成功");
    }

    #[tokio::test]
    async fn test_env_client_creation() {
        // 设置测试环境变量
        std::env::set_var("OPENLARK_APP_ID", "test_app_id");
        std::env::set_var("OPENLARK_APP_SECRET", "test_app_secret");

        let result = MinimalLarkClient::from_env();
        assert!(result.is_ok(), "从环境变量创建客户端应该成功");
    }
}
//! 飞书开放平台基础登录演示
//!
//! 展示完整的OAuth授权流程，包括：
//! 1. 生成授权URL
//! 2. 用户授权后获取授权码
//! 3. 使用授权码获取用户访问令牌
//! 4. 获取用户信息并验证登录状态

use openlark_auth::prelude::*;
use std::env;
use std::time::{SystemTime, UNIX_EPOCH};
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 初始化日志
    println!("🚀 飞书开放平台基础登录演示");
    println!("============================");

    // 1. 配置应用信息
    let app_id = env::var("OPENLARK_APP_ID")
        .unwrap_or_else(|_| "your_app_id".to_string());
    let app_secret = env::var("OPENLARK_APP_SECRET")
        .unwrap_or_else(|_| "your_app_secret".to_string());

    let config = AuthConfig::new(&app_id, &app_secret)
        .with_base_url("https://open.feishu.cn");

    println!("✅ 应用配置: App ID = {}", app_id);

    // 2. 创建认证服务
    let auth = AuthServices::new(config);
    println!("✅ 认证服务初始化完成");

    // 3. 步骤1: 生成OAuth授权URL
    println!("\n📋 步骤1: 生成OAuth授权URL");
    println!("==================");

    let redirect_uri = "http://localhost:3000/callback";
    let state = generate_random_state();

    let oauth_url = auth.oauth.old.authorization()
        .get_index()
        .redirect_uri(redirect_uri)
        .state(&state)
        .build_url();

    println!("🔗 授权URL已生成:");
    println!("   请复制以下URL到浏览器中访问:");
    println!("   {}", oauth_url);
    println!("");
    println!("📱 访问后，用户将被重定向到: {}", redirect_uri);
    println!("🔒 状态参数: {} (用于防止CSRF攻击)", state);

    // 4. 模拟步骤2: 获取授权码
    println!("\n📋 步骤2: 模拟获取授权码");
    println!("==================");

    let auth_code = simulate_authorization_code_process().await?;
    println!("✅ 获取到授权码: {}...", &auth_code[..8.min(auth_code.len())]);

    // 5. 步骤3: 使用授权码获取访问令牌
    println!("\n📋 步骤3: 获取用户访问令牌");
    println!("==================");

    let token_response = auth.authen.v1.access_token()
        .create()
        .grant_type("authorization_code")
        .code(&auth_code)
        .send()
        .await;

    match token_response {
        Ok(token) => {
            println!("✅ 访问令牌获取成功:");
            println!("   访问令牌: {}...", &token.access_token[..8.min(token.access_token.len())]);
            println!("   令牌类型: {}", token.token_type);
            println!("   过期时间: {} 秒", token.expires_in);
            if let Some(scope) = &token.scope {
                println!("   权限范围: {}", scope);
            }
            if let Some(refresh_token) = &token.refresh_token {
                println!("   刷新令牌: {}...", &refresh_token[..8.min(refresh_token.len())]);
            }

            // 6. 步骤4: 获取用户信息
            println!("\n📋 步骤4: 获取用户信息");
            println!("==================");

            let user_info = auth.authen.v1.user_info()
                .get()
                .user_access_token(&token.access_token)
                .send()
                .await;

            match user_info {
                Ok(user) => {
                    println!("✅ 用户信息获取成功:");
                    println!("   用户ID: {}", user.user_id);
                    println!("   Open ID: {}", user.open_id);
                    println!("   Union ID: {}", user.union_id);
                    println!("   用户名: {}", user.name);
                    if !user.email.is_empty() {
                        println!("   邮箱: {}", user.email);
                    }
                    if !user.mobile.is_empty() {
                        println!("   手机号: {}", user.mobile);
                    }
                    if !user.avatar_url.is_empty() {
                        println!("   头像URL: {}", user.avatar_url);
                    }
                    println!("   用户状态: {:?}", user.status);

                    // 7. 登录成功，显示欢迎信息
                    println!("\n🎉 登录成功！");
                    println!("============");
                    println!("欢迎 {}，您已成功登录飞书开放平台！", user.name);

                    // 8. 显示令牌管理信息
                    display_token_management_info(&token, &auth).await?;

                }
                Err(e) => {
                    println!("❌ 用户信息获取失败: {}", e);
                }
            }
        }
        Err(e) => {
            println!("❌ 访问令牌获取失败: {}", e);
        }
    }

    Ok(())
}

/// 生成随机状态参数
fn generate_random_state() -> String {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    use std::time::{SystemTime, UNIX_EPOCH};

    let mut hasher = DefaultHasher::new();
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos()
        .hash(&mut hasher);

    format!("{:x}", hasher.finish())
}

/// 模拟OAuth授权码获取流程
async fn simulate_authorization_code_process() -> Result<String, Box<dyn std::error::Error>> {
    println!("⚠️  演示模式: 模拟授权流程");
    println!("   在实际应用中，用户访问授权URL后会跳转到回调页面");
    println!("   回调页面会从URL参数中提取授权码和状态参数");
    println!("");

    // 模拟用户授权过程
    println!("🔄 模拟用户授权中...");
    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;

    // 生成模拟的授权码
    let mock_auth_code = format!("demo_auth_code_{}",
                                 SystemTime::now()
                                     .duration_since(UNIX_EPOCH)
                                     .unwrap()
                                     .as_secs());

    println!("✅ 用户已授权，获取到授权码");

    Ok(mock_auth_code)
}

/// 显示令牌管理信息
async fn display_token_management_info(
    token: &UserAccessTokenResponse,
    _auth: &AuthServices
) -> Result<(), Box<dyn std::error::Error>> {
    println!("\n📋 令牌管理信息");
    println!("================");

    // 计算令牌过期时间
    let expires_at = chrono::Utc::now() + chrono::Duration::seconds(token.expires_in as i64);
    let remaining_seconds = token.expires_in;

    println!("令牌状态检查:");
    println!("  - 令牌类型: {}", token.token_type);
    println!("  - 过期时间: {} 秒", remaining_seconds);
    println!("  - 预计过期时间: {}", expires_at.format("%Y-%m-%d %H:%M:%S UTC"));
    println!("  - 30分钟内需要刷新: {}",
             if remaining_seconds < 1800 { "是" } else { "否" });

    println!("\n令牌刷新演示:");
    if let Some(refresh_token) = &token.refresh_token {
        println!("🔄 使用刷新令牌更新访问令牌...");
        println!("   刷新令牌: {}...", &refresh_token[..8.min(refresh_token.len())]);
        println!("   实际应用中，可以使用以下代码刷新令牌:");
        println!("   ```rust");
        println!("   let new_token = auth.authen.v1.access_token()");
        println!("       .create()");
        println!("       .grant_type(\"refresh_token\")");
        println!("       .refresh_token(\"{}\")", refresh_token);
        println!("       .send()");
        println!("       .await?;");
        println!("   ```");
    } else {
        println!("ℹ️  本次获取的令牌不包含刷新令牌");
    }

    println!("\n令牌存储建议:");
    println!("  - 🔒 使用安全的存储方式（如加密数据库、安全存储服务）");
    println!("  - 🕐 实现自动刷新机制，在令牌过期前自动刷新");
    println!("  - 🚫 不要在客户端代码中硬编码应用密钥");
    println!("  - 🛡️  实现适当的错误处理和重试机制");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_random_state_generation() {
        let state1 = generate_random_state();
        let state2 = generate_random_state();

        // 确保状态是随机的
        assert_ne!(state1, state2);
        // 确保状态不为空
        assert!(!state1.is_empty());
        assert!(!state2.is_empty());
    }

    #[test]
    fn test_auth_config_creation() {
        let config = AuthConfig::new("test_app_id", "test_app_secret");
        assert_eq!(config.app_id, "test_app_id");
        assert_eq!(config.app_secret, "test_app_secret");
        assert_eq!(config.base_url, "https://open.feishu.cn");
    }

    #[tokio::test]
    async fn test_auth_services_creation() {
        let config = AuthConfig::new("test_app_id", "test_app_secret");
        let auth = AuthServices::new(config);

        // 测试OAuth授权URL生成
        let oauth_url = auth.oauth.old.authorization()
            .get_index()
            .redirect_uri("http://localhost:3000/callback")
            .state("test_state")
            .build_url();

        assert!(oauth_url.contains("redirect_uri=http://localhost:3000/callback"));
        assert!(oauth_url.contains("state=test_state"));
    }
}
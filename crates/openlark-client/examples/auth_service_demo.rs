//! OpenLark 认证服务演示
//!
//! 展示如何使用 openlark-auth 和 openlark-client 中的认证服务
//! 包含企业应用认证、用户身份认证和 OAuth 授权的完整示例

#[cfg(feature = "auth")]
use openlark_client::services::auth::{AuthService, TokenInfo};
#[cfg(feature = "auth")]
use openlark_client::Config;

#[cfg(feature = "auth")]
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 初始化日志
    println!("🚀 OpenLark 认证服务演示");
    println!("========================");

    // 创建客户端配置
    let config = Config::builder()
        .app_id("your_app_id")
        .app_secret("your_app_secret")
        .base_url("https://open.feishu.cn")
        .build()?;

    println!("✅ 客户端配置创建成功");
    println!("   - App ID: {}", config.app_id);
    println!("   - Base URL: {}", config.base_url);

    // 创建认证服务
    let auth_service = AuthService::new(&config);
    println!("✅ 认证服务初始化成功");

    // 演示企业应用认证
    println!("\n📋 1. 企业应用认证演示");
    println!("====================");
    await_demo_enterprise_auth(&auth_service).await?;

    // 演示用户身份认证
    println!("\n👤 2. 用户身份认证演示");
    println!("====================");
    await_demo_user_authentication(&auth_service).await?;

    // 演示 OAuth 授权
    println!("\n🔑 3. OAuth 授权演示");
    println!("==================");
    await_demo_oauth_authorization(&auth_service).await?;

    // 演示令牌管理
    println!("\n🎫 4. 令牌管理演示");
    println!("==================");
    await_demo_token_management().await?;

    // 演示错误处理
    println!("\n⚠️ 5. 错误处理演示");
    println!("==================");
    await_demo_error_handling().await?;

    println!("\n✨ 演示完成！");
    println!("");
    println!("💡 使用提示：");
    println!("   - 在生产环境中，请使用真实的 App ID 和 App Secret");
    println!("   - 令牌应该安全存储并定期刷新");
    println!("   - 建议使用配置管理服务管理敏感信息");
    println!("   - 记录详细的日志以便问题排查");

    Ok(())
}

#[cfg(feature = "auth")]
async fn await_demo_enterprise_auth(
    auth_service: &AuthService,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("🏢 获取自建应用访问令牌...");

    // 注意：实际调用需要有效的凭据
    // let token_info = auth_service.get_internal_app_access_token().await?;
    // println!("   ✅ 获取成功: {}...{}", &token_info.access_token[..8], "...");

    println!("   ℹ️  演示模式：实际调用需要有效的应用凭据");
    println!("   🔧 API: auth.v3.tenant_access_token.internal()");

    println!("🏢 获取商店应用访问令牌...");
    println!("   ℹ️  演示模式：商店应用令牌获取");
    println!("   🔧 API: auth.v3.app_access_token.store()");

    println!("📨 重新推送应用票据...");
    println!("   ℹ️  演示模式：应用票据重新推送");
    println!("   🔧 API: auth.v3.app_ticket.resend()");

    Ok(())
}

#[cfg(feature = "auth")]
async fn await_demo_user_authentication(
    auth_service: &AuthService,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("🔍 获取用户信息...");

    // 注意：实际调用需要有效的用户访问令牌
    // let user_info = auth_service.get_user_info("user_access_token").await?;
    // println!("   ✅ 获取成功: {}", user_info.name);

    println!("   ℹ️  演示模式：实际调用需要有效的用户访问令牌");
    println!("   🔧 API: authen.v1.user_info.get()");

    println!("🔄 使用授权码获取用户访问令牌...");
    println!("   ℹ️  演示模式：OAuth 授权码流程");
    println!("   🔧 API: authen.v1.access_token.create()");

    println!("🔄 刷新 OIDC 访问令牌...");
    println!("   ℹ️  演示模式：OIDC 令牌刷新");
    println!("   🔧 API: authen.v1.oidc.create_refresh_access_token()");

    Ok(())
}

#[cfg(feature = "auth")]
async fn await_demo_oauth_authorization(
    auth_service: &AuthService,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("🔗 生成 OAuth 授权 URL...");

    let redirect_uri = "https://your-app.example.com/callback";
    let scope = "user:info docs:read";
    let state = "random_state_string";

    let oauth_url = auth_service.generate_oauth_url(redirect_uri, scope, state);

    println!("   ✅ 生成成功");
    println!("   📱 请在浏览器中访问: {}", oauth_url);
    println!("   🔧 重定向 URI: {}", redirect_uri);
    println!("   🎯 权限范围: {}", scope);
    println!("   🔒 状态参数: {}", state);

    println!("📝 获取登录预授权码...");
    println!("   ℹ️  演示模式：获取预授权码");
    println!("   🔧 API: oauth.old.authorization.get_index()");

    Ok(())
}

#[cfg(feature = "auth")]
async fn await_demo_token_management() -> Result<(), Box<dyn std::error::Error>> {
    // 创建示例令牌
    let token_info = TokenInfo {
        access_token: "example_access_token_12345".to_string(),
        token_type: "Bearer".to_string(),
        expires_in: 7200,
        expires_at: chrono::Utc::now() + chrono::Duration::hours(2),
        scope: Some("user:info docs:read".to_string()),
    };

    println!("🎫 令牌状态检查:");
    println!(
        "   🔍 令牌是否过期: {}",
        if token_info.is_expired() {
            "是"
        } else {
            "否"
        }
    );
    println!("   ⏰ 剩余时间: {} 秒", token_info.remaining_seconds());
    println!(
        "   🔄 需要刷新 (30分钟内): {}",
        if token_info.needs_refresh(30) {
            "是"
        } else {
            "否"
        }
    );

    println!("\n🔐 令牌验证:");
    println!("   ✅ 验证令牌格式和有效性");
    println!("   📋 令牌信息:");
    println!("      类型: {}", token_info.token_type);
    println!("      过期时间: {} 秒", token_info.expires_in);
    println!("      权限范围: {:?}", token_info.scope);

    // 演示令牌验证
    println!("   🔍 验证访问令牌...");
    // let verification = auth_service.verify_token(&token_info.access_token).await?;
    // println!("   ✅ 验证结果: {}", if verification.valid { "有效" } else { "无效" });

    println!("   ℹ️  演示模式：实际验证需要有效的访问令牌");
    println!("   🔧 API: verify_token(access_token)");

    Ok(())
}

#[cfg(feature = "auth")]
async fn await_demo_error_handling() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚫 演示常见错误处理:");

    // 1. 配置错误
    println!("\n   1️⃣ 配置错误处理:");
    match Config::builder()
        .app_id("")
        .app_secret("invalid_secret")
        .build()
    {
        Ok(_) => println!("      ❌ 预期应该失败"),
        Err(e) => println!("      ✅ 正确捕获配置错误: {}", e),
    }

    // 2. 网络错误处理示例
    println!("\n   2️⃣ 网络错误处理:");
    println!("      💡 在实际应用中，应该重试网络请求");
    println!("      💡 使用指数退避策略");
    println!("      💡 记录详细的错误日志");

    // 3. 认证错误处理示例
    println!("\n   3️⃣ 认证错误处理:");
    println!("      💡 检查令牌是否过期");
    println!("      💡 验证应用凭据是否正确");
    println!("      💡 确认权限范围是否足够");

    // 4. 错误恢复策略
    println!("\n   4️⃣ 错误恢复策略:");
    println!("      🔄 自动刷新过期的令牌");
    println!("      📝 记录错误以便分析");
    println!("      📧 发送告警通知管理员");
    println!("      🔀 优雅降级处理");

    Ok(())
}

// 主函数的备用实现，当 auth 功能未启用时
#[cfg(not(feature = "auth"))]
fn main() {
    println!("❌ 认证服务功能未启用");
    println!("");
    println!("💡 请使用以下命令启用认证功能:");
    println!("   cargo run --example auth_service_demo --features auth");
    println!("");
    println!("📦 或者在 Cargo.toml 中添加以下依赖:");
    println!("   [dependencies]");
    println!("   openlark-client = { features = [\"auth\"] }");
}

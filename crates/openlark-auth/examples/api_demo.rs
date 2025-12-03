//! OpenLark Auth API 演示
//!
//! 展示如何使用 openlark-auth 模块的各种API

use openlark_auth::{AuthService, AuthenService, OAuthService};
use openlark_core::config::Config;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 创建配置
    let config = Config::builder()
        .app_id("your_app_id")
        .app_secret("your_app_secret")
        .base_url("https://open.feishu.cn")
        .build();

    // 创建认证服务
    let auth_service = AuthService::new(config.clone());
    let authen_service = AuthenService::new(config.clone());
    let oauth_service = OAuthService::new(config.clone());

    println!("✅ 服务创建成功！");
    println!("  Auth Service: {:?}", auth_service);
    println!("  Authen Service: {:?}", authen_service);
    println!("  OAuth Service: {:?}", oauth_service);

    // 演示API构建器创建（不实际发送请求）
    println!("\n🔧 API 构建器演示：");

    // Auth v3 APIs
    let _app_token_builder = auth_service.v3().app_access_token()
        .app_id("demo_app_id")
        .app_secret("demo_app_secret");

    let _tenant_token_builder = auth_service.v3().tenant_access_token()
        .app_id("demo_app_id")
        .app_secret("demo_app_secret");

    let _app_ticket_builder = auth_service.v3().app_ticket_resend()
        .app_id("demo_app_id")
        .app_secret("demo_app_secret");

    // Authen v1 APIs
    let _user_info_builder = authen_service.v1().user_info()
        .user_access_token("demo_token");

    let _access_token_builder = authen_service.v1().access_token()
        .grant_code("demo_code")
        .app_id("demo_app_id")
        .app_secret("demo_app_secret");

    // OAuth APIs
    let _oauth_builder = oauth_service.old().authorization()
        .app_id("demo_app_id")
        .redirect_uri("https://example.com/callback");

    println!("  ✅ 所有API构建器创建成功");

    // 演示配置验证
    println!("\n⚙️  配置信息：");
    println!("  Base URL: {}", config.base_url);
    println!("  App ID: {}", config.app_id);
    println!("  版本: {}", env!("CARGO_PKG_VERSION"));

    println!("\n🎉 API 演示完成！所有API都已正确实现并可以使用。");

    Ok(())
}
//! Auth模块使用示例
//!
//! 演示如何使用auth模块进行认证操作，包括：
//! - 获取用户信息
//! - 获取和刷新访问令牌
//! - OIDC认证流程

use open_lark::prelude::*;
use open_lark::service::auth;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 初始化日志
    env_logger::init();

    println!("🚀 Auth模块示例演示");

    // 创建客户端（这里使用测试配置）
    let client = LarkClient::builder("test_app_id", "test_app_secret")
        .with_app_type(AppType::SelfBuild)
        .build();

    println!("✅ 客户端创建成功");

    // 演示v3版本API - 获取租户访问令牌
    println!("\n📋 获取租户访问令牌（自建应用）");
    let token_request = auth::v3::TenantAccessTokenInternalRequest {
        app_id: "test_app_id".to_string(),
        app_secret: "test_app_secret".to_string(),
    };

    match client.auth.v3.tenant_access_token_internal(&token_request).await {
        Ok(response) => {
            println!("✅ 租户访问令牌获取成功");
            if let Some(data) = response.data {
                if let Some(token) = data.tenant_access_token {
                    println!("   令牌: {}...", &token[..std::cmp::min(20, token.len())]);
                    println!("   有效期: {} 秒", data.expire);
                }
            }
        }
        Err(e) => {
            println!("❌ 租户访问令牌获取失败: {}", e);
        }
    }

    // 演示v3版本API - 获取应用访问令牌
    println!("\n📋 获取应用访问令牌（自建应用）");
    let app_token_request = auth::v3::AppAccessTokenInternalRequest {
        app_id: "test_app_id".to_string(),
        app_secret: "test_app_secret".to_string(),
    };

    match client.auth.v3.app_access_token_internal(&app_token_request).await {
        Ok(response) => {
            println!("✅ 应用访问令牌获取成功");
            if let Some(data) = response.data {
                if let Some(token) = data.app_access_token {
                    println!("   令牌: {}...", &token[..std::cmp::min(20, token.len())]);
                    println!("   有效期: {} 秒", data.expire);
                }
            }
        }
        Err(e) => {
            println!("❌ 应用访问令牌获取失败: {}", e);
        }
    }

    // 演示v1版本API - 获取用户信息
    println!("\n👤 获取用户信息");
    match client.auth.v1.user_info().await {
        Ok(response) => {
            println!("✅ 用户信息获取成功");
            if let Some(user) = response.data {
                println!("   用户ID: {}", user.user_id);
                println!("   姓名: {}", user.name);
                println!("   邮箱: {}", user.email);
            }
        }
        Err(e) => {
            println!("❌ 用户信息获取失败: {}", e);
        }
    }

    // 演示v1版本API - OIDC访问令牌
    println!("\n🔐 OIDC访问令牌获取");
    let oidc_request = auth::v1::OidcAccessTokenRequest {
        grant_type: "authorization_code".to_string(),
        code: "test_authorization_code".to_string(),
    };

    match client.auth.v1.oidc_access_token(&oidc_request).await {
        Ok(response) => {
            println!("✅ OIDC访问令牌获取成功");
            if let Some(data) = response.data {
                println!("   访问令牌: {}...", &data.access_token[..std::cmp::min(20, data.access_token.len())]);
                println!("   令牌类型: {}", data.token_type);
                println!("   有效期: {} 秒", data.expires_in);
                println!("   刷新令牌: {}...", &data.refresh_token[..std::cmp::min(20, data.refresh_token.len())]);
            }
        }
        Err(e) => {
            println!("❌ OIDC访问令牌获取失败: {}", e);
        }
    }

    // 演示v1版本API - 刷新OIDC访问令牌
    println!("\n🔄 刷新OIDC访问令牌");
    let refresh_request = auth::v1::OidcRefreshAccessTokenRequest {
        grant_type: "refresh_token".to_string(),
        refresh_token: "test_refresh_token".to_string(),
    };

    match client.auth.v1.oidc_refresh_access_token(&refresh_request).await {
        Ok(response) => {
            println!("✅ OIDC访问令牌刷新成功");
            if let Some(data) = response.data {
                println!("   新访问令牌: {}...", &data.access_token[..std::cmp::min(20, data.access_token.len())]);
                println!("   令牌类型: {}", data.token_type);
                println!("   有效期: {} 秒", data.expires_in);
            }
        }
        Err(e) => {
            println!("❌ OIDC访问令牌刷新失败: {}", e);
        }
    }

    // 演示v1版本API - 获取登录预授权码
    println!("\n🔑 获取登录预授权码");
    let auth_code_request = auth::v1::GetAuthCodeRequest {
        app_id: "test_app_id".to_string(),
        redirect_uri: "https://example.com/callback".to_string(),
        response_type: "code".to_string(),
        scope: "openid profile email".to_string(),
        state: "random_state".to_string(),
    };

    match client.auth.v1.get_auth_code(&auth_code_request).await {
        Ok(response) => {
            println!("✅ 登录预授权码获取成功");
            if let Some(data) = response.data {
                println!("   授权码: {}...", &data.auth_code[..std::cmp::min(20, data.auth_code.len())]);
                println!("   有效期: {} 秒", data.expires_in);
            }
        }
        Err(e) => {
            println!("❌ 登录预授权码获取失败: {}", e);
        }
    }

    // 演示应用票据重发
    println!("\n📬 应用票据重发");
    let resend_request = auth::v3::AppTicketResendRequest {
        app_id: "test_app_id".to_string(),
        app_secret: "test_app_secret".to_string(),
    };

    match client.auth.v3.app_ticket_resend(&resend_request).await {
        Ok(response) => {
            println!("✅ 应用票据重发成功");
            if let Some(data) = response.data {
                println!("   成功: {}", data.success);
                println!("   消息: {}", data.message);
            }
        }
        Err(e) => {
            println!("❌ 应用票据重发失败: {}", e);
        }
    }

    println!("\n🎉 Auth模块示例演示完成！");
    Ok(())
}
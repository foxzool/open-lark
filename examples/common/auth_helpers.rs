//! 认证辅助工具 - 简化版本
//!
//! 提供应用认证和用户认证的便捷功能。

// 仅在启用 auth 功能时导入相关类型
#[cfg(feature = "auth")]
pub use openlark_client::services::{AuthService, TokenInfo};

/// 设置应用认证
///
/// 获取应用访问令牌，用于应用级别的API调用
///
/// # 参数
/// - `config`: 客户端配置
///
/// # 返回值
/// - `Ok(TokenInfo)`: 应用访问令牌信息
/// - `Err(String)`: 认证失败
#[cfg(feature = "auth")]
pub async fn setup_app_auth(config: &openlark_client::Config) -> std::result::Result<TokenInfo, String> {
    println!("🔐 正在获取应用访问令牌...");

    let auth_service = AuthService::new(config);

    match auth_service.get_internal_app_access_token().await {
        Ok(token_info) => {
            println!("✅ 应用访问令牌获取成功");
            Ok(token_info)
        }
        Err(e) => {
            println!("❌ 应用访问令牌获取失败: {}", e);
            Err(format!("认证失败: {}", e))
        }
    }
}

/// 创建认证服务
///
/// 从配置创建认证服务实例
///
/// # 参数
/// - `config`: 客户端配置
///
/// # 返回值
/// - `Ok(AuthService)`: 认证服务实例
/// - `Err(String)`: 创建失败
#[cfg(feature = "auth")]
pub fn create_auth_service(config: &openlark_client::Config) -> std::result::Result<AuthService, String> {
    println!("🔧 正在创建认证服务...");
    let auth_service = AuthService::new(config);
    println!("✅ 认证服务创建成功");
    Ok(auth_service)
}

/// 当 auth 功能未启用时的占位函数
#[cfg(not(feature = "auth"))]
pub async fn setup_app_auth(_config: &openlark_client::Config) -> std::result::Result<TokenInfo, String> {
    println!("⚠️  认证功能未启用，请使用 --features auth 编译");
    Err("认证功能未启用，请使用 --features auth 编译".to_string())
}

/// 当 auth 功能未启用时的占位函数
#[cfg(not(feature = "auth"))]
pub fn create_auth_service(_config: &openlark_client::Config) -> std::result::Result<(), String> {
    println!("⚠️  认证功能未启用，请使用 --features auth 编译");
    Err("认证功能未启用，请使用 --features auth 编译".to_string())
}


//! 身份认证（Authentication）服务
//!
//! 提供飞书开放平台的身份认证和授权功能，支持多种认证方式和令牌管理。
//! 这是所有API调用的基础服务，负责处理应用和用户的身份验证。
//!
//! # 核心功能
//!
//! ## 应用认证
//! - 🔑 App Access Token 获取和刷新
//! - 🏢 Tenant Access Token 管理
//! - 🔄 令牌自动刷新和缓存
//! - ⏰ 令牌有效期管理
//!
//! ## 用户认证
//! - 👤 User Access Token 获取
//! - 🔐 OAuth 2.0 授权流程
//! - 🎫 授权码换取访问令牌
//! - 🔄 用户令牌刷新机制
//!
//! ## 身份验证
//! - ✅ 令牌有效性验证
//! - 👥 用户身份信息获取
//! - 🔍 权限范围检查
//! - 🛡️ 安全策略验证
//!
//! # 令牌类型说明
//!
//! ## App Access Token
//! 应用级别的访问令牌，用于访问不需要用户授权的API接口。
//! 适用于服务端应用的后台操作。
//!
//! ## Tenant Access Token
//! 企业级别的访问令牌，用于访问特定企业的资源和数据。
//! 需要企业管理员的授权和配置。
//!
//! ## User Access Token
//! 用户级别的访问令牌，用于访问用户个人数据和执行用户操作。
//! 需要用户明确授权和同意。
//!
//! # 使用示例
//!
//! ```rust
//! use open_lark::prelude::*;
//!
//! let client = LarkClient::builder("app_id", "app_secret")
//!     .with_app_type(AppType::SelfBuilt)
//!     .build();
//!
//! // 获取认证服务
//! let auth = &client.authentication;
//!
//! // 获取App Access Token
//! // let app_token_request = GetAppAccessTokenRequest::builder()
//! //     .app_id("app_id")
//! //     .app_secret("app_secret")
//! //     .build();
//! // let token = auth.v1.app_access_token.get(app_token_request, None).await?;
//!
//! // 获取用户访问令牌
//! // let user_token_request = GetUserAccessTokenRequest::builder()
//! //     .grant_type("authorization_code")
//! //     .code("authorization_code")
//! //     .build();
//! // let user_token = auth.v1.user_access_token.get(user_token_request, None).await?;
//! ```
//!
//! # 安全注意事项
//!
//! - 🔒 妥善保管应用凭据（App ID 和 App Secret）
//! - 🕐 及时刷新即将过期的令牌
//! - 🛡️ 使用HTTPS传输敏感信息
//! - 📝 记录和监控认证相关操作
//! - 🚫 避免在客户端暴露敏感凭据
//!
//! # 错误处理
//!
//! 认证失败时会返回相应的错误信息：
//! - 无效的应用凭据
//! - 令牌已过期
//! - 权限不足
//! - 网络连接问题

use std::sync::Arc;

/// 身份认证服务 v1 版本
pub mod v1;

/// 身份认证服务
///
/// 飞书开放平台身份认证和授权的统一管理入口。提供应用认证、用户认证、
/// 令牌管理等核心身份验证功能。
///
/// # 服务架构
///
/// - **v1**: 认证服务v1版本API，提供完整的认证功能集
///
/// # 核心特性
///
/// - 🔐 多种认证方式支持
/// - 🔄 自动令牌管理和刷新
/// - 🛡️ 企业级安全保障
/// - ⚡ 高性能令牌缓存
/// - 🎯 精细化权限控制
///
/// # 适用场景
///
/// - 服务端应用身份认证
/// - 用户授权和登录
/// - API访问权限管理
/// - 企业应用集成
/// - 第三方系统对接
///
/// # 最佳实践
///
/// - 定期轮换应用凭据
/// - 实施令牌最小权限原则
/// - 建立完善的访问日志
/// - 监控异常认证行为
/// - 遵循OAuth 2.0最佳实践
pub struct AuthenService {
    /// v1版本认证API服务
    pub v1: v1::V1,
}

impl AuthenService {
    /// 创建新的身份认证服务实例
    ///
    /// # 参数
    /// - `config`: 客户端配置，包含应用凭据和认证设置
    ///
    /// # 返回值
    /// 配置完成的身份认证服务实例
    pub fn new(config: Arc<crate::core::config::Config>) -> Self {
        Self {
            v1: v1::V1::new((*config).clone()),
        }
    }
}

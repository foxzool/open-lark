//! 共享数据模型
//!
//! 提供所有认证服务共享的数据模型和类型定义。

/// 认证配置
#[derive(Debug, Clone)]
pub struct AuthConfig {
    pub app_id: String,
    pub app_secret: String,
    pub base_url: String,
}

impl AuthConfig {
    pub fn new(app_id: impl Into<String>, app_secret: impl Into<String>) -> Self {
        Self {
            app_id: app_id.into(),
            app_secret: app_secret.into(),
            base_url: "https://open.feishu.cn".to_string(),
        }
    }

    /// 设置基础URL
    pub fn with_base_url(mut self, base_url: &str) -> Self {
        self.base_url = base_url.to_string();
        self
    }
}

impl Default for AuthConfig {
    fn default() -> Self {
        Self {
            app_id: "".to_string(),
            app_secret: "".to_string(),
            base_url: "https://open.feishu.cn".to_string(),
        }
    }
}

// 重新导出错误处理系统 - 使用新的统一错误架构
pub use super::error::{
    map_feishu_auth_error, AuthError, AuthErrorBuilder, AuthErrorExt, AuthResult,
};

// 子模块
pub mod oauth;
pub mod token;
pub mod user_info;

// 重新导出所有响应类型
pub use token::{
    AppAccessTokenResponse, AppTicketRequest, AppTicketResponse, TenantAccessTokenResponse,
};

// 导出用户访问令牌响应类型
pub use crate::authen::v1::access_token::UserAccessTokenResponse;

pub use user_info::{Gender, UserInfoResponse, UserStatus};

// OAuth 相关响应类型 (暂时未使用，保留备用)
// pub use oauth::*;

mod auth;

use crate::core::config::Config;

/// 认证服务 V1 版本
///
/// 提供完整的飞书开放平台认证功能，包括：
/// - 用户信息获取和管理
/// - 应用访问令牌管理
/// - 租户访问令牌管理
/// - 应用票据管理
#[derive(Debug, Clone)]
pub struct V1 {
    pub config: Config,
    /// 用户信息服务
    pub user_info: UserInfoService,
    /// 应用访问令牌服务
    pub app_access_token: AppAccessTokenService,
    /// 租户访问令牌服务
    pub tenant_access_token: TenantAccessTokenService,
    /// 应用票据服务
    pub app_ticket: AppTicketService,
}

impl V1 {
    /// 创建认证服务 V1 实例
    ///
    /// # 参数
    /// - `config`: SDK配置信息
    ///
    /// # 示例
    ///
    /// ```rust
    /// use open_lark::prelude::*;
    /// use open_lark::service::authentication::v1::V1;
    ///
    /// let config = Config::new("app_id", "app_secret");
    /// let auth_v1 = V1::new(config);
    /// ```
    pub fn new(config: Config) -> Self {
        let user_info = UserInfoService::new(config.clone());
        let app_access_token = AppAccessTokenService::new(config.clone());
        let tenant_access_token = TenantAccessTokenService::new(config.clone());
        let app_ticket = AppTicketService::new(config.clone());

        Self {
            config,
            user_info,
            app_access_token,
            tenant_access_token,
            app_ticket,
        }
    }
}

// 重新导出所有服务类型
pub use auth::{
    AppAccessTokenResponse, AppAccessTokenService, AppTicketService, GetAppAccessTokenBuilder,
    GetAppAccessTokenInternalBuilder, GetAppAccessTokenInternalRequest, GetAppAccessTokenRequest,
    GetTenantAccessTokenBuilder, GetTenantAccessTokenInternalBuilder,
    GetTenantAccessTokenInternalRequest, GetTenantAccessTokenRequest, GetUserInfoBuilder,
    ResendAppTicketBuilder, ResendAppTicketRequest, ResendAppTicketResponse,
    TenantAccessTokenResponse, TenantAccessTokenService, UserInfo, UserInfoService,
};

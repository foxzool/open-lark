pub use auth::*;
mod auth;
pub struct V1 {
/// 用户信息服务
    pub user_info: UserInfoService,
    /// App访问令牌服务
    pub app_access_token: AppAccessTokenService,
    /// Tenant访问令牌服务
    pub tenant_access_token: TenantAccessTokenService,
    /// App Ticket服务
    pub app_ticket: AppTicketService,
}
impl V1 {
    pub fn new() -> Self {
Self {
            user_info: UserInfoService::new(config.clone()),
            app_access_token: AppAccessTokenService::new(config.clone()),
            tenant_access_token: TenantAccessTokenService::new(config.clone()),
            app_ticket: AppTicketService::new(config),
        }
}
}

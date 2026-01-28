//! Auth v3 /auth 路径下的API实现
//!
//! 包含企业应用认证相关的具体API实现，按照CSV规范拆分为独立文件

use openlark_core::config::Config;

// 导入各个API实现
mod app_access_token;
mod app_access_token_internal;
mod app_ticket_resend;
mod tenant_access_token;
mod tenant_access_token_internal;

// 重新导出各个构建器
pub use app_access_token::AppAccessTokenBuilder;
pub use app_access_token_internal::AppAccessTokenInternalBuilder;
pub use app_ticket_resend::AppTicketResendBuilder;
pub use tenant_access_token::TenantAccessTokenBuilder;
pub use tenant_access_token_internal::TenantAccessTokenInternalRequestBuilder;

/// Auth v3 API服务
#[derive(Debug)]
pub struct AuthServiceV3 {
    config: Config,
}

impl AuthServiceV3 {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// 商店应用获取app_access_token
    pub fn app_access_token(&self) -> AppAccessTokenBuilder {
        AppAccessTokenBuilder::new(self.config.clone())
    }

    /// 自建应用获取app_access_token
    pub fn app_access_token_internal(&self) -> AppAccessTokenInternalBuilder {
        AppAccessTokenInternalBuilder::new(self.config.clone())
    }

    /// 商店应用获取tenant_access_token
    pub fn tenant_access_token(&self) -> TenantAccessTokenBuilder {
        TenantAccessTokenBuilder::new(self.config.clone())
    }

    /// 自建应用获取tenant_access_token
    pub fn tenant_access_token_internal(&self) -> TenantAccessTokenInternalRequestBuilder {
        TenantAccessTokenInternalRequestBuilder::new(self.config.clone())
    }

    /// 重新获取app_ticket
    pub fn app_ticket_resend(&self) -> AppTicketResendBuilder {
        AppTicketResendBuilder::new(self.config.clone())
    }
}

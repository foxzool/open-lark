//! Open-Lark Auth Module
//!
//! 飞书身份认证相关功能接口，包含应用或用户访问凭证获取与刷新等接口。
//!
//! 此模块重新导出 openlark-core 中的认证相关功能，提供统一的访问入口。

// 重新导出 openlark-core 中的认证相关模块
pub use openlark_core::{
    app_ticket_manager::{apply_app_ticket, AppTicketManager},
    cache::{CacheEntry, QuickCache},
    constants::{AppType, FEISHU_BASE_URL, LARK_BASE_URL},
    req_option::RequestOption,
    token_manager::{PreheatingConfig, TokenManager, TokenMetrics},
};

/// 认证服务聚合器，提供统一的认证接口访问
pub struct AuthService;

impl Default for AuthService {
    fn default() -> Self {
        Self::new()
    }
}

impl AuthService {
    /// 创建新的认证服务实例
    pub fn new() -> Self {
        Self
    }

    /// 获取Token管理器实例
    pub fn token_manager() -> TokenManager {
        TokenManager::new()
    }

    /// 获取AppTicket管理器实例
    pub fn app_ticket_manager() -> AppTicketManager {
        AppTicketManager::new()
    }
}

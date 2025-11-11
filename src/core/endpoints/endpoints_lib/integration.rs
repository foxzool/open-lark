//! 集成和外部系统服务端点
//!
//! 包含搜索、联系人、外部系统集成等相关的API端点。

/// 集成和外部系统相关端点
pub struct Integration;

impl Integration {
    // ==================== 搜索服务 ====================

    /// 用户搜索
    pub const SEARCH_USER: &'static str = "/open-apis/search/v1/user";

    /// 消息搜索
    pub const SEARCH_MESSAGE: &'static str = "/open-apis/search/v2/message";

    // ==================== 联系人管理 ====================

    /// 获取用户信息
    pub const CONTACT_USER_GET: &'static str = "/open-apis/contact/v3/users/{user_id}";

    /// 部门管理
    pub const CONTACT_DEPARTMENT: &'static str = "/open-apis/contact/v3/departments";

    // ==================== 外部系统集成 ====================

    /// Webhook事件
    pub const WEBHOOK_EVENTS: &'static str = "/open-apis/webhook/v4/events";

    /// 外部应用授权
    pub const EXTERNAL_APP_AUTH: &'static str = "/open-apis/authen/v1/apps/{app_id}/access_tokens";
}
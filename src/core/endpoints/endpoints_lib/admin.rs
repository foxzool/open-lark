//! 管理和认证服务端点
//!
//! 包含应用管理、认证、租户管理等相关的API端点。

/// 管理和认证相关端点
pub struct Admin;

impl Admin {
    // ==================== 应用管理 ====================

    /// 获取应用信息
    pub const APPLICATION_GET: &'static str = "/open-apis/application/v6/applications/{app_id}";

    /// 应用版本管理
    pub const APPLICATION_VERSIONS: &'static str = "/open-apis/application/v6/applications/{app_id}/versions";

    // ==================== 认证管理 ====================

    /// 获取访问令牌
    pub const AUTH_ACCESS_TOKEN: &'static str = "/open-apis/auth/v3/tenant_access_token/internal";

    /// 刷新访问令牌
    pub const AUTH_REFRESH_TOKEN: &'static str = "/open-apis/auth/v3/tenant_access_token/refresh";

    // ==================== 租户管理 ====================

    /// 获取租户信息
    pub const TENANT_INFO: &'static str = "/open-apis/tenant/v2/tenant/info";

    /// 工作台访问数据
    pub const WORKPLACE_ACCESS_DATA: &'static str = "/open-apis/workplace/v1/workplace_access_data/search";
}
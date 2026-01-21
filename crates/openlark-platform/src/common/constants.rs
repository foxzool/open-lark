//! 平台服务常量

/// API 端点常量
pub mod endpoints {
    /// 应用引擎 API 基础路径
    pub const APP_ENGINE_BASE: &str = "/open-apis/app-engine";

    /// 目录服务 API 基础路径
    pub const DIRECTORY_BASE: &str = "/open-apis/directory";

    /// 系统管理 API 基础路径
    pub const ADMIN_BASE: &str = "/open-apis/admin";
}

/// 应用状态常量
pub mod app_status {
    /// 应用启用
    pub const ENABLED: &str = "enabled";
    /// 应用禁用
    pub const DISABLED: &str = "disabled";
    /// 应用归档
    pub const ARCHIVED: &str = "archived";
}

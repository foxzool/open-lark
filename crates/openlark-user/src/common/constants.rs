//! 用户设置常量

/// API 端点常量
pub mod endpoints {
    /// 用户设置 API 基础路径
    pub const SETTINGS_BASE: &str = "/open-apis/user/v1/settings";

    /// 用户偏好 API 基础路径
    pub const PREFERENCES_BASE: &str = "/open-apis/user/v1/preferences";
}

/// 设置类型常量
pub mod setting_type {
    /// 通知设置
    pub const NOTIFICATION: &str = "notification";
    /// 隐私设置
    pub const PRIVACY: &str = "privacy";
    /// 界面设置
    pub const UI: &str = "ui";
}

//! 平台服务常量

/// API 端点常量
pub mod endpoints {
    /// 应用引擎 API 基础路径
    pub const APP_ENGINE_BASE: &str = "/open-apis/app-engine";

    /// 目录服务 API 基础路径
    pub const DIRECTORY_BASE: &str = "/open-apis/directory";

    /// 系统管理 API 基础路径
    pub const ADMIN_BASE: &str = "/open-apis/admin";

    /// 妙搭平台 API 基础路径
    pub const SPARK_BASE: &str = "/open-apis/spark";

    /// 妙搭和飞书用户 ID 转换
    pub const SPARK_V1_DIRECTORY_USER_ID_CONVERT: &str =
        "/open-apis/spark/v1/directory/user/id_convert";
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

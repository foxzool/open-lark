//! Personal Settings 个人设置服务端点常量定义

// ===== 系统状态管理 =====

/// 系统状态管理
pub const PERSONAL_SETTINGS_V1_SYSTEM_STATUSES: &str = "/open-apis/personal_settings/v1/system_statuses";

/// 系统状态操作
pub const PERSONAL_SETTINGS_V1_SYSTEM_STATUS_OPERATION: &str =
    "/open-apis/personal_settings/v1/system_statuses/{system_status_id}";

/// 批量开启系统状态
pub const PERSONAL_SETTINGS_V1_SYSTEM_STATUS_BATCH_OPEN: &str =
    "/open-apis/personal_settings/v1/system_statuses/batch_open";

/// 批量关闭系统状态
pub const PERSONAL_SETTINGS_V1_SYSTEM_STATUS_BATCH_CLOSE: &str =
    "/open-apis/personal_settings/v1/system_statuses/batch_close";

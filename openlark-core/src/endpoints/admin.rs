//! admin 服务端点常量定义
//!
//! Admin (管理后台) 相关 API 端点常量，包括：
//! - 登录密码管理
//! - 勋章管理
//! - 勋章授予名单管理
//! - 数据报表

// ==================== 登录密码管理 ====================

/// 登录密码管理 - 重置用户密码
pub const ADMIN_V1_PASSWORD_RESET: &str = "/open-apis/admin/v1/password/reset";

// ==================== 勋章管理 ====================

/// 勋章管理 - 创建勋章
pub const ADMIN_V1_BADGES_CREATE: &str = "/open-apis/admin/v1/badges";

/// 勋章管理 - 获取勋章列表
pub const ADMIN_V1_BADGES_LIST: &str = "/open-apis/admin/v1/badges";

/// 勋章管理 - 操作勋章（获取/更新）
pub const ADMIN_V1_BADGES_OPERATION: &str = "/open-apis/admin/v1/badges/{badge_id}";

/// 勋章管理 - 上传勋章图片
pub const ADMIN_V1_BADGES_IMAGE_UPLOAD: &str = "/open-apis/admin/v1/badges/image";

// ==================== 勋章授予名单管理 ====================

/// 勋章授予名单 - 创建授予名单
pub const ADMIN_V1_BADGE_GRANTS_CREATE: &str = "/open-apis/admin/v1/badge_grants";

/// 勋章授予名单 - 获取授予名单列表
pub const ADMIN_V1_BADGE_GRANTS_LIST: &str = "/open-apis/admin/v1/badge_grants";

/// 勋章授予名单 - 操作授予名单（获取/更新/删除）
pub const ADMIN_V1_BADGE_GRANTS_OPERATION: &str = "/open-apis/admin/v1/badge_grants/{grant_id}";

// ==================== 数据报表 ====================

/// 数据报表 - 部门维度数据
pub const ADMIN_V1_DATA_REPORT_DEPARTMENT: &str = "/open-apis/admin/v1/data_report/department";

/// 数据报表 - 用户维度数据
pub const ADMIN_V1_DATA_REPORT_USER: &str = "/open-apis/admin/v1/data_report/user";

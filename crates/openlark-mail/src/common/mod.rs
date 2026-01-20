/// 通用模块
///
/// 提供openlark-mail项目中通用的工具、宏和类型定义。
pub mod api_endpoints;
pub mod api_utils;

// 重新导出API端点枚举
pub use api_endpoints::MailApiV1;

/// 通用常量定义
pub mod constants {
    /// 默认分页大小
    pub const DEFAULT_PAGE_SIZE: i32 = 20;
    /// 最大分页大小
    pub const MAX_PAGE_SIZE: i32 = 100;
}

/// 通用类型别名
pub mod types {
    /// 邮件组 ID
    pub type MailGroupId = String;
    /// 邮箱地址
    pub type EmailAddress = String;
}

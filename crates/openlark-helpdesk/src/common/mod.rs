/// 帮助台 API 端点定义。
pub mod api_endpoints;
/// 帮助台 API 通用辅助函数。
pub mod api_utils;

pub use api_endpoints::HelpdeskApiV1;

/// 帮助台公共常量。
pub mod constants {
    /// 默认分页大小。
    pub const DEFAULT_PAGE_SIZE: i32 = 20;
    /// 最大分页大小。
    pub const MAX_PAGE_SIZE: i32 = 100;
}

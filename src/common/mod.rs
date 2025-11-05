//! 共享数据模型和工具
//!
//! 本模块提供跨服务共享的数据结构、类型定义和工具函数。
//! 所有服务模块都可以使用这些通用组件，避免重复定义。

pub mod models;
pub mod types;
pub mod utils;

// 重新导出常用类型
pub use models::*;
pub use types::*;

/// 共享数据模型的版本信息
pub const COMMON_VERSION: &str = "1.0.0";

/// 常用常量
pub mod constants {
    /// 默认分页大小
    pub const DEFAULT_PAGE_SIZE: i32 = 20;

    /// 最大分页大小
    pub const MAX_PAGE_SIZE: i32 = 100;

    /// 默认超时时间（秒）
    pub const DEFAULT_TIMEOUT: u64 = 30;
}

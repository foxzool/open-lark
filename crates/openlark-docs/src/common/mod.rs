//! 通用模块
//!
//! 提供openlark-docs项目中通用的工具、宏和类型定义。

pub mod builders;

// 宏定义在builders模块中，通过lib.rs重新导出

/// 通用常量定义
pub mod constants {
    /// 默认分页大小
    pub const DEFAULT_PAGE_SIZE: i32 = 20;
    /// 最大分页大小
    pub const MAX_PAGE_SIZE: i32 = 100;
    /// 默认超时时间（秒）
    pub const DEFAULT_TIMEOUT_SECS: u64 = 30;
}

/// 通用类型别名
pub mod types {
    /// 通用字符串类型
    pub type AppToken = String;
    pub type TableId = String;
    pub type RecordId = String;
    pub type FormId = String;
    pub type ViewId = String;
    pub type FieldId = String;
    pub type RoleId = String;
    pub type UserId = String;
}

/// 通用特征定义
pub mod traits {
    /// API请求基础特征
    pub trait ApiRequest {
        type Response;
        fn validate(&self) -> openlark_core::SDKResult<()>;
        fn build_path(&self) -> String;
    }

    /// 可分页请求特征
    pub trait PaginatedRequest {
        fn page_token(self, token: impl Into<String>) -> Self;
        fn page_size(self, size: i32) -> Self;
    }

    /// 可筛选请求特征
    pub trait FilterableRequest {
        fn add_filter(self, filter: serde_json::Value) -> Self;
    }
}
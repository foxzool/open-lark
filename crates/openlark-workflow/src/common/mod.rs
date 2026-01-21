/// 通用模块
///
/// 提供openlark-workflow项目中通用的工具、宏和类型定义。
pub mod api_endpoints;
pub mod api_utils;

// 重新导出API端点枚举
pub use api_endpoints::TaskApiV2;

/// 通用常量定义
pub mod constants {
    /// 默认分页大小
    pub const DEFAULT_PAGE_SIZE: i32 = 20;
    /// 最大分页大小
    pub const MAX_PAGE_SIZE: i32 = 100;
}

/// 通用类型别名
pub mod types {
    /// 任务 GUID
    pub type TaskGuid = String;
    /// 任务清单 GUID
    pub type TasklistGuid = String;
    /// 分组 GUID
    pub type SectionGuid = String;
    /// 自定义字段 GUID
    pub type CustomFieldGuid = String;
    /// 评论 GUID
    pub type CommentGuid = String;
    /// 附件 GUID
    pub type AttachmentGuid = String;
}

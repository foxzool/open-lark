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

#[cfg(test)]
mod tests {
    use super::constants;
    use super::types;

    #[test]
    fn test_default_page_size() {
        assert_eq!(constants::DEFAULT_PAGE_SIZE, 20);
    }

    #[test]
    fn test_max_page_size() {
        assert_eq!(constants::MAX_PAGE_SIZE, 100);
    }

    #[test]
    fn test_task_guid_type() {
        let guid: types::TaskGuid = "test_guid".to_string();
        assert_eq!(guid, "test_guid");
    }

    #[test]
    fn test_tasklist_guid_type() {
        let guid: types::TasklistGuid = "test_tasklist".to_string();
        assert_eq!(guid, "test_tasklist");
    }

    #[test]
    fn test_section_guid_type() {
        let guid: types::SectionGuid = "test_section".to_string();
        assert_eq!(guid, "test_section");
    }

    #[test]
    fn test_custom_field_guid_type() {
        let guid: types::CustomFieldGuid = "test_field".to_string();
        assert_eq!(guid, "test_field");
    }

    #[test]
    fn test_comment_guid_type() {
        let guid: types::CommentGuid = "test_comment".to_string();
        assert_eq!(guid, "test_comment");
    }

    #[test]
    fn test_attachment_guid_type() {
        let guid: types::AttachmentGuid = "test_attachment".to_string();
        assert_eq!(guid, "test_attachment");
    }
}

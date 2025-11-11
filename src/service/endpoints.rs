/// 飞书开放API接口端点常量管理
///
/// 此模块重新导出统一端点定义，提供向后兼容性
///
/// 注意：这些端点定义已经迁移到 src/core/endpoints_unified.rs
/// 为了向后兼容性，这里重新导出常用的端点常量
// 重新导出统一端点系统的常用端点
pub use crate::core::endpoints_unified::Endpoints;

// 为了向后兼容性，提供常用的端点常量别名
pub const IM_V1_MESSAGES: &str = Endpoints::IM_V1_MESSAGES;
pub const IM_V1_CHATS: &str = Endpoints::IM_V1_CHATS;
pub const IM_V1_BATCH_MESSAGES: &str = Endpoints::IM_V1_BATCH_MESSAGES;
pub const IM_V2_MESSAGES: &str = "/open-apis/im/v2/messages";

// 认证相关 - V3
pub const AUTH_APP_ACCESS_TOKEN: &str = Endpoints::AUTH_V3_APP_ACCESS_TOKEN;
pub const AUTH_TENANT_ACCESS_TOKEN: &str = Endpoints::AUTH_V3_TENANT_ACCESS_TOKEN;
pub const AUTH_APP_TICKET_RESEND: &str = "/open-apis/auth/v3/app_ticket/resend";

// Wiki相关 - V2
pub const WIKI_V2_SPACES: &str = "/open-apis/wiki/v2/spaces";
pub const WIKI_V2_SPACE_NODES: &str = "/open-apis/wiki/v2/spaces/{space_id}/nodes";
pub const WIKI_V2_TASKS: &str = "/open-apis/wiki/v2/tasks";

// 云文档相关 - V1
pub const DOCS_V1_DOCUMENTS: &str = "/open-apis/docx/v1/documents";

// 云盘相关 - V1/V2
pub const DRIVE_V1_FILES: &str = Endpoints::DRIVE_V1_FILES;
pub const DRIVE_V1_FOLDERS: &str = Endpoints::DRIVE_V1_FOLDERS;
pub const DRIVE_V2_FILES: &str = "/open-apis/drive/v2/files";

// 搜索相关 - V1
pub const SEARCH_V1_DATA_SOURCES: &str = "/open-apis/search/v1/data_sources";
pub const SEARCH_V1_SCHEMA: &str = "/open-apis/search/v1/schema";
pub const SEARCH_V1_USER: &str = Endpoints::SEARCH_V1_USER;

// 电子表格相关 - V2/V3
pub const SHEETS_V2_SPREADSHEETS: &str = Endpoints::SHEETS_V2_SPREADSHEETS;
pub const SHEETS_V3_SPREADSHEETS: &str = Endpoints::SHEETS_V3_SPREADSHEETS;

// 多维表格相关 - V1
pub const BITABLE_V1_APPS: &str = Endpoints::BITABLE_V1_APPS;

// 考勤相关 - V1
pub const ATTENDANCE_V1_USER_FLOWS: &str = "/open-apis/attendance/v1/user_flows";

// 评论相关 - V1
pub const COMMENTS_V1_COMMENTS: &str = "/open-apis/drive/v1/files/{file_token}/comments";

// 权限相关 - V1/V2
pub const PERMISSION_V1_SETTINGS: &str = "/open-apis/drive/permission/v1/settings";
pub const PERMISSION_V2_SETTINGS: &str = "/open-apis/drive/permission/v2/settings";

// 画板相关 - V1
pub const BOARD_V1_WHITEBOARDS: &str = "/open-apis/board/v1/whiteboards";

// 云文档助手相关 - V1
pub const ASSISTANT_V1_CONVERSATIONS: &str = "/open-apis/ai/v1/conversations";

// 审批相关 - V4
pub const APPROVAL_V4_INSTANCES_SEARCH: &str = "/open-apis/approval/v4/instances/search";
pub const APPROVAL_V4_TASKS_SEARCH: &str = "/open-apis/approval/v4/tasks/search";
pub const APPROVAL_V4_INSTANCES_SEARCH_CC: &str = "/open-apis/approval/v4/instances/search_cc";
pub const APPROVAL_V4_APPROVALS_SEARCH: &str = "/open-apis/approval/v4/approvals/search";
pub const APPROVAL_V4_TASKS_QUERY: &str = "/open-apis/approval/v4/tasks/query";

/// API路径辅助函数
pub struct EndpointHelper;

impl EndpointHelper {
    /// 替换URL路径参数
    pub fn replace_path_params(url: &str, params: &[(&str, &str)]) -> String {
        let mut result = url.to_string();
        for (key, value) in params {
            result = result.replace(&format!("{{{}}}", key), value);
        }
        result
    }

    /// 检查URL是否包含未解析的参数
    pub fn has_unresolved_params(url: &str) -> bool {
        url.contains('{') && url.contains('}')
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_replace_path_params() {
        let result =
            EndpointHelper::replace_path_params(WIKI_V2_SPACE_NODES, &[("space_id", "space123")]);
        assert_eq!(result, "/open-apis/wiki/v2/spaces/space123/nodes");
    }

    #[test]
    fn test_has_unresolved_params() {
        assert!(EndpointHelper::has_unresolved_params(WIKI_V2_SPACE_NODES));
        assert!(!EndpointHelper::has_unresolved_params(WIKI_V2_SPACES));
    }

    #[test]
    fn test_unified_endpoints_compatibility() {
        // 验证统一端点系统的兼容性
        assert_eq!(IM_V1_MESSAGES, Endpoints::IM_V1_MESSAGES);
        assert_eq!(IM_V1_CHATS, Endpoints::IM_V1_CHATS);
        assert_eq!(SEARCH_V1_USER, Endpoints::SEARCH_V1_USER);
    }
}

/// 飞书开放API接口端点常量管理
///
/// 此模块集中管理所有API路径，提供统一的入口便于维护和更新
// 认证相关
pub const AUTH_APP_ACCESS_TOKEN: &str = "/open-apis/auth/v3/app_access_token";
pub const AUTH_TENANT_ACCESS_TOKEN: &str = "/open-apis/auth/v3/tenant_access_token";
pub const AUTH_APP_TICKET_RESEND: &str = "/open-apis/auth/v3/app_ticket/resend";

// Wiki相关 - V2
pub const WIKI_V2_SPACES: &str = "/open-apis/wiki/v2/spaces";
pub const WIKI_V2_SPACE_NODES: &str = "/open-apis/wiki/v2/spaces/{space_id}/nodes";
pub const WIKI_V2_TASKS: &str = "/open-apis/wiki/v2/tasks";

// IM即时通讯相关 - V1
pub const IM_V1_MESSAGES: &str = "/open-apis/im/v1/messages";
pub const IM_V1_CHATS: &str = "/open-apis/im/v1/chats";
pub const IM_V1_BATCH_MESSAGES: &str = "/open-apis/im/v1/batch_messages";

// IM即时通讯相关 - V2
pub const IM_V2_MESSAGES: &str = "/open-apis/im/v2/messages";

// 云文档相关 - V1
pub const DOCS_V1_DOCUMENTS: &str = "/open-apis/docx/v1/documents";

// 云盘相关 - V1
pub const DRIVE_V1_FILES: &str = "/open-apis/drive/v1/files";
pub const DRIVE_V1_FOLDERS: &str = "/open-apis/drive/v1/folders";

// 云盘相关 - V2
pub const DRIVE_V2_FILES: &str = "/open-apis/drive/v2/files";

// 搜索相关 - V1
pub const SEARCH_V1_DATA_SOURCES: &str = "/open-apis/search/v1/data_sources";
pub const SEARCH_V1_SCHEMA: &str = "/open-apis/search/v1/schema";

// 电子表格相关 - V2
pub const SHEETS_V2_SPREADSHEETS: &str = "/open-apis/sheets/v2/spreadsheets";

// 电子表格相关 - V3
pub const SHEETS_V3_SPREADSHEETS: &str = "/open-apis/sheets/v3/spreadsheets";

// 多维表格相关 - V1
pub const BITABLE_V1_APPS: &str = "/open-apis/bitable/v1/apps";

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

/// API路径辅助函数
impl EndpointHelper {
    /// 替换路径中的参数占位符
    ///
    /// # Example
    /// ```rust,ignore
    /// let path = EndpointHelper::replace_path_params(
    ///     WIKI_V2_SPACE_NODES,
    ///     &[("space_id", "space123")]
    /// );
    /// assert_eq!(path, "/open-apis/wiki/v2/spaces/space123/nodes");
    /// ```
    pub fn replace_path_params(path: &str, params: &[(&str, &str)]) -> String {
        let mut result = path.to_string();
        for (key, value) in params {
            let placeholder = format!("{{{key}}}");
            result = result.replace(&placeholder, value);
        }
        result
    }

    /// 检查路径是否包含未替换的参数
    pub fn has_unresolved_params(path: &str) -> bool {
        path.contains('{') && path.contains('}')
    }
}

pub struct EndpointHelper;

#[cfg(test)]
#[allow(unused_variables, unused_unsafe)]
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
}

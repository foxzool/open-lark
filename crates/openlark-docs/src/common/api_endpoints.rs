//! API端点定义
//!
//! 提供类型安全的API端点管理，替代字符串拼接方式。

/// Base API V2 端点枚举
#[derive(Debug, Clone, PartialEq)]
pub enum BaseApiV2 {
    /// 新增自定义角色
    RoleCreate(String),
    /// 更新自定义角色
    RoleUpdate(String, String),
    /// 列出自定义角色
    RoleList(String),
}

impl BaseApiV2 {
    /// 生成对应的 URL
    pub fn to_url(&self) -> String {
        match self {
            BaseApiV2::RoleCreate(app_token) => {
                format!("/open-apis/base/v2/apps/{}/roles", app_token)
            }
            BaseApiV2::RoleUpdate(app_token, role_id) => {
                format!("/open-apis/base/v2/apps/{}/roles/{}", app_token, role_id)
            }
            BaseApiV2::RoleList(app_token) => {
                format!("/open-apis/base/v2/apps/{}/roles", app_token)
            }
        }
    }
}

/// Bitable API V1 端点枚举
#[derive(Debug, Clone, PartialEq)]
pub enum BitableApiV1 {
    /// App管理相关
    AppCreate,
    AppCopy,
    AppGet(String),
    AppUpdate(String),
    AppList,
    DashboardList(String),

    /// 表格管理相关
    TableCreate(String),
    TableBatchCreate(String),
    TableUpdate(String, String),
    TableDelete(String, String),
    TableBatchDelete(String),
    TableGet(String, String),
    TableList(String),
    TablePatch(String, String),

    /// 字段管理相关
    FieldCreate(String, String),
    FieldUpdate(String, String, String),
    FieldDelete(String, String, String),
    FieldList(String, String),

    /// 视图管理相关
    ViewCreate(String, String),
    ViewUpdate(String, String, String),
    ViewDelete(String, String, String),
    ViewGet(String, String, String),
    ViewList(String, String),
    ViewPatch(String, String, String),

    /// 记录管理相关
    RecordCreate(String, String),
    RecordBatchCreate(String, String),
    RecordGet(String, String, String),
    RecordBatchGet(String, String),
    RecordUpdate(String, String, String),
    RecordBatchUpdate(String, String),
    RecordDelete(String, String, String),
    RecordBatchDelete(String, String),
    RecordList(String, String),
    RecordSearch(String, String),

    /// 权限管理相关
    RoleCreate(String),
    RoleUpdate(String, String),
    RoleDelete(String, String),
    RoleList(String),
    RoleMemberCreate(String, String),
    RoleMemberBatchCreate(String, String),
    RoleMemberDelete(String, String, String),
    RoleMemberBatchDelete(String, String),
    RoleMemberList(String, String),
}

impl BitableApiV1 {
    /// 生成对应的 URL
    pub fn to_url(&self) -> String {
        match self {
            // App管理
            BitableApiV1::AppCreate => "/open-apis/bitable/v1/apps".to_string(),
            BitableApiV1::AppCopy => "/open-apis/bitable/v1/apps/copy".to_string(),
            BitableApiV1::AppGet(app_token) => {
                format!("/open-apis/bitable/v1/apps/{}", app_token)
            }
            BitableApiV1::AppUpdate(app_token) => {
                format!("/open-apis/bitable/v1/apps/{}", app_token)
            }
            BitableApiV1::AppList => "/open-apis/bitable/v1/apps".to_string(),
            BitableApiV1::DashboardList(app_token) => {
                format!("/open-apis/bitable/v1/apps/{}/dashboard/list", app_token)
            }

            // 表格管理
            BitableApiV1::TableCreate(app_token) => {
                format!("/open-apis/bitable/v1/apps/{}/tables", app_token)
            }
            BitableApiV1::TableBatchCreate(app_token) => {
                format!(
                    "/open-apis/bitable/v1/apps/{}/tables/batch_create",
                    app_token
                )
            }
            BitableApiV1::TableUpdate(app_token, table_id) => {
                format!(
                    "/open-apis/bitable/v1/apps/{}/tables/{}",
                    app_token, table_id
                )
            }
            BitableApiV1::TableDelete(app_token, table_id) => {
                format!(
                    "/open-apis/bitable/v1/apps/{}/tables/{}",
                    app_token, table_id
                )
            }
            BitableApiV1::TableBatchDelete(app_token) => {
                format!(
                    "/open-apis/bitable/v1/apps/{}/tables/batch_delete",
                    app_token
                )
            }
            BitableApiV1::TableGet(app_token, table_id) => {
                format!(
                    "/open-apis/bitable/v1/apps/{}/tables/{}",
                    app_token, table_id
                )
            }
            BitableApiV1::TableList(app_token) => {
                format!("/open-apis/bitable/v1/apps/{}/tables", app_token)
            }
            BitableApiV1::TablePatch(app_token, table_id) => {
                format!(
                    "/open-apis/bitable/v1/apps/{}/tables/{}",
                    app_token, table_id
                )
            }

            // 字段管理
            BitableApiV1::FieldCreate(app_token, table_id) => {
                format!(
                    "/open-apis/bitable/v1/apps/{}/tables/{}/fields",
                    app_token, table_id
                )
            }
            BitableApiV1::FieldUpdate(app_token, table_id, field_id) => {
                format!(
                    "/open-apis/bitable/v1/apps/{}/tables/{}/fields/{}",
                    app_token, table_id, field_id
                )
            }
            BitableApiV1::FieldDelete(app_token, table_id, field_id) => {
                format!(
                    "/open-apis/bitable/v1/apps/{}/tables/{}/fields/{}",
                    app_token, table_id, field_id
                )
            }
            BitableApiV1::FieldList(app_token, table_id) => {
                format!(
                    "/open-apis/bitable/v1/apps/{}/tables/{}/fields",
                    app_token, table_id
                )
            }

            // 视图管理
            BitableApiV1::ViewCreate(app_token, table_id) => {
                format!(
                    "/open-apis/bitable/v1/apps/{}/tables/{}/views",
                    app_token, table_id
                )
            }
            BitableApiV1::ViewUpdate(app_token, table_id, view_id) => {
                format!(
                    "/open-apis/bitable/v1/apps/{}/tables/{}/views/{}",
                    app_token, table_id, view_id
                )
            }
            BitableApiV1::ViewDelete(app_token, table_id, view_id) => {
                format!(
                    "/open-apis/bitable/v1/apps/{}/tables/{}/views/{}",
                    app_token, table_id, view_id
                )
            }
            BitableApiV1::ViewGet(app_token, table_id, view_id) => {
                format!(
                    "/open-apis/bitable/v1/apps/{}/tables/{}/views/{}",
                    app_token, table_id, view_id
                )
            }
            BitableApiV1::ViewList(app_token, table_id) => {
                format!(
                    "/open-apis/bitable/v1/apps/{}/tables/{}/views",
                    app_token, table_id
                )
            }
            BitableApiV1::ViewPatch(app_token, table_id, view_id) => {
                format!(
                    "/open-apis/bitable/v1/apps/{}/tables/{}/views/{}",
                    app_token, table_id, view_id
                )
            }

            // 记录管理
            BitableApiV1::RecordCreate(app_token, table_id) => {
                format!(
                    "/open-apis/bitable/v1/apps/{}/tables/{}/records",
                    app_token, table_id
                )
            }
            BitableApiV1::RecordBatchCreate(app_token, table_id) => {
                format!(
                    "/open-apis/bitable/v1/apps/{}/tables/{}/records/batch_create",
                    app_token, table_id
                )
            }
            BitableApiV1::RecordGet(app_token, table_id, record_id) => {
                format!(
                    "/open-apis/bitable/v1/apps/{}/tables/{}/records/{}",
                    app_token, table_id, record_id
                )
            }
            BitableApiV1::RecordBatchGet(app_token, table_id) => {
                format!(
                    "/open-apis/bitable/v1/apps/{}/tables/{}/records/batch_get",
                    app_token, table_id
                )
            }
            BitableApiV1::RecordUpdate(app_token, table_id, record_id) => {
                format!(
                    "/open-apis/bitable/v1/apps/{}/tables/{}/records/{}",
                    app_token, table_id, record_id
                )
            }
            BitableApiV1::RecordBatchUpdate(app_token, table_id) => {
                format!(
                    "/open-apis/bitable/v1/apps/{}/tables/{}/records/batch_update",
                    app_token, table_id
                )
            }
            BitableApiV1::RecordDelete(app_token, table_id, record_id) => {
                format!(
                    "/open-apis/bitable/v1/apps/{}/tables/{}/records/{}",
                    app_token, table_id, record_id
                )
            }
            BitableApiV1::RecordBatchDelete(app_token, table_id) => {
                format!(
                    "/open-apis/bitable/v1/apps/{}/tables/{}/records/batch_delete",
                    app_token, table_id
                )
            }
            BitableApiV1::RecordList(app_token, table_id) => {
                format!(
                    "/open-apis/bitable/v1/apps/{}/tables/{}/records",
                    app_token, table_id
                )
            }
            BitableApiV1::RecordSearch(app_token, table_id) => {
                format!(
                    "/open-apis/bitable/v1/apps/{}/tables/{}/records/search",
                    app_token, table_id
                )
            }

            // 权限管理
            BitableApiV1::RoleCreate(app_token) => {
                format!("/open-apis/bitable/v1/apps/{}/roles", app_token)
            }
            BitableApiV1::RoleUpdate(app_token, role_id) => {
                format!("/open-apis/bitable/v1/apps/{}/roles/{}", app_token, role_id)
            }
            BitableApiV1::RoleDelete(app_token, role_id) => {
                format!("/open-apis/bitable/v1/apps/{}/roles/{}", app_token, role_id)
            }
            BitableApiV1::RoleList(app_token) => {
                format!("/open-apis/bitable/v1/apps/{}/roles", app_token)
            }
            BitableApiV1::RoleMemberCreate(app_token, role_id) => {
                format!(
                    "/open-apis/bitable/v1/apps/{}/roles/{}/members",
                    app_token, role_id
                )
            }
            BitableApiV1::RoleMemberBatchCreate(app_token, role_id) => {
                format!(
                    "/open-apis/bitable/v1/apps/{}/roles/{}/members/batch_create",
                    app_token, role_id
                )
            }
            BitableApiV1::RoleMemberDelete(app_token, role_id, member_id) => {
                format!(
                    "/open-apis/bitable/v1/apps/{}/roles/{}/members/{}",
                    app_token, role_id, member_id
                )
            }
            BitableApiV1::RoleMemberBatchDelete(app_token, role_id) => {
                format!(
                    "/open-apis/bitable/v1/apps/{}/roles/{}/members/batch_delete",
                    app_token, role_id
                )
            }
            BitableApiV1::RoleMemberList(app_token, role_id) => {
                format!(
                    "/open-apis/bitable/v1/apps/{}/roles/{}/members",
                    app_token, role_id
                )
            }
        }
    }
}

/// Minutes API V1 端点枚举
#[derive(Debug, Clone, PartialEq)]
pub enum MinutesApiV1 {
    /// 获取妙记信息
    Get(String),
    /// 下载妙记音视频文件
    MediaGet(String),
    /// 导出妙记文字记录
    TranscriptGet(String),
    /// 获取妙记统计数据
    StatisticsGet(String),
}

impl MinutesApiV1 {
    /// 生成对应的 URL
    pub fn to_url(&self) -> String {
        match self {
            MinutesApiV1::Get(minute_token) => {
                format!("/open-apis/minutes/v1/minutes/{}", minute_token)
            }
            MinutesApiV1::MediaGet(minute_token) => {
                format!("/open-apis/minutes/v1/minutes/{}/media", minute_token)
            }
            MinutesApiV1::TranscriptGet(minute_token) => {
                format!("/open-apis/minutes/v1/minutes/{}/transcript", minute_token)
            }
            MinutesApiV1::StatisticsGet(minute_token) => {
                format!("/open-apis/minutes/v1/minutes/{}/statistics", minute_token)
            }
        }
    }
}

/// Wiki API V1 端点枚举
#[derive(Debug, Clone, PartialEq)]
pub enum WikiApiV1 {
    /// 搜索Wiki
    NodeSearch,
}

impl WikiApiV1 {
    /// 生成对应的 URL
    pub fn to_url(&self) -> String {
        match self {
            WikiApiV1::NodeSearch => "/open-apis/wiki/v1/nodes/search".to_string(),
        }
    }
}

/// Docs API V1 端点枚举
#[derive(Debug, Clone, PartialEq)]
pub enum DocsApiV1 {
    /// 获取云文档内容
    ContentGet,
}

impl DocsApiV1 {
    /// 生成对应的 URL
    pub fn to_url(&self) -> String {
        match self {
            DocsApiV1::ContentGet => "/open-apis/docs/v1/content".to_string(),
        }
    }
}

/// Docx API V1 端点枚举
#[derive(Debug, Clone, PartialEq)]
pub enum DocxApiV1 {
    // 群公告相关API (7个)
    /// 获取群公告基本信息
    ChatAnnouncementGet(String),
    /// 获取群公告所有块
    ChatAnnouncementBlockList(String),
    /// 在群公告中创建块
    ChatAnnouncementBlockChildrenCreate(String, String),
    /// 批量更新群公告块的内容
    ChatAnnouncementBlockBatchUpdate(String),
    /// 获取群公告块的内容
    ChatAnnouncementBlockGet(String, String),
    /// 获取所有子块
    ChatAnnouncementBlockChildrenGet(String, String),
    /// 删除群公告中的块
    ChatAnnouncementBlockChildrenBatchDelete(String, String),

    // 文档相关API (12个)
    /// 创建文档
    DocumentCreate,
    /// 获取文档基本信息
    DocumentGet(String),
    /// 获取文档纯文本内容
    DocumentRawContent(String),
    /// 获取文档所有块
    DocumentBlockList(String),
    /// 创建块
    DocumentBlockChildrenCreate(String, String),
    /// 创建嵌套块
    DocumentBlockDescendantCreate(String, String),
    /// 更新块的内容
    DocumentBlockPatch(String, String),
    /// 获取块的内容
    DocumentBlockGet(String, String),
    /// 批量更新块的内容
    DocumentBlockBatchUpdate(String),
    /// 获取所有子块
    DocumentBlockChildrenGet(String, String),
    /// 删除块
    DocumentBlockChildrenBatchDelete(String, String),
    /// Markdown/HTML 内容转换为文档块
    DocumentConvert,
}

impl DocxApiV1 {
    /// 生成对应的 URL
    pub fn to_url(&self) -> String {
        match self {
            // 群公告相关API (7个)
            DocxApiV1::ChatAnnouncementGet(chat_id) => {
                format!("/open-apis/docx/v1/chats/{}/announcement", chat_id)
            }
            DocxApiV1::ChatAnnouncementBlockList(chat_id) => {
                format!("/open-apis/docx/v1/chats/{}/announcement/blocks", chat_id)
            }
            DocxApiV1::ChatAnnouncementBlockChildrenCreate(chat_id, block_id) => {
                format!(
                    "/open-apis/docx/v1/chats/{}/announcement/blocks/{}/children",
                    chat_id, block_id
                )
            }
            DocxApiV1::ChatAnnouncementBlockBatchUpdate(chat_id) => {
                format!(
                    "/open-apis/docx/v1/chats/{}/announcement/blocks/batch_update",
                    chat_id
                )
            }
            DocxApiV1::ChatAnnouncementBlockGet(chat_id, block_id) => {
                format!(
                    "/open-apis/docx/v1/chats/{}/announcement/blocks/{}",
                    chat_id, block_id
                )
            }
            DocxApiV1::ChatAnnouncementBlockChildrenGet(chat_id, block_id) => {
                format!(
                    "/open-apis/docx/v1/chats/{}/announcement/blocks/{}/children",
                    chat_id, block_id
                )
            }
            DocxApiV1::ChatAnnouncementBlockChildrenBatchDelete(chat_id, block_id) => {
                format!(
                    "/open-apis/docx/v1/chats/{}/announcement/blocks/{}/children/batch_delete",
                    chat_id, block_id
                )
            }

            // 文档相关API (12个)
            DocxApiV1::DocumentCreate => "/open-apis/docx/v1/documents".to_string(),
            DocxApiV1::DocumentGet(document_id) => {
                format!("/open-apis/docx/v1/documents/{}", document_id)
            }
            DocxApiV1::DocumentRawContent(document_id) => {
                format!("/open-apis/docx/v1/documents/{}/raw_content", document_id)
            }
            DocxApiV1::DocumentBlockList(document_id) => {
                format!("/open-apis/docx/v1/documents/{}/blocks", document_id)
            }
            DocxApiV1::DocumentBlockChildrenCreate(document_id, block_id) => {
                format!(
                    "/open-apis/docx/v1/documents/{}/blocks/{}/children",
                    document_id, block_id
                )
            }
            DocxApiV1::DocumentBlockDescendantCreate(document_id, block_id) => {
                format!(
                    "/open-apis/docx/v1/documents/{}/blocks/{}/descendant",
                    document_id, block_id
                )
            }
            DocxApiV1::DocumentBlockPatch(document_id, block_id) => {
                format!(
                    "/open-apis/docx/v1/documents/{}/blocks/{}",
                    document_id, block_id
                )
            }
            DocxApiV1::DocumentBlockGet(document_id, block_id) => {
                format!(
                    "/open-apis/docx/v1/documents/{}/blocks/{}",
                    document_id, block_id
                )
            }
            DocxApiV1::DocumentBlockBatchUpdate(document_id) => {
                format!(
                    "/open-apis/docx/v1/documents/{}/blocks/batch_update",
                    document_id
                )
            }
            DocxApiV1::DocumentBlockChildrenGet(document_id, block_id) => {
                format!(
                    "/open-apis/docx/v1/documents/{}/blocks/{}/children",
                    document_id, block_id
                )
            }
            DocxApiV1::DocumentBlockChildrenBatchDelete(document_id, block_id) => {
                format!(
                    "/open-apis/docx/v1/documents/{}/blocks/{}/children/batch_delete",
                    document_id, block_id
                )
            }
            DocxApiV1::DocumentConvert => "/open-apis/docx/v1/documents/blocks/convert".to_string(),
        }
    }
}

/// Wiki API V2 端点枚举
#[derive(Debug, Clone, PartialEq)]
pub enum WikiApiV2 {
    /// 获取知识空间列表
    SpaceList,
    /// 获取知识空间信息
    SpaceGet(String),
    /// 创建知识空间
    SpaceCreate,
    /// 更新知识空间设置
    SpaceSettingUpdate(String),
    /// 获取知识空间节点信息
    SpaceGetNode,
    /// 获取知识空间子节点列表
    SpaceNodeList(String),
    /// 创建知识空间节点
    SpaceNodeCreate(String),
    /// 获取知识空间成员列表
    SpaceMemberList(String),
    /// 添加知识空间成员
    SpaceMemberCreate(String),
    /// 删除知识空间成员
    SpaceMemberDelete(String, String, String), // space_id, member_id, member_type
    /// 移动知识空间节点
    SpaceNodeMove(String, String),
    /// 更新知识空间节点标题
    SpaceNodeUpdateTitle(String, String),
    /// 创建知识空间节点副本
    SpaceNodeCopy(String, String),
    /// 移动云空间文档至知识空间
    SpaceNodeMoveDocsToWiki(String),
    /// 获取任务结果
    TaskGet(String),
}

impl WikiApiV2 {
    /// 生成对应的 URL
    pub fn to_url(&self) -> String {
        match self {
            WikiApiV2::SpaceList => "/open-apis/wiki/v2/spaces".to_string(),
            WikiApiV2::SpaceGet(space_id) => {
                format!("/open-apis/wiki/v2/spaces/{}", space_id)
            }
            WikiApiV2::SpaceCreate => "/open-apis/wiki/v2/spaces".to_string(),
            WikiApiV2::SpaceSettingUpdate(space_id) => {
                format!("/open-apis/wiki/v2/spaces/{}/setting", space_id)
            }
            WikiApiV2::SpaceGetNode => "/open-apis/wiki/v2/spaces/get_node".to_string(),
            WikiApiV2::SpaceNodeList(space_id) => {
                format!("/open-apis/wiki/v2/spaces/{}/nodes", space_id)
            }
            WikiApiV2::SpaceNodeCreate(space_id) => {
                format!("/open-apis/wiki/v2/spaces/{}/nodes", space_id)
            }
            WikiApiV2::SpaceMemberList(space_id) => {
                format!("/open-apis/wiki/v2/spaces/{}/members", space_id)
            }
            WikiApiV2::SpaceMemberCreate(space_id) => {
                format!("/open-apis/wiki/v2/spaces/{}/members", space_id)
            }
            WikiApiV2::SpaceMemberDelete(space_id, member_id, member_type) => {
                format!(
                    "/open-apis/wiki/v2/spaces/{}/members/{}?member_type={}",
                    space_id, member_id, member_type
                )
            }
            WikiApiV2::SpaceNodeMove(space_id, node_token) => {
                format!(
                    "/open-apis/wiki/v2/spaces/{}/nodes/{}/move",
                    space_id, node_token
                )
            }
            WikiApiV2::SpaceNodeUpdateTitle(space_id, node_token) => {
                format!(
                    "/open-apis/wiki/v2/spaces/{}/nodes/{}/update_title",
                    space_id, node_token
                )
            }
            WikiApiV2::SpaceNodeCopy(space_id, node_token) => {
                format!(
                    "/open-apis/wiki/v2/spaces/{}/nodes/{}/copy",
                    space_id, node_token
                )
            }
            WikiApiV2::SpaceNodeMoveDocsToWiki(space_id) => {
                format!(
                    "/open-apis/wiki/v2/spaces/{}/nodes/move_docs_to_wiki",
                    space_id
                )
            }
            WikiApiV2::TaskGet(task_id) => {
                format!("/open-apis/wiki/v2/tasks/{}", task_id)
            }
        }
    }
}

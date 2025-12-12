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

/// CCM Doc API Old V1 端点枚举
/// 对应 meta.project = ccm_doc, meta.version = old
#[derive(Debug, Clone, PartialEq)]
pub enum CcmDocApiOld {
    /// 创建旧版文档
    Create,
    /// 获取旧版文档元信息
    Meta(String), // doc_token
    /// 获取旧版文档中的电子表格元数据
    SheetMeta(String), // doc_token
    /// 获取旧版文档纯文本内容
    RawContent(String), // doc_token
    /// 获取旧版文档富文本内容
    Content(String), // doc_token
    /// 编辑旧版文档内容
    BatchUpdate(String), // doc_token
}

impl CcmDocApiOld {
    /// 生成对应的 URL
    pub fn to_url(&self) -> String {
        match self {
            CcmDocApiOld::Create => "/open-apis/doc/v2/create".to_string(),
            CcmDocApiOld::Meta(doc_token) => {
                format!("/open-apis/doc/v2/meta/{}", doc_token)
            }
            CcmDocApiOld::SheetMeta(doc_token) => {
                format!("/open-apis/doc/v2/{}/sheet_meta", doc_token)
            }
            CcmDocApiOld::RawContent(doc_token) => {
                format!("/open-apis/doc/v2/{}/raw_content", doc_token)
            }
            CcmDocApiOld::Content(doc_token) => {
                format!("/open-apis/doc/v2/{}/content", doc_token)
            }
            CcmDocApiOld::BatchUpdate(doc_token) => {
                format!("/open-apis/doc/v2/{}/batch_update", doc_token)
            }
        }
    }
}

/// CCM Docs API Old V1 端点枚举
/// 对应 meta.project = ccm_docs, meta.version = old
#[derive(Debug, Clone, PartialEq)]
pub enum CcmDocsApiOld {
    /// 搜索云文档
    SearchObject,
    /// 获取元数据
    Meta,
}

impl CcmDocsApiOld {
    /// 生成对应的 URL
    pub fn to_url(&self) -> String {
        match self {
            CcmDocsApiOld::SearchObject => "/open-apis/suite/docs-api/search/object".to_string(),
            CcmDocsApiOld::Meta => "/open-apis/suite/docs-api/meta".to_string(),
        }
    }
}

/// CCM Drive Explorer API Old V2 端点枚举
/// 对应 meta.project = ccm_drive_explorer, meta.version = old
#[derive(Debug, Clone, PartialEq)]
pub enum CcmDriveExplorerApiOld {
    /// 获取我的空间（根文件夹）元数据
    RootFolderMeta,
    /// 获取文件夹元数据
    FolderMeta(String), // folder_token
    /// 新建文件
    File(String), // folder_token
    /// 删除Sheet
    FileSpreadsheets(String), // spreadsheet_token
    /// 复制文档
    FileCopy(String), // file_token
    /// 删除Doc
    FileDocs(String), // doc_token
    /// 获取文件夹下的文档清单
    FolderChildren(String), // folder_token
    /// 新建文件夹
    Folder(String), // folder_token
}

impl CcmDriveExplorerApiOld {
    /// 生成对应的 URL
    pub fn to_url(&self) -> String {
        match self {
            CcmDriveExplorerApiOld::RootFolderMeta => "/open-apis/drive/explorer/v2/root_folder/meta".to_string(),
            CcmDriveExplorerApiOld::FolderMeta(folder_token) => {
                format!("/open-apis/drive/explorer/v2/folder/{}/meta", folder_token)
            }
            CcmDriveExplorerApiOld::File(folder_token) => {
                format!("/open-apis/drive/explorer/v2/file/{}", folder_token)
            }
            CcmDriveExplorerApiOld::FileSpreadsheets(spreadsheet_token) => {
                format!("/open-apis/drive/explorer/v2/file/spreadsheets/{}", spreadsheet_token)
            }
            CcmDriveExplorerApiOld::FileCopy(file_token) => {
                format!("/open-apis/drive/explorer/v2/file/copy/files/{}", file_token)
            }
            CcmDriveExplorerApiOld::FileDocs(doc_token) => {
                format!("/open-apis/drive/explorer/v2/file/docs/{}", doc_token)
            }
            CcmDriveExplorerApiOld::FolderChildren(folder_token) => {
                format!("/open-apis/drive/explorer/v2/folder/{}/children", folder_token)
            }
            CcmDriveExplorerApiOld::Folder(folder_token) => {
                format!("/open-apis/drive/explorer/v2/folder/{}", folder_token)
            }
        }
    }
}

/// CCM Drive Permission API Old V2 端点枚举
/// 对应 meta.project = ccm_drive_permission, meta.version = old
#[derive(Debug, Clone, PartialEq)]
pub enum CcmDrivePermissionApiOld {
    /// 判断协作者是否有某权限
    MemberPermitted,
    /// 转移拥有者
    MemberTransfer,
    /// 获取云文档权限设置V2
    Public,
}

impl CcmDrivePermissionApiOld {
    /// 生成对应的 URL
    pub fn to_url(&self) -> String {
        match self {
            CcmDrivePermissionApiOld::MemberPermitted => "/open-apis/drive/permission/member/permitted".to_string(),
            CcmDrivePermissionApiOld::MemberTransfer => "/open-apis/drive/permission/member/transfer".to_string(),
            CcmDrivePermissionApiOld::Public => "/open-apis/drive/permission/v2/public/".to_string(),
        }
    }
}

/// CCM Sheet API Old V2 端点枚举
/// 对应 meta.project = ccm_sheet, meta.version = old
#[derive(Debug, Clone, PartialEq)]
pub enum CcmSheetApiOld {
    /// 操作工作表 (第一个)
    OperateSheets(String), // spreadsheet_token
    /// 更新工作表属性 (第二个)
    UpdateSheetProperties(String), // spreadsheet_token
    /// 增加行列
    DimensionRange(String), // spreadsheet_token
    /// 插入行列
    InsertDimensionRange(String), // spreadsheet_token
    /// 更新行列
    DimensionRangeUpdate(String), // spreadsheet_token
    /// 删除行列
    DimensionRangeDelete(String), // spreadsheet_token
    /// 合并单元格
    MergeCells(String), // spreadsheet_token
    /// 拆分单元格
    UnmergeCells(String), // spreadsheet_token
    /// 设置单元格样式
    Style(String), // spreadsheet_token
    /// 批量设置单元格样式
    StylesBatchUpdate(String), // spreadsheet_token
    /// 插入数据
    ValuesPrepend(String), // spreadsheet_token
    /// 追加数据
    ValuesAppend(String), // spreadsheet_token
    /// 写入图片
    ValuesImage(String), // spreadsheet_token
    /// 读取单个范围
    ValuesRange(String, String), // spreadsheet_token, range
    /// 读取多个范围
    ValuesBatchGet(String), // spreadsheet_token
    /// 向单个范围写入数据
    Values(String), // spreadsheet_token
    /// 向多个范围写入数据
    ValuesBatchUpdate(String), // spreadsheet_token
    /// 增加保护范围
    ProtectedDimension(String), // spreadsheet_token
    /// 修改保护范围
    ProtectedRangeBatchUpdate(String), // spreadsheet_token
    /// 获取保护范围
    ProtectedRangeBatchGet(String), // spreadsheet_token
    /// 删除保护范围
    ProtectedRangeBatchDel(String), // spreadsheet_token
    /// 获取表格元数据
    Metainfo(String), // spreadsheet_token
    /// 更新表格属性
    Properties(String), // spreadsheet_token
    /// 导入表格
    Import,
    /// 查询导入结果
    ImportResult,
    /// 获取条件格式
    ConditionFormats(String), // spreadsheet_token
    /// 批量创建条件格式
    ConditionFormatsBatchCreate(String), // spreadsheet_token
    /// 批量删除条件格式
    ConditionFormatsBatchDelete(String), // spreadsheet_token
    /// 批量更新条件格式
    ConditionFormatsBatchUpdate(String), // spreadsheet_token
    /// 获取数据验证规则
    DataValidation(String), // spreadsheet_token
    /// 创建数据验证规则
    DataValidationCreate(String), // spreadsheet_token
    /// 更新数据验证规则
    DataValidationUpdate(String, String), // spreadsheet_token, validation_id
    /// 删除数据验证规则
    DataValidationDelete(String, String), // spreadsheet_token, validation_id
    /// 读取单个范围 (V3)
    ReadSingleRange(String), // spreadsheet_token
    /// 读取多个范围 (V3)
    ReadMultipleRanges(String), // spreadsheet_token
    /// 写入单个范围 (V3)
    WriteSingleRange(String), // spreadsheet_token
    /// 批量写入范围 (V3)
    BatchWriteRanges(String), // spreadsheet_token
    /// 追加数据 (V3)
    AppendValues(String), // spreadsheet_token
    /// 插入数据 (V3)
    InsertValues(String), // spreadsheet_token
    /// 获取表格信息 (V3)
    GetSpreadsheet(String), // spreadsheet_token
    /// 创建表格 (V3)
    CreateSpreadsheet,
    /// 更新表格 (V3)
    UpdateSpreadsheet(String), // spreadsheet_token
    /// 添加工作表 (V3)
    AddSheet(String), // spreadsheet_token
    /// 获取工作表信息 (V3)
    GetSheet(String), // spreadsheet_token
    /// 更新工作表 (V3)
    UpdateSheet(String), // spreadsheet_token
    /// 删除工作表 (V3)
    DeleteSheet(String), // spreadsheet_token
    /// 创建筛选 (V3)
    CreateFilter(String), // spreadsheet_token
    /// 获取筛选 (V3)
    GetFilter(String), // spreadsheet_token
    /// 更新筛选 (V3)
    UpdateFilter(String), // spreadsheet_token
    /// 删除筛选 (V3)
    DeleteFilter(String), // spreadsheet_token
    /// 创建浮图 (V3)
    CreateFloatImage(String), // spreadsheet_token
    /// 获取浮图 (V3)
    GetFloatImage(String), // spreadsheet_token
    /// 更新浮图 (V3)
    UpdateFloatImage(String), // spreadsheet_token
    /// 删除浮图 (V3)
    DeleteFloatImage(String), // spreadsheet_token
    /// 删除范围 (V3)
    DeleteRange(String), // spreadsheet_token
    /// 插入维度 (V3)
    InsertDimension(String), // spreadsheet_token
    /// 移动维度 (V3)
    MoveDimension(String), // spreadsheet_token
    /// 替换范围 (V3)
    ReplaceRange(String), // spreadsheet_token
    /// 查找替换 (V3)
    FindReplace(String), // spreadsheet_token
}

impl CcmSheetApiOld {
    /// 生成对应的 URL
    pub fn to_url(&self) -> String {
        match self {
            CcmSheetApiOld::OperateSheets(spreadsheet_token) => {
                format!("/open-apis/sheets/v2/spreadsheets/{}/sheets_batch_update", spreadsheet_token)
            }
            CcmSheetApiOld::UpdateSheetProperties(spreadsheet_token) => {
                format!("/open-apis/sheets/v2/spreadsheets/{}/sheets_batch_update", spreadsheet_token)
            }
            CcmSheetApiOld::Style(spreadsheet_token) => {
                format!("/open-apis/sheets/v2/spreadsheets/{}/style", spreadsheet_token)
            }
            CcmSheetApiOld::StylesBatchUpdate(spreadsheet_token) => {
                format!("/open-apis/sheets/v2/spreadsheets/{}/styles_batch_update", spreadsheet_token)
            }
            CcmSheetApiOld::ValuesPrepend(spreadsheet_token) => {
                format!("/open-apis/sheets/v2/spreadsheets/{}/values_prepend", spreadsheet_token)
            }
            CcmSheetApiOld::ValuesAppend(spreadsheet_token) => {
                format!("/open-apis/sheets/v2/spreadsheets/{}/values_append", spreadsheet_token)
            }
            CcmSheetApiOld::ValuesImage(spreadsheet_token) => {
                format!("/open-apis/sheets/v2/spreadsheets/{}/values_image", spreadsheet_token)
            }
            CcmSheetApiOld::ValuesRange(spreadsheet_token, range) => {
                format!("/open-apis/sheets/v2/spreadsheets/{}/values/{}", spreadsheet_token, range)
            }
            CcmSheetApiOld::ValuesBatchGet(spreadsheet_token) => {
                format!("/open-apis/sheets/v2/spreadsheets/{}/values_batch_get", spreadsheet_token)
            }
            CcmSheetApiOld::Values(spreadsheet_token) => {
                format!("/open-apis/sheets/v2/spreadsheets/{}/values", spreadsheet_token)
            }
            CcmSheetApiOld::ValuesBatchUpdate(spreadsheet_token) => {
                format!("/open-apis/sheets/v2/spreadsheets/{}/values_batch_update", spreadsheet_token)
            }
            CcmSheetApiOld::DimensionRange(spreadsheet_token) => {
                format!("/open-apis/sheets/v2/spreadsheets/{}/dimension_range", spreadsheet_token)
            }
            CcmSheetApiOld::InsertDimensionRange(spreadsheet_token) => {
                format!("/open-apis/sheets/v2/spreadsheets/{}/insert_dimension_range", spreadsheet_token)
            }
            CcmSheetApiOld::DimensionRangeUpdate(spreadsheet_token) => {
                format!("/open-apis/sheets/v2/spreadsheets/{}/dimension_range", spreadsheet_token)
            }
            CcmSheetApiOld::DimensionRangeDelete(spreadsheet_token) => {
                format!("/open-apis/sheets/v2/spreadsheets/{}/dimension_range", spreadsheet_token)
            }
            CcmSheetApiOld::MergeCells(spreadsheet_token) => {
                format!("/open-apis/sheets/v2/spreadsheets/{}/merge_cells", spreadsheet_token)
            }
            CcmSheetApiOld::UnmergeCells(spreadsheet_token) => {
                format!("/open-apis/sheets/v2/spreadsheets/{}/unmerge_cells", spreadsheet_token)
            }
            CcmSheetApiOld::ProtectedDimension(spreadsheet_token) => {
                format!("/open-apis/sheets/v2/spreadsheets/{}/protected_dimension", spreadsheet_token)
            }
            CcmSheetApiOld::ProtectedRangeBatchUpdate(spreadsheet_token) => {
                format!("/open-apis/sheets/v2/spreadsheets/{}/protected_range_batch_update", spreadsheet_token)
            }
            CcmSheetApiOld::ProtectedRangeBatchGet(spreadsheet_token) => {
                format!("/open-apis/sheets/v2/spreadsheets/{}/protected_range_batch_get", spreadsheet_token)
            }
            CcmSheetApiOld::ProtectedRangeBatchDel(spreadsheet_token) => {
                format!("/open-apis/sheets/v2/spreadsheets/{}/protected_range_batch_del", spreadsheet_token)
            }
            CcmSheetApiOld::Metainfo(spreadsheet_token) => {
                format!("/open-apis/sheets/v2/spreadsheets/{}/metainfo", spreadsheet_token)
            }
            CcmSheetApiOld::Properties(spreadsheet_token) => {
                format!("/open-apis/sheets/v2/spreadsheets/{}/properties", spreadsheet_token)
            }
            CcmSheetApiOld::Import => "/open-apis/sheets/v2/import".to_string(),
            CcmSheetApiOld::ImportResult => "/open-apis/sheets/v2/import/result".to_string(),
            CcmSheetApiOld::ConditionFormats(spreadsheet_token) => {
                format!("/open-apis/sheets/v2/spreadsheets/{}/condition_formats", spreadsheet_token)
            }
            CcmSheetApiOld::ConditionFormatsBatchCreate(spreadsheet_token) => {
                format!("/open-apis/sheets/v2/spreadsheets/{}/condition_formats/batch_create", spreadsheet_token)
            }
            CcmSheetApiOld::ConditionFormatsBatchDelete(spreadsheet_token) => {
                format!("/open-apis/sheets/v2/spreadsheets/{}/condition_formats/batch_delete", spreadsheet_token)
            }
            CcmSheetApiOld::ConditionFormatsBatchUpdate(spreadsheet_token) => {
                format!("/open-apis/sheets/v2/spreadsheets/{}/condition_formats/batch_update", spreadsheet_token)
            }
            CcmSheetApiOld::DataValidation(spreadsheet_token) => {
                format!("/open-apis/sheets/v2/spreadsheets/{}/dataValidation", spreadsheet_token)
            }
            CcmSheetApiOld::DataValidationCreate(spreadsheet_token) => {
                format!("/open-apis/sheets/v2/spreadsheets/{}/dataValidation", spreadsheet_token)
            }
            CcmSheetApiOld::DataValidationUpdate(spreadsheet_token, validation_id) => {
                format!("/open-apis/sheets/v2/spreadsheets/{}/dataValidation/{}", spreadsheet_token, validation_id)
            }
            CcmSheetApiOld::DataValidationDelete(spreadsheet_token, validation_id) => {
                format!("/open-apis/sheets/v2/spreadsheets/{}/dataValidation/{}", spreadsheet_token, validation_id)
            }
            // V3 APIs
            CcmSheetApiOld::ReadSingleRange(spreadsheet_token) => {
                format!("/open-apis/sheets/v3/spreadsheets/{}/values", spreadsheet_token)
            }
            CcmSheetApiOld::ReadMultipleRanges(spreadsheet_token) => {
                format!("/open-apis/sheets/v3/spreadsheets/{}/values/batchGet", spreadsheet_token)
            }
            CcmSheetApiOld::WriteSingleRange(spreadsheet_token) => {
                format!("/open-apis/sheets/v3/spreadsheets/{}/values", spreadsheet_token)
            }
            CcmSheetApiOld::BatchWriteRanges(spreadsheet_token) => {
                format!("/open-apis/sheets/v3/spreadsheets/{}/values/batchUpdate", spreadsheet_token)
            }
            CcmSheetApiOld::AppendValues(spreadsheet_token) => {
                format!("/open-apis/sheets/v3/spreadsheets/{}/values/append", spreadsheet_token)
            }
            CcmSheetApiOld::InsertValues(spreadsheet_token) => {
                format!("/open-apis/sheets/v3/spreadsheets/{}/values/insert", spreadsheet_token)
            }
            CcmSheetApiOld::GetSpreadsheet(spreadsheet_token) => {
                format!("/open-apis/sheets/v3/spreadsheets/{}", spreadsheet_token)
            }
            CcmSheetApiOld::CreateSpreadsheet => "/open-apis/sheets/v3/spreadsheets".to_string(),
            CcmSheetApiOld::UpdateSpreadsheet(spreadsheet_token) => {
                format!("/open-apis/sheets/v3/spreadsheets/{}", spreadsheet_token)
            }
            CcmSheetApiOld::AddSheet(spreadsheet_token) => {
                format!("/open-apis/sheets/v3/spreadsheets/{}/sheets", spreadsheet_token)
            }
            CcmSheetApiOld::GetSheet(spreadsheet_token) => {
                format!("/open-apis/sheets/v3/spreadsheets/{}/sheets/query", spreadsheet_token)
            }
            CcmSheetApiOld::UpdateSheet(spreadsheet_token) => {
                format!("/open-apis/sheets/v3/spreadsheets/{}/sheets", spreadsheet_token)
            }
            CcmSheetApiOld::DeleteSheet(spreadsheet_token) => {
                format!("/open-apis/sheets/v3/spreadsheets/{}/sheets", spreadsheet_token)
            }
            CcmSheetApiOld::CreateFilter(spreadsheet_token) => {
                format!("/open-apis/sheets/v3/spreadsheets/{}/filterViews", spreadsheet_token)
            }
            CcmSheetApiOld::GetFilter(spreadsheet_token) => {
                format!("/open-apis/sheets/v3/spreadsheets/{}/filterViews/query", spreadsheet_token)
            }
            CcmSheetApiOld::UpdateFilter(spreadsheet_token) => {
                format!("/open-apis/sheets/v3/spreadsheets/{}/filterViews", spreadsheet_token)
            }
            CcmSheetApiOld::DeleteFilter(spreadsheet_token) => {
                format!("/open-apis/sheets/v3/spreadsheets/{}/filterViews", spreadsheet_token)
            }
            CcmSheetApiOld::CreateFloatImage(spreadsheet_token) => {
                format!("/open-apis/sheets/v3/spreadsheets/{}/floatImages", spreadsheet_token)
            }
            CcmSheetApiOld::GetFloatImage(spreadsheet_token) => {
                format!("/open-apis/sheets/v3/spreadsheets/{}/floatImages/query", spreadsheet_token)
            }
            CcmSheetApiOld::UpdateFloatImage(spreadsheet_token) => {
                format!("/open-apis/sheets/v3/spreadsheets/{}/floatImages", spreadsheet_token)
            }
            CcmSheetApiOld::DeleteFloatImage(spreadsheet_token) => {
                format!("/open-apis/sheets/v3/spreadsheets/{}/floatImages", spreadsheet_token)
            }
            CcmSheetApiOld::DeleteRange(spreadsheet_token) => {
                format!("/open-apis/sheets/v3/spreadsheets/{}/dimensionRange/delete", spreadsheet_token)
            }
            CcmSheetApiOld::InsertDimension(spreadsheet_token) => {
                format!("/open-apis/sheets/v3/spreadsheets/{}/dimensionRange/insert", spreadsheet_token)
            }
            CcmSheetApiOld::MoveDimension(spreadsheet_token) => {
                format!("/open-apis/sheets/v3/spreadsheets/{}/dimensionRange/move", spreadsheet_token)
            }
            CcmSheetApiOld::ReplaceRange(spreadsheet_token) => {
                format!("/open-apis/sheets/v3/spreadsheets/{}/values/batchReplace", spreadsheet_token)
            }
            CcmSheetApiOld::FindReplace(spreadsheet_token) => {
                format!("/open-apis/sheets/v3/spreadsheets/{}/values/batchFindReplace", spreadsheet_token)
            }
        }
    }
}

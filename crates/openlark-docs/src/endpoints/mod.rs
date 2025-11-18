//! OpenLark Docs 服务端点定义
//!
//! 此模块包含文档和云盘相关的所有API端点常量，从 openlark-core 迁移而来。
//! 包含云文档、表格、知识库、白板、评论、云盘、卡片和报告等完整功能。
//!
//! # 服务模块包含
//!
//! - **cloud_docs**: 云文档服务（Bitable多维表格、Sheets电子表格、Docx文档、Wiki知识库、Board白板）
//! - **drive**: 云盘服务（文件管理、文件夹管理、媒体处理、权限管理）
//! - **cardkit**: 交互式卡片组件
//! - **report**: 报告和分析服务
//! - **comment**: 评论系统
//! - **assistant**: AI助手服务
//!
//! # 使用示例
//!
//! ```rust
//! use openlark_docs::endpoints::*;
//!
//! // Bitable多维表格
//! let app_endpoint = BITABLE_V1_APPS;
//! let records_endpoint = BITABLE_V1_RECORDS;
//!
//! // Sheets电子表格
//! let sheets_endpoint = SHEETS_V3_SPREADSHEETS;
//! let values_endpoint = SHEETS_V3_SPREADSHEET_VALUES_GET;
//!
//! // Wiki知识库
//! let wiki_space_endpoint = WIKI_V2_SPACES;
//! let wiki_node_endpoint = WIKI_V2_SPACE_NODES;
//!
//! // 云盘文件
//! let drive_files_endpoint = DRIVE_V1_FILES;
//! let upload_endpoint = DRIVE_V1_FILES_UPLOAD_ALL;
//!
//! // 卡片组件
//! let cards_endpoint = CARDKIT_V1_CARDS;
//! ```

// 导入核心端点（auth, application等基础端点）
pub use openlark_core::endpoints::{apass, application, auth, platform_integration};

// ==================== Bitable (多维表格) v1 ====================

/// Bitable应用管理
/// 创建、查询、更新、复制多维表格应用
pub const BITABLE_V1_APPS: &str = "/open-apis/bitable/v1/apps";
pub const BITABLE_V1_APP_GET: &str = "/open-apis/bitable/v1/apps/{app_token}";
pub const BITABLE_V1_APP_CREATE: &str = "/open-apis/bitable/v1/apps";
pub const BITABLE_V1_APP_UPDATE: &str = "/open-apis/bitable/v1/apps/{app_token}";
pub const BITABLE_V1_APP_COPY: &str = "/open-apis/bitable/v1/apps/{app_token}/copy";

/// Bitable表格管理
/// 管理应用内的数据表格
pub const BITABLE_V1_TABLES: &str = "/open-apis/bitable/v1/apps/{app_token}/tables";
pub const BITABLE_V1_TABLE_GET: &str = "/open-apis/bitable/v1/apps/{app_token}/tables/{table_id}";
pub const BITABLE_V1_TABLE_CREATE: &str = "/open-apis/bitable/v1/apps/{app_token}/tables";
pub const BITABLE_V1_TABLE_PATCH: &str = "/open-apis/bitable/v1/apps/{app_token}/tables/{table_id}";
pub const BITABLE_V1_TABLE_DELETE: &str =
    "/open-apis/bitable/v1/apps/{app_token}/tables/{table_id}";
pub const BITABLE_V1_TABLES_BATCH_CREATE: &str =
    "/open-apis/bitable/v1/apps/{app_token}/tables/batch_create";
pub const BITABLE_V1_TABLES_BATCH_DELETE: &str =
    "/open-apis/bitable/v1/apps/{app_token}/tables/batch_delete";

/// Bitable记录管理
/// 表格数据行的增删改查操作
pub const BITABLE_V1_RECORDS: &str =
    "/open-apis/bitable/v1/apps/{app_token}/tables/{table_id}/records";
pub const BITABLE_V1_RECORD_GET: &str =
    "/open-apis/bitable/v1/apps/{app_token}/tables/{table_id}/records/{record_id}";
pub const BITABLE_V1_RECORD_CREATE: &str =
    "/open-apis/bitable/v1/apps/{app_token}/tables/{table_id}/records";
pub const BITABLE_V1_RECORD_UPDATE: &str =
    "/open-apis/bitable/v1/apps/{app_token}/tables/{table_id}/records/{record_id}";
pub const BITABLE_V1_RECORD_DELETE: &str =
    "/open-apis/bitable/v1/apps/{app_token}/tables/{table_id}/records/{record_id}";
pub const BITABLE_V1_RECORDS_BATCH_CREATE: &str =
    "/open-apis/bitable/v1/apps/{app_token}/tables/{table_id}/records/batch_create";
pub const BITABLE_V1_RECORDS_BATCH_UPDATE: &str =
    "/open-apis/bitable/v1/apps/{app_token}/tables/{table_id}/records/batch_update";
pub const BITABLE_V1_RECORDS_BATCH_GET: &str =
    "/open-apis/bitable/v1/apps/{app_token}/tables/{table_id}/records/batch_get";
pub const BITABLE_V1_RECORDS_BATCH_DELETE: &str =
    "/open-apis/bitable/v1/apps/{app_token}/tables/{table_id}/records/batch_delete";
pub const BITABLE_V1_RECORDS_SEARCH: &str =
    "/open-apis/bitable/v1/apps/{app_token}/tables/{table_id}/records/search";

/// Bitable字段管理
/// 表格列定义和配置
pub const BITABLE_V1_FIELDS: &str =
    "/open-apis/bitable/v1/apps/{app_token}/tables/{table_id}/fields";
pub const BITABLE_V1_FIELD_UPDATE: &str =
    "/open-apis/bitable/v1/apps/{app_token}/tables/{table_id}/fields/{field_id}";
pub const BITABLE_V1_FIELD_DELETE: &str =
    "/open-apis/bitable/v1/apps/{app_token}/tables/{table_id}/fields/{field_id}";

/// Bitable视图管理
/// 数据展示视图和过滤器
pub const BITABLE_V1_VIEWS: &str = "/open-apis/bitable/v1/apps/{app_token}/tables/{table_id}/views";
pub const BITABLE_V1_VIEW_GET: &str =
    "/open-apis/bitable/v1/apps/{app_token}/tables/{table_id}/views/{view_id}";
pub const BITABLE_V1_VIEW_CREATE: &str =
    "/open-apis/bitable/v1/apps/{app_token}/tables/{table_id}/views";
pub const BITABLE_V1_VIEW_PATCH: &str =
    "/open-apis/bitable/v1/apps/{app_token}/tables/{table_id}/views/{view_id}";
pub const BITABLE_V1_VIEW_DELETE: &str =
    "/open-apis/bitable/v1/apps/{app_token}/tables/{table_id}/views/{view_id}";

/// Bitable仪表盘管理
/// 数据可视化仪表盘
pub const BITABLE_V1_DASHBOARDS: &str = "/open-apis/bitable/v1/apps/{app_token}/dashboards";
pub const BITABLE_V1_DASHBOARD_COPY: &str =
    "/open-apis/bitable/v1/apps/{app_token}/dashboards/{dashboard_id}/copy";

/// Bitable角色和权限管理
/// 协作权限控制
pub const BITABLE_V1_ROLES: &str = "/open-apis/bitable/v1/apps/{app_token}/roles";
pub const BITABLE_V1_ROLE_UPDATE: &str = "/open-apis/bitable/v1/apps/{app_token}/roles/{role_id}";
pub const BITABLE_V1_ROLE_DELETE: &str = "/open-apis/bitable/v1/apps/{app_token}/roles/{role_id}";
pub const BITABLE_V1_ROLE_MEMBERS: &str =
    "/open-apis/bitable/v1/apps/{app_token}/roles/{role_id}/members";
pub const BITABLE_V1_ROLE_MEMBER_DELETE: &str =
    "/open-apis/bitable/v1/apps/{app_token}/roles/{role_id}/members/{member_id}";
pub const BITABLE_V1_ROLE_MEMBERS_BATCH_CREATE: &str =
    "/open-apis/bitable/v1/apps/{app_token}/roles/{role_id}/members/batch_create";
pub const BITABLE_V1_ROLE_MEMBERS_BATCH_DELETE: &str =
    "/open-apis/bitable/v1/apps/{app_token}/roles/{role_id}/members/batch_delete";

/// Bitable表单管理
/// 数据收集表单
pub const BITABLE_V1_FORM_GET: &str =
    "/open-apis/bitable/v1/apps/{app_token}/tables/{table_id}/forms/{form_id}";
pub const BITABLE_V1_FORM_PATCH: &str =
    "/open-apis/bitable/v1/apps/{app_token}/tables/{table_id}/forms/{form_id}";
pub const BITABLE_V1_FORM_PATCH_META: &str =
    "/open-apis/bitable/v1/apps/{app_token}/tables/{table_id}/forms/{form_id}/patch_meta";
pub const BITABLE_V1_FORM_QUESTION: &str =
    "/open-apis/bitable/v1/apps/{app_token}/tables/{table_id}/forms/{form_id}/questions";

/// Bitable工作流管理
/// 自动化工作流程
pub const BITABLE_V1_WORKFLOWS: &str = "/open-apis/bitable/v1/apps/{app_token}/workflows";
pub const BITABLE_V1_WORKFLOW_UPDATE: &str =
    "/open-apis/bitable/v1/apps/{app_token}/workflows/{workflow_id}";

// ==================== Sheets (电子表格) v2/v3 ====================

/// Sheets v2 - 电子表格基础操作
/// 兼容版本的电子表格API
pub const SHEETS_V2_SPREADSHEET_VALUES: &str =
    "/open-apis/sheets/v2/spreadsheets/{spreadsheet_token}/values/{range}";
pub const SHEETS_V2_SPREADSHEET_VALUES_RANGE: &str =
    "/open-apis/sheets/v2/spreadsheets/{spreadsheet_token}/values/{range}";
pub const SHEETS_V2_SPREADSHEET_VALUES_APPEND: &str =
    "/open-apis/sheets/v2/spreadsheets/{spreadsheet_token}/values_append";
pub const SHEETS_V2_SPREADSHEET_VALUES_PREPEND: &str =
    "/open-apis/sheets/v2/spreadsheets/{spreadsheet_token}/values_prepend";
pub const SHEETS_V2_SPREADSHEET_VALUES_BATCH_GET: &str =
    "/open-apis/sheets/v2/spreadsheets/{spreadsheet_token}/values_batch_get";
pub const SHEETS_V2_SPREADSHEET_VALUES_BATCH_UPDATE: &str =
    "/open-apis/sheets/v2/spreadsheets/{spreadsheet_token}/values_batch_update";
pub const SHEETS_V2_SPREADSHEET_VALUES_IMAGE: &str =
    "/open-apis/sheets/v2/spreadsheets/{spreadsheet_token}/values_image";

/// Sheets v2 - 样式和格式
pub const SHEETS_V2_SPREADSHEET_STYLE: &str =
    "/open-apis/sheets/v2/spreadsheets/{spreadsheet_token}/style";
pub const SHEETS_V2_SPREADSHEET_STYLES_BATCH_UPDATE: &str =
    "/open-apis/sheets/v2/spreadsheets/{spreadsheet_token}/styles_batch_update";
pub const SHEETS_V2_SPREADSHEET_MERGE_CELLS: &str =
    "/open-apis/sheets/v2/spreadsheets/{spreadsheet_token}/merge_cells";
pub const SHEETS_V2_SPREADSHEET_UNMERGE_CELLS: &str =
    "/open-apis/sheets/v2/spreadsheets/{spreadsheet_token}/unmerge_cells";

/// Sheets v2 - 维度和工作表
pub const SHEETS_V2_SPREADSHEET_DIMENSION_RANGE: &str =
    "/open-apis/sheets/v2/spreadsheets/{spreadsheet_token}/dimension_range";
pub const SHEETS_V2_SPREADSHEET_INSERT_DIMENSION_RANGE: &str =
    "/open-apis/sheets/v2/spreadsheets/{spreadsheet_token}/insert_dimension_range";
pub const SHEETS_V2_SPREADSHEET_SHEETS_BATCH_UPDATE: &str =
    "/open-apis/sheets/v2/spreadsheets/{spreadsheet_token}/sheets_batch_update";

/// Sheets v3 - 电子表格管理
/// 最新版本的电子表格API
pub const SHEETS_V3_SPREADSHEETS: &str = "/open-apis/sheets/v3/spreadsheets";
pub const SHEETS_V3_SPREADSHEET_GET: &str = "/open-apis/sheets/v3/spreadsheets/{spreadsheet_token}";

/// Sheets v3 - 数据操作
pub const SHEETS_V3_SPREADSHEET_VALUES_GET: &str =
    "/open-apis/sheets/v3/spreadsheets/{spreadsheet_token}/sheets/{sheet_id}/values/{range}";
pub const SHEETS_V3_SPREADSHEET_VALUES_APPEND: &str =
    "/open-apis/sheets/v3/spreadsheets/{spreadsheet_token}/sheets/{sheet_id}/values_append";
pub const SHEETS_V3_SPREADSHEET_VALUES_PREPEND: &str =
    "/open-apis/sheets/v3/spreadsheets/{spreadsheet_token}/sheets/{sheet_id}/values_prepend";
pub const SHEETS_V3_SPREADSHEET_VALUES_BATCH_GET: &str =
    "/open-apis/sheets/v3/spreadsheets/{spreadsheet_token}/values_batch_get";
pub const SHEETS_V3_SPREADSHEET_VALUES_BATCH_UPDATE: &str =
    "/open-apis/sheets/v3/spreadsheets/{spreadsheet_token}/values_batch_update";
pub const SHEETS_V3_SPREADSHEET_VALUES_IMAGE: &str =
    "/open-apis/sheets/v3/spreadsheets/{spreadsheet_token}/sheets/{sheet_id}/values_image";

/// Sheets v3 - 样式和格式
pub const SHEETS_V3_SPREADSHEET_STYLE: &str =
    "/open-apis/sheets/v3/spreadsheets/{spreadsheet_token}/sheets/{sheet_id}/style";
pub const SHEETS_V3_SPREADSHEET_STYLES_BATCH_UPDATE: &str =
    "/open-apis/sheets/v3/spreadsheets/{spreadsheet_token}/styles_batch_update";
pub const SHEETS_V3_SPREADSHEET_MERGE_CELLS: &str =
    "/open-apis/sheets/v3/spreadsheets/{spreadsheet_token}/sheets/{sheet_id}/merge_cells";
pub const SHEETS_V3_SPREADSHEET_UNMERGE_CELLS: &str =
    "/open-apis/sheets/v3/spreadsheets/{spreadsheet_token}/sheets/{sheet_id}/unmerge_cells";

/// Sheets v3 - 维度和工作表管理
pub const SHEETS_V3_SPREADSHEET_DIMENSION_RANGE: &str =
    "/open-apis/sheets/v3/spreadsheets/{spreadsheet_token}/sheets/{sheet_id}/dimension_range";
pub const SHEETS_V3_SPREADSHEET_DIMENSION_RANGE_INSERT: &str = "/open-apis/sheets/v3/spreadsheets/{spreadsheet_token}/sheets/{sheet_id}/insert_dimension_range";
pub const SHEETS_V3_SPREADSHEET_MOVE_DIMENSION: &str =
    "/open-apis/sheets/v3/spreadsheets/{spreadsheet_token}/sheets/{sheet_id}/move_dimension";
pub const SHEETS_V3_SPREADSHEET_SHEETS_QUERY: &str =
    "/open-apis/sheets/v3/spreadsheets/{spreadsheet_token}/sheets/query";

/// Sheets v3 - 工作表操作
pub const SHEETS_V3_SPREADSHEET_SHEET_GET: &str =
    "/open-apis/sheets/v3/spreadsheets/{spreadsheet_token}/sheets/{sheet_id}";
pub const SHEETS_V3_SPREADSHEET_SHEET_FIND: &str =
    "/open-apis/sheets/v3/spreadsheets/{spreadsheet_token}/sheets/{sheet_id}/find";
pub const SHEETS_V3_SPREADSHEET_SHEET_REPLACE: &str =
    "/open-apis/sheets/v3/spreadsheets/{spreadsheet_token}/sheets/{sheet_id}/replace";

/// Sheets v3 - 条件格式和数据验证
pub const SHEETS_V3_SPREADSHEET_CONDITION_FORMAT: &str =
    "/open-apis/sheets/v3/spreadsheets/{spreadsheet_token}/sheets/{sheet_id}/condition_format";
pub const SHEETS_V3_SPREADSHEET_DATA_VALIDATION: &str =
    "/open-apis/sheets/v3/spreadsheets/{spreadsheet_token}/sheets/{sheet_id}/data_validation";
pub const SHEETS_V3_SPREADSHEET_DATA_VALIDATION_GET: &str = "/open-apis/sheets/v3/spreadsheets/{spreadsheet_token}/sheets/{sheet_id}/data_validation/{data_validation_id}";

/// Sheets v3 - 筛选器和过滤视图
pub const SHEETS_V3_SPREADSHEET_FILTER: &str =
    "/open-apis/sheets/v3/spreadsheets/{spreadsheet_token}/sheets/{sheet_id}/filter";
pub const SHEETS_V3_SPREADSHEET_FILTER_VIEWS: &str =
    "/open-apis/sheets/v3/spreadsheets/{spreadsheet_token}/sheets/{sheet_id}/filter_views";
pub const SHEETS_V3_SPREADSHEET_FILTER_VIEW_GET: &str = "/open-apis/sheets/v3/spreadsheets/{spreadsheet_token}/sheets/{sheet_id}/filter_views/{filter_view_id}";
pub const SHEETS_V3_SPREADSHEET_FILTER_VIEW_CONDITIONS: &str = "/open-apis/sheets/v3/spreadsheets/{spreadsheet_token}/sheets/{sheet_id}/filter_views/{filter_view_id}/conditions";
pub const SHEETS_V3_SPREADSHEET_FILTER_VIEW_CONDITION_GET: &str = "/open-apis/sheets/v3/spreadsheets/{spreadsheet_token}/sheets/{sheet_id}/filter_views/{filter_view_id}/conditions/{condition_id}";

/// Sheets v3 - 保护范围和权限
pub const SHEETS_V3_SPREADSHEET_PROTECT_RANGE: &str =
    "/open-apis/sheets/v3/spreadsheets/{spreadsheet_token}/sheets/{sheet_id}/protect_range";
pub const SHEETS_V3_SPREADSHEET_PROTECT_RANGE_GET: &str = "/open-apis/sheets/v3/spreadsheets/{spreadsheet_token}/sheets/{sheet_id}/protect_range/{protect_id}";

/// Sheets v3 - 浮动图片和对象
pub const SHEETS_V3_SPREADSHEET_FLOAT_IMAGES: &str =
    "/open-apis/sheets/v3/spreadsheets/{spreadsheet_token}/sheets/{sheet_id}/float_images";
pub const SHEETS_V3_SPREADSHEET_FLOAT_IMAGE_GET: &str = "/open-apis/sheets/v3/spreadsheets/{spreadsheet_token}/sheets/{sheet_id}/float_images/{float_image_id}";

// ==================== Docx (文档) v1 ====================

/// Docx文档管理
/// 文本文档的创建、查询、编辑和转换
pub const DOCX_V1_DOCUMENTS: &str = "/open-apis/docx/v1/documents";
pub const DOCX_V1_DOCUMENT_GET: &str = "/open-apis/docx/v1/documents/{document_id}";
pub const DOCX_V1_DOCUMENT_RAW_CONTENT: &str =
    "/open-apis/docx/v1/documents/{document_id}/raw_content";
pub const DOCX_V1_DOCUMENT_CONVERT: &str = "/open-apis/docx/v1/documents/{document_id}/convert";

/// Docx文档块管理
/// 文档内容块（段落、标题、列表等）的操作
pub const DOCX_V1_DOCUMENT_BLOCKS: &str = "/open-apis/docx/v1/documents/{document_id}/blocks";
pub const DOCX_V1_DOCUMENT_BLOCK_GET: &str =
    "/open-apis/docx/v1/documents/{document_id}/blocks/{block_id}";
pub const DOCX_V1_DOCUMENT_BLOCKS_BATCH_UPDATE: &str =
    "/open-apis/docx/v1/documents/{document_id}/blocks/batch_update";
pub const DOCX_V1_DOCUMENT_BLOCKS_BATCH_DELETE: &str =
    "/open-apis/docx/v1/documents/{document_id}/blocks/batch_delete";
pub const DOCX_V1_DOCUMENT_BLOCK_CHILDREN: &str =
    "/open-apis/docx/v1/documents/{document_id}/blocks/{block_id}/children";

// ==================== Wiki (知识库) v2 ====================

/// Wiki空间管理
/// 知识库空间的创建、配置和管理
pub const WIKI_V2_SPACES: &str = "/open-apis/wiki/v2/spaces";
pub const WIKI_V2_SPACE_GET: &str = "/open-apis/wiki/v2/spaces/{space_id}";
pub const WIKI_V2_SPACE_SETTING_UPDATE: &str = "/open-apis/wiki/v2/spaces/{space_id}/settings";

/// Wiki节点管理
/// 知识库文档节点的组织和管理
pub const WIKI_V2_SPACE_NODES: &str = "/open-apis/wiki/v2/spaces/{space_id}/nodes";
pub const WIKI_V2_SPACE_NODE_GET: &str = "/open-apis/wiki/v2/spaces/{space_id}/nodes/{node_id}";
pub const WIKI_V2_SPACE_NODE_COPY: &str =
    "/open-apis/wiki/v2/spaces/{space_id}/nodes/{node_id}/copy";
pub const WIKI_V2_SPACE_NODE_MOVE: &str =
    "/open-apis/wiki/v2/spaces/{space_id}/nodes/{node_id}/move";
pub const WIKI_V2_SPACE_NODE_UPDATE_TITLE: &str =
    "/open-apis/wiki/v2/spaces/{space_id}/nodes/{node_id}/update_title";

/// Wiki成员管理
/// 知识库协作权限管理
pub const WIKI_V2_SPACE_MEMBERS: &str = "/open-apis/wiki/v2/spaces/{space_id}/members";
pub const WIKI_V2_SPACE_MEMBER_CREATE: &str = "/open-apis/wiki/v2/spaces/{space_id}/members";
pub const WIKI_V2_SPACE_MEMBER_DELETE: &str =
    "/open-apis/wiki/v2/spaces/{space_id}/members/{member_id}";

/// Wiki搜索和任务
/// 知识库内容搜索和批量操作
pub const WIKI_V2_SEARCH: &str = "/open-apis/wiki/v2/search";
pub const WIKI_V2_TASK_GET: &str = "/open-apis/wiki/v2/tasks/{task_id}";
pub const WIKI_V2_TASKS_MOVE_DOCS_TO_WIKI: &str = "/open-apis/wiki/v2/tasks/move_docs_to_wiki";

// ==================== Board (白板) v1 ====================

/// Board白板管理
/// 协作白板的创建和内容管理
pub const BOARD_V1_WHITEBOARD_THUMBNAIL: &str =
    "/open-apis/whiteboard/v1/whiteboards/{whiteboard_id}/thumbnail";
pub const BOARD_V1_WHITEBOARD_NODES: &str = "/open-apis/board/v1/whiteboards/{whiteboard_id}/nodes";

// ==================== Comment (评论) v1 ====================

/// Comment评论管理
/// 文档和内容的评论系统
pub const COMMENT_V1_COMMENTS: &str = "/open-apis/comment/v1/comments";
pub const COMMENT_V1_COMMENT_GET: &str = "/open-apis/comment/v1/comments/{comment_id}";
pub const COMMENT_V1_COMMENTS_BATCH_QUERY: &str = "/open-apis/comment/v1/comments/batch_query";
pub const COMMENT_V1_COMMENT_REPLIES: &str = "/open-apis/comment/v1/comments/{comment_id}/replies";
pub const COMMENT_V1_COMMENT_REPLY_UPDATE: &str =
    "/open-apis/comment/v1/comments/{comment_id}/replies/{reply_id}";
pub const COMMENT_V1_COMMENT_REPLY_DELETE: &str =
    "/open-apis/comment/v1/comments/{comment_id}/replies/{reply_id}";

// ==================== Assistant (AI助手) v1 ====================

/// Assistant助手服务
/// 文档AI智能助手功能
pub const ASSISTANT_V1_FILE_SUBSCRIPTION: &str =
    "/open-apis/assistant/v1/file/{file_type}/{file_id}/subscription";

// ==================== Drive (云盘) v1 ====================

/// Drive文件管理
/// 云盘文件的基础操作和管理
pub const DRIVE_V1_FILES: &str = "/open-apis/drive/v1/files";
pub const DRIVE_V1_FILE_GET: &str = "/open-apis/drive/v1/files/{file_token}";
pub const DRIVE_V1_FILE_COPY: &str = "/open-apis/drive/v1/files/{file_token}/copy";
pub const DRIVE_V1_FILE_DOWNLOAD: &str = "/open-apis/drive/v1/files/{file_token}/download";
pub const DRIVE_V1_FILE_STATISTICS: &str = "/open-apis/drive/v1/files/{file_token}/statistics";
pub const DRIVE_V1_FILE_SUBSCRIPTIONS: &str =
    "/open-apis/drive/v1/files/{file_token}/subscriptions";
pub const DRIVE_V1_FILES_SUBSCRIBE: &str = "/open-apis/drive/v1/files/subscribe";
pub const DRIVE_V1_FILES_SEARCH: &str = "/open-apis/drive/v1/files/search";
pub const DRIVE_V1_FILES_CREATE_FOLDER: &str = "/open-apis/drive/v1/files/create_folder";
pub const DRIVE_V1_FILES_CREATE_SHORTCUT: &str = "/open-apis/drive/v1/files/create_shortcut";

/// Drive上传管理
/// 文件分片上传和断点续传
pub const DRIVE_V1_FILES_UPLOAD_PREPARE: &str = "/open-apis/drive/v1/files/upload_prepare";
pub const DRIVE_V1_FILES_UPLOAD_PART: &str = "/open-apis/drive/v1/files/upload_part";
pub const DRIVE_V1_FILES_UPLOAD_FINISH: &str = "/open-apis/drive/v1/files/upload_finish";
pub const DRIVE_V1_FILES_UPLOAD_ALL: &str = "/open-apis/drive/v1/files/upload_all";

/// Drive文件夹管理
/// 文件夹结构和组织管理
pub const DRIVE_V1_FOLDERS: &str = "/open-apis/drive/v1/folders";
pub const DRIVE_V1_FOLDER_GET: &str = "/open-apis/drive/v1/folders/{folder_token}";
pub const DRIVE_V1_FOLDER_CHILDREN: &str = "/open-apis/drive/v1/folders/{folder_token}/children";
pub const DRIVE_V1_FOLDER_MOVE: &str = "/open-apis/drive/v1/folders/{folder_token}/move";
pub const DRIVE_V1_FOLDERS_ROOT_FOLDER_META: &str = "/open-apis/drive/v1/folders/root_folder_meta";

/// Drive文件版本管理
/// 文件版本历史和恢复
pub const DRIVE_V1_FILE_VERSIONS: &str = "/open-apis/drive/v1/files/{file_token}/versions";
pub const DRIVE_V1_FILE_VERSION_GET: &str =
    "/open-apis/drive/v1/files/{file_token}/versions/{version_id}";

/// Drive文件互动记录
/// 文件查看、点赞等互动记录
pub const DRIVE_V1_FILE_VIEW_RECORDS: &str = "/open-apis/drive/v1/files/{file_token}/view_records";
pub const DRIVE_V1_FILE_LIKE_RECORDS: &str = "/open-apis/drive/v1/files/{file_token}/like_records";

/// Drive媒体管理
/// 图片、视频等媒体文件处理
pub const DRIVE_V1_MEDIAS_UPLOAD_PREPARE: &str = "/open-apis/drive/v1/medias/upload_prepare";
pub const DRIVE_V1_MEDIAS_UPLOAD_PART: &str = "/open-apis/drive/v1/medias/upload_part";
pub const DRIVE_V1_MEDIAS_UPLOAD_FINISH: &str = "/open-apis/drive/v1/medias/upload_finish";
pub const DRIVE_V1_MEDIAS_UPLOAD_ALL: &str = "/open-apis/drive/v1/medias/upload_all";
pub const DRIVE_V1_MEDIAS_DOWNLOAD: &str = "/open-apis/drive/v1/medias/{media_token}/download";
pub const DRIVE_V1_MEDIAS_BATCH_GET_TMP_DOWNLOAD_URL: &str =
    "/open-apis/drive/v1/medias/batch_get_tmp_download_url";

/// Drive批量查询和导入
/// 批量操作和文件导入功能
pub const DRIVE_V1_METAS_BATCH_QUERY: &str = "/open-apis/drive/v1/metas/batch_query";
pub const DRIVE_V1_IMPORT_TASKS: &str = "/open-apis/drive/v1/import_tasks";
pub const DRIVE_V1_IMPORT_TASK_GET: &str = "/open-apis/drive/v1/import_tasks/{task_id}";
pub const DRIVE_V1_TASK_GET: &str = "/open-apis/drive/v1/tasks/{task_id}";

/// Drive Explorer v2
/// 增强的文件浏览器功能
pub const DRIVE_EXPLORER_V2_ROOT_FOLDER_META: &str =
    "/open-apis/drive/explorer/v2/root_folder_meta";
pub const DRIVE_EXPLORER_V2_FOLDER_META: &str =
    "/open-apis/drive/explorer/v2/folder/{folder_token}/meta";

// ==================== Permission (权限) v1/v2 ====================

/// Drive权限管理v1
/// 文件和文件夹的权限控制
pub const DRIVE_V1_PERMISSIONS_MEMBERS: &str = "/open-apis/drive/v1/permissions/{token}/members";
pub const DRIVE_V1_PERMISSIONS_MEMBER_GET: &str =
    "/open-apis/drive/v1/permissions/{token}/members/{member_id}";
pub const DRIVE_V1_PERMISSIONS_MEMBERS_BATCH_CREATE: &str =
    "/open-apis/drive/v1/permissions/{token}/members/batch_create";
pub const DRIVE_V1_PERMISSIONS_MEMBERS_AUTH: &str =
    "/open-apis/drive/v1/permissions/{token}/members/auth";
pub const DRIVE_V1_PERMISSIONS_MEMBERS_TRANSFER_OWNER: &str =
    "/open-apis/drive/v1/permissions/{token}/members/transfer_owner";
pub const DRIVE_V1_PERMISSIONS_PUBLIC: &str = "/open-apis/drive/v1/permissions/{token}/public";
pub const DRIVE_V1_PERMISSIONS_PUBLIC_PASSWORD: &str =
    "/open-apis/drive/v1/permissions/{token}/public/password";

/// Drive权限管理v2
/// 增强的权限管理系统
pub const DRIVE_V2_PERMISSIONS_PUBLIC: &str = "/open-apis/drive/v2/permissions/{token}/public";

// ==================== CardKit (卡片组件) v1 ====================

/// CardKit卡片管理
/// 交互式卡片组件的创建和更新
pub const CARDKIT_V1_CARDS: &str = "/open-apis/cardkit/v1/cards";
pub const CARDKIT_V1_CARD_UPDATE: &str = "/open-apis/cardkit/v1/cards/{card_id}";
pub const CARDKIT_V1_CARD_BATCH_UPDATE: &str = "/open-apis/cardkit/v1/cards/batch_update";
pub const CARDKIT_V1_CARD_SETTINGS: &str = "/open-apis/cardkit/v1/cards/{card_id}/settings";
pub const CARDKIT_V1_CARD_ELEMENTS: &str = "/open-apis/cardkit/v1/cards/{card_id}/elements";

// ==================== Report (报告服务) v1 ====================

/// Report报告和规则管理
/// 数据报告和分析功能
pub const REPORT_V1_RULES_QUERY: &str = "/open-apis/report/v1/rules/query";
pub const REPORT_V1_RULE_VIEWS_OPERATION: &str = "/open-apis/report/v1/rule_views/{rule_view_id}";
pub const REPORT_V1_TASKS_QUERY: &str = "/open-apis/report/v1/tasks/query";

// ==================== 兼容性别名 ====================

/// 为保持向后兼容性，提供一些简短的别名
/// Bitable别名
pub const BITABLE_APPS: &str = BITABLE_V1_APPS;
pub const BITABLE_TABLES: &str = BITABLE_V1_TABLES;
pub const BITABLE_RECORDS: &str = BITABLE_V1_RECORDS;

/// Sheets别名
pub const SHEETS_SPREADSHEETS: &str = SHEETS_V3_SPREADSHEETS;
pub const SHEETS_VALUES: &str = SHEETS_V3_SPREADSHEET_VALUES_GET;

/// Wiki别名
pub const WIKI_SPACES: &str = WIKI_V2_SPACES;
pub const WIKI_NODES: &str = WIKI_V2_SPACE_NODES;

/// Drive别名
pub const DRIVE_FILES: &str = DRIVE_V1_FILES;
pub const DRIVE_FOLDERS: &str = DRIVE_V1_FOLDERS;

/// CardKit别名
pub const CARDS: &str = CARDKIT_V1_CARDS;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bitable_endpoints() {
        // 验证Bitable端点
        assert!(BITABLE_V1_APPS.starts_with("/open-apis/bitable/v1/"));
        assert!(BITABLE_V1_RECORDS.contains("records"));
        assert!(BITABLE_V1_TABLES.contains("tables"));
        assert!(BITABLE_V1_APP_GET.contains("{app_token}"));
    }

    #[test]
    fn test_sheets_endpoints() {
        // 验证Sheets端点
        assert!(SHEETS_V3_SPREADSHEETS.starts_with("/open-apis/sheets/v3/"));
        assert!(SHEETS_V3_SPREADSHEET_VALUES_GET.contains("values"));
        assert!(SHEETS_V2_SPREADSHEET_VALUES.starts_with("/open-apis/sheets/v2/"));
        assert!(SHEETS_V3_SPREADSHEET_FILTER.contains("filter"));
    }

    #[test]
    fn test_docx_endpoints() {
        // 验证Docx端点
        assert!(DOCX_V1_DOCUMENTS.starts_with("/open-apis/docx/v1/"));
        assert!(DOCX_V1_DOCUMENT_BLOCKS.contains("blocks"));
        assert!(DOCX_V1_DOCUMENT_RAW_CONTENT.contains("raw_content"));
        assert!(DOCX_V1_DOCUMENT_CONVERT.contains("convert"));
    }

    #[test]
    fn test_wiki_endpoints() {
        // 验证Wiki端点
        assert!(WIKI_V2_SPACES.starts_with("/open-apis/wiki/v2/"));
        assert!(WIKI_V2_SPACE_NODES.contains("nodes"));
        assert!(WIKI_V2_SEARCH.contains("search"));
        assert!(WIKI_V2_SPACE_MEMBERS.contains("members"));
    }

    #[test]
    fn test_drive_endpoints() {
        // 验证Drive端点
        assert!(DRIVE_V1_FILES.starts_with("/open-apis/drive/v1/"));
        assert!(DRIVE_V1_FILE_DOWNLOAD.contains("download"));
        assert!(DRIVE_V1_FILES_UPLOAD_ALL.contains("upload"));
        assert!(DRIVE_V1_PERMISSIONS_MEMBERS.contains("permissions"));
        assert!(DRIVE_EXPLORER_V2_ROOT_FOLDER_META.contains("explorer"));
    }

    #[test]
    fn test_cardkit_endpoints() {
        // 验证CardKit端点
        assert!(CARDKIT_V1_CARDS.starts_with("/open-apis/cardkit/v1/"));
        assert!(CARDKIT_V1_CARD_UPDATE.contains("{card_id}"));
        assert!(CARDKIT_V1_CARD_BATCH_UPDATE.contains("batch_update"));
        assert!(CARDKIT_V1_CARD_ELEMENTS.contains("elements"));
    }

    #[test]
    fn test_report_endpoints() {
        // 验证Report端点
        assert!(REPORT_V1_RULES_QUERY.starts_with("/open-apis/report/v1/"));
        assert!(REPORT_V1_TASKS_QUERY.contains("tasks"));
        assert!(REPORT_V1_RULE_VIEWS_OPERATION.contains("rule_views"));
    }

    #[test]
    fn test_other_services_endpoints() {
        // 验证其他服务端点
        assert!(BOARD_V1_WHITEBOARD_NODES.starts_with("/open-apis/"));
        assert!(COMMENT_V1_COMMENTS.starts_with("/open-apis/comment/v1/"));
        assert!(ASSISTANT_V1_FILE_SUBSCRIPTION.starts_with("/open-apis/assistant/v1/"));
    }

    #[test]
    fn test_backward_compatibility() {
        // 验证兼容性别名
        assert_eq!(BITABLE_APPS, BITABLE_V1_APPS);
        assert_eq!(BITABLE_TABLES, BITABLE_V1_TABLES);
        assert_eq!(SHEETS_SPREADSHEETS, SHEETS_V3_SPREADSHEETS);
        assert_eq!(WIKI_SPACES, WIKI_V2_SPACES);
        assert_eq!(DRIVE_FILES, DRIVE_V1_FILES);
        assert_eq!(CARDS, CARDKIT_V1_CARDS);
    }

    #[test]
    fn test_endpoint_parameter_placeholders() {
        // 测试关键端点是否包含正确的参数占位符
        assert!(BITABLE_V1_APP_GET.contains("{app_token}"));
        assert!(BITABLE_V1_TABLE_GET.contains("{app_token}"));
        assert!(BITABLE_V1_RECORD_GET.contains("{app_token}"));
        assert!(SHEETS_V3_SPREADSHEET_GET.contains("{spreadsheet_token}"));
        assert!(DOCX_V1_DOCUMENT_GET.contains("{document_id}"));
        assert!(WIKI_V2_SPACE_GET.contains("{space_id}"));
        assert!(DRIVE_V1_FILE_GET.contains("{file_token}"));
        assert!(CARDKIT_V1_CARD_UPDATE.contains("{card_id}"));
    }
} // Endpoints and EndpointBuilder are now available directly from openlark_core::endpoints

//! cloud_docs 服务端点常量定义
//!
//! Cloud Docs (云文档) 相关 API 端点常量，包括：
//! - Bitable (多维表格) v1
//! - Sheets (电子表格) v2/v3
//! - Docx (文档) v1
//! - Wiki (知识库) v2
//! - Board (白板) v1
//! - Comment (评论) v1
//! - Assistant (AI助手) v1

// ==================== Bitable (多维表格) v1 ====================

/// Bitable应用管理
pub const BITABLE_V1_APPS: &str = "/open-apis/bitable/v1/apps";
pub const BITABLE_V1_APP_GET: &str = "/open-apis/bitable/v1/apps/{app_token}";
pub const BITABLE_V1_APP_CREATE: &str = "/open-apis/bitable/v1/apps";
pub const BITABLE_V1_APP_UPDATE: &str = "/open-apis/bitable/v1/apps/{app_token}";
pub const BITABLE_V1_APP_COPY: &str = "/open-apis/bitable/v1/apps/{app_token}/copy";

/// Bitable表格管理
pub const BITABLE_V1_TABLES: &str = "/open-apis/bitable/v1/apps/{app_token}/tables";
pub const BITABLE_V1_TABLE_GET: &str = "/open-apis/bitable/v1/apps/{}/tables/{}";
pub const BITABLE_V1_TABLE_CREATE: &str = "/open-apis/bitable/v1/apps/{app_token}/tables";
pub const BITABLE_V1_TABLE_PATCH: &str = "/open-apis/bitable/v1/apps/{app_token}/tables/{table_id}";
pub const BITABLE_V1_TABLE_DELETE: &str =
    "/open-apis/bitable/v1/apps/{app_token}/tables/{table_id}";
pub const BITABLE_V1_TABLES_BATCH_CREATE: &str =
    "/open-apis/bitable/v1/apps/{app_token}/tables/batch_create";
pub const BITABLE_V1_TABLES_BATCH_DELETE: &str =
    "/open-apis/bitable/v1/apps/{app_token}/tables/batch_delete";

/// Bitable记录管理
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
pub const BITABLE_V1_FIELDS: &str =
    "/open-apis/bitable/v1/apps/{app_token}/tables/{table_id}/fields";
pub const BITABLE_V1_FIELD_UPDATE: &str =
    "/open-apis/bitable/v1/apps/{app_token}/tables/{table_id}/fields/{field_id}";
pub const BITABLE_V1_FIELD_DELETE: &str =
    "/open-apis/bitable/v1/apps/{app_token}/tables/{table_id}/fields/{field_id}";

/// Bitable视图管理
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
pub const BITABLE_V1_DASHBOARDS: &str = "/open-apis/bitable/v1/apps/{app_token}/dashboards";
pub const BITABLE_V1_DASHBOARD_COPY: &str =
    "/open-apis/bitable/v1/apps/{app_token}/dashboards/{dashboard_id}/copy";

/// Bitable角色管理
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
pub const BITABLE_V1_FORM_GET: &str =
    "/open-apis/bitable/v1/apps/{app_token}/tables/{table_id}/forms/{form_id}";
pub const BITABLE_V1_FORM_PATCH: &str =
    "/open-apis/bitable/v1/apps/{app_token}/tables/{table_id}/forms/{form_id}";
pub const BITABLE_V1_FORM_PATCH_META: &str =
    "/open-apis/bitable/v1/apps/{app_token}/tables/{table_id}/forms/{form_id}/patch_meta";
pub const BITABLE_V1_FORM_QUESTION: &str =
    "/open-apis/bitable/v1/apps/{app_token}/tables/{table_id}/forms/{form_id}/questions";

/// Bitable工作流管理
pub const BITABLE_V1_WORKFLOWS: &str = "/open-apis/bitable/v1/apps/{app_token}/workflows";
pub const BITABLE_V1_WORKFLOW_UPDATE: &str =
    "/open-apis/bitable/v1/apps/{app_token}/workflows/{workflow_id}";

// ==================== Sheets (电子表格) v2/v3 ====================

/// Sheets v2 - 电子表格基础操作
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

/// Sheets v3 - 筛选器
pub const SHEETS_V3_SPREADSHEET_FILTER: &str =
    "/open-apis/sheets/v3/spreadsheets/{spreadsheet_token}/sheets/{sheet_id}/filter";
pub const SHEETS_V3_SPREADSHEET_FILTER_VIEWS: &str =
    "/open-apis/sheets/v3/spreadsheets/{spreadsheet_token}/sheets/{sheet_id}/filter_views";
pub const SHEETS_V3_SPREADSHEET_FILTER_VIEW_GET: &str = "/open-apis/sheets/v3/spreadsheets/{spreadsheet_token}/sheets/{sheet_id}/filter_views/{filter_view_id}";
pub const SHEETS_V3_SPREADSHEET_FILTER_VIEW_CONDITIONS: &str = "/open-apis/sheets/v3/spreadsheets/{spreadsheet_token}/sheets/{sheet_id}/filter_views/{filter_view_id}/conditions";
pub const SHEETS_V3_SPREADSHEET_FILTER_VIEW_CONDITION_GET: &str = "/open-apis/sheets/v3/spreadsheets/{spreadsheet_token}/sheets/{sheet_id}/filter_views/{filter_view_id}/conditions/{condition_id}";

/// Sheets v3 - 保护范围
pub const SHEETS_V3_SPREADSHEET_PROTECT_RANGE: &str =
    "/open-apis/sheets/v3/spreadsheets/{spreadsheet_token}/sheets/{sheet_id}/protect_range";
pub const SHEETS_V3_SPREADSHEET_PROTECT_RANGE_GET: &str = "/open-apis/sheets/v3/spreadsheets/{spreadsheet_token}/sheets/{sheet_id}/protect_range/{protect_id}";

/// Sheets v3 - 浮动图片
pub const SHEETS_V3_SPREADSHEET_FLOAT_IMAGES: &str =
    "/open-apis/sheets/v3/spreadsheets/{spreadsheet_token}/sheets/{sheet_id}/float_images";
pub const SHEETS_V3_SPREADSHEET_FLOAT_IMAGE_GET: &str = "/open-apis/sheets/v3/spreadsheets/{spreadsheet_token}/sheets/{sheet_id}/float_images/{float_image_id}";

// ==================== Docx (文档) v1 ====================

/// Docx文档管理
pub const DOCX_V1_DOCUMENTS: &str = "/open-apis/docx/v1/documents";
pub const DOCX_V1_DOCUMENT_GET: &str = "/open-apis/docx/v1/documents/{}";
pub const DOCX_V1_DOCUMENT_RAW_CONTENT: &str = "/open-apis/docx/v1/documents/{}/raw_content";
pub const DOCX_V1_DOCUMENT_CONVERT: &str = "/open-apis/docx/v1/documents/{}/convert";

/// Docx文档块管理
pub const DOCX_V1_DOCUMENT_BLOCKS: &str = "/open-apis/docx/v1/documents/{}/blocks";
pub const DOCX_V1_DOCUMENT_BLOCK_GET: &str =
    "/open-apis/docx/v1/documents/{document_id}/blocks/{block_id}";
pub const DOCX_V1_DOCUMENT_BLOCKS_BATCH_UPDATE: &str =
    "/open-apis/docx/v1/documents/{document_id}/blocks/batch_update";
pub const DOCX_V1_DOCUMENT_BLOCKS_BATCH_DELETE: &str =
    "/open-apis/docx/v1/documents/{}/blocks/batch_delete";
pub const DOCX_V1_DOCUMENT_BLOCK_CHILDREN: &str =
    "/open-apis/docx/v1/documents/{}/blocks/{}/children";

// ==================== Wiki (知识库) v2 ====================

/// Wiki空间管理
pub const WIKI_V2_SPACES: &str = "/open-apis/wiki/v2/spaces";
pub const WIKI_V2_SPACE_GET: &str = "/open-apis/wiki/v2/spaces/{}";
pub const WIKI_V2_SPACE_SETTING_UPDATE: &str = "/open-apis/wiki/v2/spaces/{}/settings";

/// Wiki节点管理
pub const WIKI_V2_SPACE_NODES: &str = "/open-apis/wiki/v2/spaces/{}/nodes";
pub const WIKI_V2_SPACE_NODE_GET: &str = "/open-apis/wiki/v2/spaces/{}/nodes/{}";
pub const WIKI_V2_SPACE_NODE_COPY: &str = "/open-apis/wiki/v2/spaces/{}/nodes/{}/copy";
pub const WIKI_V2_SPACE_NODE_MOVE: &str = "/open-apis/wiki/v2/spaces/{}/nodes/{}/move";
pub const WIKI_V2_SPACE_NODE_UPDATE_TITLE: &str =
    "/open-apis/wiki/v2/spaces/{}/nodes/{}/update_title";

/// Wiki成员管理
pub const WIKI_V2_SPACE_MEMBERS: &str = "/open-apis/wiki/v2/spaces/{}/members";
pub const WIKI_V2_SPACE_MEMBER_CREATE: &str = "/open-apis/wiki/v2/spaces/{}/members";
pub const WIKI_V2_SPACE_MEMBER_DELETE: &str = "/open-apis/wiki/v2/spaces/{}/members/{}";

/// Wiki搜索和任务
pub const WIKI_V2_SEARCH: &str = "/open-apis/wiki/v2/search";
pub const WIKI_V2_TASK_GET: &str = "/open-apis/wiki/v2/tasks/{}";
pub const WIKI_V2_TASKS_MOVE_DOCS_TO_WIKI: &str = "/open-apis/wiki/v2/tasks/move_docs_to_wiki";

// ==================== Board (白板) v1 ====================

/// Board白板管理
pub const BOARD_V1_WHITEBOARD_THUMBNAIL: &str = "/open-apis/whiteboard/v1/whiteboards/{}/thumbnail";
pub const BOARD_V1_WHITEBOARD_NODES: &str = "/open-apis/board/v1/whiteboards/{}/nodes";

// ==================== Comment (评论) v1 ====================

/// Comment评论管理
pub const COMMENT_V1_COMMENTS: &str = "/open-apis/comment/v1/comments";
pub const COMMENT_V1_COMMENT_GET: &str = "/open-apis/comment/v1/comments/{}";
pub const COMMENT_V1_COMMENTS_BATCH_QUERY: &str = "/open-apis/comment/v1/comments/batch_query";
pub const COMMENT_V1_COMMENT_REPLIES: &str = "/open-apis/comment/v1/comments/{}/replies";
pub const COMMENT_V1_COMMENT_REPLY_UPDATE: &str = "/open-apis/comment/v1/comments/{}/replies/{}";
pub const COMMENT_V1_COMMENT_REPLY_DELETE: &str = "/open-apis/comment/v1/comments/{}/replies/{}";

// ==================== Assistant (AI助手) v1 ====================

/// Assistant助手服务
pub const ASSISTANT_V1_FILE_SUBSCRIPTION: &str = "/open-apis/assistant/v1/file/{}/{}/subscription";

// ==================== Permission (权限) v1/v2 ====================

/// Drive权限管理v1
pub const DRIVE_V1_PERMISSIONS_MEMBERS: &str = "/open-apis/drive/v1/permissions/{}/members";
pub const DRIVE_V1_PERMISSIONS_MEMBER_GET: &str = "/open-apis/drive/v1/permissions/{}/members/{}";
pub const DRIVE_V1_PERMISSIONS_MEMBERS_BATCH_CREATE: &str =
    "/open-apis/drive/v1/permissions/{}/members/batch_create";
pub const DRIVE_V1_PERMISSIONS_MEMBERS_AUTH: &str =
    "/open-apis/drive/v1/permissions/{}/members/auth";
pub const DRIVE_V1_PERMISSIONS_MEMBERS_TRANSFER_OWNER: &str =
    "/open-apis/drive/v1/permissions/{}/members/transfer_owner";
pub const DRIVE_V1_PERMISSIONS_PUBLIC: &str = "/open-apis/drive/v1/permissions/{}/public";
pub const DRIVE_V1_PERMISSIONS_PUBLIC_PASSWORD: &str =
    "/open-apis/drive/v1/permissions/{}/public/password";

/// Drive权限管理v2
pub const DRIVE_V2_PERMISSIONS_PUBLIC: &str = "/open-apis/drive/v2/permissions/{}/public";

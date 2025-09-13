// CloudDocs模块端点常量
// 自动生成，用于Phase 2.4.4a优化任务
// 需要添加到 src/core/endpoints.rs 中

impl Endpoints {

    // ==================== Assistant 服务端点 ====================

    /// /open-apis/assistant/v1/file/{}/{}/subscription
    pub const ASSISTANT_V1_FILE_ID_ID_SUBSCRIPTION: &'static str = "/open-apis/assistant/v1/file/{}/{}/subscription";


    // ==================== Bitable 服务端点 ====================

    /// /open-apis/bitable/v1/apps
    pub const BITABLE_V1_APPS: &'static str = "/open-apis/bitable/v1/apps";

    /// /open-apis/bitable/v1/apps/{app_token}/dashboards
    pub const BITABLE_V1_APPS_APP_TOKEN_DASHBOARDS: &'static str = "/open-apis/bitable/v1/apps/{app_token}/dashboards";

    /// /open-apis/bitable/v1/apps/{app_token}/dashboards/{block_id}/copy
    pub const BITABLE_V1_APPS_APP_TOKEN_DASHBOARDS_BLOCK_ID_COPY: &'static str = "/open-apis/bitable/v1/apps/{app_token}/dashboards/{block_id}/copy";

    /// /open-apis/bitable/v1/apps/{app_token}/forms/{form_id}
    pub const BITABLE_V1_APPS_APP_TOKEN_FORMS_FORM_ID: &'static str = "/open-apis/bitable/v1/apps/{app_token}/forms/{form_id}";

    /// /open-apis/bitable/v1/apps/{app_token}/forms/{form_id}/questions
    pub const BITABLE_V1_APPS_APP_TOKEN_FORMS_FORM_ID_QUESTIONS: &'static str = "/open-apis/bitable/v1/apps/{app_token}/forms/{form_id}/questions";

    /// /open-apis/bitable/v1/apps/{app_token}/forms/{form_id}/questions/{question_id}
    pub const BITABLE_V1_APPS_APP_TOKEN_FORMS_FORM_ID_QUESTIONS_QUESTION_ID: &'static str = "/open-apis/bitable/v1/apps/{app_token}/forms/{form_id}/questions/{question_id}";

    /// /open-apis/bitable/v1/apps/{app_token}/roles
    pub const BITABLE_V1_APPS_APP_TOKEN_ROLES: &'static str = "/open-apis/bitable/v1/apps/{app_token}/roles";

    /// /open-apis/bitable/v1/apps/{app_token}/roles/{role_id}
    pub const BITABLE_V1_APPS_APP_TOKEN_ROLES_ROLE_ID: &'static str = "/open-apis/bitable/v1/apps/{app_token}/roles/{role_id}";

    /// /open-apis/bitable/v1/apps/{app_token}/roles/{role_id}/members
    pub const BITABLE_V1_APPS_APP_TOKEN_ROLES_ROLE_ID_MEMBERS: &'static str = "/open-apis/bitable/v1/apps/{app_token}/roles/{role_id}/members";

    /// /open-apis/bitable/v1/apps/{app_token}/roles/{role_id}/members/batch_create
    pub const BITABLE_V1_APPS_APP_TOKEN_ROLES_ROLE_ID_MEMBERS_BATCH_CREATE: &'static str = "/open-apis/bitable/v1/apps/{app_token}/roles/{role_id}/members/batch_create";

    /// /open-apis/bitable/v1/apps/{app_token}/roles/{role_id}/members/batch_delete
    pub const BITABLE_V1_APPS_APP_TOKEN_ROLES_ROLE_ID_MEMBERS_BATCH_DELETE: &'static str = "/open-apis/bitable/v1/apps/{app_token}/roles/{role_id}/members/batch_delete";

    /// /open-apis/bitable/v1/apps/{app_token}/roles/{role_id}/members/{member_id}
    pub const BITABLE_V1_APPS_APP_TOKEN_ROLES_ROLE_ID_MEMBERS_MEMBER_ID: &'static str = "/open-apis/bitable/v1/apps/{app_token}/roles/{role_id}/members/{member_id}";

    /// /open-apis/bitable/v1/apps/{app_token}/tables/{table_id}/fields
    pub const BITABLE_V1_APPS_APP_TOKEN_TABLES_TABLE_ID_FIELDS: &'static str = "/open-apis/bitable/v1/apps/{app_token}/tables/{table_id}/fields";

    /// /open-apis/bitable/v1/apps/{app_token}/tables/{table_id}/fields/{field_id}
    pub const BITABLE_V1_APPS_APP_TOKEN_TABLES_TABLE_ID_FIELDS_FIELD_ID: &'static str = "/open-apis/bitable/v1/apps/{app_token}/tables/{table_id}/fields/{field_id}";

    /// /open-apis/bitable/v1/apps/{app_token}/tables/{table_id}/records
    pub const BITABLE_V1_APPS_APP_TOKEN_TABLES_TABLE_ID_RECORDS: &'static str = "/open-apis/bitable/v1/apps/{app_token}/tables/{table_id}/records";

    /// /open-apis/bitable/v1/apps/{app_token}/tables/{table_id}/records/batch_create
    pub const BITABLE_V1_APPS_APP_TOKEN_TABLES_TABLE_ID_RECORDS_BATCH_CREATE: &'static str = "/open-apis/bitable/v1/apps/{app_token}/tables/{table_id}/records/batch_create";

    /// /open-apis/bitable/v1/apps/{app_token}/tables/{table_id}/records/batch_delete
    pub const BITABLE_V1_APPS_APP_TOKEN_TABLES_TABLE_ID_RECORDS_BATCH_DELETE: &'static str = "/open-apis/bitable/v1/apps/{app_token}/tables/{table_id}/records/batch_delete";

    /// /open-apis/bitable/v1/apps/{app_token}/tables/{table_id}/records/batch_get
    pub const BITABLE_V1_APPS_APP_TOKEN_TABLES_TABLE_ID_RECORDS_BATCH_GET: &'static str = "/open-apis/bitable/v1/apps/{app_token}/tables/{table_id}/records/batch_get";

    /// /open-apis/bitable/v1/apps/{app_token}/tables/{table_id}/records/batch_update
    pub const BITABLE_V1_APPS_APP_TOKEN_TABLES_TABLE_ID_RECORDS_BATCH_UPDATE: &'static str = "/open-apis/bitable/v1/apps/{app_token}/tables/{table_id}/records/batch_update";

    /// /open-apis/bitable/v1/apps/{app_token}/tables/{table_id}/records/search
    pub const BITABLE_V1_APPS_APP_TOKEN_TABLES_TABLE_ID_RECORDS_SEARCH: &'static str = "/open-apis/bitable/v1/apps/{app_token}/tables/{table_id}/records/search";

    /// /open-apis/bitable/v1/apps/{app_token}/tables/{table_id}/records/{record_id}
    pub const BITABLE_V1_APPS_APP_TOKEN_TABLES_TABLE_ID_RECORDS_RECORD_ID: &'static str = "/open-apis/bitable/v1/apps/{app_token}/tables/{table_id}/records/{record_id}";

    /// /open-apis/bitable/v1/apps/{app_token}/workflows
    pub const BITABLE_V1_APPS_APP_TOKEN_WORKFLOWS: &'static str = "/open-apis/bitable/v1/apps/{app_token}/workflows";

    /// /open-apis/bitable/v1/apps/{app_token}/workflows/{workflow_id}
    pub const BITABLE_V1_APPS_APP_TOKEN_WORKFLOWS_WORKFLOW_ID: &'static str = "/open-apis/bitable/v1/apps/{app_token}/workflows/{workflow_id}";

    /// /open-apis/bitable/v1/apps/{}
    pub const BITABLE_V1_APPS_ID: &'static str = "/open-apis/bitable/v1/apps/{}";

    /// /open-apis/bitable/v1/apps/{}/copy
    pub const BITABLE_V1_APPS_ID_COPY: &'static str = "/open-apis/bitable/v1/apps/{}/copy";

    /// /open-apis/bitable/v1/apps/{}/tables
    pub const BITABLE_V1_APPS_ID_TABLES: &'static str = "/open-apis/bitable/v1/apps/{}/tables";

    /// /open-apis/bitable/v1/apps/{}/tables/batch_create
    pub const BITABLE_V1_APPS_ID_TABLES_BATCH_CREATE: &'static str = "/open-apis/bitable/v1/apps/{}/tables/batch_create";

    /// /open-apis/bitable/v1/apps/{}/tables/batch_delete
    pub const BITABLE_V1_APPS_ID_TABLES_BATCH_DELETE: &'static str = "/open-apis/bitable/v1/apps/{}/tables/batch_delete";

    /// /open-apis/bitable/v1/apps/{}/tables/{}
    pub const BITABLE_V1_APPS_ID_TABLES_ID: &'static str = "/open-apis/bitable/v1/apps/{}/tables/{}";

    /// /open-apis/bitable/v1/apps/{}/tables/{}/views
    pub const BITABLE_V1_APPS_ID_TABLES_ID_VIEWS: &'static str = "/open-apis/bitable/v1/apps/{}/tables/{}/views";

    /// /open-apis/bitable/v1/apps/{}/tables/{}/views/{}
    pub const BITABLE_V1_APPS_ID_TABLES_ID_VIEWS_ID: &'static str = "/open-apis/bitable/v1/apps/{}/tables/{}/views/{}";


    // ==================== Board 服务端点 ====================

    /// /open-apis/board/v1/whiteboards/{}/nodes
    pub const BOARD_V1_WHITEBOARDS_ID_NODES: &'static str = "/open-apis/board/v1/whiteboards/{}/nodes";


    // ==================== Comment 服务端点 ====================

    /// /open-apis/comment/v1/comments/batch_query?file_type={}&file_token={}
    pub const COMMENT_V1_COMMENTS_BATCH_QUERY?FILE_TYPE=ID&FILE_TOKEN=ID: &'static str = "/open-apis/comment/v1/comments/batch_query?file_type={}&file_token={}";

    /// /open-apis/comment/v1/comments/{}/replies/{}?file_type={}&file_token={}
    pub const COMMENT_V1_COMMENTS_ID_REPLIES_ID?FILE_TYPE=ID&FILE_TOKEN=ID: &'static str = "/open-apis/comment/v1/comments/{}/replies/{}?file_type={}&file_token={}";

    /// /open-apis/comment/v1/comments/{}/replies?file_type={}&file_token={}
    pub const COMMENT_V1_COMMENTS_ID_REPLIES?FILE_TYPE=ID&FILE_TOKEN=ID: &'static str = "/open-apis/comment/v1/comments/{}/replies?file_type={}&file_token={}";

    /// /open-apis/comment/v1/comments/{}?file_type={}&file_token={}
    pub const COMMENT_V1_COMMENTS_ID?FILE_TYPE=ID&FILE_TOKEN=ID: &'static str = "/open-apis/comment/v1/comments/{}?file_type={}&file_token={}";

    /// /open-apis/comment/v1/comments?file_type={}&file_token={}
    pub const COMMENT_V1_COMMENTS?FILE_TYPE=ID&FILE_TOKEN=ID: &'static str = "/open-apis/comment/v1/comments?file_type={}&file_token={}";


    // ==================== Docx 服务端点 ====================

    /// /open-apis/docx/v1/documents
    pub const DOCX_V1_DOCUMENTS: &'static str = "/open-apis/docx/v1/documents";

    /// /open-apis/docx/v1/documents/{}
    pub const DOCX_V1_DOCUMENTS_ID: &'static str = "/open-apis/docx/v1/documents/{}";

    /// /open-apis/docx/v1/documents/{}/blocks
    pub const DOCX_V1_DOCUMENTS_ID_BLOCKS: &'static str = "/open-apis/docx/v1/documents/{}/blocks";

    /// /open-apis/docx/v1/documents/{}/blocks/batch_delete
    pub const DOCX_V1_DOCUMENTS_ID_BLOCKS_BATCH_DELETE: &'static str = "/open-apis/docx/v1/documents/{}/blocks/batch_delete";

    /// /open-apis/docx/v1/documents/{}/blocks/batch_update
    pub const DOCX_V1_DOCUMENTS_ID_BLOCKS_BATCH_UPDATE: &'static str = "/open-apis/docx/v1/documents/{}/blocks/batch_update";

    /// /open-apis/docx/v1/documents/{}/blocks/{}
    pub const DOCX_V1_DOCUMENTS_ID_BLOCKS_ID: &'static str = "/open-apis/docx/v1/documents/{}/blocks/{}";

    /// /open-apis/docx/v1/documents/{}/blocks/{}/children
    pub const DOCX_V1_DOCUMENTS_ID_BLOCKS_ID_CHILDREN: &'static str = "/open-apis/docx/v1/documents/{}/blocks/{}/children";

    /// /open-apis/docx/v1/documents/{}/convert
    pub const DOCX_V1_DOCUMENTS_ID_CONVERT: &'static str = "/open-apis/docx/v1/documents/{}/convert";

    /// /open-apis/docx/v1/documents/{}/raw_content
    pub const DOCX_V1_DOCUMENTS_ID_RAW_CONTENT: &'static str = "/open-apis/docx/v1/documents/{}/raw_content";


    // ==================== Drive 服务端点 ====================

    /// /open-apis/drive/explorer/v2/folder/{folder_token}/meta
    pub const DRIVE_EXPLORER_V2_FOLDER_FOLDER_TOKEN_META: &'static str = "/open-apis/drive/explorer/v2/folder/{folder_token}/meta";

    /// /open-apis/drive/explorer/v2/root_folder/meta
    pub const DRIVE_EXPLORER_V2_ROOT_FOLDER_META: &'static str = "/open-apis/drive/explorer/v2/root_folder/meta";

    /// /open-apis/drive/v1/files
    pub const DRIVE_V1_FILES: &'static str = "/open-apis/drive/v1/files";

    /// /open-apis/drive/v1/files/create_folder
    pub const DRIVE_V1_FILES_CREATE_FOLDER: &'static str = "/open-apis/drive/v1/files/create_folder";

    /// /open-apis/drive/v1/files/create_shortcut
    pub const DRIVE_V1_FILES_CREATE_SHORTCUT: &'static str = "/open-apis/drive/v1/files/create_shortcut";

    /// /open-apis/drive/v1/files/search
    pub const DRIVE_V1_FILES_SEARCH: &'static str = "/open-apis/drive/v1/files/search";

    /// /open-apis/drive/v1/files/subscribe
    pub const DRIVE_V1_FILES_SUBSCRIBE: &'static str = "/open-apis/drive/v1/files/subscribe";

    /// /open-apis/drive/v1/files/upload_all
    pub const DRIVE_V1_FILES_UPLOAD_ALL: &'static str = "/open-apis/drive/v1/files/upload_all";

    /// /open-apis/drive/v1/files/upload_finish
    pub const DRIVE_V1_FILES_UPLOAD_FINISH: &'static str = "/open-apis/drive/v1/files/upload_finish";

    /// /open-apis/drive/v1/files/upload_part
    pub const DRIVE_V1_FILES_UPLOAD_PART: &'static str = "/open-apis/drive/v1/files/upload_part";

    /// /open-apis/drive/v1/files/upload_prepare
    pub const DRIVE_V1_FILES_UPLOAD_PREPARE: &'static str = "/open-apis/drive/v1/files/upload_prepare";

    /// /open-apis/drive/v1/files/{}
    pub const DRIVE_V1_FILES_ID: &'static str = "/open-apis/drive/v1/files/{}";

    /// /open-apis/drive/v1/files/{}/copy
    pub const DRIVE_V1_FILES_ID_COPY: &'static str = "/open-apis/drive/v1/files/{}/copy";

    /// /open-apis/drive/v1/files/{}/download
    pub const DRIVE_V1_FILES_ID_DOWNLOAD: &'static str = "/open-apis/drive/v1/files/{}/download";

    /// /open-apis/drive/v1/files/{}/like_records
    pub const DRIVE_V1_FILES_ID_LIKE_RECORDS: &'static str = "/open-apis/drive/v1/files/{}/like_records";

    /// /open-apis/drive/v1/files/{}/statistics
    pub const DRIVE_V1_FILES_ID_STATISTICS: &'static str = "/open-apis/drive/v1/files/{}/statistics";

    /// /open-apis/drive/v1/files/{}/subscriptions/{}
    pub const DRIVE_V1_FILES_ID_SUBSCRIPTIONS_ID: &'static str = "/open-apis/drive/v1/files/{}/subscriptions/{}";

    /// /open-apis/drive/v1/files/{}/versions
    pub const DRIVE_V1_FILES_ID_VERSIONS: &'static str = "/open-apis/drive/v1/files/{}/versions";

    /// /open-apis/drive/v1/files/{}/versions/{}
    pub const DRIVE_V1_FILES_ID_VERSIONS_ID: &'static str = "/open-apis/drive/v1/files/{}/versions/{}";

    /// /open-apis/drive/v1/files/{}/view_records
    pub const DRIVE_V1_FILES_ID_VIEW_RECORDS: &'static str = "/open-apis/drive/v1/files/{}/view_records";

    /// /open-apis/drive/v1/folders
    pub const DRIVE_V1_FOLDERS: &'static str = "/open-apis/drive/v1/folders";

    /// /open-apis/drive/v1/folders/root_folder_meta
    pub const DRIVE_V1_FOLDERS_ROOT_FOLDER_META: &'static str = "/open-apis/drive/v1/folders/root_folder_meta";

    /// /open-apis/drive/v1/folders/{}
    pub const DRIVE_V1_FOLDERS_ID: &'static str = "/open-apis/drive/v1/folders/{}";

    /// /open-apis/drive/v1/folders/{}/children
    pub const DRIVE_V1_FOLDERS_ID_CHILDREN: &'static str = "/open-apis/drive/v1/folders/{}/children";

    /// /open-apis/drive/v1/folders/{}/move
    pub const DRIVE_V1_FOLDERS_ID_MOVE: &'static str = "/open-apis/drive/v1/folders/{}/move";

    /// /open-apis/drive/v1/import_tasks
    pub const DRIVE_V1_IMPORT_TASKS: &'static str = "/open-apis/drive/v1/import_tasks";

    /// /open-apis/drive/v1/import_tasks/{}
    pub const DRIVE_V1_IMPORT_TASKS_ID: &'static str = "/open-apis/drive/v1/import_tasks/{}";

    /// /open-apis/drive/v1/medias/batch_get_tmp_download_url
    pub const DRIVE_V1_MEDIAS_BATCH_GET_TMP_DOWNLOAD_URL: &'static str = "/open-apis/drive/v1/medias/batch_get_tmp_download_url";

    /// /open-apis/drive/v1/medias/upload_all
    pub const DRIVE_V1_MEDIAS_UPLOAD_ALL: &'static str = "/open-apis/drive/v1/medias/upload_all";

    /// /open-apis/drive/v1/medias/upload_finish
    pub const DRIVE_V1_MEDIAS_UPLOAD_FINISH: &'static str = "/open-apis/drive/v1/medias/upload_finish";

    /// /open-apis/drive/v1/medias/upload_part
    pub const DRIVE_V1_MEDIAS_UPLOAD_PART: &'static str = "/open-apis/drive/v1/medias/upload_part";

    /// /open-apis/drive/v1/medias/upload_prepare
    pub const DRIVE_V1_MEDIAS_UPLOAD_PREPARE: &'static str = "/open-apis/drive/v1/medias/upload_prepare";

    /// /open-apis/drive/v1/medias/{}/download
    pub const DRIVE_V1_MEDIAS_ID_DOWNLOAD: &'static str = "/open-apis/drive/v1/medias/{}/download";

    /// /open-apis/drive/v1/metas/batch_query
    pub const DRIVE_V1_METAS_BATCH_QUERY: &'static str = "/open-apis/drive/v1/metas/batch_query";

    /// /open-apis/drive/v1/permissions/{}/members/auth?type={}&perm={}
    pub const DRIVE_V1_PERMISSIONS_ID_MEMBERS_AUTH?TYPE=ID&PERM=ID: &'static str = "/open-apis/drive/v1/permissions/{}/members/auth?type={}&perm={}";

    /// /open-apis/drive/v1/permissions/{}/members/batch_create?type={}
    pub const DRIVE_V1_PERMISSIONS_ID_MEMBERS_BATCH_CREATE?TYPE=ID: &'static str = "/open-apis/drive/v1/permissions/{}/members/batch_create?type={}";

    /// /open-apis/drive/v1/permissions/{}/members/transfer_owner?type={}
    pub const DRIVE_V1_PERMISSIONS_ID_MEMBERS_TRANSFER_OWNER?TYPE=ID: &'static str = "/open-apis/drive/v1/permissions/{}/members/transfer_owner?type={}";

    /// /open-apis/drive/v1/permissions/{}/members/{}?type={}&member_type={}
    pub const DRIVE_V1_PERMISSIONS_ID_MEMBERS_ID?TYPE=ID&MEMBER_TYPE=ID: &'static str = "/open-apis/drive/v1/permissions/{}/members/{}?type={}&member_type={}";

    /// /open-apis/drive/v1/permissions/{}/members?type={}
    pub const DRIVE_V1_PERMISSIONS_ID_MEMBERS?TYPE=ID: &'static str = "/open-apis/drive/v1/permissions/{}/members?type={}";

    /// /open-apis/drive/v1/permissions/{}/public/password?type={}
    pub const DRIVE_V1_PERMISSIONS_ID_PUBLIC_PASSWORD?TYPE=ID: &'static str = "/open-apis/drive/v1/permissions/{}/public/password?type={}";

    /// /open-apis/drive/v1/permissions/{}/public?type={}
    pub const DRIVE_V1_PERMISSIONS_ID_PUBLIC?TYPE=ID: &'static str = "/open-apis/drive/v1/permissions/{}/public?type={}";

    /// /open-apis/drive/v1/tasks/{}
    pub const DRIVE_V1_TASKS_ID: &'static str = "/open-apis/drive/v1/tasks/{}";

    /// /open-apis/drive/v2/permissions/{}/public
    pub const DRIVE_V2_PERMISSIONS_ID_PUBLIC: &'static str = "/open-apis/drive/v2/permissions/{}/public";

    /// /open-apis/drive/v2/permissions/{}/public?type={}
    pub const DRIVE_V2_PERMISSIONS_ID_PUBLIC?TYPE=ID: &'static str = "/open-apis/drive/v2/permissions/{}/public?type={}";


    // ==================== Sheets 服务端点 ====================

    /// /open-apis/sheets/v2/spreadsheets/{spreadsheet_token}/dimension_range
    pub const SHEETS_V2_SPREADSHEETS_SPREADSHEET_TOKEN_DIMENSION_RANGE: &'static str = "/open-apis/sheets/v2/spreadsheets/{spreadsheet_token}/dimension_range";

    /// /open-apis/sheets/v2/spreadsheets/{spreadsheet_token}/insert_dimension_range
    pub const SHEETS_V2_SPREADSHEETS_SPREADSHEET_TOKEN_INSERT_DIMENSION_RANGE: &'static str = "/open-apis/sheets/v2/spreadsheets/{spreadsheet_token}/insert_dimension_range";

    /// /open-apis/sheets/v2/spreadsheets/{spreadsheet_token}/merge_cells
    pub const SHEETS_V2_SPREADSHEETS_SPREADSHEET_TOKEN_MERGE_CELLS: &'static str = "/open-apis/sheets/v2/spreadsheets/{spreadsheet_token}/merge_cells";

    /// /open-apis/sheets/v2/spreadsheets/{spreadsheet_token}/sheets_batch_update
    pub const SHEETS_V2_SPREADSHEETS_SPREADSHEET_TOKEN_SHEETS_BATCH_UPDATE: &'static str = "/open-apis/sheets/v2/spreadsheets/{spreadsheet_token}/sheets_batch_update";

    /// /open-apis/sheets/v2/spreadsheets/{spreadsheet_token}/style
    pub const SHEETS_V2_SPREADSHEETS_SPREADSHEET_TOKEN_STYLE: &'static str = "/open-apis/sheets/v2/spreadsheets/{spreadsheet_token}/style";

    /// /open-apis/sheets/v2/spreadsheets/{spreadsheet_token}/styles_batch_update
    pub const SHEETS_V2_SPREADSHEETS_SPREADSHEET_TOKEN_STYLES_BATCH_UPDATE: &'static str = "/open-apis/sheets/v2/spreadsheets/{spreadsheet_token}/styles_batch_update";

    /// /open-apis/sheets/v2/spreadsheets/{spreadsheet_token}/unmerge_cells
    pub const SHEETS_V2_SPREADSHEETS_SPREADSHEET_TOKEN_UNMERGE_CELLS: &'static str = "/open-apis/sheets/v2/spreadsheets/{spreadsheet_token}/unmerge_cells";

    /// /open-apis/sheets/v2/spreadsheets/{spreadsheet_token}/values
    pub const SHEETS_V2_SPREADSHEETS_SPREADSHEET_TOKEN_VALUES: &'static str = "/open-apis/sheets/v2/spreadsheets/{spreadsheet_token}/values";

    /// /open-apis/sheets/v2/spreadsheets/{spreadsheet_token}/values/{range}
    pub const SHEETS_V2_SPREADSHEETS_SPREADSHEET_TOKEN_VALUES_RANGE: &'static str = "/open-apis/sheets/v2/spreadsheets/{spreadsheet_token}/values/{range}";

    /// /open-apis/sheets/v2/spreadsheets/{spreadsheet_token}/values_append
    pub const SHEETS_V2_SPREADSHEETS_SPREADSHEET_TOKEN_VALUES_APPEND: &'static str = "/open-apis/sheets/v2/spreadsheets/{spreadsheet_token}/values_append";

    /// /open-apis/sheets/v2/spreadsheets/{spreadsheet_token}/values_batch_get
    pub const SHEETS_V2_SPREADSHEETS_SPREADSHEET_TOKEN_VALUES_BATCH_GET: &'static str = "/open-apis/sheets/v2/spreadsheets/{spreadsheet_token}/values_batch_get";

    /// /open-apis/sheets/v2/spreadsheets/{spreadsheet_token}/values_batch_update
    pub const SHEETS_V2_SPREADSHEETS_SPREADSHEET_TOKEN_VALUES_BATCH_UPDATE: &'static str = "/open-apis/sheets/v2/spreadsheets/{spreadsheet_token}/values_batch_update";

    /// /open-apis/sheets/v2/spreadsheets/{spreadsheet_token}/values_image
    pub const SHEETS_V2_SPREADSHEETS_SPREADSHEET_TOKEN_VALUES_IMAGE: &'static str = "/open-apis/sheets/v2/spreadsheets/{spreadsheet_token}/values_image";

    /// /open-apis/sheets/v2/spreadsheets/{spreadsheet_token}/values_prepend
    pub const SHEETS_V2_SPREADSHEETS_SPREADSHEET_TOKEN_VALUES_PREPEND: &'static str = "/open-apis/sheets/v2/spreadsheets/{spreadsheet_token}/values_prepend";

    /// /open-apis/sheets/v3/spreadsheets
    pub const SHEETS_V3_SPREADSHEETS: &'static str = "/open-apis/sheets/v3/spreadsheets";

    /// /open-apis/sheets/v3/spreadsheets/{spreadsheet_token}/sheets/query
    pub const SHEETS_V3_SPREADSHEETS_SPREADSHEET_TOKEN_SHEETS_QUERY: &'static str = "/open-apis/sheets/v3/spreadsheets/{spreadsheet_token}/sheets/query";

    /// /open-apis/sheets/v3/spreadsheets/{spreadsheet_token}/sheets/{sheet_id}
    pub const SHEETS_V3_SPREADSHEETS_SPREADSHEET_TOKEN_SHEETS_SHEET_ID: &'static str = "/open-apis/sheets/v3/spreadsheets/{spreadsheet_token}/sheets/{sheet_id}";

    /// /open-apis/sheets/v3/spreadsheets/{spreadsheet_token}/sheets/{sheet_id}/filter
    pub const SHEETS_V3_SPREADSHEETS_SPREADSHEET_TOKEN_SHEETS_SHEET_ID_FILTER: &'static str = "/open-apis/sheets/v3/spreadsheets/{spreadsheet_token}/sheets/{sheet_id}/filter";

    /// /open-apis/sheets/v3/spreadsheets/{spreadsheet_token}/sheets/{sheet_id}/find
    pub const SHEETS_V3_SPREADSHEETS_SPREADSHEET_TOKEN_SHEETS_SHEET_ID_FIND: &'static str = "/open-apis/sheets/v3/spreadsheets/{spreadsheet_token}/sheets/{sheet_id}/find";

    /// /open-apis/sheets/v3/spreadsheets/{spreadsheet_token}/sheets/{sheet_id}/move_dimension
    pub const SHEETS_V3_SPREADSHEETS_SPREADSHEET_TOKEN_SHEETS_SHEET_ID_MOVE_DIMENSION: &'static str = "/open-apis/sheets/v3/spreadsheets/{spreadsheet_token}/sheets/{sheet_id}/move_dimension";

    /// /open-apis/sheets/v3/spreadsheets/{spreadsheet_token}/sheets/{sheet_id}/replace
    pub const SHEETS_V3_SPREADSHEETS_SPREADSHEET_TOKEN_SHEETS_SHEET_ID_REPLACE: &'static str = "/open-apis/sheets/v3/spreadsheets/{spreadsheet_token}/sheets/{sheet_id}/replace";

    /// /open-apis/sheets/v3/spreadsheets/{}
    pub const SHEETS_V3_SPREADSHEETS_ID: &'static str = "/open-apis/sheets/v3/spreadsheets/{}";

    /// /open-apis/sheets/v3/spreadsheets/{}/protect_range
    pub const SHEETS_V3_SPREADSHEETS_ID_PROTECT_RANGE: &'static str = "/open-apis/sheets/v3/spreadsheets/{}/protect_range";

    /// /open-apis/sheets/v3/spreadsheets/{}/protect_range/{}
    pub const SHEETS_V3_SPREADSHEETS_ID_PROTECT_RANGE_ID: &'static str = "/open-apis/sheets/v3/spreadsheets/{}/protect_range/{}";

    /// /open-apis/sheets/v3/spreadsheets/{}/sheets/{}/conditionFormat
    pub const SHEETS_V3_SPREADSHEETS_ID_SHEETS_ID_CONDITIONFORMAT: &'static str = "/open-apis/sheets/v3/spreadsheets/{}/sheets/{}/conditionFormat";

    /// /open-apis/sheets/v3/spreadsheets/{}/sheets/{}/dataValidation
    pub const SHEETS_V3_SPREADSHEETS_ID_SHEETS_ID_DATAVALIDATION: &'static str = "/open-apis/sheets/v3/spreadsheets/{}/sheets/{}/dataValidation";

    /// /open-apis/sheets/v3/spreadsheets/{}/sheets/{}/dataValidation/{}
    pub const SHEETS_V3_SPREADSHEETS_ID_SHEETS_ID_DATAVALIDATION_ID: &'static str = "/open-apis/sheets/v3/spreadsheets/{}/sheets/{}/dataValidation/{}";

    /// /open-apis/sheets/v3/spreadsheets/{}/sheets/{}/dimension_range
    pub const SHEETS_V3_SPREADSHEETS_ID_SHEETS_ID_DIMENSION_RANGE: &'static str = "/open-apis/sheets/v3/spreadsheets/{}/sheets/{}/dimension_range";

    /// /open-apis/sheets/v3/spreadsheets/{}/sheets/{}/dimension_range:insert
    pub const SHEETS_V3_SPREADSHEETS_ID_SHEETS_ID_DIMENSION_RANGE:INSERT: &'static str = "/open-apis/sheets/v3/spreadsheets/{}/sheets/{}/dimension_range:insert";

    /// /open-apis/sheets/v3/spreadsheets/{}/sheets/{}/filter_views
    pub const SHEETS_V3_SPREADSHEETS_ID_SHEETS_ID_FILTER_VIEWS: &'static str = "/open-apis/sheets/v3/spreadsheets/{}/sheets/{}/filter_views";

    /// /open-apis/sheets/v3/spreadsheets/{}/sheets/{}/filter_views/{}
    pub const SHEETS_V3_SPREADSHEETS_ID_SHEETS_ID_FILTER_VIEWS_ID: &'static str = "/open-apis/sheets/v3/spreadsheets/{}/sheets/{}/filter_views/{}";

    /// /open-apis/sheets/v3/spreadsheets/{}/sheets/{}/filter_views/{}/conditions
    pub const SHEETS_V3_SPREADSHEETS_ID_SHEETS_ID_FILTER_VIEWS_ID_CONDITIONS: &'static str = "/open-apis/sheets/v3/spreadsheets/{}/sheets/{}/filter_views/{}/conditions";

    /// /open-apis/sheets/v3/spreadsheets/{}/sheets/{}/filter_views/{}/conditions/{}
    pub const SHEETS_V3_SPREADSHEETS_ID_SHEETS_ID_FILTER_VIEWS_ID_CONDITIONS_ID: &'static str = "/open-apis/sheets/v3/spreadsheets/{}/sheets/{}/filter_views/{}/conditions/{}";

    /// /open-apis/sheets/v3/spreadsheets/{}/sheets/{}/float_images
    pub const SHEETS_V3_SPREADSHEETS_ID_SHEETS_ID_FLOAT_IMAGES: &'static str = "/open-apis/sheets/v3/spreadsheets/{}/sheets/{}/float_images";

    /// /open-apis/sheets/v3/spreadsheets/{}/sheets/{}/float_images/{}
    pub const SHEETS_V3_SPREADSHEETS_ID_SHEETS_ID_FLOAT_IMAGES_ID: &'static str = "/open-apis/sheets/v3/spreadsheets/{}/sheets/{}/float_images/{}";

    /// /open-apis/sheets/v3/spreadsheets/{}/sheets/{}/merge_cells
    pub const SHEETS_V3_SPREADSHEETS_ID_SHEETS_ID_MERGE_CELLS: &'static str = "/open-apis/sheets/v3/spreadsheets/{}/sheets/{}/merge_cells";

    /// /open-apis/sheets/v3/spreadsheets/{}/sheets/{}/style
    pub const SHEETS_V3_SPREADSHEETS_ID_SHEETS_ID_STYLE: &'static str = "/open-apis/sheets/v3/spreadsheets/{}/sheets/{}/style";

    /// /open-apis/sheets/v3/spreadsheets/{}/sheets/{}/styles/batch_update
    pub const SHEETS_V3_SPREADSHEETS_ID_SHEETS_ID_STYLES_BATCH_UPDATE: &'static str = "/open-apis/sheets/v3/spreadsheets/{}/sheets/{}/styles/batch_update";

    /// /open-apis/sheets/v3/spreadsheets/{}/sheets/{}/unmerge_cells
    pub const SHEETS_V3_SPREADSHEETS_ID_SHEETS_ID_UNMERGE_CELLS: &'static str = "/open-apis/sheets/v3/spreadsheets/{}/sheets/{}/unmerge_cells";

    /// /open-apis/sheets/v3/spreadsheets/{}/values/batch_get
    pub const SHEETS_V3_SPREADSHEETS_ID_VALUES_BATCH_GET: &'static str = "/open-apis/sheets/v3/spreadsheets/{}/values/batch_get";

    /// /open-apis/sheets/v3/spreadsheets/{}/values/batch_update
    pub const SHEETS_V3_SPREADSHEETS_ID_VALUES_BATCH_UPDATE: &'static str = "/open-apis/sheets/v3/spreadsheets/{}/values/batch_update";

    /// /open-apis/sheets/v3/spreadsheets/{}/values/{}
    pub const SHEETS_V3_SPREADSHEETS_ID_VALUES_ID: &'static str = "/open-apis/sheets/v3/spreadsheets/{}/values/{}";

    /// /open-apis/sheets/v3/spreadsheets/{}/values/{}/append
    pub const SHEETS_V3_SPREADSHEETS_ID_VALUES_ID_APPEND: &'static str = "/open-apis/sheets/v3/spreadsheets/{}/values/{}/append";

    /// /open-apis/sheets/v3/spreadsheets/{}/values/{}/prepend
    pub const SHEETS_V3_SPREADSHEETS_ID_VALUES_ID_PREPEND: &'static str = "/open-apis/sheets/v3/spreadsheets/{}/values/{}/prepend";

    /// /open-apis/sheets/v3/spreadsheets/{}/values_image
    pub const SHEETS_V3_SPREADSHEETS_ID_VALUES_IMAGE: &'static str = "/open-apis/sheets/v3/spreadsheets/{}/values_image";


    // ==================== Whiteboard 服务端点 ====================

    /// /open-apis/whiteboard/v1/whiteboards/{}/thumbnail
    pub const WHITEBOARD_V1_WHITEBOARDS_ID_THUMBNAIL: &'static str = "/open-apis/whiteboard/v1/whiteboards/{}/thumbnail";


    // ==================== Wiki 服务端点 ====================

    /// /open-apis/wiki/v2/search
    pub const WIKI_V2_SEARCH: &'static str = "/open-apis/wiki/v2/search";

    /// /open-apis/wiki/v2/spaces
    pub const WIKI_V2_SPACES: &'static str = "/open-apis/wiki/v2/spaces";

    /// /open-apis/wiki/v2/spaces/{}
    pub const WIKI_V2_SPACES_ID: &'static str = "/open-apis/wiki/v2/spaces/{}";

    /// /open-apis/wiki/v2/spaces/{}/members
    pub const WIKI_V2_SPACES_ID_MEMBERS: &'static str = "/open-apis/wiki/v2/spaces/{}/members";

    /// /open-apis/wiki/v2/spaces/{}/members/{}
    pub const WIKI_V2_SPACES_ID_MEMBERS_ID: &'static str = "/open-apis/wiki/v2/spaces/{}/members/{}";

    /// /open-apis/wiki/v2/spaces/{}/nodes
    pub const WIKI_V2_SPACES_ID_NODES: &'static str = "/open-apis/wiki/v2/spaces/{}/nodes";

    /// /open-apis/wiki/v2/spaces/{}/nodes/{}
    pub const WIKI_V2_SPACES_ID_NODES_ID: &'static str = "/open-apis/wiki/v2/spaces/{}/nodes/{}";

    /// /open-apis/wiki/v2/spaces/{}/nodes/{}/copy
    pub const WIKI_V2_SPACES_ID_NODES_ID_COPY: &'static str = "/open-apis/wiki/v2/spaces/{}/nodes/{}/copy";

    /// /open-apis/wiki/v2/spaces/{}/nodes/{}/move
    pub const WIKI_V2_SPACES_ID_NODES_ID_MOVE: &'static str = "/open-apis/wiki/v2/spaces/{}/nodes/{}/move";

    /// /open-apis/wiki/v2/spaces/{}/nodes/{}/update_title
    pub const WIKI_V2_SPACES_ID_NODES_ID_UPDATE_TITLE: &'static str = "/open-apis/wiki/v2/spaces/{}/nodes/{}/update_title";

    /// /open-apis/wiki/v2/spaces/{}/setting
    pub const WIKI_V2_SPACES_ID_SETTING: &'static str = "/open-apis/wiki/v2/spaces/{}/setting";

    /// /open-apis/wiki/v2/tasks/move_docs_to_wiki
    pub const WIKI_V2_TASKS_MOVE_DOCS_TO_WIKI: &'static str = "/open-apis/wiki/v2/tasks/move_docs_to_wiki";

    /// /open-apis/wiki/v2/tasks/{}
    pub const WIKI_V2_TASKS_ID: &'static str = "/open-apis/wiki/v2/tasks/{}";

}

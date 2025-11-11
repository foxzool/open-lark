//! 内容管理服务端点

/// 内容管理服务端点
pub struct Content;

impl Content {
    /// / 下载文件 (需要使用 EndpointBuilder::replace_param 替换 {file_token})
    pub const LINGO_FILE_UPLOAD: &'static str = "/open-apis/lingo/v1/file/upload";
    /// 知识库管理
    pub const LINGO_FILE_DOWNLOAD: &'static str = "/open-apis/lingo/v1/file/download/{file_token}";
    /// / 新建文件夹
    pub const DRIVE_V1_GET_META: &'static str = "/open-apis/drive/v1/metas/{token}";
    /// / 获取文件夹中的文件清单
    pub const DRIVE_V1_CREATE_FOLDER: &'static str = "/open-apis/drive/v1/files/create_folder";
    /// / 复制文件或文件夹
    pub const DRIVE_V1_LIST_FILES: &'static str = "/open-apis/drive/v1/files";
    /// / 移动文件或文件夹
    pub const DRIVE_V1_COPY: &'static str = "/open-apis/drive/v1/files/{file_token}/copy";
    /// / 删除文件或文件夹
    pub const DRIVE_V1_MOVE: &'static str = "/open-apis/drive/v1/files/{file_token}/move";
    /// / 获取文件下载链接
    pub const DRIVE_V1_DELETE: &'static str = "/open-apis/drive/v1/files/{file_token}";
    /// / 分片上传文件-预上传
    pub const DRIVE_V1_DOWNLOAD: &'static str = "/open-apis/drive/v1/files/{file_token}/download";
    /// / 分片上传文件-分片上传
    pub const DRIVE_V1_UPLOAD_PREPARE: &'static str = "/open-apis/drive/v1/files/upload_prepare";
    /// / 分片上传文件-完成上传
    pub const DRIVE_V1_UPLOAD_PART: &'static str = "/open-apis/drive/v1/files/upload_part";
    /// ==================== 审批服务端点 ====================
    pub const DRIVE_V1_UPLOAD_FINISH: &'static str = "/open-apis/drive/v1/files/upload_finish";
    /// ===== 实例管理端点 =====
    pub const APPROVAL_V4_FILE_UPLOAD: &'static str = "/open-apis/approval/v4/files/upload";
    /// / 流式语音识别
    pub const SPEECH_TO_TEXT_V1_FILE_RECOGNIZE: &'static str =
        "/open-apis/speech_to_text/v1/speech/file_recognize";
    /// Bitable 多维表格服务
    pub const ASSISTANT_V1_FILE_SUBSCRIPTION: &'static str =
        "/open-apis/assistant/v1/file/{}/{}/subscription";
    /// / 删除文件或文件夹
    pub const DRIVE_V1_FILE_DOWNLOAD: &'static str = "/open-apis/drive/v1/files/{}/download";
    /// / 文件版本管理
    pub const DRIVE_V1_FILES_SUBSCRIBE: &'static str = "/open-apis/drive/v1/files/subscribe";
    /// / 文件订阅管理
    pub const DRIVE_V1_FILE_VERSION_GET: &'static str = "/open-apis/drive/v1/files/{}/versions/{}";
    /// / 查询云文档事件订阅状态
    pub const DRIVE_V1_FILE_SUBSCRIPTIONS: &'static str =
        "/open-apis/drive/v1/files/{}/subscriptions/{}";
    /// / 取消云文档事件订阅
    pub const DRIVE_V1_FILE_GET_SUBSCRIBE: &'static str =
        "/open-apis/drive/v1/files/{}/get_subscribe";
    /// / 文件夹管理
    pub const DRIVE_V1_FILE_DELETE_SUBSCRIBE: &'static str =
        "/open-apis/drive/v1/files/{}/delete_subscribe";
    /// / 文件上传管理
    pub const DRIVE_V1_FOLDERS_ROOT_FOLDER_META: &'static str =
        "/open-apis/drive/v1/folders/root_folder_meta";
    /// / 媒体文件管理
    pub const DRIVE_V1_FILES_UPLOAD_FINISH: &'static str =
        "/open-apis/drive/v1/files/upload_finish";
    /// / 导入任务管理
    pub const DRIVE_V1_MEDIAS_BATCH_GET_TMP_DOWNLOAD_URL: &'static str =
        "/open-apis/drive/v1/medias/batch_get_tmp_download_url";
    /// / 导出任务管理
    pub const DRIVE_V1_IMPORT_TASK_GET: &'static str = "/open-apis/drive/v1/import_tasks/{}";
    /// / 元信息管理
    pub const DRIVE_V1_EXPORT_TASK_GET: &'static str = "/open-apis/drive/v1/export_tasks/{}";
    /// / 任务管理
    pub const DRIVE_V1_METAS_BATCH_QUERY: &'static str = "/open-apis/drive/v1/metas/batch_query";
    /// / 权限管理
    pub const DRIVE_V1_TASK_GET: &'static str = "/open-apis/drive/v1/tasks/{}";
    /// / Drive v2 权限管理
    pub const DRIVE_V1_PERMISSIONS_PUBLIC_PASSWORD: &'static str =
        "/open-apis/drive/v1/permissions/{}/public/password";
    /// / Drive Explorer
    pub const DRIVE_V2_PERMISSIONS_PUBLIC: &'static str =
        "/open-apis/drive/v2/permissions/{}/public";
    /// Sheets 电子表格服务
    pub const DRIVE_EXPLORER_V2_FOLDER_META: &'static str =
        "/open-apis/drive/explorer/v2/folder/{folder_token}/meta";
    /// / 电子表格工作表管理 - v3
    pub const SHEETS_V3_SPREADSHEET_PATCH: &'static str = "/open-apis/sheets/v3/spreadsheets/{}";
    /// / 电子表格数据操作 - v3
    pub const SHEETS_V3_SPREADSHEET_SHEET_GET: &'static str =
        "/open-apis/sheets/v3/spreadsheets/:spreadsheetToken/sheets/{sheet_id}";
    /// / 电子表格查找替换 - v3
    pub const SHEETS_V3_SPREADSHEET_VALUES_IMAGE: &'static str =
        "/open-apis/sheets/v3/spreadsheets/{}/values_image";
    /// / 电子表格合并拆分单元格 - v3
    pub const SHEETS_V3_SPREADSHEET_SHEET_REPLACE: &'static str =
        "/open-apis/sheets/v3/spreadsheets/:spreadsheetToken/sheets/{sheet_id}/replace";
    /// / 电子表格样式管理 - v3
    pub const SHEETS_V3_SPREADSHEET_UNMERGE_CELLS: &'static str =
        "/open-apis/sheets/v3/spreadsheets/{}/sheets/{}/unmerge_cells";
    /// / 电子表格行列管理 - v3
    pub const SHEETS_V3_SPREADSHEET_STYLES_BATCH_UPDATE: &'static str =
        "/open-apis/sheets/v3/spreadsheets/{}/sheets/{}/styles/batch_update";
    /// / 电子表格条件格式 - v3
    pub const SHEETS_V3_SPREADSHEET_MOVE_DIMENSION: &'static str =
        "/open-apis/sheets/v3/spreadsheets/:spreadsheetToken/sheets/{sheet_id}/move_dimension";
    /// / 电子表格数据验证 - v3
    pub const SHEETS_V3_SPREADSHEET_CONDITION_FORMAT: &'static str =
        "/open-apis/sheets/v3/spreadsheets/{}/sheets/{}/conditionFormat";
    /// / 电子表格保护范围 - v3
    pub const SHEETS_V3_SPREADSHEET_DATA_VALIDATION_GET: &'static str =
        "/open-apis/sheets/v3/spreadsheets/{}/sheets/{}/dataValidation/{}";
    /// / 电子表格筛选器 - v3
    pub const SHEETS_V3_SPREADSHEET_PROTECT_RANGE_GET: &'static str =
        "/open-apis/sheets/v3/spreadsheets/{}/protect_range/{}";
    /// / 电子表格筛选视图 - v3
    pub const SHEETS_V3_SPREADSHEET_FILTER: &'static str =
        "/open-apis/sheets/v3/spreadsheets/:spreadsheetToken/sheets/{sheet_id}/filter";
    /// / 电子表格筛选视图条件 - v3
    pub const SHEETS_V3_SPREADSHEET_FILTER_VIEW_GET: &'static str =
        "/open-apis/sheets/v3/spreadsheets/{}/sheets/{}/filter_views/{}";
    /// / 电子表格浮动图片 - v3
    pub const SHEETS_V3_SPREADSHEET_FILTER_VIEW_CONDITION_GET: &'static str =
        "/open-apis/sheets/v3/spreadsheets/{}/sheets/{}/filter_views/{}/conditions/{}";
    /// / 电子表格管理 - v2
    pub const SHEETS_V3_SPREADSHEET_FLOAT_IMAGE_GET: &'static str =
        "/open-apis/sheets/v3/spreadsheets/{}/sheets/{}/float_images/{}";
    /// / 电子表格样式管理 - v2
    pub const SHEETS_V2_SPREADSHEET_VALUES_IMAGE: &'static str =
        "/open-apis/sheets/v2/spreadsheets/:spreadsheetToken/values_image";
    /// / 电子表格合并拆分单元格 - v2
    pub const SHEETS_V2_SPREADSHEET_STYLES_BATCH_UPDATE: &'static str =
        "/open-apis/sheets/v2/spreadsheets/:spreadsheetToken/styles_batch_update";
    /// / 电子表格行列管理 - v2
    pub const SHEETS_V2_SPREADSHEET_UNMERGE_CELLS: &'static str =
        "/open-apis/sheets/v2/spreadsheets/:spreadsheetToken/unmerge_cells";
    /// / 电子表格工作表管理 - v2
    pub const SHEETS_V2_SPREADSHEET_INSERT_DIMENSION_RANGE: &'static str =
        "/open-apis/sheets/v2/spreadsheets/:spreadsheetToken/insert_dimension_range";
    /// Wiki 知识库服务
    pub const SHEETS_V2_SPREADSHEET_SHEETS_BATCH_UPDATE: &'static str =
        "/open-apis/sheets/v2/spreadsheets/:spreadsheetToken/sheets_batch_update";
    /// / 知识库空间成员管理
    pub const WIKI_V2_SPACE_CREATE: &'static str = "/open-apis/wiki/v2/spaces";
    /// / 知识库节点管理
    pub const WIKI_V2_SPACE_MEMBER_DELETE: &'static str = "/open-apis/wiki/v2/spaces/{}/members/{}";
    /// / 知识库设置管理
    pub const WIKI_V2_SPACE_NODE_UPDATE_TITLE: &'static str =
        "/open-apis/wiki/v2/spaces/{}/nodes/{}/update_title";
    /// / 知识库搜索
    pub const WIKI_V2_SPACE_SETTING_UPDATE: &'static str = "/open-apis/wiki/v2/spaces/{}/setting";
    /// / 知识库任务管理
    pub const WIKI_V2_SEARCH: &'static str = "/open-apis/wiki/v2/search";
    /// ==================== 邮件服务端点 ====================
    pub const WIKI_V2_TASK_GET: &'static str = "/open-apis/wiki/v2/tasks/{}";
}

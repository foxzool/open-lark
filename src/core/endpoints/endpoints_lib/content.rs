//! 内容管理服务端点
//!
//! 包含云盘、文件管理、媒体处理、文档等相关的API端点。

/// 内容管理相关端点
pub struct Content;

impl Content {
    // ==================== 文件基础操作 ====================

    /// 文件管理 - 基础文件端点
    pub const FILES: &'static str = "/open-apis/drive/v1/files";

    /// 获取文件详情
    pub const FILE_GET: &'static str = "/open-apis/drive/v1/files/{file_token}";

    /// 复制文件
    pub const FILE_COPY: &'static str = "/open-apis/drive/v1/files/{file_token}/copy";

    /// 下载文件
    pub const FILE_DOWNLOAD: &'static str = "/open-apis/drive/v1/files/{file_token}/download";

    /// 获取文件统计信息
    pub const FILE_STATISTICS: &'static str =
        "/open-apis/drive/v1/files/{file_token}/statistics";

    /// 获取文件查看记录
    pub const FILE_VIEW_RECORDS: &'static str =
        "/open-apis/drive/v1/files/{file_token}/view_records";

    /// 获取文件点赞记录
    pub const FILE_LIKE_RECORDS: &'static str =
        "/open-apis/drive/v1/files/{file_token}/like_records";

    /// 创建文件夹
    pub const FILES_CREATE_FOLDER: &'static str = "/open-apis/drive/v1/files/create_folder";

    /// 创建快捷方式
    pub const FILES_CREATE_SHORTCUT: &'static str = "/open-apis/drive/v1/files/create_shortcut";

    /// 搜索文件
    pub const FILES_SEARCH: &'static str = "/open-apis/drive/v1/files/search";

    /// 订阅文件
    pub const FILES_SUBSCRIBE: &'static str = "/open-apis/drive/v1/files/subscribe";

    /// 获取文件版本列表
    pub const FILE_VERSIONS: &'static str = "/open-apis/drive/v1/files/{file_token}/versions";

    /// 获取指定文件版本
    pub const FILE_VERSION_GET: &'static str =
        "/open-apis/drive/v1/files/{file_token}/versions/{version_id}";

    /// 文件订阅管理
    pub const FILE_SUBSCRIPTIONS: &'static str =
        "/open-apis/drive/v1/files/{file_token}/subscriptions/{subscription_id}";

    // ==================== 文件上传 ====================

    /// 文件上传 - 一次性上传
    pub const FILES_UPLOAD_ALL: &'static str = "/open-apis/drive/v1/files/upload_all";

    /// 文件上传 - 准备分片上传
    pub const FILES_UPLOAD_PREPARE: &'static str = "/open-apis/drive/v1/files/upload_prepare";

    /// 文件上传 - 分片上传
    pub const FILES_UPLOAD_PART: &'static str = "/open-apis/drive/v1/files/upload_part";

    /// 文件上传 - 完成分片上传
    pub const FILES_UPLOAD_FINISH: &'static str = "/open-apis/drive/v1/files/upload_finish";

    // ==================== 文件夹管理 ====================

    /// 文件夹管理 - 基础文件夹端点
    pub const FOLDERS: &'static str = "/open-apis/drive/v1/folders";

    /// 获取文件夹详情
    pub const FOLDER_GET: &'static str = "/open-apis/drive/v1/folders/{folder_token}";

    /// 获取文件夹子项
    pub const FOLDER_CHILDREN: &'static str = "/open-apis/drive/v1/folders/{folder_token}/children";

    /// 移动文件夹
    pub const FOLDER_MOVE: &'static str = "/open-apis/drive/v1/folders/{folder_token}/move";

    /// 获取根文件夹元数据
    pub const FOLDERS_ROOT_FOLDER_META: &'static str = "/open-apis/drive/v1/folders/root_folder_meta";

    // ==================== 媒体文件管理 ====================

    /// 媒体管理 - 一次性上传媒体文件
    pub const MEDIAS_UPLOAD_ALL: &'static str = "/open-apis/drive/v1/medias/upload_all";

    /// 媒体管理 - 准备媒体文件分片上传
    pub const MEDIAS_UPLOAD_PREPARE: &'static str = "/open-apis/drive/v1/medias/upload_prepare";

    /// 媒体管理 - 媒体文件分片上传
    pub const MEDIAS_UPLOAD_PART: &'static str = "/open-apis/drive/v1/medias/upload_part";

    /// 媒体管理 - 完成媒体文件分片上传
    pub const MEDIAS_UPLOAD_FINISH: &'static str = "/open-apis/drive/v1/medias/upload_finish";

    /// 媒体管理 - 下载媒体文件
    pub const MEDIAS_DOWNLOAD: &'static str = "/open-apis/drive/v1/medias/{media_token}/download";

    /// 媒体管理 - 批量获取临时下载链接
    pub const MEDIAS_BATCH_GET_TMP_DOWNLOAD_URL: &'static str =
        "/open-apis/drive/v1/medias/batch_get_tmp_download_url";

    // ==================== 导入任务管理 ====================

    /// 导入任务 - 创建导入任务
    pub const IMPORT_TASKS: &'static str = "/open-apis/drive/v1/import_tasks";

    /// 导入任务 - 获取导入任务状态
    pub const IMPORT_TASK_GET: &'static str = "/open-apis/drive/v1/import_tasks/{task_id}";

    // ==================== 元数据与任务 ====================

    /// 元数据 - 批量查询文件元数据
    pub const METAS_BATCH_QUERY: &'static str = "/open-apis/drive/v1/metas/batch_query";

    /// 任务管理 - 获取任务状态
    pub const TASK_GET: &'static str = "/open-apis/drive/v1/tasks/{task_id}";

    // ==================== 权限管理 v1 ====================

    /// 权限管理 - 文件成员权限
    pub const PERMISSIONS_MEMBERS: &'static str = "/open-apis/drive/v1/permissions/{token}/members";

    /// 权限管理 - 获取指定成员权限
    pub const PERMISSIONS_MEMBER_GET: &'static str =
        "/open-apis/drive/v1/permissions/{token}/members/{member_id}";

    /// 权限管理 - 批量创建成员权限
    pub const PERMISSIONS_MEMBERS_BATCH_CREATE: &'static str =
        "/open-apis/drive/v1/permissions/{token}/members/batch_create";

    /// 权限管理 - 成员权限认证
    pub const PERMISSIONS_MEMBERS_AUTH: &'static str =
        "/open-apis/drive/v1/permissions/{token}/members/auth";

    /// 权限管理 - 转移所有者
    pub const PERMISSIONS_MEMBERS_TRANSFER_OWNER: &'static str =
        "/open-apis/drive/v1/permissions/{token}/members/transfer_owner";

    /// 权限管理 - 公共链接权限 v1
    pub const PERMISSIONS_PUBLIC: &'static str = "/open-apis/drive/v1/permissions/{token}/public";

    /// 权限管理 - 公共链接密码
    pub const PERMISSIONS_PUBLIC_PASSWORD: &'static str =
        "/open-apis/drive/v1/permissions/{token}/public/password";

    // ==================== 权限管理 v2 ====================

    /// 权限管理 - 公共链接权限 v2
    pub const V2_PERMISSIONS_PUBLIC: &'static str = "/open-apis/drive/v2/permissions/{token}/public";

    // ==================== 文件浏览器 v2 ====================

    /// 文件浏览器 - 获取根文件夹元数据
    pub const EXPLORER_V2_ROOT_FOLDER_META: &'static str =
        "/open-apis/drive/explorer/v2/root_folder/meta";

    /// 文件浏览器 - 获取文件夹元数据
    pub const EXPLORER_V2_FOLDER_META: &'static str =
        "/open-apis/drive/explorer/v2/folder/{folder_token}/meta";

    // ==================== 云文档端点 ====================

    /// 云文档基础操作
    pub const CLOUD_DOCS_BASE: &'static str = "/open-apis/cloud-docs";

    /// 获取文档信息
    pub const DOC_INFO: &'static str = "/open-apis/cloud-docs/docs/{doc_id}";

    /// 文档协作
    pub const DOC_COLLABORATION: &'static str = "/open-apis/cloud-docs/docs/{doc_id}/collaboration";

    /// 文档评论
    pub const DOC_COMMENTS: &'static str = "/open-apis/cloud-docs/docs/{doc_id}/comments";

    /// 文档历史版本
    pub const DOC_HISTORY: &'static str = "/open-apis/cloud-docs/docs/{doc_id}/history";
}

// 向后兼容性别名
#[allow(dead_code)]
pub mod legacy {
    pub const DRIVE_V1_FILES: &str = Content::FILES;
    pub const DRIVE_V1_FILE_GET: &str = Content::FILE_GET;
    pub const DRIVE_V1_FILE_COPY: &str = Content::FILE_COPY;
    pub const DRIVE_V1_FILE_DOWNLOAD: &str = Content::FILE_DOWNLOAD;
    pub const DRIVE_V1_FILES_CREATE_FOLDER: &str = Content::FILES_CREATE_FOLDER;
    pub const DRIVE_V1_FILES_SEARCH: &str = Content::FILES_SEARCH;
    pub const DRIVE_V1_FOLDERS: &str = Content::FOLDERS;
    pub const DRIVE_V1_FOLDER_GET: &str = Content::FOLDER_GET;
    pub const DRIVE_V1_UPLOAD_PREPARE: &str = Content::FILES_UPLOAD_PREPARE;
    pub const DRIVE_V1_UPLOAD_PART: &str = Content::FILES_UPLOAD_PART;
    pub const DRIVE_V1_UPLOAD_FINISH: &str = Content::FILES_UPLOAD_FINISH;
}
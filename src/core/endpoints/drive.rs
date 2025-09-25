//! drive 服务端点常量定义
//!
//! Drive (云盘) 相关 API 端点常量，包括：
//! - 文件管理 (v1)
//! - 文件夹管理 (v1)
//! - 媒体文件处理 (v1)
//! - 权限管理 (v1/v2)
//! - 导入任务管理
//! - 文件浏览器 (v2)

// ==================== 文件基础操作 ====================

/// 文件管理 - 基础文件端点
pub const DRIVE_V1_FILES: &str = "/open-apis/drive/v1/files";

/// 文件管理 - 获取文件详情
pub const DRIVE_V1_FILE_GET: &str = "/open-apis/drive/v1/files/{}";

/// 文件管理 - 复制文件
pub const DRIVE_V1_FILE_COPY: &str = "/open-apis/drive/v1/files/{}/copy";

/// 文件管理 - 下载文件
pub const DRIVE_V1_FILE_DOWNLOAD: &str = "/open-apis/drive/v1/files/{}/download";

/// 文件管理 - 获取文件统计信息
pub const DRIVE_V1_FILE_STATISTICS: &str = "/open-apis/drive/v1/files/{}/statistics";

/// 文件管理 - 获取文件查看记录
pub const DRIVE_V1_FILE_VIEW_RECORDS: &str = "/open-apis/drive/v1/files/{}/view_records";

/// 文件管理 - 获取文件点赞记录
pub const DRIVE_V1_FILE_LIKE_RECORDS: &str = "/open-apis/drive/v1/files/{}/like_records";

/// 文件管理 - 创建文件夹
pub const DRIVE_V1_FILES_CREATE_FOLDER: &str = "/open-apis/drive/v1/files/create_folder";

/// 文件管理 - 创建快捷方式
pub const DRIVE_V1_FILES_CREATE_SHORTCUT: &str = "/open-apis/drive/v1/files/create_shortcut";

/// 文件管理 - 搜索文件
pub const DRIVE_V1_FILES_SEARCH: &str = "/open-apis/drive/v1/files/search";

/// 文件管理 - 订阅文件
pub const DRIVE_V1_FILES_SUBSCRIBE: &str = "/open-apis/drive/v1/files/subscribe";

/// 文件管理 - 获取文件版本列表
pub const DRIVE_V1_FILE_VERSIONS: &str = "/open-apis/drive/v1/files/{}/versions";

/// 文件管理 - 获取指定文件版本
pub const DRIVE_V1_FILE_VERSION_GET: &str = "/open-apis/drive/v1/files/{}/versions/{}";

/// 文件管理 - 文件订阅管理
pub const DRIVE_V1_FILE_SUBSCRIPTIONS: &str = "/open-apis/drive/v1/files/{}/subscriptions/{}";

// ==================== 文件上传 ====================

/// 文件上传 - 一次性上传
pub const DRIVE_V1_FILES_UPLOAD_ALL: &str = "/open-apis/drive/v1/files/upload_all";

/// 文件上传 - 准备分片上传
pub const DRIVE_V1_FILES_UPLOAD_PREPARE: &str = "/open-apis/drive/v1/files/upload_prepare";

/// 文件上传 - 分片上传
pub const DRIVE_V1_FILES_UPLOAD_PART: &str = "/open-apis/drive/v1/files/upload_part";

/// 文件上传 - 完成分片上传
pub const DRIVE_V1_FILES_UPLOAD_FINISH: &str = "/open-apis/drive/v1/files/upload_finish";

// ==================== 文件夹管理 ====================

/// 文件夹管理 - 基础文件夹端点
pub const DRIVE_V1_FOLDERS: &str = "/open-apis/drive/v1/folders";

/// 文件夹管理 - 获取文件夹详情
pub const DRIVE_V1_FOLDER_GET: &str = "/open-apis/drive/v1/folders/{}";

/// 文件夹管理 - 获取文件夹子项
pub const DRIVE_V1_FOLDER_CHILDREN: &str = "/open-apis/drive/v1/folders/{}/children";

/// 文件夹管理 - 移动文件夹
pub const DRIVE_V1_FOLDER_MOVE: &str = "/open-apis/drive/v1/folders/{}/move";

/// 文件夹管理 - 获取根文件夹元数据
pub const DRIVE_V1_FOLDERS_ROOT_FOLDER_META: &str = "/open-apis/drive/v1/folders/root_folder_meta";

// ==================== 媒体文件管理 ====================

/// 媒体管理 - 一次性上传媒体文件
pub const DRIVE_V1_MEDIAS_UPLOAD_ALL: &str = "/open-apis/drive/v1/medias/upload_all";

/// 媒体管理 - 准备媒体文件分片上传
pub const DRIVE_V1_MEDIAS_UPLOAD_PREPARE: &str = "/open-apis/drive/v1/medias/upload_prepare";

/// 媒体管理 - 媒体文件分片上传
pub const DRIVE_V1_MEDIAS_UPLOAD_PART: &str = "/open-apis/drive/v1/medias/upload_part";

/// 媒体管理 - 完成媒体文件分片上传
pub const DRIVE_V1_MEDIAS_UPLOAD_FINISH: &str = "/open-apis/drive/v1/medias/upload_finish";

/// 媒体管理 - 下载媒体文件
pub const DRIVE_V1_MEDIAS_DOWNLOAD: &str = "/open-apis/drive/v1/medias/{}/download";

/// 媒体管理 - 批量获取临时下载链接
pub const DRIVE_V1_MEDIAS_BATCH_GET_TMP_DOWNLOAD_URL: &str =
    "/open-apis/drive/v1/medias/batch_get_tmp_download_url";

// ==================== 导入任务管理 ====================

/// 导入任务 - 创建导入任务
pub const DRIVE_V1_IMPORT_TASKS: &str = "/open-apis/drive/v1/import_tasks";

/// 导入任务 - 获取导入任务状态
pub const DRIVE_V1_IMPORT_TASK_GET: &str = "/open-apis/drive/v1/import_tasks/{}";

// ==================== 元数据与任务 ====================

/// 元数据 - 批量查询文件元数据
pub const DRIVE_V1_METAS_BATCH_QUERY: &str = "/open-apis/drive/v1/metas/batch_query";

/// 任务管理 - 获取任务状态
pub const DRIVE_V1_TASK_GET: &str = "/open-apis/drive/v1/tasks/{}";

// ==================== 权限管理 v1 ====================

/// 权限管理 - 文件成员权限
pub const DRIVE_V1_PERMISSIONS_MEMBERS: &str = "/open-apis/drive/v1/permissions/{}/members";

/// 权限管理 - 获取指定成员权限
pub const DRIVE_V1_PERMISSIONS_MEMBER_GET: &str = "/open-apis/drive/v1/permissions/{}/members/{}";

/// 权限管理 - 批量创建成员权限
pub const DRIVE_V1_PERMISSIONS_MEMBERS_BATCH_CREATE: &str =
    "/open-apis/drive/v1/permissions/{}/members/batch_create";

/// 权限管理 - 成员权限认证
pub const DRIVE_V1_PERMISSIONS_MEMBERS_AUTH: &str =
    "/open-apis/drive/v1/permissions/{}/members/auth";

/// 权限管理 - 转移所有者
pub const DRIVE_V1_PERMISSIONS_MEMBERS_TRANSFER_OWNER: &str =
    "/open-apis/drive/v1/permissions/{}/members/transfer_owner";

/// 权限管理 - 公共链接权限 v1
pub const DRIVE_V1_PERMISSIONS_PUBLIC: &str = "/open-apis/drive/v1/permissions/{}/public";

/// 权限管理 - 公共链接密码
pub const DRIVE_V1_PERMISSIONS_PUBLIC_PASSWORD: &str =
    "/open-apis/drive/v1/permissions/{}/public/password";

// ==================== 权限管理 v2 ====================

/// 权限管理 - 公共链接权限 v2
pub const DRIVE_V2_PERMISSIONS_PUBLIC: &str = "/open-apis/drive/v2/permissions/{}/public";

// ==================== 文件浏览器 v2 ====================

/// 文件浏览器 - 获取根文件夹元数据
pub const DRIVE_EXPLORER_V2_ROOT_FOLDER_META: &str =
    "/open-apis/drive/explorer/v2/root_folder/meta";

/// 文件浏览器 - 获取文件夹元数据
pub const DRIVE_EXPLORER_V2_FOLDER_META: &str =
    "/open-apis/drive/explorer/v2/folder/{folder_token}/meta";

// ==================== 兼容性别名 ====================

/// 兼容性别名 - 获取元数据
pub const DRIVE_V1_GET_META: &str = "/open-apis/drive/v1/metas/{token}";

/// 兼容性别名 - 创建文件夹
pub const DRIVE_V1_CREATE_FOLDER: &str = "/open-apis/drive/v1/files/create_folder";

/// 兼容性别名 - 列出文件
pub const DRIVE_V1_LIST_FILES: &str = "/open-apis/drive/v1/files";

/// 兼容性别名 - 复制文件
pub const DRIVE_V1_COPY: &str = "/open-apis/drive/v1/files/{file_token}/copy";

/// 兼容性别名 - 移动文件
pub const DRIVE_V1_MOVE: &str = "/open-apis/drive/v1/files/{file_token}/move";

/// 兼容性别名 - 删除文件
pub const DRIVE_V1_DELETE: &str = "/open-apis/drive/v1/files/{file_token}";

/// 兼容性别名 - 下载文件
pub const DRIVE_V1_DOWNLOAD: &str = "/open-apis/drive/v1/files/{file_token}/download";

/// 兼容性别名 - 准备上传
pub const DRIVE_V1_UPLOAD_PREPARE: &str = "/open-apis/drive/v1/files/upload_prepare";

/// 兼容性别名 - 分片上传
pub const DRIVE_V1_UPLOAD_PART: &str = "/open-apis/drive/v1/files/upload_part";

/// 兼容性别名 - 完成上传
pub const DRIVE_V1_UPLOAD_FINISH: &str = "/open-apis/drive/v1/files/upload_finish";

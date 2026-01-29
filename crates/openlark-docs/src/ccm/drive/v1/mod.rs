/// Drive v1 API 模块
///
/// 提供云空间文件管理相关的API功能,包括:
/// - 文件操作:列表、创建、删除、移动、复制等
/// - 文件元数据:批量查询文件元数据
/// - 文件上传下载:小文件上传、分片上传、素材上传下载等
/// - 权限管理:协作者权限、公开权限、密码保护等
/// - 导入导出:文件导入导出任务管理
/// - 版本管理:文档版本创建、查询、删除
/// - 评论管理:评论和回复的增删改查
/// - 媒体管理:媒体上传任务、分享链接等
/// - 统计分析:文件统计、查看记录等
/// - 密码保护:文件密码的增删改查
/// - 文件搜索:文件搜索功能
pub mod export_task;
pub mod file;
pub mod import_task;
pub mod media;
pub mod meta;
pub mod permission;

// 显式导出 - 避免使用 glob reexport
pub use export_task::{
    CreateExportTaskRequest, CreateExportTaskResponse, DownloadExportRequest, ExportTaskResult,
    GetExportTaskRequest, GetExportTaskResponse,
};

pub use import_task::{
    CreateImportTaskRequest, CreateImportTaskResponse, GetImportTaskRequest, GetImportTaskResponse,
    ImportTaskResult,
};

pub use media::{
    BatchGetTmpDownloadUrlRequest, BatchGetTmpDownloadUrlResponse, DownloadMediaRequest,
    TmpDownloadUrlInfo, UploadAllMediaRequest, UploadAllMediaResponse, UploadFinishMediaRequest,
    UploadFinishMediaResponse, UploadPartMediaRequest, UploadPartMediaResponse,
    UploadPrepareMediaRequest, UploadPrepareMediaResponse,
};

pub use meta::BatchQueryMetaRequest;

// 注意: permission 模块已使用显式导出

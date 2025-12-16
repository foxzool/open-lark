/// Drive v1 API 模块
///
/// 提供云空间文件管理相关的API功能，包括：
/// - 文件操作：列表、创建、删除、移动、复制等
/// - 文件元数据：批量查询文件元数据
/// - 文件上传下载：小文件上传、分片上传、素材上传下载等
/// - 权限管理：协作者权限、公开权限、密码保护等
/// - 导入导出：文件导入导出任务管理
/// - 版本管理：文档版本创建、查询、删除
/// - 评论管理：评论和回复的增删改查
/// - 媒体管理：媒体上传任务、分享链接等
/// - 统计分析：文件统计、查看记录等
/// - 密码保护：文件密码的增删改查
/// - 文件搜索：文件搜索功能

pub mod file;
pub mod meta;
// pub mod upload;
pub mod media;
pub mod permission;
pub mod import_task;
pub mod export_task;
pub mod file_version;
// pub mod task_check;
pub mod password;
pub mod search;

// 重新导出所有模块
pub use file::*;
pub use meta::*;
// pub use upload::*;
pub use media::*;
pub use permission::*;
pub use import_task::*;
pub use export_task::*;
pub use file_version::*;
pub use comment::*;
// pub use task_check::*;
pub use password::*;
pub use search::*;
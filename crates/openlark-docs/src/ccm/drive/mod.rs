/// 云盘权限模型模块。
pub mod permission;
/// 云盘 v1 接口模块。
pub mod v1;
/// 云盘 v2 接口模块。
pub mod v2;

/// 重新导出云盘权限模型。
pub use permission::models;

/// 重新导出 v1 导出任务接口。
pub use v1::export_task;
/// 重新导出 v1 文件接口。
pub use v1::file;
/// 重新导出 v1 导入任务接口。
pub use v1::import_task;
/// 重新导出 v1 媒体接口。
pub use v1::media;
/// 重新导出 v1 元数据接口。
pub use v1::meta;
/// 重新导出 v1 权限接口。
pub use v1::permission as v1_permission;

/// 重新导出 v2 文件接口。
pub use v2::file as drive_v2_file;
/// 重新导出 v2 权限接口。
pub use v2::permission as v2_permission;

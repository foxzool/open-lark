/// Drive v2 API 模块
///
/// 提供云空间文件管理的增强功能
pub mod file;
pub mod permission;

// 使用通配符导出所有子模块
pub use file::*;
pub use permission::*;

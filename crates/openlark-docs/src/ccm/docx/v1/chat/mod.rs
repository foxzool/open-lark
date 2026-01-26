/// 聊天公告模块
///
/// 提供聊天公告的管理功能，包括公告创建、获取、区块管理等。
pub mod announcement;

// 使用通配符导出所有子模块
pub use announcement::*;

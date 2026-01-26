/// Wiki 知识库服务模块
///
/// 提供企业知识库和Wiki管理功能。
pub mod v1;
pub mod v2;

// 使用通配符导出所有子模块,避免维护大量重复的导出列表
pub use v1::*;
pub use v2::*;

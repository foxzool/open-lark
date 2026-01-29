/// ccm/docx模块 - 文档块内容管理
///
/// 按照bizTag/project/version/resource/name.rs模式组织
/// 包含chat公告和document操作的相关API
/// 数据模型定义（模型，不算 API）
pub mod documents;
pub mod models;
pub mod v1;

// 使用通配符导出所有子模块
pub use models::*;
pub use v1::*;

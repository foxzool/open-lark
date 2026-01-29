/// Docs文档服务
///
/// 提供飞书文档的完整管理功能，包括：
/// - 创建和删除文档
/// - 读取和更新文档内容
/// - 文档块操作和内容管理
/// - 文档版本控制和历史管理
/// - 协作编辑和权限管理
// pub mod v1;  // 暂时禁用，待修复
pub mod v2;

// 重新导出所有服务类型，避免名称冲突
// pub use v1::{DocsServiceV1, DocumentService};

pub use v2::{CreateDocBuilder, CreateDocService};

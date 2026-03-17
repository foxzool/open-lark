/// 云文档 v1 API 模块
///
/// 提供云文档内容获取相关的API功能。
pub mod content;
pub mod models;

// 使用通配符导出所有子模块
// content 模块显式导出
pub use content::{GetDocsContentRequest, GetDocsContentResponse};
// models 模块显式导出
pub use models::DocsContent;

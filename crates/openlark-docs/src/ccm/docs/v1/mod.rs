/// 云文档内容接口模块。
pub mod content;
/// 云文档内容模型模块。
pub mod models;

/// 重新导出云文档内容请求类型。
pub use content::{GetDocsContentRequest, GetDocsContentResponse};
/// 重新导出云文档内容模型。
pub use models::DocsContent;

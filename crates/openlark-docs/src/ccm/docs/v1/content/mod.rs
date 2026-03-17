/// 文档内容模块
///
/// 提供云文档内容的获取功能，包括文档详细信息、文本内容等。
pub mod get;

// 使用通配符导出所有子模块
// get 模块显式导出
pub use get::{
    GetDocsContentRequest,
    GetDocsContentResponse,
};

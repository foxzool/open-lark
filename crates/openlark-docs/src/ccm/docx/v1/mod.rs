/// 文档处理 v1 API 模块
///
/// 提供文档处理和聊天公告相关的API功能，包括文档创建、内容管理、区块操作等。
pub mod chat;
pub mod document;

// 使用通配符导出所有子模块
pub use chat::*;
pub use document::*;

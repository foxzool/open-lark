//! CCM Docs V1 Docs API 模块
//!
//! 云文档内容管理API，包含2个API：
//! - search_object: 搜索云文档
//! - meta: 获取元数据

// 导出所有API函数
pub mod search_object;
pub mod meta;

// 重新导出主要的API函数，方便外部使用
pub use search_object::search_object;
pub use meta::get_meta;
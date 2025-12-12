//! CCM Doc V1 API 模块
//!
//! 旧版文档相关API实现，包含6个API：
//! - create: 创建旧版文档
//! - meta: 获取旧版文档元信息
//! - sheet_meta: 获取旧版文档中的电子表格元数据
//! - raw_content: 获取旧版文档纯文本内容
//! - content: 获取旧版文档富文本内容
//! - batch_update: 编辑旧版文档内容

// 导出所有API函数
pub mod create;
pub mod meta;
pub mod sheet_meta;
pub mod raw_content;
pub mod content;
pub mod batch_update;

// 导出模型定义
pub mod models;

// 重新导出主要的API函数，方便外部使用
pub use create::create_document;
pub use meta::get_document_meta;
pub use sheet_meta::get_sheet_meta;
pub use raw_content::get_raw_content;
pub use content::get_document_content;
pub use batch_update::batch_update_document;

// 重新导出模型
pub use models::*;
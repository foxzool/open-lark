/// Baike 词条管理模块
///
/// 提供免审词条的创建、获取、列表、搜索等功能。

pub mod create;
pub mod extract;
pub mod get;
pub mod highlight;
pub mod list;
pub mod r#match;
pub mod search;
pub mod update;

// 显式导出 - 避免使用 glob reexport
pub use create::{CreateEntityReq, CreateEntityRequest, CreateEntityResp};

pub use extract::{ExtractEntityRequest, ExtractEntityResponse};

pub use get::{GetEntityRequest, GetEntityResp};

pub use highlight::{HighlightEntityRequest, HighlightEntityResponse};

pub use list::{ListEntityRequest, ListEntityResp};

pub use r#match::{MatchEntityRequest, MatchEntityResp};

pub use search::{SearchEntityRequest, SearchEntityResponse};

pub use update::{UpdateEntityReq, UpdateEntityRequest, UpdateEntityResp};
